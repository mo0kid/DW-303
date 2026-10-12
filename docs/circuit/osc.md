# Pitch CV, VCO and square shaper — current fits

Authority: [reconciled netlist](../../reference/schematic/netlist.md),
[VCO transcription](../../reference/schematic/transcription/vco.md), and
[binding contract](contracts.md). Fitting rounds 2 and 3 change only `crates/dw303-osc`
and this document. The session-2 tuning round has the same scope. No dependencies
were added. Calibration uses the 346 session-1 and 274 session-2
read-only TD-3 WAV/JSON pairs dated 2026-10-12, with the unchanged
[scoring harness](../harness.md). These are unit-specific output-audio fits,
not internal-node measurements or proof of original TB-303 accuracy.

`Components::NOMINAL` retains its integration API name but now contains the
fitted defaults below. Printed resistors/capacitors not listed as fitted remain
at their transcribed values. The earlier source annotations, 5.5–12 V, are no
longer substituted for C33 voltage: `saw_low` and `saw_high` now describe the
capacitor cycle endpoints **before** the explicitly modelled Q28 follower.
`VcoOut.saw` remains Q28 source voltage and `VcoOut.square` Q8 collector voltage;
method signatures and SI units remain unchanged. Round 3 changes Q8's junction
law and fitted base/emitter parameters. The session-2 tuning round fits VR2's
resistive track. Round 10 refits Q8 parameters with the revised VCF coupling;
its combined results and limitations are in
[the round-10 report](vcf-fitting-round10-2026-10-12.md).
Earlier score sections are historical evidence.

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

VR2 remains a **50k(B) pot**, with a small fitted departure from the ideal linear
resistance track. The previous law loaded a symmetric
midpoint voltage and forced ±700 cents. A pot between ground and an effective
pin-3 rail instead loads toward the reference through R118:

```
u = 2*x - 1
z(x) = x + x*(1-x)*u*(bow + skew*u)
Rth(x) = VR2*z(x)*(1-z(x))
Vw(x) = (Vrail*z(x)*R118 + Vref*Rth(x)) / (R118 + Rth(x))
ΔVbe_tune = gain_tune * (Vw(x) - Vw(0.5))
```

The inferred effective rail is **5.402046 V**, near the hypothesised 5.333 V
reference, and VR2 is fitted **50k → 44.945454k**. This gives **−1086.620 /
+891.852 cents** relative to centre. `tuning_octaves` represents the total
endpoint span, **1400 → 1978.472 cents**, rather than half the range. The
rail is derived from that span and the nominal expo gain; it is an effective
calibration, not identification of the unreadable pin-3 glyph O3. The load
uses a stiff 64/12 V reference approximation and omits the small CV-dependent
base-node/loading interaction. The original session measured only rotations
0, 0.5 and 1. Session 2 adds .25/.75 at MIDI 24/48/72. Their consistent residuals
require a modest change in the intermediate wiper fraction; see the latest fit
below. The smooth track approximation uses the same segment resistances for
voltage division and Thevenin loading, rather than a direct cents correction.
The panel fractions retain the 0–10 legend over 300 degrees with **zero common
placement offset**. Track nonlinearity and small operator placement errors
remain inseparable without direct resistance/angle measurements.

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
period inversely while leaving converter current fixed. Reset duration remains an ideal regenerative step; output captures do not
isolate its duration (see the round-3 reset audit below).

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

## Q8 square shaper — current round-3 model

The topology is unchanged: **R35** directly drives Q8's base in parallel with
**C10 + R34 10k**; **R45/C11** feed/store the emitter voltage; **R36 10k** returns
the collector to **5.333 V**. There is no coupling high-pass in this block.
C10 voltage and C11 emitter voltage are physical state variables. Backward-Euler
companions eliminate the base/emitter nodes in favour of base current. The
forward-junction law now varies with current instead of switching at one fixed
Veb:

```
ib_ref = 100 uA / beta
Veb(ib) = Veb_ref + Vt * ln(ib / ib_ref)
R_active = 1/g_base + (beta+1)/g_emitter
R_saturated = 1/g_base + 1/(g_emitter+g_collector)
R*ib + Vt*ln(ib/ib_ref) = Vdrive - Veb_ref
```

`transistor_vbe` now means **Veb at 100 uA forward collector current**, at the
selected model temperature; it is no longer a fixed switching drop. The
reference current only parameterises the junction curve: equivalently
`Is = 100 uA * exp(-Veb_ref/Vt)`. It is not an extra amplitude control.
Forward-active `ic=beta*ib`; in saturation `Vc=Ve-Vec_sat` and `ie=ib+ic`,
including collector current in C11's charge balance. This is a reduced
exponential-junction model, **not full Ebers–Moll**. The saturation branch keeps
the same forward base-junction law; reverse-active transport, beta rolloff,
Early effect and parasitic capacitances remain omitted and marked VERIFY.

The scalar inverse is `u + ln(u) = t` (`u=W(exp(t))`), with four bounded Halley
steps and no warm-start dependence. Rate-dependent resistances, current scales
and logarithmic offsets are prepared outside `tick`. The hot logarithm uses
binary range reduction and the atanh series through degree 13. Its omitted
series tail is bounded by 4.6e-13; tests compare it with independent `libm::log`
to 5e-13 and check inverse residuals below 5e-10 over drives -29.999 to 20,000.
Flushing `t<=-30` omits at most `Vt*exp(-30) < 3.3e-15 V` across the effective
load at 400 K. Tick uses fixed arrays already present in the VCO, bounded loops,
no allocation, locks or new dependencies.

Final component/device estimates and octave residuals are recorded in the
round-3 evidence below. The printed R34/R36, rails, C33, Q28 and all pitch,
tuning and slide defaults are retained. No source equaliser or per-key
correction was introduced.

## Complete VERIFY register

Every code `VERIFY:` item, including inherited approximations, is covered here.

| Item | Nominal / previous → fitted estimate; evidence and limit |
|---|---|
| C35 | 0.22 → 0.232 uF (+5.45%); eight slide chirp fits, 22 → 23.2 ms physical tau; assumed ±10% capacitor tolerance; output-only, grid-limited residual |
| TM4 `reference_hz` | 32.703195663 → 32.577173303 Hz; remove median +6.6842 cents across 49 notes; residual pitch median/p90 about 0.485/1.286 cents |
| VR2 pin-3 / `tuning_octaves`, O3 | ±700 → −1086.620/+891.852 cents; effective reference-rail hypothesis 5.333 → 5.402046 V; retained endpoint calibration; pin-3 wiring remains unidentified; combined residuals below |
| VR2 `tuning_resistance` | 50k → 44.945454k (−10.11%); endpoint asymmetry with R118 loading; actual pot tolerance unknown |
| VR2 `tuning_track_bow` | Ideal B track 0 → 0.102496922; central differential resistance density 1 → 1.051248 of mean; session-2 .25/.75 same-key tuning medians; assumed smooth track variation, not a measured manufacturer law |
| VR2 `tuning_track_skew` | Ideal B track 0 → −0.038276829; electrical fractions .25/.75 → .238597/.757815; no common angle offset; track shape and placement covary, intermediate rotations other than .25/.75 unmeasured; combined residuals below |
| R100 PTC coefficient | Ideal 1/300 per kelvin; curve unprinted; no warm-up/drift capture |
| C33 lower endpoint / Q24/Q25, O5/O6 | Former ideal source annotation 5.5 → capacitor 5.2 V; Q24/Q25 difference absorbs unprinted junction physics; Q25 NPN chosen from function |
| C33 upper endpoint / Q27/D25, O6 | Former ideal source annotation 12 → capacitor 11.5 V; reset drops inferred, not individually identified |
| Q28 IDSS / pinch-off | Ideal follower → 0.6 mA / −1 V; harmonic balance across five octaves; no internal voltage or device curve; near lower search boundaries |
| Q28 model reduction | Shockley saturation and triode load line; omit gate current, capacitances and Q8 base-network back-loading into Q28; estimates covary with Q8 and reset endpoints |
| R35 | 100k → 102.3096k (+2.310%); round-10 octave fit with current VCF coupling; round-3 was 100.435k; ±5% search bound assumed |
| R45 | 22k → 22.7705k (+3.502%); round-10 octave fit; round-3 was 22.7955k; ±5% search bound assumed |
| C10 | 10 → 9.282418 nF (−7.18%); round-3 square harmonic/phase fit; round-2 was 8.455 nF; ±20% search bound assumed |
| C11 | 1 → 0.8372997 uF (−16.27%); round-10 octave duty/tilt fit; round-3 was 0.8342997 uF; ±20% search bound assumed |
| Q8 Veb, O6 | Former fixed 0.65 / round-2 0.5425 V → exponential-law reference 0.500 V at Ic=100 uA, 300 K; different parameter meaning; effective Is≈3.98e-13 A; lower 0.50–0.70 V search boundary; device curve and temperature law unmeasured |
| Q8 beta | Estimated 200 → 428.5591 (round-3 423.5591); constant forward beta; fits waveform, not independently measured |
| Q8 Vec_sat | Estimated 0.2 → 0.04573268 V (round-3 0.04323268 V); constant saturation clamp; fits waveform, not independently measured |
| `cv_for_note` | IC10 supply unprinted; ideal logic high = 64/12 V; selected DAC resistor mismatch omitted |
| `PitchCv`, O2 | Already-latched note; shared IC9/slide clock ambiguity delegated to sequencer |
| `PitchCv`, O7/O6 | Ideal IC12 and IC11b; omit switch Ron, settling, clamps and Q29/Q30 switching; IC12 supply unprinted |
| `SquareShaper` reduction | Forward exponential base junction with beta, and a Vec clamp with KCL; saturated base current still uses the forward law, omitting the reverse junction, reverse-active transport, beta rolloff, Early effect and parasitics; no measured transistor model |
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
retained Q28/C33 model and all saw renders are unchanged. At roughly 2 kHz,
the saw errors from three octaves are similar: H16/C3 +0.728 dB, H8/C4
+0.740 dB, H4/C5 +0.717 dB. At roughly 4 kHz, H16/C4 +2.433 dB and H8/C5
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

## Round-2 full-session scoring and regression audit (harness v1)

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

## Round-2 validation and remaining limits

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


## Round-3 fitting evidence (harness v2)

The starting release reproduces **1.0198058937** overall, matching
`tools/analysis/harness/results/round2-v2.json`. Every trial and build runs in
this oscillator worktree. Scratch scripts, immutable capture symlinks, hashed
replay caches and score artifacts are under `/tmp`; captures, scoring code,
other crates, manifests and dependencies are unchanged. No commits are made.

A source-harmonic surrogate proposed bounded R35/R45/C10/C11 and unprinted Q8
device estimates. It compared complex Fourier coefficients over five octaves
with relative output harmonics, then real engine replay accepted candidates.
Seventy local coordinate replays compared the constant-drop and exponential
junction models. The cheaper constant-drop fit reached **0.9412** waveform
aggregate, while the retained exponential model reaches **0.8029**. Joint
Q28/end-point proposals did not establish an identifiable improvement and were
not adopted. No note-phase reset, source gain correction, note table, downstream
compensation or metric change was introduced.

### Nominal and previous values to fitted defaults

Printed tolerances are unavailable. Search bounds assume ±5% for resistors,
±20% for capacitors, Veb_ref 0.50–0.70 V, beta 100–600 and Vec_sat 0.04–0.20 V.
The device bounds are estimates, not manufacturer specifications. Veb_ref ends
at its lower bound and covaries with beta and saturation; audio cannot uniquely
identify these parameters.

| Parameter | Transcribed nominal / prior estimate | Round 2 | Round 3 |
|---|---:|---:|---:|
| R35 | 100 kΩ | 103.660 kΩ | 100.4346 kΩ (+0.435%) |
| R45 | 22 kΩ | 22.776 kΩ | 22.7955 kΩ (+3.62%) |
| C10 | 10 nF | 8.455 nF | 9.282418 nF (−7.18%) |
| C11 | 1 µF | 0.819 µF | 0.8342997 µF (−16.57%) |
| Q8 Veb | Estimated fixed 0.65 V | Fixed 0.5425 V | 0.500 V at Ic=100 µA, 300 K |
| Q8 beta | Estimated 200 | 389.5 | 423.5591 |
| Q8 Vec_sat | Estimated 0.2 V | 0.08055 V | 0.04323268 V |

C11 unloaded recharge is **19.018 ms**; C10's off-state time constant is
**1.0251 ms**. All pitch, tuning, slide, C33 and Q28 parameters remain at the
round-2 defaults. Q8's reference-current parameterisation changes the meaning
of its Veb field; 0.5425 → 0.500 V is not a constant threshold adjustment.

### Harmonic magnitude and phase evidence

Only CUTOFF=1, RESONANCE=0, ENV MOD=0, unaccented saw/square octave captures
are used for waveform acceptance. Each sustained segment is 0.6–1.1 s after
its latency-aligned first onset, matching the harness. Magnitudes below the
harness's hardware-relative −60 dB threshold remain censored.

| Square MIDI key | Harmonic RMS before → after (dB) | Log bands before → after (dB) |
|---|---:|---:|
| 24 | 1.729 → 1.312 | 1.186 → 1.471 |
| 36 | 1.007 → 0.678 | 1.412 → 0.448 |
| 48 | 1.258 → 0.476 | 0.832 → 0.578 |
| 60 | 0.757 → 0.601 | 0.750 → 0.636 |
| 72 | 0.922 → 0.857 | 2.089 → 1.840 |

All five square harmonic RMS distances improve. C3's surviving even lines are
better matched: MIDI 48 H10/H12/H14 signed residuals change **+1.835/+2.275/
+2.778 → +0.036/−0.192/−0.426 dB**. Low-order cancellation remains imperfect:
its H2/H4 residuals are **+0.976/+1.225 dB**. At MIDI 24, H19 changes
**−5.921 → −1.698 dB**, but the log-band distance increases **0.285 dB**.
That single-capture tradeoff is retained because its first-20 harmonic fit and
the aggregate improve. Upper square MIDI 72 retains **+1.615 dB at H11** and
**1.840 dB** log-band distance.

Phase uses Hann-weighted complex demodulation at each recording's measured
fundamental. `psi_h = arg(X_h) - h*arg(X_1)` removes free-running cycle phase.
Subtracting saw psi from square psi at the same MIDI key suppresses common
linear output-path phase, then comparing model minus hardware constrains Q8's
relative shape. Only lines at least −40 dB below **each** hardware waveform's
largest first-20 harmonic are retained; H1 is excluded because its invariant
is zero. No time shift, phase correction, deconvolution or phase term is added
to the harness.

| MIDI key | Resolved lines | Square-minus-saw relative-phase RMS before → after (degrees) |
|---|---:|---:|
| 24 | 19 | 33.218 → 27.907 |
| 36 | 17 | 13.602 → 11.259 |
| 48 | 8 | 5.609 → 5.003 |
| 60 | 7 | 3.161 → 3.036 |
| 72 | 4 | 2.462 → 2.361 |

These diagnostic phases improve at all five octaves, but are not internal-node
phase measurements. Separate hardware takes, weak/null harmonics, envelope
age and nonlinear output loading limit exact cancellation. Phase artifacts:
`/tmp/osc-r3-phase.json` and `/tmp/osc-r3-final-phase.json`.

### Saw reset and upper-harmonic audit

Saw renders and all saw harmonic distances are **exactly unchanged**. The
printed waveform draws a vertical regenerative return within the annotated
9 ms A-key cycle; the drawing does not determine a microsecond reset time.
The audio-output folds include the open VCF, coupling and VCA and cannot be
read as C33 charge/discharge edges. The historical round-2 approximate frequency
labels above have been corrected by a factor of two; explicit MIDI keys and
measured carrier frequencies avoid octave-label ambiguity here.

With fixed converter current, finite reset time implies
`f = f_ideal / (1 + f_ideal*t_reset)`. A linear reset contributes approximately
`sinc(pi*f_harmonic*t_reset)` to the harmonic magnitudes. A diagnostic best
centre/width affine retrim against the 49 pitch captures gives:

| Linear reset duration | Attenuation at 8 kHz | Predicted pitch median / p90 after best affine retrim (cents) |
|---|---:|---:|
| 5 µs | 0.023 dB | 0.338 / 0.894 |
| 10 µs | 0.092 dB | 0.332 / 0.780 |
| 25 µs | 0.579 dB | 0.966 / 1.852 |
| 50 µs | 2.420 dB | 2.622 / 4.293 |

This is an analytical sensitivity check, not rendered/fitted reset circuitry.
Short reset times are not excluded by pitch, but have too little rolloff to
explain the remaining upper harmonics. Longer times worsen the present pitch
p90 of **1.325 cents** even with a generous independent affine retrim. Other
nonlinear recovery laws are not ruled out. No reset-duration parameter or
retrim is adopted without identifiable recovery evidence.

Current resolved saw residuals at **about 4.17 kHz** are **+1.451 dB**
(MIDI 60 H16) and **+0.932 dB** (MIDI 72 H8); MIDI 72 H11 at **5.73 kHz** is
**+1.426 dB**. A 50 µs linear reset attenuates 4.17 kHz by only about 0.63 dB.
Higher saw lines are often below the −60 dB hardware censoring boundary, so
8 kHz is a sensitivity example, not a confidently measured isolated source
pole. Low MIDI 24/36 balance also retains output-coupling uncertainty. Finite
reset, Q28/Q8 back-loading, source parasitics, VCF/VCA and interface response
cannot be separated by these output captures. A source low-pass would not
identify the responsible circuit element; none is introduced.

### Full-session regression audit

Both full runs score **346 captures with zero failures**, under the same
harness-v2 analysis hash. Overall weighted score **1.0198058937 → 0.9920475818**
(**−2.72%**). Waveform aggregate **1.0527177208 → 0.8028929137** (**−23.73%**).

| Category | Before median / p90 | After median / p90 | Aggregate before → after |
|---|---:|---:|---:|
| pitch | 0.0957 / 0.2651 | 0.0957 / 0.2651 | 0.1380 → 0.1380 |
| tuning | 0.0995 / 0.5593 | 0.0995 / 0.5593 | 0.2145 → 0.2145 |
| waveform | 0.9161 / 1.4625 | 0.6195 / 1.3531 | 1.0527 → 0.8029 |
| vcf | 0.5762 / 4.1235 | 0.5762 / 4.1235 | 1.4631 → 1.4631 |
| envelopes | 0.6346 / 0.8764 | 0.6346 / 0.8764 | 0.6951 → 0.6951 |
| amplitude | 1.0204 / 3.0017 | 1.0204 / 3.0017 | 1.5157 → 1.5157 |
| accent | 0.2452 / 0.9055 | 0.2452 / 0.9055 | 0.4103 → 0.4103 |
| slide | 0.0000 / 0.1565 | 0.0000 / 0.1565 | 0.0391 → 0.0391 |
| gate | 0.5174 / 0.8480 | 0.5174 / 0.8480 | 0.6001 → 0.6001 |
| behavior | 2.8152 / 2.9732 | 2.8152 / 2.9732 | 2.8547 → 2.8547 |

Every other category aggregate, including pitch/tuning/slide and amplitude,
is unchanged. No category worsens by 2%. Waveform native harmonic RMS
median/p90 **0.8740/1.3049 → 0.7740/1.0742 dB**; log-band median/p90
**0.8068/1.4798 → 0.6074/1.5082 dB**. The small log-band p90 increase is included
in the accepted aggregate, not hidden.

Artifacts: `/tmp/osc-fit-before.json`, `/tmp/osc-r3-score.json`,
`/tmp/osc-r3-overall.json`, with matching Markdown. Final renderer SHA-256:
`b13259123a93622c5872ab7bc4c81f21b78d25622346032205316000ab16317c`.
Analysis SHA-256:
`c62e2c0e72028d55d33f49a37f3d2acee6917575107bd33dfe376d10a1e741bb`.

### Validation and remaining limits

Offline package-scoped tests across all ten workspace packages: **119 passed,
4 existing manual timing tests ignored**. Oscillator tests: **17 passed**, with
its one manual timing test ignored normally. Its separately run release timing
test passes: **384,000 VCO ticks at 192 kHz in 187.90 ms**. All-target oscillator
clippy with warnings denied, scoped rustfmt and whitespace checks pass.
New tests independently check the exponential load-line residual over the
bounded drive range and Kirchhoff current balance for C10/C11 at all supported
rate extremes. The previous exact off-state collector equality is now a
**1e-7 V tolerance**, because a forward exponential has subthreshold current.

Serial Full Circuit renderer holds, 10 s at host 48 kHz, open cutoff, minimum
resonance/env/decay/accent, full volume:

| MIDI key | Wall CPU (% of one core) | User / sys CPU (s) | Max Newton iterations |
|---|---:|---:|---:|
| 24 | 17.4% | 1.73 / 0.00 | 3 |
| 48 | 15.2% | 1.51 / 0.00 | 3 |

The initial unoptimised log-based inverse exceeded the CPU budget and was
replaced by cached companion constants and the verified reduced-range log
polynomial. The final timing meets the approximate 18% hold-workload budget;
construction, worst-case callback deadlines and sustained glides are not
claimed by those holds. The hot path remains safe `no_std` Rust with no
allocation or locks, a fixed four-step inverse and the existing bounded
harmonic loops.

A scratch auditor copies the current renderer's MIDI replay/parser and links
the final release engine libraries, adding only post-render VCF diagnostics.
It covers **346 captures / 384,960,000 circuit ticks**, **zero Newton
non-convergence**, **zero sanitised inputs**, maximum **4 iterations**.
Audit source/output: `/tmp/osc-r3-solver-audit`; result log: `results.txt`.
No workspace renderer, voice or VCF source is changed for that audit.

The complete VERIFY register above covers every code annotation. Principal
limits remain the inferred Q8 device/reference curve and reduced saturation
law, unmeasured parasitics/back-loading and reset recovery, finite source
harmonic bandwidth, output-only identification, and inherited DAC/switch/expo
approximations. There is no manufacturer Q8 model, isolated internal-node TD-3
measurement or new SPICE validation in this fitting round.


## Session-2 tuning fit (harness v5)

The starting release exactly reproduced the combined baseline **0.9660342817**
from `s12-baseline.json`: 620 sidecars, 629 analysis rows, zero failures. The
same-key centre reference for session-2 tuning is session-1 `pitch-kKEY-t50`,
as specified by the unchanged harness. All builds, replays and scores ran in
this oscillator worktree; source changes are limited to this crate and document.

An ideal B track with the existing loading predicts about **−478.63 / +433.72
cents** at .25/.75. Hardware medians are **−502.788 / +447.289 cents**. The
signed errors are consistent across all three keys: .25 is +23.5 to +24.2 cents,
.75 is −14.3 to −12.1 cents. Altering only total VR2 resistance and effective
rail cannot retain the accurate endpoints while matching both intermediates.
An analytical least-squares trial of that two-parameter linear-track model has
12.26 cents RMS residual. Adding only symmetric track bow lowers it to 2.69
cents RMS, but retains a common intermediate bias. Those are analytical trials
with baseline estimator offsets, not claims of separate engine scores.

The retained fit changes only the resistance distribution of the B track. Let
`u=2*x−1` for panel rotation `x`. Normalized differential track resistance is

```
rho(x) = 1 + (bow/2)*(1 - 3*u*u) + skew*(u - 2*u*u*u)
z(x) = integral_0^x rho(t) dt
     = x + x*(1-x)*u*(bow + skew*u)
R_lower = VR2*z(x); R_upper = VR2*(1-z(x))
```

This is a smooth low-order approximation to a nonuniform resistive track,
**VERIFY**, not an identified factory taper. Both voltage division and source
resistance use these same track segments before the existing R118 loading.
`bow: 0 → 0.10249692235426172`, `skew: 0 → −0.03827682943247928`; nominal
zero means the printed ideal linear B track. Total VR2 **50k → 44.945454k**,
R118 **100k**, effective rail **5.402046 V** and total span **1978.472 cents**
are retained from the previous calibration. No other fitted component changes.

The track's differential density is **0.8592–1.0535** times its mean; it stays
strictly positive. Validation bounds bow ±.25 and skew ±.15 guarantee density
at least .6. The midpoint remains exactly .5, and both endpoints remain exact.
Unloaded wiper fractions at .25/.75 become **.238596687 / .757814860**; maximum
fraction displacement anywhere is **.012051**, about 1.21% of total resistance.
Thus no pot-end tolerance is spent beyond the previous total-resistance fit.
These are assumed plausible B-track variations, not a tolerance specification
for the original K161B0-50kB or the TD-3 replacement.

The coefficients fit the median hardware-minus-baseline-estimator target at
each intermediate position across the three keys. The static expo law becomes
**−502.746697 / +447.290989 cents**. Real replay, rather than that surrogate,
supplies all reported scores. Small endpoint replay-estimator differences
(<.005 cents) remain because smoothed tuning traverses intermediate positions
on the way to a stop, changing the free-running carrier's phase.

No common knob placement offset is needed or introduced. Panel rotation still
means legend 0–10 over 300 degrees. The electrical track changes are equivalent
to −3.421/+2.344 degrees at .25/.75 for an unloaded ideal linear track; this
illustrates the placement/track ambiguity, not an applied angle correction.
The six session-2 captures contain only two distinct intermediate control
positions, so they do not establish the full track shape independently. A
resistance-versus-measured-angle sweep is needed to distinguish these effects.

### Combined score and regression audit

Both full runs score **620 sidecars / 629 analysis rows with zero failures**.
Overall **0.9660342817 → 0.9153439122 (−5.25%)**. Tuning aggregate
**2.3280823630 → 0.1990868448 (−91.45%)**; normalized median/p90
**1.500320/4.811369 → 0.090374/0.525224**. Native absolute tuning error
median/p90 **7.501601/24.056846 → 0.451872/2.626121 cents**.

| Category | Before median / p90 | After median / p90 | Aggregate before → after |
|---|---:|---:|---:|
| pitch | 0.097183 / 0.264985 | 0.097183 / 0.264985 | 0.139134 → 0.139134 |
| tuning | 1.500320 / 4.811369 | 0.090374 / 0.525224 | 2.328082 → 0.199087 |
| waveform | 0.577513 / 1.226971 | 0.577513 / 1.226971 | 0.739878 → 0.739878 |
| vcf | 0.521996 / 1.250907 | 0.521996 / 1.250907 | 0.704224 → 0.704224 |
| envelopes | 0.304544 / 0.493104 | 0.304544 / 0.493104 | 0.351684 → 0.351684 |
| amplitude | 1.036286 / 2.404905 | 1.036286 / 2.404905 | 1.378441 → 1.378441 |
| accent | 0.215252 / 0.775566 | 0.215252 / 0.775566 | 0.355331 → 0.355331 |
| slide | 0.000000 / 0.156499 | 0.000000 / 0.156499 | 0.039125 → 0.039125 |
| gate | 0.522083 / 0.590399 | 0.522083 / 0.590399 | 0.539162 → 0.539162 |
| behavior | 2.059199 / 2.527236 | 2.059199 / 2.527236 | 2.176209 → 2.176209 |
| interaction | 1.798547 / 3.321572 | 1.798547 / 3.321572 | 2.179304 → 2.179304 |
| repeatability | 1.016496 / 1.407178 | 1.016496 / 1.407178 | 1.114167 → 1.114167 |

Every other category aggregate is **exactly unchanged**, including pitch,
waveform, slide and amplitude. Only the twelve tuning rows change their
per-capture metrics; every centre-tuning capture has identical metrics and
amplitude metrics. No category worsens by 2%.

| Capture | Signed tuning residual before → after (cents) |
|---|---:|
| pitch-k24-t0 | +0.5982 → +0.6029 |
| pitch-k24-t100 | -2.8638 → -2.8656 |
| pitch-k48-t0 | -0.2494 → -0.2496 |
| pitch-k48-t100 | -0.3829 → -0.3829 |
| pitch-k72-t0 | +0.0924 → +0.0924 |
| pitch-k72-t100 | +2.7590 → +2.7590 |
| tuning2-k24-t25 | +23.5061 → -0.5208 |
| tuning2-k24-t75 | -14.2593 → -1.1793 |
| tuning2-k48-t25 | +24.1180 → +0.0322 |
| tuning2-k48-t75 | -13.5699 → -0.0367 |
| tuning2-k72-t25 | +24.2084 → +0.0898 |
| tuning2-k72-t75 | -12.1394 → +1.4306 |

The largest remaining residual is **−2.866 cents** at MIDI 24 / t100 and
**+2.759 cents** at MIDI 72 / t100. The reference upper-stop offsets themselves
vary **894.249 → 889.093 cents** over those keys, whereas the ideal expo model's
static stop shift is key-independent. One global VR2 track cannot remove that
key variation; pair mismatch/reset recovery, estimator/downstream response,
and inter-session reference drift remain confounded. The fitted intermediate
replays have maximum absolute residual **1.431 cents**. No per-key correction,
centre retrim, phase reset or output-path compensation was added.

Artifacts: `/tmp/osc-tuning-before.json`, `/tmp/osc-tuning-after.json` (tuning
subset), `/tmp/osc-tuning-overall.json`, plus exact-final-binary reports
`/tmp/osc-tuning-final.json` and `/tmp/osc-tuning-overall-final.json`, and their
matching Markdown reports.
After-fit cache and solver audit are under `/tmp/osc-tuning-cache` and
`/tmp/osc-tuning-solver-audit`. The initial baseline run used the harness's
ignored default cache; no capture or harness source was changed.

Final scored renderer SHA-256:
`650d2f1cf7bb6e6c463b0c9117c48270b4aacda26b5c9ffc5bc8c9ffdcff15b1`.
The final rebuild after comment/rustfmt cleanup changed the executable hash;
fresh replay verified **all 620 WAVs byte-identical** to the earlier fitted
binary `fb4a4ec3ba097fa0733548fa46d779d52bb2c4b6f76b6a202289f27c919ebbea`.
Analysis SHA-256:
`7fbc7305a654c4fa061c7434777f57539e5f3becd2f36dd36a2d026c44bc4d83`.

### Validation and remaining limits

Offline package-scoped tests across all ten workspace packages: **133 passed,
5 existing manual timing tests ignored**. Oscillator: **20 passed**, including
new checks for intermediate tuning at 44.1/48/96/192/384 kHz, exact unchanged
static centre/endpoints, identical centre saw/square samples, positive track
resistance density and monotonic tuning at validation bounds. The waveform
frequency measurement test now covers all five tuning positions. Oscillator
all-target clippy with warnings denied, scoped rustfmt and whitespace checks
pass.

A scratch auditor copied the current renderer's replay/parser and linked this
worktree's fitted release engine, adding only post-replay diagnostic checks and
omitting WAV payload writes. Across **620 captures / 701,184,000 circuit ticks**,
there were **zero Newton non-convergence**, **zero sanitized inputs**, and a
maximum of **4 iterations**. Source and log:
`/tmp/osc-tuning-solver-audit/{main.rs,replay.rs,json.rs,results.txt}`. No workspace
renderer or solver source was changed.

Serial Full Circuit renderer holds, 10 seconds at host 48 kHz, open cutoff,
zero resonance/env/decay/accent and full volume:

| MIDI key / tuning | Render CPU (% of one core) | User / sys CPU (s) | Max Newton iterations |
|---|---:|---:|---:|
| 24 / .50 | 10.8% | 1.05 / .01 | 3 |
| 48 / .25 | 10.7% | 1.06 / .00 | 3 |
| 72 / .75 | 8.8% | .87 / .00 | 3 |

All holds have zero non-convergence and sanitized inputs; logs are
`/tmp/osc-tuning-cpu-k{24,48,72}.log`. The separately run release oscillator
benchmark passes: **384,000 ticks at 192 kHz in 87.09 ms**. These holds exclude
construction and are not worst-case callback/glide guarantees. The new track
law adds only fixed arithmetic on a frequency-cache miss; no allocations,
locks, iterative solves, new transcendental calls or loops are added to tick.

New VERIFY items are precisely the two resistive-track coefficients and their
placement/track ambiguity, included in the complete register above. Inherited
VERIFY items remain: effective VR2 pin-3 rail/total resistance; IC10 supply and
ideal DAC; slide switch/buffer/clamps and latch-clock ambiguity; C35 tolerance;
TM4 current and R102 unit; R100 temperature law; ideal Q26/regulator; reset
thresholds and instantaneous recovery; Q28 device law/parasitics/back-loading;
Q8 junction/beta/saturation reduction; R35/R45/C10/C11 tolerances. No new
internal-node measurement, manufacturer pot law, thermal sweep or SPICE
validation is claimed. Remaining tuning error is limited chiefly by upper-stop
key variation and sparse/inter-session angle references; resolving them needs
measured pot resistance/angle and denser repeated same-session tuning captures.

## Decision 2026-10-13: Roland tuning specification over the TD-3 unit

The TUNING control now follows the TB-303 service notes instead of the measured
TD-3, at the owner's request:

| | TD-3 fit (previous) | Now |
|---|---|---|
| A at centre (normal octave) | 109.576 Hz, 6.6 cents flat (`reference_hz` 32.577173) | **110.000 Hz** (p. 7 calibration; `reference_hz` 32.703196 = 12-TET C1) |
| TUNING range | −1086.6 / +891.9 cents (VR2 44.945 kΩ, bow 0.1025, skew −0.0383) | **±700 cents** (p. 1 specification) |
| VR2 model | Loaded resistive track | Ideal unloaded linear control (`tuning_resistance` 1 Ω, bow and skew 0) |

The TD-3 measurements stay in the session data and the harness, and the
pitch/tuning categories now measure the deliberate difference from that unit.
The reason, from the project decisions: the original schematic and
specification are authoritative, and TD-3 deviations are not adopted silently.
The TD-3's flat calibration and wider TUNING range are properties of that unit
or its design, not of the TB-303.

## Free-running VCO phase (authentic "wobble"), 2026-10-13

The VCO runs freely and is never reset at a note. Each note therefore starts at
a different point in the waveform, so the audible leading transient shifts from
step to step and wraps once per waveform cycle. With repeated notes this sounds
like a repeating swing or tempo wobble. The sequencer's triggers stay
sample-exact; tests confirm 125.000 ms steps at 120 BPM.

The step count before the wobble repeats is `1 / frac(f0 × step duration)`. It
depends on the note and the tempo: about 5.7 steps for C2 at 120 BPM.

| Repeated C2 at 120 BPM, ms after trigger | Transient timing |
|---|---|
| DW-303 | 10.2, 7.4, 4.7, 2.0, **14.6**, 11.9, … |
| TD-3 (`combo-c04-e00-r00-k36-phrase`) | 6.3, 4.3, 2.4, 0.5, −1.3, −3.0, **10.0**, 8.1 |

The TD-3 drifts slightly slower because that unit is tuned 6.6 cents flat.
This behaviour is kept deliberately (owner decision). Do not add note-on
oscillator reset or sync.

## Decision 2026-10-13 (later): TD-3 tuning restored

At the owner's request, TUNING again uses the TD-3 unit fit, replacing the
Roland specification described above:

- `reference_hz` 32.577173 (A ≈ 109.58 Hz at centre, 6.6 cents flat)
- VR2 44.945 kΩ, bow 0.1025, skew −0.0383
- range −1086.6 / +891.9 cents

The Roland values are recorded above in case they are wanted again: A =
110 Hz, ±700 cents, ideal linear control. Pitch and tuning harness scores
return to the fitted baseline (overall 0.698, v7).
