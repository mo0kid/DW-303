# DW-303 embedded CLAP editor

The default `native-editor` feature embeds a Cocoa WebKit view using
`ravet-webview` and `ravet-surface`. Other platforms retain the DSP adapter;
`--no-default-features` also builds without native editor dependencies.
Only Cocoa embedded views are advertised. Creation, attachment, resizing and
teardown follow RAVET's gain-clap owner/callback implementation.

The panel uses the original `media/DW-303_UI.svg` geometry, gradients and outlined
lettering. SVG filter definitions/attributes and black knob discs were removed;
CSS draws shaded rotary knobs. The assembled page, including RAVET controls and
styles, remains below the 1 MiB inline limit. No remote resources load.
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

## Front-panel wire protocol

Parameter messages retain the existing exact three-field `edit`, `begin`,
`value`, `end` protocol, schema IDs and finite physical values. Panel messages
now reach the audio-thread sequencer editor:

```json
{"type":"panel","action":"button","key":"key-c","edge":"press"}
{"type":"panel","action":"button","key":"key-c","edge":"release"}
{"type":"panel","action":"mode","value":3}
{"type":"panel","action":"pattern-group","value":6}
{"type":"panel","action":"section","value":1}
```

Button objects require exactly four fields; selector objects exactly three.
Unknown keys/edges/actions, missing or extra fields, fractional/negative/out of
range values, and messages over 4096 bytes reject before queueing. MODE accepts
0..3, PATT.GROUP 0..6, section 0..1. MODE labels are Track Write, Track Play,
Pattern Play, Pattern Write; section 0/1 is A/B. PATT.GROUP emits a track detent
whose paired group also selects pattern memory. Native adapters may use
`Dw303Main::panel_action`; queue fullness is reported without waiting.

Pointer down/up, cancel/lost capture, keyboard Enter/Space down/up, blur and
assistive-technology click activation submit distinct physical edges. RUN/STOP
submits a panel edge, and RUN parameter 12 reflects the clock through CLAP's
existing parameter refresh. MODE and PATT.GROUP remain local rotary controls,
never CLAP parameter IDs 100/101. Snapshot selection drives their values and
mode/group/section/pattern/step status text. The supplied artwork's mismatched
hit-target semantic labels and Tempo/Volume bindings are corrected in JS;
layout and CSS are unchanged.

Snapshots include all 21 physical lamps with values 0=off, 1=on, 2=blink.
The page uses a shared 2.5 Hz blink phase. `selection` includes zero-based mode,
editMode, group, track, one-based pattern and step, and section `A`/`B`.
JSON construction and display work remain on the main thread.

See [panel-link.md](panel-link.md) for thread contracts, precise physical lamp
and button mappings, state v2/v1 behavior, test evidence, all VERIFY items and
remaining limitations. The JS wiring probe is
`node crates/dw303-clap/tests/panel.js`. `python3 editor/verify.py` still assembles
the exact RAVET page; native WebKit rendering currently fails under the sandbox
with launchservices/file sandbox-extension errors and a timeout. Run it outside
the sandbox to verify actual WebKit layout, controls and screenshot output.

Manual host checks remain: panel alignment at 50%, 75% and 125%, drag/Shift/Alt,
wheel, keyboard/defaults, automation gestures, restore while editing, and
close/reopen while playing or holding a panel key. Hardware/manual comparison
is still needed for PATT.GROUP detents and original LED multiplexing; the
existing engine TEMPO range remains VERIFY. No new circuit values are claimed.
