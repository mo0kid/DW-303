# Audio-thread front-panel link

`dw303-engine::link::PanelLink` is shared by an `Arc` created in CLAP
`new_shared`, the WebView callbacks, and `Dw303Engine::with_link` at activation.
`new()` remains usable by offline renderers. Activation copies the last published
memory, so deactivate/reactivate preserves edited patterns. Factory memory puts
the demo in group I, section A, pattern 1; all other memories use codec defaults.

The audio engine owns `Memory` and `Editor`. At block start it attempts a pending
restore, then consumes at most 256 panel actions. It calls `Editor::action` and
`apply_transport`; rendering calls `Editor::tick`. Preview outputs use a fixed
256-command queue, preserving attack/release pairs received in the same block.
A mode change or RUN toggle cancels obsolete preview commands. MIDI notes retain
the existing lowest-C offset; panel previews already contain sequencer pitches.
`last_command()` exposes the last command before the MIDI offset for diagnostics.

## Thread exchange

- Actions occupy 256 AtomicU32 slots. Release/acquire head/tail publication
  preserves ordering and distinct press/release edges, including counter wrap.
  There is one consumer. A one-attempt producer CAS rejects concurrent producer
  entry with `Full`; no spinning, waiting, allocation or overwrite occurs.
  Main/native callers must retry a rejected action unchanged.
- Fifty logical LEDs use two bits per lamp in two AtomicU64 words. Off/on/blink
  encode as 0/1/2. A sequence counter covers lamps and a status word, including
  mode, edit submode, group, section, pattern, track, step, and run status.
  SeqCst accesses prevent a reader accepting torn words. UI reads try at most
  three times and retain the previous coherent display on contention. Playback
  step comes from the editor's logical step lamps; stopped step is its cursor.
- Audio memory publication uses `Mutex<Memory>::try_lock` once, only after a
  change. A dirty flag retains a failed publication for the next block. Save
  blocks only on the main thread and clones fixed storage.
- Load decodes completely before applying anything. Main replaces a pending
  `Option<Memory>` and bumps an AtomicU32 generation. Audio tries both pending
  and published mutexes in the same order, publishes the adopted memory, then
  acknowledges the generation and takes it. Contention acknowledges nothing;
  adoption retries next block. Editor, sequencer and voice reset to stopped
  group I/A/1 Pattern Play. Queued panel actions follow the restore in block order.

No audio exchange path contains allocation, blocking lock acquisition, panic
calls or unbounded retry. Fixed-storage memory clones and the constructor's
main-thread snapshot are distinct from real-time processing. This is inspection
and behavior-test evidence, not an allocation-counter measurement.

## Transport and persistence

RUN (parameter 12) edits translate to RunStop press/release when the requested
state differs from the editor latch. Panel RUN goes through the same Editor.
Clock state is reflected into RUN after rendering; CLAP publishes that value
and requests a main-thread VALUES rescan. Host sync follows host transport,
including when host transport overrides a panel stop. MIDI follows its clock
messages. Host/MIDI latch changes update the editor without operating the clock.
In Host sync, stop the host before entering a stopped write sequence.

State v2 is the existing `ParameterState` record with version 2, followed by a
little-endian u32 length and the sequencer's complete versioned `Memory` codec.
The length must equal `Memory::ENCODED_SIZE`; the outer and inner codecs validate
magic, versions, values and exact lengths. V1 accepts only the original complete
parameter record and restores factory memory. Invalid input applies neither
parameters nor memory. Restore intentionally clears transient edit/held/run state
on audio adoption, so a saved RUN value is subsequently reflected as stopped
unless external transport restarts playback. Patterns and all seven tracks save;
held keys, cursors, chains queued for playback and undo history do not.

## Artwork and selector mapping

The SVG was rendered and its labels read. MODE has four positions in legend
order: Track Write, Track Play, Pattern Play, Pattern Write. JS positions their
indicators at approximately 45/75/105/135 degrees, using the existing CSS.
PATT.GROUP has seven positions, approximately -105 through 75 degrees in
30-degree increments. Every position submits `Track(0..6)`: track pairs 1/2
select group I, 3/4 II, 5/6 III, and track 7 IV, also in pattern modes. In pattern
modes the knob only chooses the group the pattern keys address: the selected
and playing pattern change when a pattern key is next pressed (queued to the
pattern boundary while running), never on the knob turn itself.

The 100/200 keys select section A/B in Normal mode; the same physical switches
are ACCENT/SLIDE in pitch mode. TIME uses DOWN=note, UP=tie, ACCENT=rest. Hold
FUNCTION with DOWN (STEP) to enter length and with UP to select triplets.
FUNCTION+C/D implements DEL/INS through the sequencer. All physical switches
send press and release, including secondary STEP/triplet aliases.

A mouse holds only one button, so Shift-click latches any panel button down
(red outline) until it is clicked again. Shift-click PATTERN CLEAR in Normal
mode clears the selected pattern in one step (it sends the pattern-key + CLEAR
chord). Other chords work by latching: latch a pattern key and click CLEAR; latch FUNCTION and click DOWN (STEP)
once per step; latch DOWN/UP while entering pitches. A latched button survives
focus changes.

Computer-keyboard shortcuts use physical key positions (`KeyboardEvent.code`),
so they follow this layout on any keyboard layout. Held keys are real holds, so
chords such as FUNCTION + STEP need no Shift-click. Shift + 1 clears the selected
pattern. Cmd/Ctrl/Alt combinations pass through; Space and Return on a focused
panel button still activate that button. Tooltips show each key.

| Keys | Buttons |
| --- | --- |
| A W S E D F T G Y H U J K | C C# D D# E F F# G G# A A# B C' (also pattern 1–8 on white keys) |
| Z X C V | DOWN (note/STEP), UP (tie/3-4), ACCENT (rest/A), SLIDE (B) |
| 1 2 3 4 | PATTERN CLEAR, FUNCTION, PITCH MODE, TIME MODE |
| Space, `.`, `,` | RUN/STOP, WRITE/NEXT (TAP), BACK |

In a plugin host, the panel must have keyboard focus, and hosts may keep keys
such as Space for their own transport.

The old HTML target IDs were misplaced relative to the artwork. JS preserves
DOM IDs and coordinates but maps `function` to CLEAR, `key-down` to FUNCTION,
`clear` to TIME MODE, `time-mode` to BACK, and `key-up/key-accent/key-slide/key-back`
to DOWN/UP/ACCENT/SLIDE. It corrects runtime accessible labels. Parameter 8 TEMPO
binds to the left rotary target `param-7`; parameter 7 VOLUME to right `param-8`.

Every physical lamp is present in snapshots:

| DOM suffix after `led-` | Source |
| --- | --- |
| run | Actual clock running state |
| pitch-c | NORMAL mode (`modes[0]`) |
| function | PITCH mode (`modes[1]`) |
| pattern | TIME mode (`modes[2]`) |
| pitch-c-sharp, pitch-d-sharp, pitch-f-sharp, pitch-g-sharp, pitch-a-sharp | Black-key `pitch` lamps |
| step-1..8 | Shared white-key pitch/selector lamps |
| step-9..10 | DOWN/UP modifiers; stopped Pattern Write FUNCTION shows 3/4 on step-10 |
| step-11..12 | Section A/B in Normal; ACCENT/SLIDE in pitch/time |

Logical `back`, `step_mode` and the auxiliary sixteen `steps` have no dedicated
lamps in this artwork; FUNCTION exposes triplet state through the shared UP lamp.
The auxiliary steps supply status text. Pattern Write submode routing follows
the engine's RUN lock, retaining Normal/Pitch/Time until stopped. LEDs remain
logical in JSON; JS resolves blink locally with a common 200 ms half-cycle
(2.5 Hz), clearing missing lamps on each snapshot.

## Validation and remaining uncertainty

- `cargo test --offline -p dw303-engine -p dw303-clap`: 7 engine and 17 CLAP tests
  pass. Tests cover concurrent queue ordering, full queues and both edges, all
  lamp fields, contended publication/adoption, reactivation, same-block preview
  attacks, four-step pitch/time entry through gate/note voice diagnostics,
  v2 round trip, v1 migration and corrupt/truncated records.
- Scoped clippy with `--all-targets -- -D warnings` and rustfmt pass.
- `node crates/dw303-clap/tests/panel.js`: dependency-free custom-wiring probe
  passes. It checks pointer/keyboard/cancel edges, artwork bindings, sections,
  FUNCTION secondary keys, four/seven rotary positions, LEDs and status. RAVET
  widget implementations are stubbed; this does not claim browser rendering.
- Release bundling and clap-validator: 34 passed, 0 failed, 0 warnings, 10 skipped.
- `python3 editor/verify.py`: assembly succeeds (under 1 MiB), but actual WebKit
  times out after sandbox-extension failures for launchservices and its temporary
  directory. Run it outside the sandbox for native rendering evidence. Its
  existing files were outside the assigned edit scope; supplementary wiring
  checks reside in the CLAP crate.

All VERIFY items in this assignment:

1. Seven PATT.GROUP detents to track pairs/groups in pattern mode need checking
   against original switch wiring/manual; the seven positions are visible, but
   pair-to-group semantics come from the sequencer's existing interpretation.
2. Shared LED multiplexing, including Normal section lamps versus pitch/time
   modifiers and recall/playback details, needs original firmware/hardware
   comparison. The mapping follows physical artwork rather than claiming an
   independently verified firmware reproduction.
3. Existing TEMPO range 40–300 BPM remains unverified (`params.rs`). No new
   analog component values or circuit topologies were introduced.

Inherited sequencer VERIFY items remain documented in `docs/sequencer.md`.
A panel event has block-start timing rather than a host timestamp. Short previews
received together become adjacent one-sample attack/release events. Published
memory can lag a changed block while the save mutex is contended; retries retain
changes, but saving before publication may capture the previous snapshot. Panel
queue overflow appears as a UI error and requires repeating rejected edges;
there is no asynchronous delivery acknowledgment or release-slot reservation.
Host scheduling is required for panel actions to run. Native WebKit rendering,
DAW automation recording, hardware lamp behavior, and close-during-held-key
lifecycle behavior still need manual verification.
