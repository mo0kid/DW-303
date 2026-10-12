# VCF revision 2: schematic ladder and coupling network

Authority: [reconciled netlist](../../reference/schematic/netlist.md) and the
[complete VCF transcription](../../reference/schematic/transcription/vcf.md).
The Revision 2 contract is implemented by `tick(input, ladder_current,
resonance) -> VcfOut`. Input is the WAVEFORM switch common voltage; current
is Q11 collector current in amperes; outputs are AC volts at C14 negative
terminal / VR4A terminal 3 and the **loaded** VR4A wiper. `Components` cites
printed designators. The current fitted device/loading parameters and
validation are in [vcf-fitting-round5-2026-10-12.md](vcf-fitting-round5-2026-10-12.md);
printed-only calibration and performance numbers below are historical.

## Circuit reduction

The four differential rung voltages are ordered bottom to top: C18 18 nF,
C19/C24/C26 33 nF. Define `v = R - L`. Q12 is the fixed-tail input pair;
Q13/Q15, Q14/Q17 and Q16/Q23 are diode-connected rungs; Q22 has fixed bases
at BIAS_Q22 and acts as the common-base top termination. With `S = 2 VT`,
`I = Q11 collector current`, `Iladder = I beta12/(beta12+1)`, and differential capacitor current equal to
half the arm-current difference:

```
j_input = (Iladder/2) tanh((base_left - base_right)/S)
j_edge[i] = (Iladder/2) tanh((v[i] - v[i+1])/S)
j_top = (Iladder/2) tanh(v[3]/S)
C[i] dv[i]/dt = j_previous - j_edge[i]
```

The rung capacitors carry equal and opposite arm currents. With negligible
base/common-mode currents, each rung's arm-current sum is the Q12 collector-current sum.
Eliminating its common-mode voltage gives the `tanh` law above; the previous
fixed-common-mode `sinh` law incorrectly allowed that sum to grow with signal.
The rung small-signal conductance is `Iladder/(4 VT)`. The inferred common-mode
voltages are algebraic, so no new capacitor state is needed.

This retains the loaded, tridiagonal ladder, rather than four buffered poles.
R69 2.2K is in the common tail path and does not change an externally specified
ideal Q11 collector current. Its headroom limit belongs to FilterControl;
finite-beta corrections and common-mode redistribution remain approximations.
BIAS_Q22, BIAS_IN, BIAS_DIFF and BIAS_FB are treated as stiff, settled AC
references. R95/C28 and the bias decouplers are not dynamic audio states.

Six additional capacitor states represent C17, C25, C27, C14, C15 and C23,
with voltages measured relative to the quiescent biases. No envelope state
or arbitrary cutoff law is used. For C17 state `h17`:

```
dh17/dt = (input - h17) / ((R62 + R70) C17) + R70/(R62+R70) j_input/(beta12 C17)
base_left_unloaded = R70/(R62 + R70) * (input - h17)
```

R62=220K, R70=2.2K give 0.009901 attenuation (−40.09 dB),
C17=1µF gives 222.2 ms / 0.7163 Hz. The input DC is rejected after settling;
reset followed by a nonzero waveform DC naturally produces a startup transient.

Q12 differential base currents now load both the input and feedback sources:

```
Rleft = R62 || R70 = 2178.218 ohm
Rright = R97 || R98 = 1803.279 ohm
u0 = base_left_unloaded - base_right_unloaded
u_intrinsic = u0 - (Rleft + Rright) j_input / beta12
j_input = (Iladder/2) tanh(u_intrinsic/(2 VT))
dh23/dt additionally subtracts R98/(R97+R98) j_input/(beta12 C23)
```

`beta12=300` is a fitted, unmeasured device estimate (**VERIFY**). The settled
DC base-current sum remains absorbed in the bias. Small-signal Q12 conductance
includes the frequency-dependent source degeneration
`g12 = g / (1 + g (Zleft+Zright)/beta12)`. The continuous calibration pole and
independent AC tests use that same physical loading, including C17/C23.
A bounded five-update intrinsic-voltage Newton solve uses tanh addition and a
small-step Taylor evaluation to avoid repeated exponentials; it solves the
same implicit current law. C17/C23 form another 2×2 Schur block, retaining ten
capacitor voltages, with no new allocation, lock or unbounded tick loop.

Crossed C25/C27 (each 0.1µF) feed Q21R/Q21L respectively:

```
b_right = -v[3]/2 - h25
b_left  = +v[3]/2 - h27
base_delta = Ic/beta * tanh((b_left-b_right)/(2 VT))
i_right = b_right/R114 - base_delta
i_left  = b_left/R113 + base_delta
dh25/dt = i_right/C25
dh27/dt = i_left/C27
C26 dv[3]/dt additionally subtracts (i_left-i_right)/2
```

R113/R114=100K alone yield 10 ms / 15.915 Hz corners. The current reduction
also includes complementary nonlinear Q21 base currents. Their zero-signal
linearisation is still `r_pi = beta VT / Ic`, with a loaded corner of about
67.91 Hz; the DC base-current sum is absorbed into the settled bias. Both
currents load the ladder top. Constant beta and stiff common-mode bias remain
VERIFY assumptions.
Q20 tail current is derived from R116/R112/R109's three-resistor divider,
R108=2.2K and the estimated Vbe:

```
I20 = max(0, (5.333 R109/(R116+R112+R109) - Vbe)/R108)
Ic = (I20/2) beta/(beta+1)
Q19_E_AC = output_gain_correction Ic R110 tanh((b_left-b_right)/(2 VT))
```

For the printed-only gain with infinite beta and correction one,
R110=2.2K and I20≈0.5126 mA give gain≈10.907 and collector swing≈0.564 V.
The current fitted gain correction and finite beta give 99.19 V/V and
5.13 V swing; the missing gain is explicitly provisional in the fit register. Q19 is an ideal emitter follower of Q21R's
collector; Q21 input polarity gives an in-phase input/output and subtractive
feedback at Q12R. This replaces the old arbitrary gain 20/headroom 0.2 V.

VR4A is a 50K **linear** track, terminal 1 grounded. For rotation `p`,
C14 voltage `h14`, C15 voltage `h15` and Q18's input branch current `ifb`:

```
out = Q19_E_AC - h14
Rth = 50K p (1-p)
ifb = (p out - h15)/(Rth + R67 + R99)
tap = p out - Rth ifb
dh14/dt = (out/50K + p ifb)/C14
dh15/dt = ifb/C15
Q18_E_AC = R99 ifb
base_right = R98/(R97+R98) * (Q18_E_AC - h23)
dh23/dt = (Q18_E_AC - h23)/((R97+R98) C23)
```

C14/C15/C23=1µF; R67=2.2K, R99=100K, R97=10K, R98=2.2K.
C15's corner is approximately 1.56 Hz at either pot endpoint, decreasing
with wiper source resistance. C23 gives 0.180328 injection and a 12.2 ms /
13.045 Hz corner. C14 drives the track and feedback branch together in the
implicit solve. At maximum resonance the tap equals output exactly; at
intermediate rotation it differs from an unloaded `p * output`.

The receiving VCA's C21/R121 and C22/R122 loading is not fed back into this
crate by the binding interface. C14's unloaded track corner is 3.18 Hz;
the transcription's approximate 3.9 Hz includes the external VCA load.

## Solver and economy mode

All ten capacitor equations use implicit trapezoidal companions,
`residual = x - history - (T/2) f(x)` and analytic `J = identity - (T/2) Jf`.
The Newton solve includes the complete resonance loop without a sample delay.
It uses at most eight iterations and eight backtracking trials per iteration,
with absolute residual tolerance 1e-13 V and hold-on-failure telemetry.
The fixed-tail `tanh` law remains finite for arbitrarily large trial voltages;
its derivative is bounded between zero and one.

The linear correction eliminates the coupling states with a Schur reduction,
including C25/C27's complementary-base-current 2×2 block and C14/C15's
loaded-pot 2×2 block, then solves the tridiagonal ladder
and its feedback corner. Fixed-size partial-pivot elimination is the fallback.
Static reciprocals, RC rates and transistor scale/gain are cached on construction.
Warm starts use cubic extrapolation of four accepted endpoints for smoothly
changing controls (current change ≤5%, resonance change ≤0.02); otherwise
endpoint Euler prediction is used. Predictions exceeding 10 mV on any state
revert to the previous solution. Prediction changes only the initial guess.

Economy linearises the diode arms and Q22 termination, preserving the same
ladder loading, all six couplings, loaded pot, nonlinear Q12/Q21 pairs and
implicit feedback. It omits interior-junction distortion. Both modes are
allocation-free, `no_std`, safe Rust and bounded. Inputs are sanitised and
counted; protection bounds are ±12 V input, 0–1 mA current and 0–1 rotation.
State below 1e-30 V is flushed. Supported rates are 44.1–384 kHz.

## TM3 calibration and validation

`Vcf::calibration_current(ring_hz)` is a **control-thread** helper, not an
audio tick operation. It searches the continuous small-signal characteristic
for a damped complex pole at the requested imaginary frequency, including
loaded C25/C27, C14, C15 and C23 at RES max. The bounded search returns
`Option<f64>` amperes. Pass this target to FilterControl's TM3 calibration;
ENV MOD, DECAY and ACCENT minima and CUTOFF centre are integrator settings.
The helper does not assume a knob-to-current mapping.

Historical printed-only, infinite-beta 500 Hz pole calibration gave **23.1996 µA**.
For the current fitted components, see the round-3 report above. A small impulse produces
first peak-to-peak periods of **1.896 / 1.906 / 1.906 ms** at 48/96/192 kHz,
inside page 7's 2 ms ±0.5 ms adjustment window. Early peaks contain other
modes, so their spacing differs slightly from the isolated pole's 2 ms.
The model rings strongly damped and does **not** sustain the provisional
model's self-oscillation; no undocumented gain boost is used to force it.
The 5.2/5.8 V annotations are not calibration targets: the transcription
explicitly flags their inconsistent node attachment.

Validation commands (package scoped, offline):

```
cargo test --offline -p dw303-vcf -- --nocapture
cargo clippy --offline -p dw303-vcf --all-targets -- -D warnings
rustfmt --edition 2024 --check crates/dw303-vcf/src/lib.rs crates/dw303-vcf/tests/physics.rs
cargo test --offline --release -p dw303-vcf --test physics stage_timing -- --ignored --nocapture
```

Tests replace the deleted frozen provisional regression/reference files:
analytic Jacobian by central differences; sparse corrections against pivoted
elimination; independent frequency-domain netlist response (three currents,
three resonance settings, four frequencies), within 0.15 dB; sample rates
48/96/192 kHz and Economy comparison within 0.15 dB; printed gains/corners;
DC rejection and loaded wiper endpoints; page-7 ringing; lifecycle/sanitisation;
extreme grids at five rates plus one million samples per quality. No
non-convergence occurs in those runs. These are equation/netlist checks,
not SPICE or measured-unit golden validation.

## Performance evidence

The previous optimized crate measured **113.031 ms** before edits for 384,000
samples at 192 kHz, 3 V sinusoidal input (`sin(n*0.02)`), 20 µA and resonance
0.7, excluding construction and I/O. The replacement measured **118.428 ms**
Full and **106.444 ms** Economy for the identical workload. Full is **1.048×**
the old cost, within the requested approximately 1.3× bound. Final Full used
768,000 Newton iterations (mean two, max three) and zero non-convergence.
Wall-clock timing is workload and machine dependent; this is a stage benchmark,
not a whole-voice or worst-case CPU claim. The old frozen algorithm was deleted
as required, so the repeatable retained test times only the replacement.

Final checks: **eight tests passed**, one manual timing test ignored in normal
runs; the manual timing test passed; package-only offline clippy with warnings
denied passed; direct rustfmt checks passed. No new dependency was added.

## VERIFY register and known weaknesses

1. `Components::thermal_voltage`: estimated VT=25.85 mV at 300 K;
   transistor temperature and ideality are not printed.
2. `Components::vbe`: Q20 Vbe=0.65 V is an estimate. It sets tail current,
   Q21 gain, clipping amplitude and calibration current. The schematic's
   unspecified individual transistor types, Is/β and matching remain unknown.
3. `Vcf::flow`: algebraic common-mode elimination neglects ladder base currents,
   Early effect, common-mode/junction capacitance and common-mode Q21 loading.
   Differential Q21 loading is retained. Stiff settled rails remain assumed.
4. `Vcf::nodes`: Q18/Q19 are ideal unity followers with fixed settled bias,
   infinite β and no Early effect/parasitics. Q20 base loading is omitted;
   Q21 includes complementary nonlinear base currents, but retains constant beta,
   fixed tail current and settled common-mode base bias. Q18's actual bias is particularly β-sensitive (R66+R99=200K).

5. `Components::output_beta`: fitted Q21 beta 302.5 is unprinted and unmeasured.
6. `Components::output_gain_correction`: fitted gain multiplier 9.125 remains
   unresolved against the printed Q20/R108/R110 gain; see the round-5 fit register.
7. `Components::input_beta`: Q12 beta 300 is inferred from output audio, not
   measured at the ladder microamp bias. Constant beta, matched devices and
   stiff common-mode bias remain approximations; see the round-5 register.

Additional transcription open items: TP6's attachment / printed DC levels
remain unresolved and are excluded from level calibration. Gang-B direction
and ENV/ACCENT rest levels belong to the ENV integrator, not this signal crate.
Bias-rail motion, R69/Q11 saturation headroom, ladder/follower base-current loading,
external VCA input loading and follower clipping are not dynamic modelled.
Consequently strong damping, large-signal fidelity and absolute levels need
SPICE / hardware checks. No claim of original-unit amplitude fidelity is made.
Aliasing relies on voice oversampling; trapezoidal warping remains near Nyquist.
