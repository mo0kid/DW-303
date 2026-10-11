# MEG, VEG, accent, filter control, VCA and output

`dw303-env` implements all six contract stages, `Components`, `StageError`, and
per-stage non-finite input diagnostics. State and output are f64 SI quantities.
There are no dependencies beyond the existing `libm`, no unsafe code, no heap
allocation, and no iteration in the sample paths. Instances are thread-confined
(the immutable FilterControl interface counts bad inputs using `Cell`).

This is a circuit-informed reduced model, **not a verified schematic capture**.
The reference is the Roland TB-303 service notes, dated 19 February 1982:
[main board schematic](https://www.manualslib.com/manual/4220660/Roland-Tb-303.html?page=5)
and [envelope modulation notes](https://www.manualslib.com/manual/4220660/Roland-Tb-303.html?page=8).
The schematic's legible decay annotations give 200 ms minimum and 2.5 s maximum.
Those endpoints guide the estimated resistance-capacitance products; they do
not independently establish the component values chosen below. No SPICE or
hardware golden traces have been obtained for this implementation.

## State and equations

MEG stores the capacitor voltage and the remaining trigger charge-pulse duration.
A trigger closes a finite-resistance charging switch for 4 ms. The charging
solution is `V = Vs + (Vold - Vs) exp(-dt/(Ra*C))`. Then it discharges with
`tau = C*(Rmin + DECAY*Rpot)`, 0.2–2.5 s nominal. Accent bypasses the rheostat,
including when it changes during an existing tail. Charge and discharge portions
of a sample are split at the pulse endpoint. Retrigger starts from existing
charge; it does not reset the capacitor. With the nominal charging path the
4 ms pulse reaches 98.17% of the 5 V source. DECAY is estimated linear.

VEG similarly stores capacitor voltage and a 1.5 ms charging-pulse remainder.
Its attack RC is 0.33 ms and gate-high decay RC is 3.3 s. Gate low immediately
cancels charging and switches to a 10 ms discharge RC. Gate low takes priority
over trigger; a rising gate without trigger does not initiate charging. Thus
slid notes preserve the existing state when the sequencer suppresses trigger.

AccentSweep stores the smoothing capacitor voltage, even between accented notes.
The ACCENT pot is estimated linear. Its diode source is
`Vs = max(ACCENT*MEG - Vdiode, 0)`. While conducting, charge and leak conductances
act together: `tau = 1/(1/tau_charge + 1/tau_leak)` and
`Vtarget = Vs*tau/tau_charge`. Otherwise only the leak acts. If leaking charge
crosses the instantaneous source during a sample, the interval is split at
`tau_leak*ln(Vold/Vs)`. A fixed-drop ideal diode cannot discharge the capacitor
back into MEG. The second RESONANCE gang scales the output as
`Vsweep = Vcap*(0.25 + 0.75*RESONANCE)`; it is independent of the feedback gang
passed to VCF. Repeated accents accumulate toward a bounded periodic solution.

FilterControl returns **ladder bias amperes**, not Hz or normalised cutoff:

```
octaves = (CUTOFF - 0.5)*8
        + audio(ENV_MOD)*(MEG - 2.5 V)*(1 octave/V)
        + Vsweep*(1 octave/V)
I_ladder = (1 uA)*2^octaves
```

At centre cutoff with ENV MOD and sweep zero, bias is 1 uA. CUTOFF alone spans
62.5 nA–16 uA. ENV MOD rotates around a reference voltage, moving the low and
high envelope endpoints, rather than adding an always-positive envelope. The
physical matched-transistor law is `I = Iref*exp(delta_Vbe/VT)`; one octave
corresponds to `VT*ln(2)` at the transistor pair. The equation above expresses
estimated summing-network attenuation directly in octaves. `thermal_voltage`
currently belongs to the VCA driver; it does not temperature-compensate the
filter law. Bias is limited to 1 pA–1 mA, matching the current VCF input guard. `dw303-vcf` receives this value directly
as its `bias_current` argument; cutoff frequency must be determined by that
ladder's capacitor/junction model. Absolute frequency calibration is outstanding.

Vca models a transistor-fed BA662 transconductance cell. Effective drive is
`D(V) = VT*ln(1 + exp((V - Vbe)/VT))`, with an overflow-safe linear branch above
40 thermal volts. This is exponential below the junction knee and asymptotically
linear above it. The BA662 gain is proportional to control current, so
`gain = leakage + D(Vcontrol)/D(Vunity)`, rather than an exponential gain for the
entire envelope. `Vcontrol = VEG + accent_gate*ACCENT*1.5 V`.
Signal transfer is `scale*tanh(Vin/scale)*gain`, representing a differential
pair with an estimated input attenuation. Control bleed adds `0.001*Vcontrol`
at the output node; the output stage removes its static DC. Accent injection
is a switched level estimate: this interface has no MEG input, so it cannot
reproduce the complete MEG-derived accent control current into the original VCA.

OutputStage stores the voltage across the series coupling capacitor. For a
piecewise-constant input, its charge update is exact. The returned resistor
voltage is the interval average `delta*(1-exp(-dt/tau))*tau/dt`, then multiplied
by the audio-taper VOLUME divider and the buffer gain. Nominal `tau = 50 ms`
(`fc = 3.183 Hz`). Static DC decays to zero; changing volume does not rescale
stored charge. Audio taper is `(81^rotation - 1)/80`, yielding 0, 0.1 and 1 at
rotations 0, 0.5 and 1. This is an estimate, not a measured Roland taper table.

## VERIFY inventory

All of these appear in source doc comments; descriptive names avoid assigning
unconfirmed reference designators. Values are intended as replaceable estimates.

| Parameter | Estimate / item to verify |
|---|---|
| `meg_capacitance` | 1 uF; likely C62, verify designation and value |
| `meg_decay_resistance` | Equivalent minimum 200 kohm |
| `decay_pot_resistance` | Effective 2.3 Mohm span and linear rheostat law |
| `meg_attack_resistance` | Equivalent charging source/switch 1 kohm |
| `meg_charge_pulse` | 4 ms trigger pulse |
| `meg_source_voltage` | 5 V charge source |
| `veg_capacitance` | 1 uF |
| `veg_attack_resistance` | 330 ohm |
| `veg_decay_resistance` | 3.3 Mohm |
| `veg_release_resistance` | 10 kohm gate-off discharge |
| `veg_charge_pulse` | 1.5 ms |
| `veg_source_voltage` | 5 V |
| `accent_capacitance` | 1 uF |
| `accent_charge_resistance` | 100 kohm |
| `accent_discharge_resistance` | 1 Mohm |
| `accent_diode_voltage` | Fixed 0.6 V diode drop |
| `resonance_sweep_floor` | Linear second gang, 0.25–1 output scaling |
| `filter_reference_current` | 1 uA at centre cutoff, no modulation |
| `cutoff_octaves` | Eight-octave cutoff span, linear panel rotation |
| `envelope_octaves_per_volt` | 1 octave/V maximum |
| `envelope_reference_voltage` | 2.5 V pivot |
| `accent_octaves_per_volt` | 1 octave/V |
| `vca_threshold_voltage` | 0.6 V driver knee |
| `thermal_voltage` | 26 mV effective junction thermal voltage |
| `vca_unity_voltage` | 5 V unity reference |
| `vca_accent_voltage` | 1.5 V switched control injection |
| `vca_signal_scale` | 1 V effective differential-pair input scale |
| `vca_leakage` | 1e-5 off-state signal gain |
| `vca_feedthrough` | 0.001 control/output voltage ratio |
| `output_capacitance` | 1 uF effective series coupling capacitance |
| `output_load_resistance` | 50 kohm effective fixed load |
| `output_buffer_gain` | Unity |
| Component identification | All reference designators and equivalent network reductions |
| AccentSweep topology | Fixed-drop diode, no reverse leakage, unloaded gang divider |
| FilterControl topology | Summing polarity, gains, pivot, current reference and 1 pA–1 mA bounds |
| Vca topology | Soft diode driver and reduced BA662 pair, input attenuation, accent injection and bleed |
| OutputStage topology | Single coupling cap and fixed load; extra caps and pot loading omitted |
| Pot tapers | Estimated VOLUME and ENV MOD 10%-at-midpoint audio law |
| Envelope charge topology | Finite trigger pulses, charge-path equivalents and neglect of simultaneous discharge during charge |

## Lifecycle and numerical policy

`prepare` accepts 44.1–384 kHz inclusive, preserving state and diagnostics.
Invalid preparation leaves the clock untouched. New instances use 48 kHz.
`reset` clears all charge, pending pulses and diagnostics, retaining the clock.
Non-finite tick inputs become zero and increment a saturating per-argument
counter. Panel inputs clamp to 0–1; analog inputs clamp to conservative voltage
bounds (MEG/VEG/sweep 0–12 V, audio +/-24 V). Component non-finites/negatives
use safe fallbacks; zero resistances/capacitances are bounded through RC products
of 1 ns–1e9 s. Positive component values are bounded to 1e9 before multiplication.
Stored voltage tails below 1e-24 V are flushed; no denormal-mode framework is
needed by this crate. These numerical guards are not circuit calibration.

## Validation and remaining work

The integration tests verify component-derived MEG decay at four DECAY positions
and accent override, attack charging, VEG fixed decay and gate-off truncation,
accent accumulation over notes 1–8 and saturation over 64 notes, the RESONANCE
gang ratio, exponential converter units/pivot/limits, VCA sub-knee exponential
and above-knee linear gain, accent boost/bleed, and DC removal/audio taper.
Time constants are compared at 48/96/192 kHz; accent curves agree within 2 mV.
Long-run stress covers 500,000 samples at each of 44.1/48/96/192/384 kHz with
extreme panel/audio inputs, checking finite and non-subnormal outputs. Unit tests
cover exact RC composition, tail flushing, component guards and diode crossing.

The equations are deterministic and pass the behavioural tests, but circuit
accuracy still needs a legible schematic capture and SPICE transient comparison.
There are no verified component identities, transistor temperature model,
capacitor leakage/tolerance measurements, pot taper measurements, full BA662
internal model, buffer clipping or multi-capacitor output network. Coupling-load
interaction with VOLUME and RESONANCE is reduced to fixed impedances. Aliasing
from the memoryless VCA nonlinearity relies on the voice's oversampling.

## Performance pass (2026-10-11)

The circuit equations, component guards, public interfaces and VERIFY inventory
above are unchanged. Fixed component RC products and the VCA unity-drive
normaliser are calculated at construction. Output `expm1` is calculated at
construction/preparation. MEG/VEG/accent RC exponentials cache the exact result
using bitwise duration/time-constant keys, including partial pulse and diode
crossing intervals; a changed clock changes the duration key. FilterControl's
ENV MOD taper and OutputStage's VOLUME taper cache by bitwise sanitised panel
rotation. Sanitisation and diagnostic counting still execute for every input.
Reset retains coefficient caches because only charge/pulses/diagnostics change.
The signal-dependent filter exponential, VCA driver and signal nonlinearity,
and accent diode-crossing logarithm still execute as required by the equations.

Before modifying the library, its implementation was frozen in
`crates/dw303-env/tests/support/reference.rs`. The regression test compares all
six public stages plus `Vca::gain`, across 48/96/192 kHz with live rate changes,
reset, gate/trigger/accent changes, knob sweeps, extreme/non-finite signals,
non-integral trigger pulses and invalid component configurations. Voltage
comparisons use 1e-6 absolute or relative above 1 V; bias-current comparisons use
1e-6 relative. Diagnostics are compared exactly. Existing behaviour tests,
regression tests, rustfmt checks and all-target clippy with `-D warnings` pass.
There are 12 passing tests and one explicitly ignored manual timing benchmark.

The requested release renderer (8 s, 48 kHz host / 192 kHz circuit) measured
2.477 s / **31.0% of one core** before, and 1.772 s / **22.1%** after. The two
WAV files compared byte-identically. These whole-workspace timings can include
concurrent changes in other crates and do not isolate this crate's contribution.
The <=10% whole-voice target remains unmet in this measurement.

An isolated same-binary benchmark against the frozen reference, using opaque
stage references to prevent compile-time loop-invariant hoisting, measured
0.106744 s (1.33% core) before versus 0.066158 s (0.83% core) cached: **1.61x**
for six stages over 8 s at 192 kHz. This synthetic control sequence is not a
whole-voice timing and uses a constant audio input. Reproduce with:

```
cargo test --offline --release -p dw303-env --test performance_regression benchmark_cached_against_reference -- --ignored --nocapture
```

macOS `sample` was attempted against a running 24-second render, but could not
inspect the process under the available permissions; no sampled call-stack
profile was obtained. Existing schematic/SPICE/hardware validation gaps remain;
this pass adds no component certainty or new VERIFY items.
