# Pitch CV, VCO and square shaper

Implemented in `crates/dw303-osc`, using the binding stage API in
[contracts.md](contracts.md). This is a deterministic, physically motivated
first implementation, **not yet a schematic-validated TB-303 replica**.
The service notes are the authority; the available scan did not support reliable
component identification during this implementation. Estimated values have no
invented reference designators. All outstanding checks appear below.

## Nodes and equations

`PitchCv` maps sequencer note zero to 0 V; each semitone adds 1/12 V.
`cv_for_note` is available both as a free function and an associated function.
Without slide, the DAC clamps the slide capacitor voltage. With slide,
`V[n+1] = Vtarget + (V[n] - Vtarget) exp(-dt/(Rs Cs))`.
This is the exact sample-held RC solution, giving 60 ms nominal time constant
independent of interval or rate. It differs from TPT only in discretisation:
there is no integration error for piecewise constant sequencer DAC voltage.

The matched transistor pair uses `Vt = (k/q) T` and
`Iout = Iref exp(deltaVbe/Vt)`. At nominal temperature, the calibrated input
attenuation is `Vt_nominal ln(2)` volts of differential Vbe per volt of CV.
The linear TUNING rotation adds `(rotation - 0.5) * tuning_octaves` to CV;
nominal endpoints are ±6 semitones. Temperature affects octave tracking through
Vt; a full thermistor/temperature-compensation network is not modelled.

The charge core solves `C dV/dt = Iout (1 + V/Vearly)` analytically between
low and high thresholds. The reset is an exponential discharge to an asymptote
slightly below the low endpoint; it reaches the low endpoint exactly after the
chosen finite reset interval. The reset asymptote and duration avoid a voltage
jump at either boundary, but derivative corners remain.

Let `r` be the reset fraction and
`Qcycle = C Vearly ln((Vearly+Vhigh)/(Vearly+Vlow)) / (1-r)`.
The period is `Qcycle/Iout`. Reference current is calibrated once from nominal
components and `reference_hz`; component variations then change frequency.
`charging_current_a` exposes that current. The oscillator stores cycle phase,
an exact coordinate of the capacitor's periodic analytic voltage solution,
rather than numerically integrating a ramp and introducing pitch error.
There is no transistor reset recovery memory beyond this analytic cycle.

**Reset tracking assumption:** conductance during reset scales with converter
current, keeping the reset fraction constant and octave tracking exact. At
65.406 Hz, 0.0025 cycle is 38.2 µs; at 1 kHz it is 2.5 µs. This is an unverified
reduction, not a claim that the hardware has a constant reset fraction.
Fixed reset time and imperfect tracking need SPICE/hardware calibration.

The continuous charge/reset solution is projected analytically into its Fourier
coefficients. This is equivalent to convolution with a band-limited kernel,
covering both reset edges and slope discontinuities. It is an alternative to
BLEP/BLAMP, not a naive oscillator with a low-pass added afterward. Up to 256
harmonics are summed using a sine/cosine recurrence. Harmonics pass unchanged
below 0.40 × stage rate, fade with a raised cosine, and vanish at 0.48 × rate.
A second smooth taper spans harmonics 224–256. The saw output includes its DC
bias: approximately 0.5–5 V before reconstruction overshoot. These endpoints
fold the source-follower gain/offset into the waveform-node calibration.

The square is generated **from the saw voltage**, with no phase comparator.
Input coupling capacitor voltage is stored explicitly. Its high-pass voltage
drives a PNP base around a DC bias. With emitter at the supply, collector KCL is

```
Vc/Rc = alphaF Is [exp((Ve-Vb)/Vt)-1]
       - Is [exp((Vc-Vb)/Vt)-1]
```

A 32-iteration bisection between ground and supply covers forward operation,
cutoff and reverse base-collector conduction in saturation. The exponential
arguments are bounded for numerical safety. A collector capacitor rounds edges;
a second coupling capacitor removes DC and tilts the plateau. RC states use
exact sample-held solutions. The collector equation assumes stiff base/emitter
drive; finite base current and emitter/buffer loading require a fuller model.
`SquareShaper` is public for stage-level validation; `Vco` owns one internally.

## Lifecycle and real-time behaviour

`new` supplies a usable 48 kHz initial state. `prepare` accepts finite rates in
44.1–384 kHz, preserves states, and returns `InvalidSampleRate` otherwise.
`reset` clears phase/capacitors and diagnostics without changing rate.
No unsafe code, heap allocation, panics, or unbounded iterations occur in ticks.
The production library is `no_std` and uses only the existing `libm` dependency.
Invalid component values fall back individually to nominal. Nearly coincident
saw thresholds also fall back, avoiding singular charge coefficients.

Non-finite signal/knob inputs are replaced with zero and counted through
`diagnostics()`. Read-only `frequency_hz` sanitises without incrementing counters.
Tuning is clamped to the panel range; CV is limited to ±16 V. Rendering limits
frequency to 0.45 × stage rate and counts affected samples. Static frequency
and charging-current queries retain the uncapped physical expo law.
Tiny RC state values below 1e-24 V are flushed to zero, well above denormal scale.
The Fourier arrays are built once in `new`; ticks use bounded stack/state work.
There is no runtime performance-budget benchmark yet; this solution costs more
CPU than a small BLEP kernel, especially for bass notes.

## Validation evidence

Run only the assigned package, offline:

```
cargo test --offline -p dw303-osc -- --nocapture
cargo clippy --offline -p dw303-osc --all-targets -- -D warnings
cargo --offline fmt -p dw303-osc -- --check
```

Tests cover DAC mapping for every u8 index; upwards/downwards slide intervals;
exact RC timing at 44.1, 48, 96, 192 and 384 kHz; measured saw pitch within 1 cent
for notes 0, 12, 24, 36, 48 and 60 at those rates; static octave ratios;
measured tuning endpoints; thermal-law tracking; timing-capacitor sensitivity;
Ebers–Moll collector KCL residual; asymmetric saw rise and charging curvature;
non-50% square duty, varying plateau and DC rejection; square input RC timing;
finite outputs at extreme CV/tuning and every supported test rate; invalid
inputs/rates/components; tiny-state flushing; a 2,000,000-sample run at 48 kHz
(41.7 seconds) and deterministic reset.

**Saw alias metric:** 65,536 samples at 192 kHz, fundamental bin 341,
999.0234375 Hz. The nearby coherent frequency avoids window leakage. A radix-2
FFT implements ideal 4× decimation by restricting evaluation to bins below
24 kHz. Non-harmonic energy summed over those bins, relative to fundamental
energy, measures **−243.32 dB**, passing the −80 dB requirement. Minimum
fundamental and second-harmonic energies guard against silence or sine-only
substitutes. This is a numerical floor for a steady analytic harmonic source;
it does not establish that the complete instrument aliases at that level.
No SPICE or hardware waveform agreement is claimed.

## Complete VERIFY register

Each row corresponds to an explicit `VERIFY:` code comment. Values below are
estimates pending confirmation, even when they resemble standard part values.

| Item | Implemented estimate / outstanding check |
|---|---|
| `slide_resistance` | 100 kΩ; identify switch path and resistor(s) |
| `slide_capacitance` | 0.6 µF; confirm capacitor and resulting 60 ms RC |
| `reference_hz` | 65.406391325 Hz; confirm note-zero/octave and trim calibration |
| `tuning_octaves` | 1 octave total; confirm wiring, range and pot taper |
| `timing_capacitance` | 10 nF; identify actual timing capacitor |
| `saw_low` | 0.5 V; confirm reset minimum and waveform terminal offset |
| `saw_high` | 5 V; confirm threshold and waveform terminal scaling |
| `early_voltage` | 60 V; fit charging transistor output resistance |
| `reset_fraction` | 0.0025; verify frequency dependence of reset duration |
| `reset_time_constants` | 4.6; verify reset conductance/asymptote and recovery |
| `square_supply` | 5.333 V; confirm emitter supply/node |
| `square_bias` | 0.60 V below emitter; confirm DC base network |
| `square_drive` | 0.40; confirm saw-to-base attenuation |
| `square_input_resistance` | 100 kΩ; confirm effective input coupling load |
| `square_input_capacitance` | 100 nF; identify coupling capacitor |
| `square_collector_resistance` | 22 kΩ; identify collector load |
| `square_collector_capacitance` | 150 pF; fit junction/parasitic and following-stage loading |
| `square_output_resistance` | 100 kΩ; confirm waveform-switch/filter loading |
| `square_output_capacitance` | 220 nF; identify output coupling capacitor |
| `transistor_is` | 1e-14 A; fit an appropriate 2SA733 part model |
| `transistor_alpha` | 0.995; fit gain, particularly near saturation |
| DAC mapping (`cv_for_note`) | Ideal 1 V/oct; verify absolute offset/resistor-ladder errors |
| Square topology reduction (`SquareShaper`) | Verify stiff base/emitter assumption; omitted base-current/emitter/buffer impedances |
| Reset topology reduction (`Vco`) | Verify current-tracking reset conductance against actual reset network |
| Source follower (`Vco`) | Verify gain/offset represented by endpoints; no separate buffer state |
| Charge curvature (`Vco`) | Verify Early-effect equation against the actual charging network |

Nominal temperature is explicitly a chosen 298.15 K operating point, not a
claimed hardware component value. Boltzmann/charge ratio is a physical constant.

## Known weaknesses and next measurements

Every component value/topology reduction in the VERIFY register needs schematic
and SPICE checking. Absolute node levels, square duty versus pitch, reset
recovery and tune/slide calibration therefore remain provisional. The ideal DAC
omits resistor mismatch. The expo law omits transistor offset, thermistor
compensation and temperature dependence of reference current. Reset and source
follower are reduced models rather than a full transient transistor network.

The 256-harmonic cap reduces very-low-note brightness, especially at high stage
rates. Static harmonic projection does not suppress sidebands from rapidly
changing pitch or tuning. The square nonlinearity introduces new high harmonics;
its alias budget is not measured or guaranteed by the saw test. The voice's
oversampling/decimation remains necessary. RC discretisation holds drive over a
sample and omits feedback from transistor base current. A base-drive/collector
SPICE sweep and per-note hardware waveforms are needed before claiming accurate
TB-303 emulation. No runtime allocation instrumentation or CPU benchmark was
run; allocation freedom follows from the fixed-storage production implementation.

Reference authority: [Roland TB-303 Service Notes scan](https://manuals.plus/m/71f1fcca7c758c8dbacf98dc21ee3002403ac1134016b6e45ee08cd3fa7e809f.pdf).
