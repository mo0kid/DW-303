# DW-303 standalone (phase 8)

The macOS standalone uses the existing DW-303 engine, factory pattern memory,
front-panel editor and circuit voice. No circuit components or tuning constants
are introduced here. The unchanged `editor/index.html` is assembled with
`ravet_surface::embed_web_controls`, exactly as in the CLAP editor. The window
opens at 1227 × 589 logical points and locks its aspect ratio. RAVET's standard
Audio/MIDI title-bar popup remains enabled.

## Launch and play

From the workspace root:

```sh
cargo run --offline -p dw303-standalone --bin DW-303
```

Open **Audio/MIDI** in the title bar, choose an output device, output channel
pair(s), sample rate, buffer size and optional MIDI inputs, then press **Start**.
Devices are opened only by Start. With **SYNC = Internal** (the first-launch
default), **RUN/STOP** plays the factory demo pattern at **TEMPO**. Set Volume and
Cut Off Freq to taste. With RUN stopped, MIDI notes play the monophonic voice;
last-note priority, overlapping-note slide and velocity accent come from the
shared sequencer. Choose **SYNC = MIDI clock** for MIDI Start/Continue/Stop and
Timing Clock. The Host sync option remains in the shared page, but this app
supplies no DAW host transport; use Internal or MIDI clock.

Build a local ad-hoc signed bundle:

```sh
python3 scripts/bundle-app.py
open target/standalone/release/DW-303.app
```

The built bundle is
`target/standalone/release/DW-303.app`.
`--profile debug` produces `target/standalone/debug/DW-303.app`. The script gets
the actual executable path from the package-scoped Cargo build, so a configured
Cargo target directory is supported. The bundle identifier is
`co.djwaudio.dw303.app`; it embeds the page in the binary and needs no separate
media assets. The script verifies the signature. Developer ID signing and
notarisation are separate release work; this is a local ad-hoc bundle.

## State and lifecycle

Closing the window or using Cmd-Q stops/drops the audio stream on the control
thread, closes MIDI connections, applies remaining queued edits through the
hardware-free renderer and saves params plus all pattern/track memory to
`~/Library/Application Support/DW-303/state.bin`. It uses the same product ID,
parameter records, v2 memory payload and length validation as CLAP; v1 parameter
state restores factory memory. The bounded read rejects oversized state.
Writes use a same-directory temporary file, file sync, atomic rename and
parent-directory sync. Save errors reach stderr. A bad restore shows an editor
error and starts with factory defaults. RUN is cleared on launch so restored
state never starts playback automatically. Other parameters, including SYNC,
restore as saved.

When audio is stopped, a prepared offline renderer services edits and snapshots
on the main thread; pattern editing works before selecting a device. This also
advances Internal playback silently if RUN is enabled. Starting/stopping or
changing audio devices recreates the DSP at the selected rate and resets voice
history and sequencer position; parameter values and pattern memory survive.
Audio/MIDI preferences belong to RAVET's standard popup and do not auto-start
connections on launch. Abrupt termination cannot run quit-time persistence.

## Callback design and limits

Parameter changes use a 256-slot atomic single-producer/single-consumer FIFO.
Values are schema-validated and canonicalized before publication; full queues
reject whole edits and show an error. Gesture ownership is validated on the
control thread. Panel edges use the existing bounded `PanelLink` queue. The
snapshot exposes the same `params`, `leds`, `selection`, `error` and `pending`
fields as CLAP, plus RAVET's settings inventory/status fields. Protocol code is
kept within this crate because engine and CLAP are owned by parallel assignments;
those files were left unchanged. A small bounded JSON codec avoids introducing
any registry dependencies. JSON allocations occur only on the control thread.

RAVET's device adapter subdivides output into at most 512-frame DSP blocks.
The f32 mono engine output is duplicated to stereo and routed to the selected
pairs; unselected channels are silent. Scratch is prepared before playback.
Each block drains at most 256 parameter edits and 64 MIDI packets, round-robin
across at most 16 selected inputs. MIDI ring overflow resets the engine and
boundedly discards queued packets to prevent a stuck note. Denormals use
`ScopedDenormalMode`. The standalone callback introduces no allocation,
blocking lock, I/O or unbounded loop.

**Strict no-lock requirement remains unmet in the inherited engine:**
`PanelLink::adopt_memory` and `publish_memory` call `Mutex::try_lock` for memory
restore/publication. These never wait, but they are still lock operations. Fixing
that exchange requires an engine change outside this assignment. No claim is
made that the entire composed callback is lock-free or allocation-instrumented.

MIDI timestamps currently arrive at frame zero of the next DSP block, following
the RAVET tutorial. There is bounded draining but no timestamp-to-sample mapping;
MIDI clock jitter and backlog need device validation. Hardware hotplug/reconnect,
physical MIDI/audio output, latency and callback deadlines have not been tested.
Audio input/MIX IN and MIDI output are not provided by this standalone host.
RAVET currently supplies the generic “Quit RAVET Examples” menu label; Cmd-Q closes
the owned window and runs this app's save path.

## VERIFY register

Inherited editor mappings copied from CLAP retain their uncertainty comments:

- `protocol::parse_panel`: seven selector detents map tracks 1/2, 3/4, 5/6, 7
  onto pattern groups; compare original pattern-mode switch wiring.
- `protocol::panel_lamps`: shared lamps follow artwork positions; compare
  firmware multiplexing with hardware.

There are no new circuit-topology or component-value VERIFY items. Circuit
uncertainties remain in the underlying stage documentation.

## Verification (2026-10-11)

- `cargo build --offline -p dw303-standalone`: passed.
- `cargo test --offline -p dw303-standalone`: 6 passed. Covers state v2/v1,
  corruption and atomic replacement, parameter validation/FIFO/full/wraparound,
  10,000 concurrent edits, panel/gesture protocol, stereo rendering, synthetic
  MIDI notes/clock/transport and dropped-note-off recovery. Opens no devices.
- `cargo clippy --offline -p dw303-standalone --all-targets -- -D warnings`:
  passed.
- `cargo --offline fmt -p dw303-standalone -- --check`: passed.
- `cargo test --offline -p dw303-clap`: 17 passed; existing behavior/tests intact.
- `python3 scripts/bundle-app.py`: release bundle built; ad-hoc codesign strict
  verification passed.
- `cargo run --offline -p dw303-standalone -- --ui-test`: attempted hidden
  main-thread WebKit/settings/parameter/RUN fixture, **failed** because the
  workspace sandbox denied WebKit cache directories and LaunchServices/XPC
  services. No real audio/MIDI devices were opened, and user instrument state is
  neither read nor written by this fixture. Native editor behavior remains
  unverified in this environment. Run the same command outside the sandbox for
  the hardware-free UI check.
