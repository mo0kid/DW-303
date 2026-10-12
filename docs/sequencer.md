# Sequencer core

`dw303-sequencer` is `no_std`, forbids unsafe through workspace lints, has no
allocator dependency, and depends only on the existing `ravet-foundation` path
crate. It models the original digital note/gate/accent/slide controls. Analog
slide lag, envelope capacitors and accent-sweep voltages belong to the circuit
crates; this core supplies `VoiceCommand` using the contract's relative semitone
index (Down C = 0, Mid C = 12, Up C = 24, upper C = 36). MIDI keyboard keys are
passed through as semitone indices; an engine wanting a particular absolute
MIDI-to-oscillator tuning must establish that reference separately.

## Data and original memory organisation

A bank contains four groups, each with A/B sections of eight patterns: 64
patterns. `PatternRef::index()` gives group × 16 + section × 8 + pattern.
Pitch-mode `Step` entries hold C..C', octave, accent and outgoing slide. The
independent time array holds Note, Tie (the original sustain time entry), or
Rest. Only Note consumes a pitch entry. This separation preserves the original
pitch/time entry concept for the later front-panel editor. Pattern length counts
time entries; the pitch cursor rewinds with the pattern. A tie inherits pitch,
accent and the outgoing slide latch of its sounding note. An all-tie pattern
has no sounding note and is silent. A rest cuts continuation history.

The original has seven tracks with 64 measure memories each. The owner's manual
also describes combining adjacent track memories for longer arrangements: the
maximum beginning at tracks 1..7 is 256, 256, 256, 256, 192, 128, 64 measures.
Here each `Track` owns 64 entries and length 0..64; combined-track memory borrowing
and the panel's track-to-pattern-group restrictions are left for the editor.
References may select any group, and tracks repeat at their stored length.
Each track entry stores a pattern reference and a signed transpose. Empty tracks
are silent. `tick_track` also sustains ties and slides across measure boundaries,
and supports mixed straight/triplet patterns using beat-based pattern durations.

The primary evidence is Roland's original [TB-303 owner's manual](https://cdn.roland.com/assets/media/pdf/TB-303_OM.pdf).
The full PDF exceeded the browser's fetch limit; the [transcription of the same
original manual](https://manuals.plus/m/2fc6b5c2ce3b581e621075a9ba2b69785eb7ddbbf05941809a477c3e2e4c917a)
was inspected, especially printed pages 52–57 and 65–66. These are original-manual
claims, rather than claims about the modern Roland software instrument.

## Triplets and timing

Straight mode subdivides each quarter into four sixteenths. The original triplet
mode is **three steps per quarter**, each one third of a beat: twelve time entries
span four quarter-note beats. It is not a fixed 3/4 time signature. A 3/4 pattern
can use twelve straight steps or nine triplet steps. The original manual lists
15 as the maximum triplet-mode step count; this storage API deliberately allows
1..16 in both modes as requested. A later authentic front-panel editor can impose
the original 15-step triplet limit.

`Timing::NOMINAL.gate_fraction` is 0.5: an ordinary note gates for the first half
of its step. A following Tie holds the gate through the current step; the final
unslid tie releases halfway through its own step. Outgoing slide holds the gate
through the source and its ties. The next note changes pitch with `slide=true`
and `trigger=false`. Its own outgoing slide determines whether its gate is held
again. Rest always gates off at its start. Accent follows the sounding pitch
entry, and changes at a new Note even when slid. Attack pulses last one sample.
The digital controls are reconstructed on host seeks; a mid-step start does not
create a new attack, but its gate reflects the current phase.

### VERIFY register

- `Timing::gate_fraction`: 0.5 is a best estimate of the original CPU-generated
  non-slide duty cycle. Neither a firmware disassembly nor a measured original
  gate waveform has established it here. It is adjustable for calibration.
- `VoiceCommand::slide` and the playback slide-latch rule: outgoing slide keeps
  the source gate high and enables destination pitch lag. Exact slide-switch
  assertion/deassertion timing around the CPU's gate/CV update, and inheritance
  through ties, need measurement against an original unit. No extra overlap
  milliseconds or anticipatory CV change have been invented.
- `TrackEntry::transpose`: signed -12..12 storage is a useful API extension.
  Original key-shift limits and encoding need checking against the panel/manual
  diagrams; do not treat this signed representation as a reconstruction of the
  original RAM or firmware format.

No analog component values are represented here, so there is no analog
`Components` struct or claimed schematic RC model in this digital stage.

## Clock and engine integration

`prepare` accepts finite sample rates 44.1–384 kHz; the default is 48 kHz.
Tempo accepts finite 0 < BPM <= 1000. Invalid numeric tempo/position inputs retain
the last valid clock setting and increment diagnostics, since zero is not a usable
clock tempo. Invalid externally supplied sample phases are zeroed and counted.
Other phases are clamped. Pattern lengths/pitches and transpose are defensively
bounded during playback, even if a control-thread caller skipped validation.

Internal and host timing use an absolute sample count relative to a beat anchor,
rather than repeated phase addition. Mathematical boundaries fall on the first
sample at or after the beat edge. A 1e-9-step roundoff tolerance snaps values
extremely close to a mathematical boundary. Continuous host blocks retain the
anchor when their positions agree within 1e-7 beats; therefore block partitioning
does not change edge samples. A tempo change reanchors the timeline without
clearing phase. Explicit discontinuities, changes of playing state, or unexpected
beat positions resynchronise history. Negative host positions support pre-roll.
Missing transport fields retain the previous setting. Host tempo ramps and loops
inside a callback require caller-side block splits and fresh transport contexts;
`TransportDetails` is not automatically scheduled by this core.

Apply events **before** rendering their `Event.frame_offset` sample, preserving
same-frame order. For host mode call `begin_host_block` before its first sample.
`Sequencer::event(EventData)` dispatches MIDI clocks and live notes. Then call
`tick(&pattern)` or `tick_track(&memory, track_index)` once per sample. External
clocks may supply `StepEvent` directly to `tick_at`. Use one playback mode
consistently; call `reset` when switching between pattern and track mode.

MIDI Start rewinds; the first following TimingClock starts step zero. Continue
retains subdivision/pulse position; Stop freezes it. Six pulses form a straight
step; eight form a triplet step. Subdivision edges occur on actual received clock
pulses, so clock arrival jitter is retained. Between pulses, the last measured
pulse interval estimates gate phase; before a second pulse it uses the configured
BPM (default 120). There is no PLL or speculative subdivision edge. Lost MIDI
clocks can freeze a gate; explicit Stop or engine disconnect handling must clear
it. Call handlers at the sample timestamp supplied by the adapter.

Stopped playback uses `MonoNoteInput`: fixed storage covers all 2,048 channel/key
identities; last note wins, and releases return to the most recent held note
without attack. Overlap enables slide. Default accent threshold is velocity >=100.
Repeated note-on replaces the same channel/key identity; zero MIDI velocity is
note-off. The raw MIDI path preserves integer velocity; foundation NoteOn uses
rounded normalised velocity. MIDI reset and CC120/123 release keys. Extended note
IDs, scoped expressions, sustain pedal, pitch bend and UMP are not implemented.
Held keyboard state is tracked while the sequencer runs; those attack pulses are
consumed, so stopping with keys held resumes them as gate state without a new
attack. A fresh stopped-mode NoteOn generates an attack normally.

## Versioned binary state

Every top-level record has four ASCII magic bytes and version byte 1. Integers are
single bytes, so byte order is irrelevant. Nested records omit their headers.

| Record | Magic | Bytes |
| --- | --- | ---: |
| Pattern | D3PT | 39 |
| Bank | D3BK | 2181 |
| Track | D3TR | 134 |
| Memory | D303 | 3084 |

Pattern payload: length byte, triplet byte (0/1), then sixteen pairs of packed
pitch and time bytes. Pitch low nibble is 0..12, bits 4..5 are octave 0..2, bit 6
accent and bit 7 slide. Time is 0=Note, 1=Tie, 2=Rest. Track payload: length byte
then 64 reference/transpose pairs. Reference is the flat 0..63 index; transpose
stores signed semitones plus 12 (0..24). Memory holds a bank followed by seven
tracks. Unused entries are saved and validated, preserving partially edited state.

`encode` writes into caller storage and returns the exact written size; larger
output buffers are allowed. On error a prefix may be overwritten. `decode` takes
exactly that prefix, rejects trailing/truncated data, wrong magic/version and
invalid fields, and returns only a fully validated record. The format has no
checksum: mutations that remain valid field values cannot be identified as
corruption. `validate` is a control-thread operation, not a per-sample operation.
No dependencies or root workspace files were intentionally changed.

## Validation and limits

Sixteen integration tests pass with `cargo test --offline -p dw303-sequencer`.
They cover exact edges at 73/120/137.5/241 BPM in both subdivisions with 1/7/64/512
frame partitions, internal/host equality, mid-step start, backward seek/loop jump,
stop and phase-preserving tempo change, MIDI six/eight-pulse timing and half-step
gate release, gate durations at 48/96/192 kHz, slide/tie/rest and pitch cursors,
track-boundary ties/slides and one-step looping, mixed triplet tracks, live MIDI
priority/legato/accent/channel identity, all 2,048 held notes, full-state codec
round trips, malformed/truncated/trailing records, and invalid controls.

`cargo clippy --offline -p dw303-sequencer --all-targets -- -D warnings` passes.
All assigned Rust files are formatted with edition-2024 rustfmt. Hot paths use
fixed arrays and bounded loops (at most 16 rhythm positions for a pattern, 64
track entries and up to 1,024 positions for track note-history reconstruction,
and 2,048 identities for keyboard updates); there is no allocation or unsafe code.
Track reconstruction favours correctness over CPU cost; benchmark it in the
engine before optimising. No allocator instrumentation, CPU deadline measurement,
hardware captures, firmware comparisons or analog golden tests were performed.
The timing VERIFY items, combined-track editing, original group restrictions,
within-block host automation and missing extended MIDI features remain limits.

## Front-panel editor (`edit.rs`)

`edit::Editor` adds a fixed-storage controller around the existing `Memory`.
No persisted fields changed: all four codecs remain version 1, with identical
sizes and no migration needed. Existing engine entry points are preserved.
The added `Sequencer::tick_chain_at`, `tick_track_from`, and `track_cursor`
methods supply cross-pattern history and designated-measure playback.

Send `PanelAction` selector changes and **both** button edges. Repeated down
edges are ignored. `Pitch(0..12)` and `Number(1..8)` share the identity of their
white keys; an adapter should send one representation per physical switch.
`Clear` also represents BAR RESET and D.C.; `Tap` represents WRITE/NEXT.
Semantic `Delete`, `Insert`, `Step`, `Triplet`, and `Time` actions let adapters
bind the panel's secondary legends explicitly. In Normal mode white keyboard
keys select patterns; in Pitch mode they enter pitches. DOWN/UP/ACCENT also
serve as note/sustain/rest in Time mode.

`action(&mut memory, action)` returns `EditOutput`: whether memory changed,
an optional `VoiceCommand` preview edge, and an optional transport request.
Publish changed memory through the engine's existing snapshot mechanism.
Call `apply_transport` once on the audio-side sequencer for each request.
Then `Editor::tick` can replace the engine's direct pattern/track tick when
using panel-owned transport. It advances the clock once per sample, supports
internal/host/MIDI clock, consumes preview triggers after one sample, and
retains stopped live-MIDI fallback. The preview in `EditOutput` is an alternative
integration route: do not deliver its trigger again if using `Editor::tick`.
Control and audio callers must transfer state at a safe boundary; this controller
is not a synchronisation primitive and must not be concurrently mutated.

The transition table covers stopped Pattern Write's Normal/Pitch/Time modes.
Procedures supported by the scenario tests:

- Select a pattern, hold its selector, and press CLEAR. Default straight length
  is sixteen. Hold FUNCTION and press STEP once per required rhythm position.
  FUNCTION + triplet sets twelve positions; explicit triplet length caps at fifteen.
- PITCH MODE rewinds the pitch cursor. Pitch keys write and advance, with held
  DOWN/UP choosing octave. BACK returns one position for correction. FUNCTION
  returns to Normal. Re-enter PITCH MODE and TAP to recall each pitch; hold TAP
  to toggle accent, slide or octave on the recalled pitch.
- TIME MODE rewinds the independent rhythm cursor. Note/sustain/rest advance
  it without sounding a preview. TAP checks time data without changing it.
  The last rhythm position automatically returns to Normal.
- In Pattern Play hold two selector endpoints in the same four-pattern block
  (1–4 or 5–8). The inclusive ascending range repeats. A cross-block chord
  selects only the latest key. Queue another selection while running; it takes
  over when the current **series** ends. With a single pattern this is its bar end.
  Mixed straight/triplet series preserve beat duration. Slide and tie history
  crosses pattern boundaries, including the first pattern after a queued switch.
- Track detents pair groups I/I/II/II/III/III/IV with tracks 1–7; selecting a
  different pattern group in track mode is rejected. TRACK WRITE, CLEAR, RUN,
  pattern selection, then TAP commits a measure. Hold PITCH MODE and press a
  keyboard key to shift it. A selector chain commits its patterns in order.
  CLEAR while running arms D.C.; TAP writes the last measure and truncates the
  active track there. CLEAR while stopped rewinds without erasing the track.
- While stopped hold FUNCTION and enter a measure number using selector digits
  (DOWN=9, UP=0). FUNCTION + DEL removes it. FUNCTION + INS arms insertion before
  it; RUN, choose a pattern, TAP, STOP commits the insertion and shifts following
  measures. Full tracks cannot insert beyond capacity. TRACK PLAY starts from
  the designated measure and subsequently loops through the stored arrangement.

The original-manual evidence is the Roland PDF linked above, inspected through
its OCR transcription, principally printed pages 10–16, 40–43, 54–60, and 69–72.
The supplied SVG's lamp and switch geometry was also inspected. Raster rendering
was unavailable in this environment, so secondary-symbol placement remains a
verification item. These tests encode procedures and chosen interpretations;
they do not establish firmware or hardware equivalence.

### Panel LED output

`leds(&memory)` provides C..C', selector lamps, A/B, Normal/Pitch/Time, RUN,
BACK/sign, DOWN/UP/ACCENT/SLIDE, and step-mode indications. Shared white-key
pitch and selector outputs agree, so the adapter does not drive one physical
lamp with conflicting values. Playing/selected patterns blink; other active
chain participants stay lit. Queued patterns blink. `Led::lit(phase)` resolves
Off/On/Blink using the host's common blink phase; blink frequency is not baked
into sample timing. The sixteen-element `steps` array is an additional logical
cursor view, **not** a claim that the original has sixteen step LEDs. Battery
state belongs to the host; RUN/BATTERY here indicates transport only.

### Additional VERIFY register

All new uncertainty annotations are in `edit.rs`:

1. `Leds` / `Editor::leds`: exact firmware LED multiplexing, queued selection,
   FUNCTION step-count display, recall and last-step indications, BACK/sign
   indication and blink cadence require an original-unit comparison. The current
   display is deterministic and accounts for shared white-key emitters.
2. `PanelAction::Mode`: implicit stop on MODE change implements the manual's
   instruction to stop first; original live-switch behaviour is unspecified.
3. `Button::Back`: Time-mode BACK and stopped track BACK browsing are useful
   deterministic interpretations. The original BACK/D.S. multiplexing is not
   fully reproduced; there is no persisted D.S. marker in the current model.
4. `Editor::bar_digit`: FUNCTION-held decimal measure entry and DOWN=9/UP=0
   secondary legend mapping need confirmation against a clearer panel scan.
5. `Editor::pitch`: track/pattern shift treats C as zero, C' as +12, and held
   DOWN as the -12..0 range. Exact original negative-shift procedure is uncertain.
   Pitch entry saturates at sixteen until BACK or PITCH MODE; behaviour beyond
   the sixteenth original pitch has not been established.
6. `Editor::preview_note`: preview gate remains high until key/TAP release;
   analog VEG supplies decay. Original CPU preview-gate pulse width is unmeasured.
7. CLEAR uses deterministic Mid-C/default rhythm storage rather than emulating
   undocumented stale/random pitch data on the original machine.
8. `Editor::tick`: a discontinuous host seek restarts the current chain at the
   new beat. Original hardware has no corresponding host-seek history.

The pre-existing VERIFY items above (gate duty, slide-switch timing and signed
track transpose representation) still apply. No analog component estimates were
introduced.

### Editor validation and remaining limits

`cargo test --offline -p dw303-sequencer` passes 27 integration tests: the
16 existing core tests plus 11 panel scenarios. New coverage includes pitch/time
entry, correction, recall-held flags, octave preview, length/triplet limits,
clear chords, shared key identity, duplicate downs, exhausted pitch memory,
series queues, cross-block rejection, mixed timing, inter-pattern slide/tie,
queued-switch history, eight-pulse MIDI triplets, track shift/D.C./chained write,
insert/delete, designated playback, LEDs/blink phase, preview trigger consumption,
invalid inputs, full-track bounds and codec round trips after edits.

Scoped Clippy with `--all-targets -- -D warnings` and rustfmt checks pass.
The editor allocates nothing and uses fixed-size arrays. Playback uses bounded
history reconstruction through the existing track renderer; no unbounded loop,
unsafe code, new dependency or codec change was introduced. CPU deadlines and
allocator instrumentation have not been measured.

Remaining limitations: shared-memory extended tracks (65–256 measures), persisted
D.S./sign markers, real-time metronome/TAP rhythm recording, original power-up RAM
artefacts, full decimal measure/step-count LED encoding and hardware-calibrated
lamp timing are not implemented. BAR addressing is limited to the model's 64
measures. Semantic secondary-key inputs are preferable until panel legend mapping
is verified. Memory edits and editor transients require host-side coherent
publication; undo/redo and persistent front-panel selection remain host concerns.


### Session-1 fitting round 2: MIDI versus analog release

The three replay patterns were inspected against their exact note-on/off
schedules. Last-note priority, velocity >=100 accent and overlap-driven slide
remain consistent with the available evidence, so `MonoNoteInput` is unchanged.
The rests-ties capture contains explicit MIDI note gates/rests; it does not
independently test the internal sequencer's tie-state interpretation.

The gate captures' audible release occurs roughly 3–6 ms after the former
instantaneous model release. The schematic's C72/Q36 gate-supply state now
accounts for that interval in `dw303-env`; digital note-off remains at its
scheduled sample. Pattern-classic's brightness-decay fit becomes reliable after
this analog change. See [ENV round-2 evidence](circuit/env.md) for the fit,
remaining gate-attack error and unverified device parameters. Output-only audio
and sidecar send times do not establish internal MIDI gate/CPU timing, so no
extra sequencer delay, overlap retrigger or velocity rule was inferred.
