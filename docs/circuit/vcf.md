# VCF: differential diode ladder

`dw303-vcf` implements the contract's `Vcf`, `Quality`, `VcfDiagnostics`,
`Components` and `StageError`. It is `no_std`, uses only existing `libm`, has no
unsafe code, and stores six capacitor voltages in fixed arrays. Inputs and
outputs are AC voltages relative to the circuit's quiescent bias, in volts;
`bias_current` is the total tail current, in amperes. DC rail offsets are
removed by the differential reduction, not represented as signal voltages.

This is a reduced nonlinear circuit model, with a physically loaded four-pole
core. It is not yet a verified component-for-component service-note netlist.
The core linearisation has an independent published-polynomial cross-check;
absolute gains, large-signal behaviour and the full coupling network need
schematic/SPICE/hardware verification. All such estimates are listed below.

## Reduced netlist and equations

Number the differential voltages across ladder capacitors bottom to top
`v0..v3`. Nominal capacitances are provisionally `[18, 33, 33, 33] nF`.
The input differential pair injects a current into the bottom node. Three
junction pairs connect adjacent nodes; the fourth terminates the top node
against the AC reference. Thus each capacitor loads the stage below it.
This is **not** a cascade of buffered Moog ladder stages.

Let `I` be total ladder bias current and `S = 2 V_T`. A matched input pair
obeying the exponential junction law and sharing a fixed tail current gives

```
I_left / I_right = exp(u / V_T)
I_left + I_right = I
(I_left - I_right) / 2 = (I/2) tanh(u / S) = j_in
```

For interior diode-connected junctions, under the fixed geometric-mean
arm-current/common-mode approximation, the arm currents are
`(I/2) exp(+delta/(2 V_T))` and `(I/2) exp(-delta/(2 V_T))`. Their half-difference,
the current charging a cross-ladder capacitor, is

```
j_k = (I/2) sinh((v_k - v_(k+1)) / S), k = 0,1,2
j_3 = (I/2) sinh(v3 / S)
C0 dv0/dt = j_in - j_0
Ck dvk/dt = j_(k-1) - j_k, k = 1,2,3
```

These exponential currents preserve loading between neighbouring stages.
The factor one-half converts differential arm current into cross-capacitor
current. Constant common-mode bias is a reduction, not an exact elimination
of all large-signal common-mode dynamics. The interior `sinh` law should not
be replaced by four independent `tanh` one-poles: the input pair's fixed-tail
constraint is different from the interior diode connections.

The output differential amplifier is represented by an input-referred
voltage `a = H tanh(v3/H)` and node output `G a`. Nominally `H = 0.2 V` and
`G = 20`. The instantaneous differential amplifier approximation omits its
transistor currents and parasitic poles.

Two effective series coupling sections are explicitly in the feedback loop:

```
dh1/dt = (a - h1) / (R1 C1)
dh2/dt = (a - h1 - h2) / (R2 C2)
p = a - h1 - h2
u = attenuation * waveform_input - k * p
k = resonance_rotation * maximum_feedback_gain
```

`h1,h2` are effective coupling-capacitor voltages referred to the amplifier
input; output gain and feedback attenuation are absorbed into `k`. Each RC
is provisionally 15 kohm × 1 uF (10.61 Hz). The loop transfer includes
`P(s) = product[s/(s + 1/(R_i C_i))]`, so feedback vanishes at DC and is
attenuated at low ladder current/cutoff. These two sections are an estimate
of the full loop, not a claim that the original contains only two coupling
capacitors. There is no additional cutoff-dependent heuristic multiplying
resonance. Pot rotation is currently a provisional linear law; the accent
gang belongs to the envelope crate.

## Bias current, cutoff and independent linear transfer

The input transconductance before differential-to-capacitor conversion is
`I/(2 V_T)`. With the cross-ladder capacitors used here the conductance is
`g = I/(4 V_T)` and each local integration rate is `g/C`. The difference is
solely the definition of differential current/capacitance. There is no
arbitrary Hz-per-knob mapping or prewarping tied to a reference sample rate.
All ladder pole frequencies scale with `I`, except for the fixed coupling
network and digital frequency warping. The lowest unresonant pole is much
lower than `g/C`; calling each local rate the overall -3 dB cutoff would be
incorrect.

The linear admittance matrix is tridiagonal. Its diagonal entries are
`[s C0 + g, s C1 + 2g, s C2 + 2g, s C3 + 2g]`; off-diagonals are `-g`.
The forcing vector is `[g u, 0, 0, 0]`. Writing its determinant as `D(s)`,

```
L(s) = v3/u = g^4 / D(s)
output/input = attenuation * G * L(s) / (1 + k P(s) L(s))
```

Tests evaluate `D(s)` independently with the determinant recurrence, not
with the time-domain solver. For `C0=C/2`, `C1=C2=C3=C` and `z=s C/g`,

```
L(s) = 1 / (0.5 z^4 + 4 z^3 + 10 z^2 + 8 z + 1)
```

This agrees with the TB-303 core polynomial derived in
[Stinchcombe's original diode-ladder analysis](https://www.timstinchcombe.co.uk/index.php?pge=diode).
An explicit polynomial test checks that agreement. The nominal 18/33 ratio
is close to, but not exactly, one-half; the general determinant retains it.
At high frequency the core tends to 24 dB/octave, with a broad intermediate
slope due to the spread of loaded poles and unequal capacitors. Four equal,
buffered poles would produce a different curve.

[Stinchcombe's coupling-network study](https://www.timstinchcombe.co.uk/index.php?pge=diode2)
shows why the real loop needs further refinement: additional coupling
sections produce more poles and zeros, including a low-frequency peak.
The service notes are the authority, available as the
[Roland service-note scan](https://manuals.plus/m/768b42aace12df30692361da70ad22654ad53f99f7e61ed4d0ef534122027eb7).
The readable web text was inspected; the circuit image values were not
successfully resolved in this implementation session. No component value
is promoted to schematic-verified on the basis of that text.

## Discretisation and Newton solver

For six-state `x` and continuous derivatives `f(x,u,I,k)`, the trapezoidal
companion form is

```
r(x_n) = x_n - history - (T/2) f(x_n,u_n,I_n,k_n)
history_next = x_n + (T/2) f(x_n,u_n,I_n,k_n)
J_r = identity - (T/2) J_f
```

Capacitor integration is trapezoidal even with time-varying controls; the
history retains the previous endpoint derivative. The ladder, amplifier,
and coupling sections are solved together without a sample delay in the
feedback loop. This is a ZDF solve, not delayed explicit feedback.

The analytic derivatives use `cosh(delta/S)/S` for each interior junction,
`(1 - tanh(u/S)^2)/S` for the input pair, and `1 - tanh(v3/H)^2` for the
amplifier. The unit test checks every Jacobian entry with central differences
in both qualities. The linear solve eliminates the two feedback RC states, followed by the
tridiagonal ladder and its feedback corner, using stack values. A six-by-six
partial-pivot Gaussian elimination remains the fallback for non-finite or
near-zero pivots. The initial guess extrapolates the previous endpoint
derivative only when every predicted voltage step is at most 1 mV; otherwise
it uses the previous solution. Neither solver changes the residual equations.

Newton has at most **eight iterations**, with absolute infinity-norm
residual tolerance **1e-13 V** across all six companion equations. Each step
has at most eight backtracking trials, halving damping until the residual
decreases. A final residual check allows the eighth update to converge.
Accepted backtracking evaluations are reused for the next iteration, history
and output. Worst case is eight linear solves and 66 flow/Jacobian evaluations
(one initial evaluation, 64 backtracking evaluations, and one history
reevaluation if tiny-state flushing changes the solution). There are no data-dependent
unbounded loops, heap calls or unsafe operations.

Failed convergence retains the previous physical state/history and counts
one `non_convergence`. This bounded hold fallback prioritises finite state;
it is not an accuracy guarantee for settings outside the tested range.
Junction arguments beyond magnitude 20 use a continuous linear tangent to
`sinh`, avoiding overflow in rejected trial points. This is inactive at the
tested operating levels. Stored values below 1e-30 V are flushed to zero.

Non-finite scalar tick inputs become zero and are individually counted.
Finite input protection limits waveform input to ±12 V, current to 0..1 mA,
and knob rotation to 0..1. Limits are numerical guards, not a physical rail
clipping model. Diagnostics use saturating counters. `prepare` accepts
44.1–384 kHz, clears state on success, and preserves state on error. `reset`
clears state and counters. Quality switching preserves capacitor state and
companion history. Invalid/nonpositive component values are replaced with
nominals at construction; their accepted finite range is 1e-15..1e9 SI units.

## Economy

Economy replaces each interior `sinh(delta/S)` with `delta/S`; the differential
input pair and output amplifier remain nonlinear. The same loaded netlist,
capacitors, feedback RC states and implicit integration are preserved. It
avoids four interior exponentials per flow/Jacobian evaluation
and generally uses fewer iterations. It still uses Newton for the input-pair
feedback closure. This avoids introducing a different phase response or a
delayed feedback path in the cheaper mode. It loses interior-junction drive
distortion and is not intended to match Full at large drive. No wall-clock
speedup claim is made without a benchmark.

## Validation performed

Package-only, offline checks:

- `cargo test --offline -p dw303-vcf -- --nocapture`: **9 passed** (one Jacobian
  unit test and eight integration tests), zero failures; no doctests.
- `cargo clippy --offline -p dw303-vcf --all-targets -- -D warnings`: passed.
- Direct `rustfmt --edition 2024 --check` on both Rust files: passed. The cargo
  fmt subcommand does not accept `--offline`, so direct rustfmt was used.

Evidence covered by the tests:

- Full sinusoidal magnitude vs the independent analytic reduced netlist:
  1/10/50 uA, resonance 0/0.3/0.6, seven frequencies spanning local normalised
  frequency 0.01..0.6, error below 0.5 dB. Measurement includes at least two
  complete cycles and at least 1.5 s settling (longer for low bias).
- Four-pole asymptote: about 24 dB/octave at normalised frequencies 12 and 24,
  including a measured time-domain response; unequal vs equal capacitor
  shape differs by more than 1 dB at local normalised frequency 1.
- Economy vs Full below 0.05 dB at low drive, at 100/500/1500 Hz, 10 uA and
  resonance 0.5. Full magnitude differs by less than 0.15 dB between
  48/96/192 kHz at those frequencies.
- Seeded self-oscillation continues with zero input at maximum resonance:
  at 192 kHz, 10 uA produces 667.965 Hz, 0.037358 Vrms; 20 uA produces
  1320.771 Hz, 0.039175 Vrms. Frequency ratio is approximately 1.977.
  Frequency agrees within 2% at 48/96/192 kHz. Exactly silent reset stays
  silent; the deterministic model does not invent analog noise.
- Feedback attenuation at low cutoff: analytic resonant enhancement falls
  with bias, measured 0.1 uA/10 Hz response changes less than 1 dB with
  resonance, and the 1 nA seeded response does not sustain the high-current
  oscillation amplitude.
- 35,840 consecutive samples per sweep run, **both** qualities at
  44.1/48/96/192/384 kHz: seven currents from 0 to 1 mA, four resonance
  settings, five input amplitudes from 0 to 12 V, with abrupt transitions.
  **Zero non-convergence** across all runs. Before the performance pass,
  Full mean iterations were 2.571..2.945;
  Economy 2.297..2.657; maximum eight. This is the bounded-CPU iteration
  statistic smoke test, not a timing benchmark.
- One million samples **per quality**, maximum resonance, 6 V peak input,
  abruptly alternating 0.1/50 uA bias: finite bounded output and zero
  non-convergence. Lifecycle, non-finite input counts and exact silence
  are also tested.

These validate the implemented equations, not original-unit fidelity.
No SPICE golden data, TD-03 captures or allocator instrumentation were
available. CPU timing from the later performance pass appears below.
Scope restrictions prevent creating `reference/spice` assets.

## VERIFY inventory and remaining weaknesses

Every uncertain field in `Components` carries a `VERIFY:` doc comment. The
following are estimates pending checks against the actual service schematic:

1. Ladder capacitances **18/33/33/33 nF**, bottom-to-top placement, and service
   reference designators. Do not confuse them with tenfold larger values in
   some clone netlists. The exact designators are deliberately not guessed.
2. Thermal voltage **25.85 mV**, corresponding approximately to 300 K; actual
   transistor temperature and ideality factors.
3. Effective feedback capacitors **1 uF each**: identify every original
   coupling capacitor and replace the two-section approximation as needed.
4. Effective feedback resistances **15 kohm each**, including frequency- and
   pot-position-dependent loading.
5. Output differential amplifier gain **20**, transistor topology/bias,
   output polarity convention and loading.
6. Amplifier input-referred headroom **0.2 V** and the symmetric tanh
   approximation; actual clipping will be asymmetric and bias-dependent.
7. Maximum input-referred feedback gain **24** and provisional **linear**
   RESONANCE taper/wiring; verify against the actual feedback gang and trim.
8. Waveform-to-input-pair attenuation **0.02**, source impedance, original
   signal amplitude and DC removal. The 6 V peak stability test is an
   intentionally conservative twice-3 V estimate, not a measured amplitude.
9. Fixed common-mode diode-arm reduction, documented on `flow`: assess
   common-mode redistribution, finite beta, Early effect and junction
   capacitances with a full transistor netlist. The exponential reduction
   alone does not establish large-signal accuracy.

Other known limitations: input coupling and the full differential amplifier
are not separate nodal networks; multiple real coupling poles/zeros and the
associated bass peak may be misrepresented; temperature is fixed; aliasing
is not independently measured (the voice should oversample); trapezoidal
frequency warping remains near Nyquist; Economy needs large-signal listening
and CPU benchmarking. Sample-rate tests establish the stated operating points,
not accuracy at every current/frequency combination. The fallback can freeze
state on persistent failure; diagnostics must be monitored in integration.

## Performance pass (2026-10-11)

The original algorithm was frozen in `tests/support/reference.rs` before any
production changes. Regression tests compare Full and Economy at 48/96/192 kHz
under sinusoidal drive, rest/gate transitions, accent-like drive changes,
continuous resonance sweeps, abrupt bias changes from zero through 1 mA,
overdrive, non-finite signals, quality switching and reset. A separate test
compares 192,000 consecutive seeded self-oscillation samples at each rate.
The bound is `1e-6 * max(1, abs(reference))` (1 uV absolute / 1 ppm relative).
All waveform comparisons and non-convergence comparisons pass.

The implementation retains the original equations, eight Newton iterations,
eight backtracking trials, **1e-13 V** residual tolerance, junction continuation,
flush thresholds and hold-on-failure fallback:

- Cache capacitance reciprocals, thermal scaling and feedback RC rates.
- Reuse the accepted trial's residual/Jacobian for the next Newton iteration;
  reuse its final flow and amplifier value for history/output unless flushing
  changes the state.
- Evaluate sinh/cosh together using one exponential and its reciprocal.
- Solve the same six linear equations using their sparse ladder/feedback
  structure; reuse pivot reciprocals. The original pivoted dense solver remains
  the numerical fallback. A unit test compares both solutions and checks the
  matrix residual across both qualities, all three rates, zero–1 mA bias and
  resonance extremes, plus deliberate fallback activation.
- Predict the next solution from the previous endpoint derivative only for
  predicted changes <=1 mV per node. An unrestricted predictor failed the
  extreme-current stability sweep and was rejected. The restricted predictor
  passes it and reduces typical renderer iterations from three to two.

Final package checks: **12 tests passed**, one manual timing test ignored by
normal runs; offline clippy `--all-targets -- -D warnings` passed; all assigned
Rust files pass direct rustfmt checks. Existing analytic response, Jacobian,
self-oscillation, rate-independence and long-run/extreme-grid tests remain
passing with zero non-convergence.

Reproducible stage comparison (384,000 samples at 192 kHz, 3 V sinusoidal drive,
20 uA bias, resonance 0.7):

```
cargo test --offline --release -p dw303-vcf --test regression performance_comparison -- --ignored --nocapture
```

Frozen VCF: **281.306 ms**; optimized VCF: **109.159 ms**, **2.58x** faster.
Construction and WAV I/O are excluded. This benchmark retains the same residual
tolerance and Full quality.

The requested eight-second full-voice renderer measured **31.0%** of one core
before the pass (2.484 s) and **12.6% / 12.5%** on final repeats
(1.005 / 1.004 s). Final VCF telemetry: 1,536,000 samples, 3,095,581 iterations,
maximum three, **zero non-convergence**; the baseline used 4,607,970 iterations.
The **<=10% target remains unmet**. Other crates were under concurrent editing;
these full-workspace timings do not isolate VCF changes. The frozen-stage
benchmark does. macOS `sample` was attempted but could not examine the running
renderer under current execution permissions.

All **nine VERIFY items** in the inventory above remain unchanged, with no new
schematic-validation claim. Known weaknesses of the reduced circuit remain.
The sparse solver can fall back to dense elimination for unusual components;
both paths are bounded. The predictor's 1 mV cutoff is a numerical initial-guess
safeguard, not a circuit voltage limit or altered model equation. Direct
exp/reciprocal sinh evaluation loses some relative precision for tiny junction
arguments; the specified absolute waveform regression and original tight
Newton/stability tests pass. Economy-specific wall-clock speedup and full
hardware/SPICE fidelity remain unmeasured.
