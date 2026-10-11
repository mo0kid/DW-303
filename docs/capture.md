# TD-03 audio-output measurement rig

This rig measures the closed unit's audio output. The TB-303 reconciled netlist
and circuit contract remain the authority; TD-03 differences calibrate panel
laws and expose model weaknesses. No internal CV/gate measurements are implied.
No hardware capture has been performed or validated during this implementation.

`tools/dw303-capture` uses RAVET's `ravet-device` CPAL f32 input and its prepared
65,536-frame SPSC ring, plus `ravet-midi-device` output. Default/optional audio
codec features are disabled. The explicitly requested cached TOML parser is the
only direct registry dependency. No network or new downloaded dependency is used.
There is no custom audio callback: RAVET publishes complete input frames without
file I/O or locking. A separate writer selects the requested channel and streams
signed PCM24 mono WAV. The timer thread sends MIDI and never runs on the audio
callback. Native CPAL/CoreMIDI guarantees remain those of RAVET, not this tool.

## Operator workflow

Use Python 3.11+ (stdlib `tomllib`) and NumPy for analysis. Build only the capture
package offline. Device enumeration is read-only; recording requires explicit run.

```sh
cargo build --offline -p dw303-capture --release
cargo run --offline -p dw303-capture -- list
cargo run --offline -p dw303-capture -- loopback reference/captures/manifest.toml \
  --audio-device AUDIO_ID --channel 1 --midi-out 'EXACT OUTPUT NAME' \
  --rate 96000 --out reference/captures/2026-10-11-calibration/
cargo run --offline -p dw303-capture -- run reference/captures/manifest.toml \
  --capture vcf-page7 --audio-device AUDIO_ID --channel 1 \
  --midi-out 'EXACT OUTPUT NAME' --rate 96000 \
  --out reference/captures/2026-10-11/
```

IDs are exact backend-local audio IDs from `list`; MIDI names must match exactly
and be unique. Channels are 1-based. The interface must support f32 input at
96 kHz on the selected channel; the file is quantized to 24-bit. It does not
increase the interface's effective resolution. No resampling or device fallback.

1. Fill `[session]` in the manifest: serial, firmware, exact model label, power
   supply, interface calibration and neutralisation of unit-specific features.
2. Warm up for at least 30 minutes. Use the same supply and output level every
   session. Disable distortion and any non-TB-303 features; confirm the manual.
3. Independently measure interface gain/response with a known source or interface
   analog loopback. Do not infer the interface response from the synth itself.
   Record gain in dBFS per volt and response as a JSON file containing ascending
   `frequency_hz` and matching `gain_db` arrays. Avoid clipping on accented notes.
4. Run `loopback` (the name describes MIDI-to-audio onset calibration; no audio
   output device is opened). Keep the input quiet during pre-roll. It sends an
   unaccented held note through the reference unit, detects its first onset, and
   reports the combined MIDI/voice/input delay in the JSON sidecar. Repeat several
   times to estimate its median and jitter. A null onset means no calibration.
5. For normal runs, optionally pass `--latency-ms VALUE` using that median to
   advance each send deadline while retaining the intended sample-grid timestamp.
   Valid compensation range is ±500 ms; loopback itself requires zero compensation.
   `--input-latency-ms VALUE` records a separately measured input-only estimate;
   absent input-only evidence this field is null. Never call the combined delay
   the interface input latency.
6. Use `session` for a batch, or `run --capture ID` for a single measurement.
   A session asks metadata and temperature once and prints only changed controls
   before each panel confirmation. Use a printed template: legend units are 0–10
   and clock positions are approximate. Stored-pattern sessions ask for pattern
   programming and MIDI clock reception once before recording starts.
7. Preserve WAV/JSON together. Files are created exclusively and never overwritten.
   A failed run can leave a partial WAV or empty sidecar. `session --resume`
   preserves incomplete/invalid existing pairs under `incomplete/ID-attempt-N/`
   before retrying; single runs still require a fresh output path. `valid=false`, clipping, missing onset, FIFO drops,
   backend errors or stalled input require investigation/repetition. Dropped
   frames are counted and invalidate timing rather than being concealed.

## Practical measurement sessions

Preview the essentials, then record the same selection:

```sh
cargo run --offline -p dw303-capture -- plan reference/captures/manifest.toml \
  --select '*' --session-file reference/captures/sessions/01-essentials.toml
cargo run --offline -p dw303-capture -- session reference/captures/manifest.toml \
  --select '*' --session-file reference/captures/sessions/01-essentials.toml \
  --audio-device AUDIO_ID --channel 1 --midi-out 'EXACT OUTPUT NAME' \
  --rate 96000 --out reference/captures/2026-10-11-essentials/ --resume
```

Quote wildcard selectors so the shell does not expand them. `--select` accepts
`*` and `?` globs or comma-separated ID prefixes, for example
`'pitch-k24,wave-square,vcf-ring-c??'`. A selector must match the manifest;
selection is the union of selectors intersected with a session file's exact IDs.
Unknown/duplicate session IDs and empty intersections are errors. A session file
has `name = "..."` and `captures = ["manifest-id", ...]`; its list order does not
replace manifest order. `plan` requires no devices or operator input. With
`--resume --out DIR` it previews only pending captures; rate defaults to 96000
for dry runs. The plan lists panel prompts, per-entry control-change counts and
estimated duration, including each recording's 1 s pre-roll and 0.5 s post-roll,
1 s between unchanged panels, and 15 s per panel confirmation. Estimates exclude
warmup, metadata/pattern setup and device startup overhead.

The essentials contain **363 captures, 78 panel prompts, about 59.4 minutes**:
loopback; all 61 chromatic notes at centre tuning; tuning endpoints on MIDI keys
24/48/72; saw and square per octave; page-7 calibration; 11 resonance-max ringing
positions; four notes × six cutoff × three resonance positions at both normal
and accent velocities; all DECAY/ENV MOD positions; 11 ACCENT positions ×
1/2/4/8 consecutive accents × resonance min/max; all slides, three scripted gate
tempos and their stored-pattern companions; and the three behavioural patterns.
The 30-minute drift capture is excluded. `02-vcf-grid.toml` references all
**2,772** grid IDs: 231 panel prompts, about 354.2 minutes (5 h 54 min).
Use the same commands with that file.

Identical panels form one group. Entries within each group keep manifest order.
The first group contains the first selected manifest entry; subsequent groups
use a greedy nearest-panel path counting changed controls (waveform counts as
one), breaking ties by first manifest occurrence. This minimises the number of
panel confirmations to one per distinct panel, but is not a globally optimal
knob-travel path. TEMPO comes from scripted MIDI timing and is not a manual knob
in this matrix. Clock-only captures need the one stored pattern programmed at
session setup; sessions containing different stored-pattern instructions must
be split. Instructions still print without an extra per-capture confirmation.

Metadata edits apply to sidecars for this invocation, leaving the manifest and
curated file unchanged. Enter preserves each manifest field; blank temperature
is null. Each resumed invocation asks metadata again because conditions can
change. The initial pending panel always prints all controls. Later panels show
only `old → new` values; unchanged panels continue after the settle gap.
`n/N` counts pending captures, and the remaining estimate includes future panel
confirmations. The loopback entry always runs without latency compensation;
its result is reported but is not automatically applied to later captures.
Manifest ordering may put it after pitch captures: run standalone `loopback`
first when compensation is needed for the whole session.

Resume skips only a valid sidecar and a complete matching PCM24 mono WAV, with
matching ID, panel settings, rate and expected frame count. RIFF/data lengths,
format fields and physical file size are checked. Empty/truncated/malformed
sidecars, invalid captures and incomplete WAVs remain pending. Older sidecars
without `complete` are supported when valid and structurally complete. Checks
are conservative for edited sidecars (panel JSON must retain the tool's canonical
format) and do not checksum sample contents. A plan never moves files. Recording
with resume archives any pending existing pair; recording without resume refuses
existing paths. The tool retains exclusive creation for new measurements.

Interactive sessions use the macOS `stty` utility and a terminal input reader,
without unsafe signal code or additional dependencies. Keyboard Ctrl-C requests
exit after the **current capture's full duration**, allowing its WAV header and
sidecar to finish normally and releasing notes/clock; while awaiting input it
exits immediately. Terminal settings restore on normal/error exit. Piped input
is rejected for sessions. External SIGINT, forced termination and terminal loss
are not covered by this keyboard handling; use resume to recover incomplete
pairs. Very long custom selections may take a long time to finish the current
capture. Native audio/MIDI streams reopen for each capture; startup overhead and
hardware interruption behaviour remain unmeasured.

## Matrix and measurement interpretation

The checked-in TOML is fully expanded: 3,416 unique entries, including calibration.
Every entry contains the complete panel state, tempo, duration, clock selection,
and notes `[start_quarter_beat, MIDI_key, duration_quarter_beats, velocity]`.
Notes use exact wire velocities (normally 80 unaccented and 120 accented); if
changing the configurable threshold, verify those values still straddle it or
edit the note rows. Channel is configurable at the top (1–16); overlapping note
lifetimes implement slide. Adjacent notes send Off before On. Ties use one longer
note, rests omit notes. Clock mode sends Start before the first 24-PPQN pulse and
Stop at capture end. Notes can accompany clock, or a manually stored pattern can
be played with clock alone. Clock transport is off for ordinary note-mode captures.

| Family | Coverage |
|---|---|
| Pitch | Every chromatic MIDI key 24–84 at tuning 0, .25, .5, .75, 1; 30-minute held-note drift |
| Waveforms | Saw/square on C1–C6, cutoff max/resonance and env mod min |
| Ringing | 21 cutoff positions, resonance max, low C; page-7 centre cutoff/max res/min env mod, decay, accent/saw/C |
| Harmonic grid | Six C octaves × 21 cutoff × 11 resonance × normal/accent velocity; accent knob max |
| Envelopes | Eleven decay and env mod positions; long holds for amplitude/spectral tracking |
| Accent | Eleven accent levels × 1–8 consecutive accents × min/mid/max resonance |
| Slide | 1, 5, 12, 24 semitones both directions; 0.2-beat overlap |
| Gate | 80/120/160 BPM, scripted short/tied/rested notes and manual stored-pattern companions |
| Behaviour | Classic, accent/slide, rest/tie schedules |

The harmonic grid samples the low-cutoff resonance-peak loss, passband reduction,
resonance strength and accent-induced saturation. Reuse the matching waveform
capture as the reference spectrum, at equal pitch, amplitude calibration and
matched envelope age. A four-second sustained gate is not a constant-amplitude
source: the VEG decays even while held. Accent transient spectra therefore need
matched windows and are not small-signal transfer functions.

The page-7 condition targets damped saw-edge ringing near the documented 2 ms
period. Lack of self-oscillation is a valid result. Compare ring frequency/decay,
harmonic ratios and passband loss before adjusting feedback. The unresolved
Q21 gain, injection ratio, transistor wiring/models discussed in
`docs/circuit/vcf-resonance.md` are circuit-stage questions; this rig introduces
no component values or unverified replacement circuit topology.

## Timing and sidecar

Schedules round each beat/clock position independently to the nominal 96 kHz
sample grid. Timer deadlines derive from one `Instant` epoch and never accumulate
relative sleep errors. Waits are cancellable in 10 ms chunks and cancellation
sends All Notes Off and Stop. There is one second of pre-roll and half a second
of post-roll. Capture/writer failure cancels the timer; input stall timeout is
five seconds. Sends record requested sample/time, latency-adjusted deadline and
actual pre-send wall time. OS jitter and MIDI serialization remain measurable.

RAVET's input API does **not** expose CPAL capture timestamps, so the first sample
epoch is estimated from first ring arrival minus the drained batch duration.
This includes backend/worker uncertainty and cannot prove hardware sample-exact
scheduling. Hardware sample rate can drift against the monotonic timer during
long runs. Audio onset recovery and repeated calibration are necessary. Exact
hardware-clock locking would require a new RAVET API and is outside this scope.

The sidecar includes complete session/settings/mapping, exact device names/IDs,
room temperature, sample format/channel/count, MIDI timestamps, onset index,
combined latency estimate, optional independent input estimate, compensation,
clipping/FIFO/error counters and validity. Onset detection uses an absolute 0.01
FS threshold or eight times the pre-roll RMS noise floor, whichever is larger.
Low-level/faded/strongly filtered notes may need manual onset recovery. For such
captures do not blindly apply that threshold's delay as a MIDI latency correction.

## Analysis

Each script accepts a PCM WAV and `--channel`, `--start`, `--seconds`; stdout is
strict JSON. Always exclude the one-second pre-roll before pitch/envelope fitting.

```sh
python3 tools/analysis/selftest.py
python3 tools/analysis/f0.py CAPTURE.wav --start 1.2 --seconds 2 --hint 130.81 --track
python3 tools/analysis/envelope.py CAPTURE.wav --start 1.1 --seconds 3
python3 tools/analysis/harmonics.py CAPTURE.wav --start 1.2 --seconds 2 \
  --f0 130.81 --reference OPEN_FILTER.wav --interface-response CALIBRATION.json
python3 tools/analysis/ringing.py CAPTURE.wav --start EDGE_TIME --seconds 0.025
python3 tools/analysis/compare.py reference/captures/manifest.toml SESSION_DIR \
  --capture wave-saw-k48 --renderer target/release/dw303-render --out REPORT.md
```

- `f0.py`: linearly interpolated positive crossings, fundamental-isolated analytic
  phase regression, and sliding-window tracks. Specify a measured/approximate
  fundamental hint to avoid harmonic ambiguity. Tracking windows smear rapid
  slides; shorten only when enough fundamental cycles remain.
- `envelope.py`: RMS and magnitude-weighted spectral centroid; fit
  `baseline + amplitude*exp(-t/tau)` after attack. Supply a separately measured
  baseline. Centroid cannot uniquely identify internal MEG/cutoff in this closed
  nonlinear system; fit centroid with care and a calibrated cutoff map.
- `harmonics.py`: Hann-window complex projections at exact harmonic frequencies;
  divide by a matching open-filter spectrum for indirect dB response. Unresolved
  reference harmonics are null. Optional independently measured interface response
  is removed; extrapolation is rejected. Square's missing harmonics cannot be used.
- `ringing.py`: positive peak interpolation, period regression and log peak-decay
  fitting, plus an API for averaging caller-identified saw edges. Crop reset
  transients, subtract baseline, and end before the next edge. Multiple ringing
  modes, noise peaks and ongoing excitation violate the single-mode assumption.
- `compare.py`: invokes the existing renderer CLI with `--hold KEY`, `--set
  ID=VALUE`, `--tempo`, rate and duration. Aligns capture using recovered onset;
  reports pitch cents, raw level delta, fundamental-normalised harmonic shape and
  amplitude tau delta in Markdown. Missing/invalid/unsupported captures get explicit
  status rows. Absolute level is raw dBFS, not volts, until calibration is applied.
  The renderer fixes hold velocity at .5 and has no arbitrary MIDI schedule input:
  accented holds, slides, clock patterns and multi-note entries cannot be rendered
  faithfully. They are reported unavailable, with no renderer extensions or proxy
  patterns. Only the pre-note-off held segment is compared. Full behavioural
  comparison remains dependent on a renderer CLI capable of replaying schedules.

## VERIFY register and known limits

1. **VERIFY:** actual reference model label (TD-03 versus TD-3), firmware/manual,
   MIDI channel, accent velocity threshold, last-note priority and overlap slide.
   The top-level mapping records estimates; notes use explicit raw velocities.
2. **VERIFY:** unit's playable MIDI range and octave nomenclature. Matrix uses
   chromatic 24–84 (C1–C6); TB-303 contract's lowest C is 32.70 Hz. Out-of-range
   notes or octave remapping must be identified from actual captures, not assumed.
3. **VERIFY:** 300-degree knob travel, 7 o'clock/5 o'clock endpoints and panel
   legends. Console clock positions assume this travel; centre is noon. Use a
   checked printed template and measure panel fractions, including extra cutoff
   and resonance half-step positions.
4. **VERIFY:** page-7 key C octave (estimated C1), ring-period interpretation and
   identical control positions on the reference unit. Preserve actual settings.
5. **VERIFY:** clock reception, external sync mode and manually entered pattern
   gate/tie/rest/accent/slide semantics on the reference firmware. Scripted note
   gates are input stimuli, not proof of original sequencer gate duration.
6. **VERIFY:** first-sample epoch estimate, native MIDI send jitter, interface
   latency and long-run audio-clock drift. Hardware sample alignment is unproven;
   input-only latency remains null without separate measurement.
7. **VERIFY:** input gain, frequency response and clipping margin, warmup, room
   temperature, supply identity and neutralisation of non-TB-303 features. Fields
   marked FILL remain operator work; the tool cannot verify a physical knob.
8. **VERIFY:** onset threshold/delay at measured levels and cutoff settings;
   combined first-note delay includes synth attack and is not pure MIDI transport.
9. **VERIFY:** 1 s inter-capture settling is an operator estimate, not a measured
   circuit time constant; validate residual envelopes/accumulated accent between
   captures on the unit. The 15 s panel adjustment estimate and total session
   durations require an operator timing check. Warmup/setup/startup are excluded.

Synthetic evidence cannot establish fidelity to hardware, circuit-stage time
constants inferred from a centroid, analog absolute voltages without calibration,
OS scheduling deadlines, or comparisons the existing renderer cannot express.

## Validation

Run offline package tests and Clippy; format only this package's source and test files:

```sh
rustfmt --edition 2024 tools/dw303-capture/src/*.rs tools/dw303-capture/tests/*.rs
cargo test --offline -p dw303-capture
cargo clippy --offline -p dw303-capture --all-targets -- -D warnings
python3 tools/analysis/selftest.py
```

Rust checks cover absolute clock rounding at 80/120/160 BPM, note ordering,
positive slide overlaps, PCM24 endpoints/NaN/RIFF padding, JSON escaping and the
full manifest. Synthetic Python checks cover crossing/phase pitch including a
harmonic-rich waveform, drift-window tracking, known exponential time constants,
centroid, harmonic amplitudes/ratios/interface correction, damped ringing at
three sample rates, edge averaging, PCM24 decoding and comparison metrics/CLI
command generation. Tests open no native devices. Only `list` is permissible
for a read-only native smoke check during implementation.

Validation executed 2026-10-11: **7 Rust tests passed**, package Clippy passed
with warnings denied, rustfmt check passed, and **7 Python test groups passed**.
The Python suite also invoked all five script CLIs on synthetic/missing-capture
fixtures. No native device was opened, and the real renderer was not executed;
renderer command construction and comparison math were checked synthetically.

Session validation extends the existing suite with grouping/greedy ordering versus
naive order, numeric panel equality, delta legend/clock output, selector errors,
curated-file ID/count/duration checks, dry-run report output, resume rejection of
invalid/truncated pairs, preservation of incomplete files, and executable plan
commands with deliberately bogus device IDs. No device is opened by these tests.

Session changes validated 2026-10-11: **13 Rust tests passed** (11 unit and two
executable dry-run tests); package Clippy passed with warnings denied and
rustfmt check passed. A pseudo-terminal keyboard Ctrl-C check exited before any
capture, restored operator terminal flags/control characters (macOS may retain
its transient PENDIN flag), and created no recording path. In-recording Ctrl-C
and native stream startup remain untested on hardware. The prior Python analysis
results above were not rerun for this session-only change.
