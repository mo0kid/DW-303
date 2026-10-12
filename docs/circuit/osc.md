# Pitch CV, VCO and square shaper — fitting round 2

Authority: [reconciled netlist](../../reference/schematic/netlist.md),
[VCO transcription](../../reference/schematic/transcription/vco.md), and
[binding contract](contracts.md). Round 2 changes only `crates/dw303-osc`
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

SLIDE high connects C35 through IC12 A to the ladder. Printed **0.22 uF** and
100k give **22 ms**. The round-2 fit uses **0.232 uF**, +5.45%, giving **23.2 ms**,
50.98 ms 10–90% and 106.84 ms to 99%. This is within an assumed ±10% capacitor
tolerance, not a verified part marking. IC11b follows this voltage at TP4/CV OUT.
SLIDE low connects C35 to the buffered output through IC12 B and precharges
it immediately. `PitchCv` retains the exact held-input RC solution. The selected
DAC resistors remain nominal; changing their resistance would be less defensible
than fitting C35. Output chirp fitting cannot distinguish capacitor tolerance
from switch resistance and downstream phase.

## Exponential converter and tuning

Q26 is the **2SC1583 matched NPN pair**. IC11a regulates Q26a against the
5.333 V reference. R103 220k supplies about 30.3 uA, supplemented by
TM4 47k/R102. R104 2.2k/C34 1 nF regulator dynamics are omitted. O1, the
R102 unit ambiguity, remains open; TM4's effective current is calibrated.
The service adjustment is A=110 Hz at CV=2.75 V. The retained round-1 session fit changes the
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

The retained cycle charge is **63 nC**, and the fitted A sink current **6.9033 uA**
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

Fitted **R35=103.660k**, **R45=22.776k**, **C10=8.455 nF**, and **C11=0.819 uF**
use plausible ±5% resistor / ±20% capacitor search bounds. Those bounds are
assumptions, not verified tolerances for the original parts. Q8 estimates are
**Veb=0.5425 V**, **beta=389.5**, **Vec_sat=0.08055 V**. The unloaded emitter recharge
is now **18.654 ms**, and off-state C10 discharge **0.961 ms**. The Q8 junction
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
| C35 | 0.22 → 0.232 uF (+5.45%); eight slide chirp fits, 22 → 23.2 ms physical tau; assumed ±10% capacitor tolerance; output-only, grid-limited residual |
| TM4 `reference_hz` | 32.703195663 → 32.577173303 Hz; remove median +6.6842 cents across 49 notes; residual pitch median/p90 about 0.485/1.286 cents |
| VR2 pin-3 / `tuning_octaves`, O3 | ±700 → −1086.620/+891.852 cents; effective reference-rail hypothesis 5.333 → 5.402046 V; residual tuning median/p90 about 0.713/2.829 cents; wiring and intermediate rotations unidentified |
| VR2 `tuning_resistance` | 50k → 44.945454k (−10.11%); endpoint asymmetry with R118 loading; actual pot tolerance unknown |
| R100 PTC coefficient | Ideal 1/300 per kelvin; curve unprinted; no warm-up/drift capture |
| C33 lower endpoint / Q24/Q25, O5/O6 | Former ideal source annotation 5.5 → capacitor 5.2 V; Q24/Q25 difference absorbs unprinted junction physics; Q25 NPN chosen from function |
| C33 upper endpoint / Q27/D25, O6 | Former ideal source annotation 12 → capacitor 11.5 V; reset drops inferred, not individually identified |
| Q28 IDSS / pinch-off | Ideal follower → 0.6 mA / −1 V; harmonic balance across five octaves; no internal voltage or device curve; near lower search boundaries |
| Q28 model reduction | Shockley saturation and triode load line; omit gate current, capacitances and Q8 base-network back-loading into Q28; estimates covary with Q8 and reset endpoints |
| R35 | 100k → 103.660k (+3.66%); square harmonic fit; ±5% search bound assumed |
| R45 | 22k → 22.776k (+3.53%); square harmonic fit; ±5% search bound assumed |
| C10 | 10 → 8.455 nF (−15.45%); square harmonic fit; ±20% search bound assumed |
| C11 | 1 → 0.819 uF (−18.1%); octave duty/tilt harmonic fit; ±20% search bound assumed |
| Q8 Veb, O6 | 0.65 → 0.5425 V; constant forward drop, near lower fit boundary; real current/temperature dependence omitted |
| Q8 beta | Estimated 200 → 389.5; fits waveform, not independently measured |
| Q8 Vec_sat | Estimated 0.2 → 0.08055 V; fits waveform, not independently measured |
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

## Round-2 fitting evidence

The starting release binary exactly reproduced **1.7142928144**, including all
round-1 category metrics. Forty-two bounded Q8 coordinate trials used the
unchanged harness on temporary symlinks to the ten read-only waveform pairs.
A constructor/source-harmonic surrogate was used to propose candidates; only
real engine replay accepted the fit. No per-key correction, source equaliser,
level compensation, note reset or scoring change was introduced.

Round-1 → round-2: R35 **102.940 → 103.660k**, C10 **8.402 → 8.455 nF**, Q8
Veb **0.55 → 0.5425 V**, beta **362 → 389.5**, Vec_sat **0.08 → 0.08055 V**.
R45 and C11 stay **22.776k / 0.819 uF**. Q8 estimates remain correlated with
source shape and output filtering, not independently measured transistor data.

A stronger Q28 curvature trial (C33 endpoints 5.268/11.324 V, IDSS 1.256 mA,
|Vp| 3.421 V) reduced saw harmonic RMS at C3/C4/C5 from 0.551/1.401/1.460 to
0.415/1.235/1.253 dB, but increased C2 error 0.447 → 0.752 dB and square errors
into 6–12 dB. Bounded Q8 retuning could not recover it. It was rejected. The
retained Q28/C33 model and all saw renders are unchanged. At roughly 4 kHz,
the saw errors from three octaves are similar: H16/C3 +0.728 dB, H8/C4
+0.740 dB, H4/C5 +0.717 dB. At roughly 8 kHz, H16/C4 +2.433 dB and H8/C5
+1.711 dB remain. This common trend suggests output-path bandwidth; finite reset
or Q28 bandwidth may contribute, but output-only evidence does not identify
which circuit element. An arbitrary oscillator low-pass would conceal that
uncertainty, and a large reset duration would impair the fitted pitch tracking.

## Low-note amplitude investigation

The VCO receives CV and tuning, but no gate/trigger. Read-only voice inspection
confirms `vco.reset()` runs at voice reset/prepare, while each circuit tick
advances the VCO before processing trigger/VEG. Note-on does not reset phase.
Q28 is a static load line; there is no low-frequency coupling capacitor in the
saw path. The modeled saw has identical **1.690597 V AC RMS** across MIDI keys
24–26 at 48/96/192 kHz, measured over complete cycles, and no drift across 32
cycles. A new test checks this independently of the amplitude scorer. C11/Q8
state cannot change the modeled saw; omitted Q8 back-loading is still VERIFY.

For baseline `pitch-k24/25/26-t50`, sustained 0.2–2.5 s, normalising each segment
by its own RMS and using the same dB distance/floor as the harness:

| RMS window | key 24 | key 25 | key 26 |
|---|---:|---:|---:|
| 10 ms (harness) | 16.604 dB | 15.581 dB | 14.732 dB |
| 40 ms (diagnostic) | 2.050 dB | 2.164 dB | 2.176 dB |
| 100 ms (diagnostic) | 0.882 dB | 1.077 dB | 0.833 dB |

A diagnostic constant shift of 12.5/10.5/10.5 ms reduces 10-ms errors to
3.758/3.831/3.392 dB. No shift or longer RMS window is applied to scored audio.
C1's 30.7 ms period is longer than the 10 ms window; hardware/model reset pulses
fall in different windows and deep inter-pulse troughs amplify dB error.
Different free-running phase and waveform/coupling shape therefore contribute
strongly to this metric. The surviving source's phase-independent amplitude
is steady, so there is no evidence for a long oscillator amplitude decay to
repair. This does not rule out missing oscillator/device effects in hardware,
or downstream envelope/release error. Internal Q28 voltage captures or
phase-controlled repetitions would be needed to separate those causes.

## Full-session scoring and regression audit

Overall weighted error **1.7143 → 1.6553** (-3.44%). Both runs analyse **346 captures with zero failures**.

| Category | Before median / p90 | After median / p90 | Weighted aggregate before → after | Change |
|---|---:|---:|---:|---:|
| pitch | 0.0957 / 0.2665 | 0.0957 / 0.2665 | 0.1384 → 0.1384 | +0.00% |
| tuning | 0.0995 / 0.5594 | 0.0995 / 0.5594 | 0.2145 → 0.2145 | +0.00% |
| waveform | 1.2043 / 1.6337 | 1.2016 / 1.5846 | 1.3116 → 1.2974 | -1.09% |
| vcf | 0.4838 / 4.6203 | 0.4838 / 4.6203 | 1.5180 → 1.5180 | +0.00% |
| envelopes | 0.7121 / 0.8997 | 0.7121 / 0.8997 | 0.7590 → 0.7590 | +0.00% |
| amplitude | 2.1973 / 5.6355 | 2.1931 / 5.6608 | 3.0568 → 3.0600 | +0.10% |
| accent | 0.2385 / 0.9681 | 0.2385 / 0.9681 | 0.4209 → 0.4209 | +0.00% |
| slide | 0.3248 / 0.5074 | 0.0000 / 0.1565 | 0.3705 → 0.0391 | -89.44% |
| gate | 0.6567 / 0.7192 | 0.6567 / 0.7192 | 0.6723 → 0.6723 | +0.00% |
| behavior | 5.1122 / 13.5298 | 4.7740 / 13.4621 | 7.2166 → 6.9460 | -3.75% |

No category worsens by more than 2%. Amplitude's small +0.10% regression is
retained; the overall score and slide/pattern behavior improve. Saw, pitch,
tuning, VCF, envelopes, accent and gate metrics are unchanged. Waveform's
weighted aggregate improves **1.09%**, primarily its p90 and square C5 log
spectrum. This is a small compromise, not an across-the-board harmonic fit:
median/p90 harmonic RMS **1.364/1.502 → 1.385/1.593 dB**, and log-spectral
**1.114/1.889 → 1.119/1.881 dB**. The objective includes both distances.

| Square MIDI key | Harmonic RMS before → after (dB) | Log-spectral before → after (dB) |
|---|---:|---:|
| 24 | 1.068 → 1.580 | 1.441 → 1.462 |
| 36 | 1.359 → 1.258 | 1.817 → 1.850 |
| 48 | 1.496 → 1.711 | 1.163 → 1.185 |
| 60 | 1.370 → 1.370 | 1.065 → 1.054 |
| 72 | 1.557 → 1.553 | 2.533 → 2.164 |

Slide native absolute log2-tau error median/p90 **0.0812/0.1269 → 0/0.0391**;
weighted category **0.3705 → 0.0391**. Six of eight chirp estimates land in the
same grid cell as hardware; upward one-semitone and one-octave residuals remain
0.0500 and 0.0344 octaves. Zero median is grid agreement, not proof of exact C35
identification. Fitted physical tau remains **23.2 ms** independently of rate.
Pitch absolute cents median/p90 **0.4786/1.3325**, tuning **0.4975/2.7970**, are
unchanged. The slow RC fit also lowers weighted behavior error **3.75%**.

Final renderer SHA-256: `298870ea564a804c846be21e46f0d50299184c274e4e6fb401bc6ada0e3e99fa`.

Analysis SHA-256: `40c5b9e78b7da6fa0015099413f1cb4ed950867633254a0bfa2175cbbd2f1cc4`.

Artifacts: `/tmp/osc-round2-before.json`, `/tmp/osc-round2-score.json`,
`/tmp/osc-round2-overall.json`, with matching Markdown. All replay caches,
trial reports and diagnostic tools are under `/tmp`; capture data and harness
sources remain read-only. Only oscillator source/tests and this stage document
are changed. No new dependencies, commits or branch/history operations.

## Validation and remaining limits

Offline package-scoped tests across all ten workspace packages: **115 passed,
4 existing manual timing tests ignored**. Oscillator tests: **15 passed**, with
one manual benchmark ignored in the normal run. Oscillator all-target clippy
with `-D warnings`, rustfmt check and whitespace checks pass. The explicitly run
release benchmark passes: **384,000 ticks at 192 kHz in 186.51 ms**; it excludes
construction and cycles held pitches rather than sustained glide.

A scratch auditor linked the final release engine libraries and reused the
renderer replay/JSON code, checking VCF diagnostics after every capture. It
processed **346 captures / 384,960,000 circuit ticks**, **zero Newton
non-convergence**, **zero sanitised inputs**, maximum **4 iterations**. It only
added post-render diagnostics and discarded WAV payload writes; no workspace
renderer/voice/solver source changed. Log:
`/tmp/osc-round2-solver-audit/results.txt`.

Full Circuit renderer CPU, 10-second held notes at host 48 kHz, open cutoff,
zero resonance/env/decay/accent and full volume, measured serially:

| MIDI key | Starting oscillator wall CPU | Final wall CPU | Final user / sys CPU time | Max Newton iterations |
|---|---:|---:|---:|---:|
| 24 | 15.8% | 15.7% | 1.56 / 0.00 s | 3 |
| 48 | 14.0% | 13.6% | 1.35 / 0.00 s | 3 |

Both final holds report zero solver non-convergence and sanitised inputs.
Earlier wall measurements during shared machine activity were 30.8%/25.1%;
repeated `/usr/bin/time` measurements and the starting-component comparison
established the figures above. This is a measured hold workload, not a
worst-case callback deadline guarantee. Logs are
`/tmp/osc-round2-final-cpu-k24.log` and `k48.log`; final release was rebuilt after
the starting-component CPU ablation and its hash matches the scored binary.

The tick algorithm is unchanged: no added allocation, locks, solver, loops or
math calls. The crate stays `no_std`, safe Rust, with bounded harmonic loops.
Its finite harmonic cap reduces low-note high-frequency bandwidth; nonlinear
square shaping still relies on voice oversampling/decimation. Instantaneous
reset, ideal Q26 current, static Q28 without gate current/capacitances/back-load,
and piecewise-linear Q8 remain model weaknesses. Resolving residual bandwidth
and even-harmonic cancellations needs better internal-node/device measurements.
No SPICE or hardware-internal validation is claimed.
