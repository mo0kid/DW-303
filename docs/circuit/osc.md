# Pitch CV, VCO and square shaper — fitting round 1

Authority: [reconciled netlist](../../reference/schematic/netlist.md),
[VCO transcription](../../reference/schematic/transcription/vco.md), and
[binding contract](contracts.md). This round changes only `crates/dw303-osc`
and this document. No dependencies were added. Calibration uses the 346
read-only session-1 TD-3 WAV/JSON pairs dated 2026-10-12, with the unchanged
[scoring harness](../harness.md). These are unit-specific output-audio fits,
not internal-node measurements or proof of original TB-303 accuracy.

`Components::NOMINAL` retains its integration API name but now contains the
fitted defaults below. Printed resistors/capacitors not listed as fitted remain
at their transcribed values. The earlier source annotations, 5.5–12 V, are no
longer substituted for C33 voltage: `saw_low` and `saw_high` now describe the
capacitor cycle endpoints **before** the explicitly modelled Q28 follower.
`VcoOut.saw` remains Q28 source voltage and `VcoOut.square` Q8 collector voltage;
method signatures and SI units remain unchanged.

## Pitch CV and slide

The selected 200k R74–R90/R96 network has 100k series arms and a 100k Thevenin
resistance. R78 terminates at the reference rail. The ideal DAC output is
`(code+1)/12 V`, with `code=min(note+11,63)`: C=1 V, A=2.75 V,
upper C=4 V, maximum 64/12 V. The latch-clock/slide shared-net ambiguity O2
remains delegated to the sequencer under the binding API.

SLIDE high connects C35 **0.22 uF** to the ladder through IC12 A, giving
**22 ms**, 48.34 ms 10–90%, and 101.31 ms to 99%. IC11b follows this voltage
at TP4/CV OUT. SLIDE low connects C35 to the buffered output through IC12 B
and precharges it immediately. `PitchCv` retains the exact held-input RC
solution; this fitting round does not change slide components or timing.

## Exponential converter and tuning

Q26 is the **2SC1583 matched NPN pair**. IC11a regulates Q26a against the
5.333 V reference. R103 220k supplies about 30.3 uA, supplemented by
TM4 47k/R102. R104 2.2k/C34 1 nF regulator dynamics are omitted. O1, the
R102 unit ambiguity, remains open; TM4's effective current is calibrated.
The service adjustment is A=110 Hz at CV=2.75 V. The session fit changes the
lowest-C reference **32.703195663 → 32.577173303 Hz**, a **−6.6842 cent**
adjustment, to remove the median error across 49 centre-tuning notes. This is
not a claim that the capture's C1=32.5555 Hz is reproduced exactly: residual
pitch varies with key. Model A becomes **109.5761 Hz**.

R106 27k plus TM5 4.7k, R118 100k, and R100 560 ohms form the expo divider.
At 300 K, TM5 is solved to about **3.5204k** for exactly **1 V/oct**. Its position
is held under component variation. R100's assumed ideal 1/300 per-kelvin PTC
compensates Q26 thermal voltage. Tracking remains 1200 cents/oct versus the
measured 1200.09; the difference is only 0.36 cent over four octaves. No per-key
pitch table or unconstrained correction was fitted.

VR2 remains a **linear B-taper pot**. The previous law loaded a symmetric
midpoint voltage and forced ±700 cents. A pot between ground and an effective
pin-3 rail instead loads toward the reference through R118:

```
Rth(x) = VR2*x*(1-x)
Vw(x) = (Vrail*x*R118 + Vref*Rth(x)) / (R118 + Rth(x))
ΔVbe_tune = gain_tune * (Vw(x) - Vw(0.5))
```

The inferred effective rail is **5.402046 V**, near the hypothesised 5.333 V
reference, and VR2 is fitted **50k → 44.945454k**. This gives **−1086.620 /
+891.852 cents** relative to centre. `tuning_octaves` represents the total
endpoint span, **1400 → 1978.472 cents**, rather than half the range. The
rail is derived from that span and the nominal expo gain; it is an effective
calibration, not identification of the unreadable pin-3 glyph O3. The load
uses a stiff 64/12 V reference approximation and omits the small CV-dependent
base-node/loading interaction. Only rotations 0, 0.5 and 1 were measured;
t25/t75 remain predictions. No polynomial taper or endpoint lookup is used.

## C33 reset and Q28 follower

Q26b sinks current from **C33 10 nF**, giving a falling capacitor ramp. Q24/R101
10k establish the lower threshold; Q25 pulls Q27's PNP base down, regenerating
an upward reset through D25. Q25 remains the topology-based NPN choice O5.
The fitted capacitor endpoints are **5.2 / 11.5 V**, representing unequal
Q24/Q25 Vbe and finite Q27/D25 reset drops. They were inferred jointly with
Q28/Q8 estimates from octave waveform captures, not measured at C33. The
previous 5.5 / 12 V values were source-node annotations used as an ideal
follower approximation, so this is also a correction of node interpretation.

The cycle charge is now **63 nC**, and the fitted A sink current **6.9033 uA**
(previous calibrated approximation 65 nC / 7.15 uA). C33/endpoints vary the
period inversely while leaving converter current fixed. Reset duration is
still neglected; the captures do not isolate sub-microsecond recovery.

Q28 **2SK30(O)** drives the printed **R105 10k** load from the 12 V drain rail.
Its fitted estimates are **IDSS=0.6 mA, |Vp|=1 V**. The static load line solves
`Vs/R105 = IDSS*(1 + (Vg-Vs)/|Vp|)^2` in saturation. When `Vg+|Vp|>12 V`,
it uses the triode law with finite drain headroom:
`I = IDSS/|Vp|² * (2*(Vgs+|Vp|)*Vds - Vds²)`.
Both branches use analytic, numerically stable quadratic roots. Thus the
follower's gain, DC voltage and gentle saw curvature arise from its load,
not a separate offset applied only to the square path. Gate current, junction
capacitance and detailed measured device curves remain VERIFY items.

The source endpoints are approximately **5.263 / 11.054 V**, excluding kernel
ringing. Constructor-only midpoint quadrature (1024 points) projects the small
curvature residual around the exact falling linear ramp. The residual has
zero endpoint jump; the ramp's reset Fourier coefficients remain analytic.
There are at most **256 harmonics**, a raised-cosine taper over 224–256, and
rate-dependent pass/stop edges of 0.40/0.48 times the internal rate. Optional
bandwidth has a 20% transition. Reconstruction may overshoot near reset.
Construction uses fixed arrays and bounded loops; `tick` performs no follower
solve, quadrature or allocation. Frequency/bandwidth caching and continuous
slide frequency remain intact.

## Q8 square shaper

The topology is unchanged: **R35** directly drives Q8's base in parallel with
**C10 + R34 10k**; **R45/C11** feed/store the emitter voltage; **R36 10k** returns
the collector to **5.333 V**. There is no coupling high-pass in this block.
C10 voltage and C11 emitter voltage are physical state variables. Backward-Euler
companions solve cutoff, forward-active and saturation branches analytically:
`ib = g_base*(Ve-Veb)-drive`, `ic=beta*ib`, `ie=ib+ic` in active operation,
and `Vc=Ve-Vec_sat` in saturation, with base and collector current loading C11.

Fitted **R35=102.940k**, **R45=22.776k**, **C10=8.402 nF**, and **C11=0.819 uF**
use plausible ±5% resistor / ±20% capacitor search bounds. Those bounds are
assumptions, not verified tolerances for the original parts. Q8 estimates are
**Veb=0.55 V**, **beta=362**, **Vec_sat=0.08 V**. The unloaded emitter recharge
is now **18.654 ms**, and off-state C10 discharge **0.949 ms**. The Q8 junction
law remains piecewise linear; no measured transistor model is claimed.

Candidate fits compared octave collector harmonic ratios against baseline
output residuals, then were checked through real MIDI replay and the unchanged
harness. This separates source-shape estimation from the downstream response
as far as output-only measurements allow. No VCF, VCA, output or interface
high-pass compensation was introduced. Harmonic cancellation constrains duty
more usefully than audio-output midpoint duty, which those stages distort.

## Complete VERIFY register

Every code `VERIFY:` item, including inherited approximations, is covered here.

| Item | Nominal / previous → fitted estimate; evidence and limit |
|---|---|
| TM4 `reference_hz` | 32.703195663 → 32.577173303 Hz; remove median +6.6842 cents across 49 notes; residual pitch median/p90 about 0.485/1.286 cents |
| VR2 pin-3 / `tuning_octaves`, O3 | ±700 → −1086.620/+891.852 cents; effective reference-rail hypothesis 5.333 → 5.402046 V; residual tuning median/p90 about 0.713/2.829 cents; wiring and intermediate rotations unidentified |
| VR2 `tuning_resistance` | 50k → 44.945454k (−10.11%); endpoint asymmetry with R118 loading; actual pot tolerance unknown |
| R100 PTC coefficient | Ideal 1/300 per kelvin; curve unprinted; no warm-up/drift capture |
| C33 lower endpoint / Q24/Q25, O5/O6 | Former ideal source annotation 5.5 → capacitor 5.2 V; Q24/Q25 difference absorbs unprinted junction physics; Q25 NPN chosen from function |
| C33 upper endpoint / Q27/D25, O6 | Former ideal source annotation 12 → capacitor 11.5 V; reset drops inferred, not individually identified |
| Q28 IDSS / pinch-off | Ideal follower → 0.6 mA / −1 V; harmonic balance across five octaves; no internal voltage or device curve; near lower search boundaries |
| Q28 model reduction | Shockley saturation and triode load line; omit gate current, capacitances and Q8 base-network back-loading into Q28; estimates covary with Q8 and reset endpoints |
| R35 | 100k → 102.940k (+2.94%); square harmonic fit; ±5% search bound assumed |
| R45 | 22k → 22.776k (+3.53%); square harmonic fit; ±5% search bound assumed |
| C10 | 10 → 8.402 nF (−15.98%); square harmonic fit; ±20% search bound assumed |
| C11 | 1 → 0.819 uF (−18.1%); octave duty/tilt harmonic fit; ±20% search bound assumed |
| Q8 Veb, O6 | 0.65 → 0.55 V; constant forward drop, near lower fit boundary; real current/temperature dependence omitted |
| Q8 beta | Estimated 200 → 362; fits waveform, not independently measured |
| Q8 Vec_sat | Estimated 0.2 → 0.08 V; fits waveform, not independently measured |
| `cv_for_note` | IC10 supply unprinted; ideal logic high = 64/12 V; selected DAC resistor mismatch omitted |
| `PitchCv`, O2 | Already-latched note; shared IC9/slide clock ambiguity delegated to sequencer |
| `PitchCv`, O7/O6 | Ideal IC12 and IC11b; omit switch Ron, settling, clamps and Q29/Q30 switching; IC12 supply unprinted |
| `SquareShaper` | Piecewise-linear Q8 omits smooth Ebers–Moll curvature, reverse-active operation and parasitics |
| Reset recovery | Instantaneous regenerative reset; no device recovery time fit |
| Q26 | Ideal constant-current sink; omit Early effect and pair mismatch |
| IC11a/R104/C34 and TM4/R102, O1 | Ideal regulator; calibrate net current rather than resolve printed R102 “100” unit |

O4 (R96 designator in the R86 slot) has no electrical effect. TM4=47k follows
reconciliation L3; its missing glyph remains uncertain. O6 also covers reset,
clamp and slide devices whose types are unprinted. Hardware output duty, edge
times and tilt remain distorted by downstream coupling, filtering and envelope
age. Component fits are correlated and cannot uniquely identify an internal
node from audio. Neither finite reset slope nor gate junction effects have
been measured here; the remaining spectrum must not justify arbitrary source
corrections to compensate another stage.

## Scoring and regression audit

Full-session weighted error: **4.2606 → 2.1329** (49.94% lower); **346 captures, zero failed analyses**.

| Category | Baseline median / p90 | Fitted median / p90 | Aggregate before → after | Change |
|---|---:|---:|---:|---:|
| pitch | 1.3368 / 1.4806 | 0.0970 / 0.2571 | 1.3728 → 0.1370 | -90.02% |
| tuning | 57.9278 / 77.3190 | 0.1426 / 0.5657 | 62.7756 → 0.2484 | -99.60% |
| waveform | 2.6489 / 7.7615 | 0.7695 / 1.6044 | 3.9271 → 0.9782 | -75.09% |
| vcf | 0.5826 / 6.3357 | 0.5237 / 6.6848 | 2.0209 → 2.0639 | +2.13% |
| envelopes | 2.0215 / 2.7821 | 1.9104 / 2.6605 | 2.2116 → 2.0979 | -5.14% |
| amplitude | 4.6226 / 7.1624 | 4.4130 / 7.1719 | 5.2575 → 5.1027 | -2.94% |
| accent | 0.2368 / 1.0505 | 0.2306 / 1.0353 | 0.4402 → 0.4318 | -1.91% |
| slide | 0.2349 / 3.5675 | 0.3386 / 3.5675 | 1.0681 → 1.1458 | +7.28% |
| gate | 0.6567 / 0.7192 | 0.6567 / 0.7428 | 0.6723 → 0.6782 | +0.88% |
| behavior | 5.7806 / 5.9606 | 5.9457 / 7.5749 | 5.8256 → 6.3530 | +9.05% |

Native-unit median/p90 absolute errors for the assigned categories:

| Metric | Before | After |
|---|---:|---:|
| pitch / cents | 6.6842 / 7.4028 | 0.4848 / 1.2856 |
| tuning / cents | 289.6391 / 386.5952 | 0.7131 / 2.8285 |
| waveform / harmonic_rms_db | 3.4705 / 10.1620 | 0.6870 / 1.4865 |
| waveform / log_spectral_db | 1.8678 / 5.3610 | 0.9593 / 1.9608 |

Final renderer SHA-256: `afebe1d00fb5d5d45b4b41a17f1bbfcb4644178e307bde2d8c6f390761d48c14`.
Analysis SHA-256: `40c5b9e78b7da6fa0015099413f1cb4ed950867633254a0bfa2175cbbd2f1cc4`.
Subset report: `/tmp/osc-score.json`; full report: `/tmp/osc-overall-score.json`;
reproduced baseline: `/tmp/osc-before.json`.

The reproducible original build matched the published **4.2606** baseline.
Reports and hashed replay cache were written under `/tmp`, never into another
agent's assigned files. All captures remain included; failed analyses are not
discarded. Category contribution is `0.75*median + 0.25*p90`; a category subset
score is not the full-session objective.

Three category aggregates worsen by more than 2% and are explicitly retained:

- **VCF +2.13%:** all three native-unit median errors improve, but tail peak
  detection changes. At `vcf-grid-k60-c20-r00-v80` (open cutoff, zero resonance),
  the measured model peak switches from 523 to 4170 Hz although peak height
  remains −0.005528 dB and passband remains −0.037314 dB: an essentially flat
  response has no uniquely identifiable resonance peak. Its scored frequency
  error rises from −33% to +433%. Some high-resonance peak-height errors also
  increase with the physically lower source amplitude, e.g. k36/c16/r10/v80
  2.90 → 4.81 dB. The source is not gain-compensated to tune VCF saturation;
  those nonlinear response errors need the VCF owner's joint calibration.
- **Slide +7.28%:** only `slide-5-down` changes its accepted chirp-grid fit:
  21.25 → 20.5 ms, against hardware 22 ms. Other seven scored slide results
  are unchanged, including the existing low-confidence/bound penalty on
  `slide-24-up`. Physical C35 tau remains exactly 22 ms at every rate.
- **Behavior +9.05%:** all three patterns are saw at high resonance. Pitch
  calibration alone moves oscillation phase and the selected brightness
  fitting segment; the isolated ablation below produces a worse regression.
  This score cannot be restored by reverting the Q8 fit (which is inactive
  for saw) or follower while retaining accurate pitch. The final source model
  lowers absolute level error for all three patterns, but band-trajectory and
  pattern-classic brightness-fit errors remain higher than baseline.

A separate pitch/tuning-only ablation kept the original ideal saw and original
Q8 values. It reproduced the slide median change (0.2349 → 0.3386) and worsened
behavior further (median/p90 **6.0724/14.5181**, including a failed brightness
fit). This establishes that reverting the follower cannot restore the baseline
behavior score while retaining the pitch calibration. Pattern-classic's fitted
brightness segment changed from 26 to 40 frames when the centroid maximum
moved; this is a multi-note pattern, not a clean MEG decay measurement. This
tradeoff is retained explicitly rather than correcting pitch incorrectly or
modifying another stage to suppress a scoring feature. Pattern trajectory and
gate timing residuals remain work for the voice/envelope/sequencer owners.

## Validation and real-time limits

Package-scoped offline tests passed across every workspace package: **112 tests**
(14 oscillator tests and 98 in the other nine packages), with the four existing
manual timing tests ignored in normal runs. The oscillator manual release
benchmark also passed: **384,000 ticks at 192 kHz in 167.35 ms**, excluding
construction. Clippy for all oscillator targets passed with `-D warnings`, and
package-scoped rustfmt passed. All Cargo commands used `--offline` and explicit
package selections.

Full Circuit renderer CPU was checked separately, after scoring finished,
at 48 kHz for ten-second holds with open cutoff, zero resonance/env/decay/accent,
and full volume:

| MIDI key | One-core CPU | Max Newton iterations | Non-convergence / sanitised inputs |
|---|---:|---:|---:|
| 24 (C1) | 17.5% | 3 | 0 / 0 |
| 48 (C3) | 13.6% | 3 | 0 / 0 |

These are wall-clock measurements on this machine, not a worst-case audio
callback deadline guarantee. The final 346-capture replay completed without
renderer faults or failed analyses. A temporary offline replay auditor linked
the exact release engine/path-crate libraries and reused the renderer's JSON
and replay code, collecting diagnostics after each completed capture. It
processed **346 captures / 384,960,000 circuit ticks**, with **zero Newton
non-convergence and zero sanitised inputs** (maximum **4** iterations). The
only replay changes were post-processing diagnostics checks and discarding
WAV payload writes. Audit log: `/tmp/osc-solver-audit/results.txt`. No
workspace renderer, voice or solver source was modified.

Tests cover DAC annotations/saturation, bidirectional 22 ms slide, tuning
monotonicity/asymmetry, pitch and tracking at 44.1/48/96/192/384 kHz, capacitor
current/period sensitivity, Q28 saturation/triode KCL and load sensitivity,
source DC/endpoints, Q8 plateau/duty and octave-dependent even-harmonic null,
fitted C10/C11 time constants, invalid inputs/lifecycle, coherent saw alias energy, deterministic reset and two million
finite-state samples. The alias result is a numerical floor for a steady
analytic saw under ideal decimation, not square or whole-instrument alias
performance. The harmonic cap reduces bandwidth at very low notes; pitch
modulation can create sidebands, and the nonlinear square still relies on
voice oversampling/decimation. The crate remains `no_std`, has no unsafe,
allocation, locks, iterative solver or unbounded tick loops. No new allocation
counter instrumentation or hardware/SPICE validation is claimed.
