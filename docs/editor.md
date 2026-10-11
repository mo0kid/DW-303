# DW-303 embedded CLAP editor

The default `native-editor` feature embeds a Cocoa WebKit view using
`ravet-webview` and `ravet-surface`. Other platforms retain the DSP adapter;
`--no-default-features` also builds without native editor dependencies.
Only Cocoa embedded views are advertised. Creation, attachment, resizing and
teardown follow RAVET's gain-clap owner/callback implementation.

The panel uses the original `media/DW-303_UI.svg` geometry, gradients and outlined
lettering. SVG filter definitions/attributes and black knob discs were removed;
CSS draws shaded rotary knobs. The assembled page, including RAVET controls and
styles, is 432,132 bytes, below the 1 MiB inline limit. No remote resources load.
Default size is 1227×589 logical points; width ranges from 818 to 2045 and height
is rounded from the 1636:785 aspect ratio (393–981 points). The design canvas
scales uniformly. CLAP resize hints advertise that ratio; size adjustment is
bounded and idempotent, including rounded endpoint sizes.

## Parameters and gestures

| ID | Control | Physical values |
|---|---|---|
| 1–6 | Tuning, Cut Off Freq, Resonance, Env Mod, Decay, Accent | 0–1 panel rotation |
| 7 | Volume | 0–1 panel rotation |
| 8 | Tempo | 40–300 BPM, default 120 |
| 9 | Waveform | 0 saw, 1 square |
| 10 | Sync | 0 Internal, 1 Host, 2 MIDI clock |
| 11 | Quality | 0 Economy, 1 Full |
| 12 | Run / Stop | 0 stopped, 1 running |

Parameter metadata and defaults come from the engine schema. Circuit taper
remains in the circuit crates. Knobs use `RavetControls.Knob`, with its `--angle`
scaled from the framework's 270° to 300° visual travel. Vertical drag, Shift fine
drag, double-click default and keyboard navigation are framework interactions.
A capture listener maps Alt to Shift for fine drag; wheel commits use the same
knob transport, with Shift/Alt ten times finer. All knobs have ARIA labels and
framework slider roles; switches/selects and sequencer buttons use native
accessible HTML controls. Assistive-technology behavior has not been audited.

`ParameterAttachment` validates each edit and owns continuous gestures. A
32-entry SPSC atomic queue carries complete edits or continuous begin/value/end
commands. One gesture owns the producer at a time, and one slot is reserved for
its end. Consumers perform at most 96 iterations, retain rejected output events
and request another params flush (or process if HostParams is unavailable).
Values apply to the engine/shared state only after the host accepts the CLAP
value event. Audio process/active flush drains editor edits before host input;
inactive flush invalidates queued edits when incoming host values change.
Audio reflection requests a main-thread VALUES rescan. Restore generations drop
stale edits while closing any already accepted begin. Per-parameter generation
words and one-shot compare/exchange stop stale audio reflection overwriting a
restore. No heap activity, locks or unbounded retry loops occur in this queue or
its audio consumer. UI JSON, HTML assembly and snapshots allocate on the main
thread. End delivery after destruction still depends on host scheduling.

The Rust `Dw303Main` API exposes `edit_parameter`, `begin_parameter_gesture`,
`update_parameter_gesture`, and `end_parameter_gesture` for future native control
surfaces. Host/editor values refresh through the WebView snapshot callback;
refresh does not submit edits. Individual parameter reads are atomic; a set of
values is not a simultaneous audio-block snapshot.

## Wire protocol and sequencer placeholders

Parameter messages contain exactly three fields:

```json
{"type":"value","id":2,"value":0.7}
```

Types are `edit`, `begin`, `value`, `end`. IDs must belong to the schema; values
must be finite after conversion to f32. Messages over 4096 UTF-8 bytes, malformed
JSON, unknown IDs/types, missing/extra fields and nonnumeric values reject.
Attachments clamp/snap finite physical values using schema rules.

`window.panel({action, key?, value?})` posts `{type:"panel", ...}`. Rust validates
and safely ignores these messages. Actions are `pitch`, `function`,
`pattern-group`, `mode`; key strings are at most 32 ASCII alphanumeric/hyphen
characters; values are integers 0–16. Unknown fields/actions reject. MODE and
PATT.GROUP use local rotary controls with IDs 101/100 internally; they never
send these IDs to the CLAP parameter interface.

Stable button IDs are `run-stop`, `function`, `pitch-mode`, `time-mode`, `tap`,
`clear`, and `key-` followed by `c`, `d`, `e`, `f`, `g`, `a`, `b`, `c-up`,
`c-sharp`, `d-sharp`, `f-sharp`, `g-sharp`, `a-sharp`, `down`, `up`, `accent`,
`slide`, or `back`. Rotary IDs are `pattern-group` and `mode`.

Snapshots have `params` keyed by decimal parameter ID, `leds` keyed by lamp
name, optional `error`, and `pending`. Lamp values are 0/1; the page also accepts
booleans. SVG lamp DOM IDs use `led-<name>`: `run`, `function`, `pattern`,
`pitch-c`, `pitch-c-sharp`, `pitch-d-sharp`, `pitch-f-sharp`, `pitch-g-sharp`,
`pitch-a-sharp`, and `step-1` through `step-12`. Rust currently supplies RUN only.
RUN reflects parameter 12, including host automation; it is not measured
sequencer playback telemetry. The remaining sequencer actions and indicators
await the edit-state-machine bridge.

## Verification, 2026-10-11

- Package unit tests pass: protocol validation/bounds, all twelve parameter
  round trips through CLAP events and state, continuous gestures, full-generation
  ID preservation, backpressure, overflow/end reservation, restore cancellation,
  stale reflection protection, concurrent queue ordering, inline page bound and
  resize idempotence.
- `rustfmt --edition 2024 --check crates/dw303-clap/src/{lib,gui,editor}.rs` passes.
- `cargo clippy --offline -p dw303-clap --all-targets --no-deps -- -D warnings`
  passes for this crate. The required command without `--no-deps` currently
  fails on three `chunks_exact_to_as_chunks` warnings in the concurrently edited
  `dw303-osc/src/lib.rs:595–597`; those paths are outside this assignment.
- Default and no-default-features debug builds pass; release bundling via
  `python3 scripts/bundle-clap.py` passes and ad hoc signs the macOS bundle.
- Full `/tmp/clapval/clap-validator validate target/clap/release/DW-303.clap`
  finishes with **33 passed, 1 failed, 10 skipped, 0 warnings**. Parameter
  conversions now pass after fixing RUN's discrete display strings. The remaining
  `process-varying-sample-rates` failure is activation at 8000 Hz: 4× oversampling
  produces 32000 Hz, rejected by the circuit crates' 44100–384000 Hz preparation
  bounds. Correct rate support requires work outside this assignment; no rate
  substitution or silent audio fallback was introduced to mask it.
- Inline SVG parses as XML and both assembled JavaScript scripts pass Node's
  syntax check. Actual rendering is **not verified** in this sandbox: hidden
  WebKit could not create native sandbox extensions, headless Firefox aborted,
  and computer-use access to Firefox was unavailable. No screenshot is claimed.

For an unsandboxed hidden WebKit render and keyboard/default/wheel/snapshot
probe, run `python3 editor/verify.py`. It creates only temporary compiled/HTML
artifacts and writes `/tmp/dw303-editor.png`. To assemble a standalone static
preview for a normal browser, run `python3 editor/verify.py --assemble-only` and
open `/tmp/dw303-editor.html`; static parameter edits explicitly report the absent
host bridge. No `editor/preview.html` is required.

Manual host verification still required: open the bundle in a Cocoa CLAP host,
check artwork/control alignment at 50%, 75% and 125%, exercise drag/Shift/Alt,
wheel, keyboard and defaults, record automation with begin/value/end gestures,
restore state during a drag, and close/reopen the editor while transport runs.
Check RUN/STOP and waveform/sync/quality host updates and RUN lamp refresh.

## VERIFY items and weaknesses

- **VERIFY:** MODE detent names/order and the provisional six-position mapping
  against the TB-303 owner's manual before sequencer wiring. PATT.GROUP's four
  positions reflect the plan; both controls currently emit ignored placeholder
  actions and do not represent persistent sequencer state.
- Existing engine **VERIFY:** original TEMPO range (40–300 BPM). The editor
  deliberately follows the existing schema and introduces no circuit values.
- Sequencer editing, pattern state, other lamps, human visual/keyboard/AX checks
  and real DAW automation/lifecycle evidence remain pending. RAVET's synchronous
  JS post acknowledges submission, not eventual host acceptance; queue errors
  appear on the following snapshot. A host that refuses events indefinitely
  prevents delivery. All controls rely on the host's Cocoa main thread/run loop.
