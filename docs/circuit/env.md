# Revision 2: trigger, MEG, accent, converter, VEG, OTA and output

Authority: [contract](contracts.md), [netlist](../../reference/schematic/netlist.md),
and the complete [ENV](../../reference/schematic/transcription/env.md),
[VCF](../../reference/schematic/transcription/vcf.md) and
[VCA](../../reference/schematic/transcription/vca.md) transcriptions.
Printed RC and pot values replace the provisional estimates. `Components`
comments identify their designators. Session-1 tolerance and pot fits are
explicitly marked below; device estimates remain flagged.

## API and integration

All eight contracted stages implement `new(Components)`, `prepare(rate)`,
`reset()` and the Revision 2 signal methods, with f64 SI volts/amperes, `no_std`,
no unsafe code and only the existing libm dependency. Preparation accepts
44.1–384 kHz, preserves capacitor charge, and rejects invalid clocks without
changing the rate. Construction uses 48 kHz; reset restores quiescent DC and
clears diagnostics. Non-finite tick arguments become zero and increment a
saturating per-stage counter. Knobs represent panel rotation 0–1.

The integrator must wire these nodes in order:

```text
pulse = TriggerPulse.tick(trigger)
meg = Meg.tick(pulse, accent_latch, decay)
accent_wiper = Accent.tick(accent_latch, meg, accent_knob)
sweep = AccentSweep.tick(accent_wiper, resonance)
ladder_current = FilterControl.ladder_current(cutoff, env_mod, meg, sweep)
Veg.set_gate(gate)
veg_current = Veg.tick(pulse)
vcf = Vcf.tick(waveform_voltage, ladder_current, resonance)
vca = Vca.tick(VcfSignals { output: vcf.output, resonance_tap: vcf.resonance_tap }, veg_current, accent_wiper)
output = OutputStage.tick(vca, volume, mix_in)
```

The additional `Veg::set_gate(bool)` represents D35/Q31's emitter supply:
`tick(pulse)` cannot infer gate duration from a 1.4 ms pulse. Set it before each
VEG tick. Default/reset is gate high, allowing the contracted pulse-only method
to expose decay. A low gate removes envelope current while C42 keeps recharging.
Tied/slid notes preserve C42 by suppressing triggers. Accent arguments are the
already latched IC13 state; CPU/strobe timing is outside this block.

## Circuit equations and state

**Trigger:** Q38 stores differentiated C54 drive, initially `12 − Vbe` on a
rising trigger. Holding the input does not restart it. Drive decays through
`R148*C54 = 0.47 ms`; conduction ends when `drive/R148 <= Vbe/R149`.
The nominal model gives 1.345 ms, close to the transcribed approximate 1.4 ms,
with at most one sample of quantisation. TRIG is approximately 11.8 V.

**MEG (transcribed nominal values):** C62 charges through D37/R152 (100 ohms), approximately 100 us,
while R138+VR6 also discharges it. Peak C62 is approximately 11.2 V.
Between pulses, `V = Vfloor + (Vinitial−Vfloor)*exp(−t/tau)`, with
`tau = (R138 + taper*VR6)*C62`. D26–D28 use a fixed-drop 1.8 V clamp.
Q41/Q40 subtract two Vbe drops, so ENV_OUT peaks near 9.9 V and rests near
0.5 V. IC12 shorts VR6 under accent: tau is 68 ms independent of DECAY.
Ordinary decay spans 68 ms–1.068 s. The printed 100%→10% times derive as
157 ms and 2.459 s. MAX 2.5 s agrees within 2%; MIN 200 ms is 27% above the
nominal RC result and is not substituted for the printed components.

**Accent:** IC12 gates ENV_OUT onto linear VR7. The wiper is
`accent_latch ? ENV_OUT*rotation : 0`.

**Sweep (transcribed nominal values):** D24 feeds R46, then VR4B terminal 1; C13 is at terminal 3.
R72 loads the wiper through R71 into the stiff Q9 reference. C13 therefore
rests at VQ9, not ground. For resonance fraction k, source-to-wiper resistance
is `R46+k*VR4B`, and wiper-to-C13 resistance is `(1−k)*VR4B`. The load is
R72+R71. Each diode state has an exact one-capacitor solution; a sample splits
analytically if leaking C13 crosses the D24 turn-on threshold. The diode blocks
discharge back into MEG. At full resonance the unloaded charging resistance
is 97 kohms, conducting loaded tau is `(97K || 102.2K)*1uF = 49.77 ms`, and
D24-off discharge tau is 102.2 ms. Low resonance selects mostly direct accent;
high resonance selects the accumulated C13 node. There is no arbitrary sweep
floor or independent resonance envelope gain.

**Converter:** Q9 is a PNP follower of equal R64/R65: VQ9 is
`5.333/2 + Vbe = 3.3165 V`. VR3 and VR5 terminate toward this reference through
R47 and R61. Their Thevenin loading is included. The VR5 envelope fraction is
`(R61+taper*VR5)/(R61+VR5)`; CCW retains the printed R61's small residual feed.

```text
re = R63 + Thevenin_resistance(VR5/R61)
delta = [(V_VR5w−VQ9)/re + (V_sweep−VQ9)/R72] / [1/R71+1/re+1/R72]
I_cutoff = max((V_VR3w−VQ9−Vbe)/(R73+TM3+VR3_source_resistance), 0)
I_ladder = (Is11/Is10)*I_cutoff*exp(delta/VT)
```

The loaded sensitivities approach the netlist's 1.79 V/oct and 0.81 V/oct.
ENV MOD lowers resting cutoff when ENV_OUT is below VQ9, consistent with page 8.
CUTOFF changes reference current through its A-taper track. R69 and estimated
Q12/Q11 headroom limit Q11 current to approximately 0.530 mA.

**Historical Revision-2 integrator calibration:** TM3 was **62,917.60490068002 ohms**, within its
printed 0–470K range. At centre CUTOFF, ENV MOD/DECAY/ACC full CCW, settled
MEG 0.5 V and sweep VQ9, the documented ladder current is
**7.372454385731738 uA**. The analytical test uses `g=I/(4*VT)`, unequal rung
capacitors 18/33/33/33 nF, the loaded tridiagonal admittance determinant, and
C15/C23/C25/C27/C14 coupling phase. Its loop transfer is negative real at
500 Hz: a 2 ms period, inside page 7's 2 ms ±0.5 ms target. This verifies the
phase-derived current boundary, not the final nonlinear VCF ring period or
amplitude. The integrator must check those with the assembled VCF and may adjust
TM3. TP6's ambiguous 5.2/5.8 V levels are not used for calibration.

**VEG:** physical state is C41 at Q32's base and C42 at Q31's base. D36/R134
charges C41, unloaded tau 2.2 ms; estimated beta and R130 limit Q32 sink current.
C42 discharges through R132/Q32/R130, approximately 200 us saturated tau, with
C41 stretching the reset. The transcribed C42 recharges toward 12 V through R123, nominal tau 1.5 s;
the session fit below uses 1.575 s.
The original fixed-Vbe Q31 reduction supplied
`gate ? max((12−Vsat−V_D35−Vbe−V_C42)/(R131+R133),0) : 0` amperes.
Session-1 replaces that hard cutoff with the junction equation below; peak
current remains approximately 47 uA and the tail decays continuously.

**VCA:** accent current flows through R119/C36/R120/the VCA duplicate D27.
Threshold is `V_D27+V_pin1`; C36 off tau is 1.551 ms, conducting tau 0.4945 ms.
Integration splits analytical diode crossings. Steady accent current is
`max((Vaccent−V_D27−V_pin1)/(R119+R120),0)`. It adds to Q31's Iabc.

Trapezoidal capacitor companions solve the shared R124 input node for both
C21/R121 (VCF output) and VCA C22/R122 (resonance wiper), including mutual
loading. R125 provides the bias-only negative input. The approximately
−40.3/−33.4 dB attenuations and 72/156 Hz corners follow the netlist.
The non-inverting OTA law is `Iout = mirror_ratio*Iabc*tanh(Vdiff/(2*VT))`,
so `gm = mirror_ratio*Iabc/(2*VT)`. Its load is **R128 || (R129+R127)**,
52.526 kohms. A unity buffer drives C38 around 5.333 V, with estimated rail
headroom. C38 into the VR8 track gives approximately 3.18 Hz coupling.
`control_current()` exposes total Iabc for current-domain validation.

**Output:** VR8's A-taper wiper resistance `VR8*k*(1−k)` loads R155/C59.
Q34/Q33 is reduced to its closed-loop resistor gains: `−R153/(R155+R_wiper)`
and `−R153/R156`. At full volume these are −2.128 and −1. MIX IN is independent
of VOLUME. C59/C58 give approximately 3.4/1.6 Hz coupling. C56/R161/R165 give
1.576 Hz at the unloaded output and a 100/101 divider. Each coupling capacitor
retains its own charge. Mixer headroom is bounded around its estimated bias.

## VERIFY register and open items

| Source flag | Unprinted assumption / outstanding check |
|---|---|
| `diode_drop` | Generic Si Vf = 0.6 V; per-designator types, Is, ideality, leakage and current dependence are unprinted. Fixed clamp drops omit R117 bias dependence and the logarithmic near-floor tail. |
| `vbe` | Generic Q9/Q10/Q11/Q31/Q32/Q40/Q41 Vbe = 0.65 V; individual Is/beta/matching/temperature are unprinted. Q39's 2SK30(Y) type is printed, but individual Idss and buffer sink dynamics are omitted. |
| `saturation` | Q36/Q38/Q32 Vsat and buffer headroom = 0.2 V; output resistances and rail-dependent clipping need device validation. |
| `beta` | Q32 beta = 200; C41 loading/reset stretch needs SPICE/device data. |
| `thermal_voltage` | VT = 25.9 mV, approximately 300 K; no measured temperature/tempco calibration. |
| `expo_is_ratio` | Nominal matching estimate 1; preceding VCF fit uses 3.074, with voice calibration around 3.134. Matching is unprinted and inseparable from TM3 in these output captures. See vcf-calibration.md. |
| `ota_control_voltage` | BA662A pin 1 = 0.65 V; determines accent diode current and source headroom. |
| `ota_mirror_ratio` | BA662A current transfer factor = 1; gm, offsets, feedthrough and buffer characteristics are not printed. |
| `tm3` | Historical 62.918K analytical trim superseded by preceding VCF 470K/expo-ratio fit; see vcf-calibration.md for assembled ring/grid evidence. |
| A-pot law | VR3 retains the preceding VCF fit; VR5/VR6 now have separate measured electrical track tables below. VR8 retains `(81^rotation−1)/80`, 10% at half rotation. A markings are printed; exact tracks are not. |
| VR4 gang direction | Assume both move toward terminal 3 clockwise; VCF open item 3 does not establish correlation. |

Other transcription open items remain explicit: duplicate C22/D27 are block
qualified; R147's optional/selected notation is neglected against the assumed
gate supply; IC12's unshown supplies/Ron are reduced to ideal switches; CPU
pin-8 clock/slide semantics are delegated to sequencing. BA662A pin 4 has no
model function. TP6 DC placement remains unresolved. Headphone J2 wiring,
triangle supply and LA4140 gain are outside the line-output model.

## Validation and remaining limits

The frozen provisional reference and old regression tests were removed.
Netlist tests check trigger width, MEG charging/buffer/clamp/decay, accent gating,
sweep loading/discharge/accumulation, expo reference/sensitivities/compliance,
TM3 analytical phase, VEG recharge/gate removal, C36 current, OTA gm/compression,
both input AC responses, mixer gains/volume loading/independent MIX IN, and DC
rejection. RC/AC checks cover 48/96/192 kHz. One million extreme/non-finite
samples cover 44.1/48/96/192/384 kHz, checking finite/non-subnormal outputs.
Unit tests check exact RC composition, rate preservation and tail flushing.

No sample-path allocations, unbounded loops, unsafe code or data-dependent
indices exist. Values below 1e-24 are flushed. Invalid/nonpositive component
values fall back to nominal (TM3 zero is valid); numerical RC guards protect
extreme configurations. Fixed coefficients, input companions, RC exponentials,
pot tapers and loaded pot networks are cached. RC cache keys include duration;
preparation refreshes input companions. A tiny-signal cubic tanh has <1.4e-21
absolute error for |Vdiff/(2VT)| < 1e-4; otherwise the OTA uses libm tanh.

Circuit reductions still requiring SPICE/hardware validation: Q38's saturation
pulse omits R150/C55 reservoir sag and gradual collector turn-off. MEG uses
fixed junction drops and an ideal Darlington. Q32 is a beta-limited reduction,
not Ebers–Moll. VR7 source impedance (up to 12.5K), cross-loading between the
VCA/VCF accent branches, and modulation-induced movement of EXP_B at the sweep
termination are not fed back across voltage-only block boundaries. Q9 is stiff.
C38 sees the full VR8 track; frequency/volume-dependent C59 loading is handled
downstream without feedback into C38. Bias-filter supply dynamics, discrete
mixer open-loop dynamics and headphone output are omitted. No full SPICE or
hardware accuracy claim is made.

## Final checks and timing (2026-10-11)

Package-scoped offline tests and all-target clippy with `-D warnings` pass;
the three Rust files are rustfmt-clean. There are 12 passing tests and two
ignored manual profiling/benchmark entry points. The manual release benchmark
was also executed successfully. The final eight-stage loop measured
0.072356–0.078854 s for eight seconds at 192 kHz, approximately 0.90–0.99% of
one core. The old cached six-stage benchmark measured 0.066706 s immediately
before replacement: the new loop costs 1.08–1.18x, below the approximate 1.3x
budget. Both use opaque stage references, the same note/knob sequence and a
constant 0.3 V audio input; Revision 2 additionally processes the 0.21 V
resonance tap and the new trigger/accent stages. This is isolated crate timing,
not a whole-voice or audio hardware measurement.

Reproduce the manual timing with:

```sh
cargo test --offline --release -p dw303-env --test performance benchmark_revision_two -- --ignored --nocapture
```


## Session-1 fitting round (2026-10-12)

Baseline rebuilt in this worktree is the published `0b6ba604…` executable:
4.2606 overall, 346 captures, no analysis failures. The preceding VCF fit
(VR3, TM3, expo matching and voice ring calibration) is retained. No capture,
metric, MIDI timing, root manifest or other crate was edited. Voice changes
are confined to the documented `FULL_SCALE_VOLTS` calibration.

### Physical changes and VERIFY register

These are fitted defaults, not revisions to the authoritative transcription.
Every changed printed value is within the stated assumed component tolerance;
those tolerances themselves need confirmation against this particular clone.
The data identify output behavior, not unique internal part values.

| VERIFY item | Transcribed / previous → fitted | Evidence and limitation |
|---|---|---|
| `FULL_SCALE_VOLTS` | 5.12 → 11.595 V/digital full scale | Baseline model-minus-hardware overall RMS median was **+7.1009 dB**. Increasing this divisor corrects the sign. This is a TD-3/UMC1820 recording-chain calibration; absolute volts remain unmeasured. |
| `c42` | C42 1 → **1.05 µF** | +5% electrolytic tolerance; R123 remains 1.5 MΩ, hence recharge τ = **1.575 s**. Hardware retains an audible held-note tail around 3.2 s, beyond the previous hard current cutoff. Cannot separate capacitor tolerance, Q31 junction parameters and OTA behavior from output alone. |
| `q31_reference_current`, Q31 junction law | Fixed 0.65 V threshold → **0.65 V at 47 µA**, ideality 1, VT 25.9 mV | Reuses the previous generic Vbe/temperature estimates. Solves `Vgate−V_D35−V_C42 = (R131+R133) I + Vbe + VT ln(I/47µA)` with two or four bounded Newton updates of the positive Lambert branch. Avoids the artificial fixed-Vbe cutoff. Q31 Is, ideality, beta/base loading and temperature are not internally measured. |
| `c62` | C62 1 → **0.95 µF** | −5% electrolytic tolerance fit. |
| `r138` | R138 68 → **71.4 kΩ** | +5% assumed resistor tolerance; with fitted C62, minimum/accent τ = **67.83 ms**, preserving the original 68 ms accent decay. |
| `vr6` | VR6 1 → **0.8 MΩ** | −20% assumed pot-track tolerance limit. Maximum MEG τ = **827.83 ms**, rather than nominal 1068 ms. Effective output decay is also shaped by the exponential filter control and unequal VCF poles. |
| `vr6_taper` | Generic 10%-at-half exponential → fitted monotone track below | Ordinary midpoint MEG τ = **190.8 ms** versus nominal 168 ms. Session output log-cutoff target was about 183 ms versus the old model's 142 ms; electrical τ and audio proxy τ are not interchangeable. |
| `vr5_taper` | Shared generic A law → separate monotone track below | Preserve the low-half law and strengthen the upper-turn envelope excursion. Hardware ENV MOD 0.7–1 spectra require a steeper upper track than the generic exponential. Full-turn electrical sensitivity is unchanged. These output spectra also depend on VCF/device reductions. |
| `c13` | C13 1 → **0.8 µF** | −20% assumed electrolytic tolerance limit; full-resonance conducting sweep τ **39.82 ms**, off τ **81.76 ms**, instead of 49.77/102.2 ms. Intended to improve accent attack brightness; accumulation remains a separate check. |

Track knots at rotations `0, .1, .2, …, 1` are electrical wiper fractions:

```text
VR5: 0, .00689807, .01760141, .03420803, .05967361,
     .1, .16180566, .36, .65, .92, 1
VR6: 0, .038644737, .07275, .10875, .133381579,
     .161802632, .195907895, .398644737, .616539474, .815486842, 1
```

Interpolation is bounded, monotone and allocation-free. Invalid component or
track data fall back to the documented defaults. Q31 retains gate removal of
current and the C41/Q32 reset circuit; C42 remains the physical voltage state.
The junction solver's residual and continuous monotone tail are tested at
48/96/192 kHz. No arbitrary audio envelope or gate smoothing was introduced.

### Coupling investigation

C21 **10 → 8.2 nF** was evaluated and **reverted**. It reduced C1 saw H2/H1
error from 2.261 to 1.808 dB, but the aggregate waveform and VCF errors worsened
by approximately 3.9% and 2.5%. All audio couplings therefore retain their
printed values: C21/C22 10 nF; C38/C56/C58/C59 1 µF. Their nominal corners
cannot explain a uniform 8 dB correction within ordinary component tolerance.
The rebuilt baseline already has a smaller saw-bass discrepancy than the older
prebuilt comparison. Square harmonic errors change sign with pitch and remain
largely outside this stage's available coupling corrections.

### Remaining model limits

The earlier VERIFY register still applies: generic diode/junction drops,
saturation/headroom, Q32 beta, temperature, expo matching, BA662A pin-1 voltage
and mirror gain, uncertain exact pot tracks and VR4 gang direction. The
historical analytical TM3 paragraph above describes Revision 2; current
assembled-voice trimming is documented in `vcf-calibration.md` (470 kΩ TM3
endpoint and fitted expo ratio), not that original 62.918 kΩ setting.

Q31 base-current feedback into C42 and temperature dependence are omitted.
Accent cross-loading and transistor/OTA gm compression are not independently
identified. Short pattern trajectories retain gate/MIDI arrival jitter and
filter-envelope errors; no timing compensation was fitted. Coupling to the
interface input impedance is uncalibrated. The model emits no synthetic noise,
so normalized very-low-level tails can differ from the interface noise floor.

### Final scoring and validation

Full-session error **4.2606 → 3.7446** (**12.1% reduction**), 346/346 captures, zero analysis failures.
Final renderer SHA-256: `fe371e5d0b6346ee6e4e925f93f52638652b283530212be8e84092bb28403f6d`.
Analysis SHA-256: `40c5b9e78b7da6fa0015099413f1cb4ed950867633254a0bfa2175cbbd2f1cc4`.

| Category | Median before → after | p90 before → after | Contribution change |
|---|---:|---:|---:|
| amplitude | 4.6226 → 2.3831 | 7.1624 → 5.6194 | -39.28% |
| envelopes | 2.0215 → 0.7102 | 2.7821 → 0.9644 | -65.02% |
| accent | 0.2368 → 0.2463 | 1.0505 → 0.9008 | -6.88% |
| behavior | 5.7806 → 4.8988 | 5.9606 → 4.9676 | -15.61% |

No unassigned category contribution worsened by more than **0.03%**.
Accent median rises 4.0%, while p90 improves 14.2%; its specified
`0.75 median + 0.25 p90` category contribution improves 6.9%. This is the
remaining accent tradeoff, rather than a claim of universal capture improvement.

Absolute-level median/p90 error is **7.1009/8.9573 → 0.6334/1.9093 dB**.
Amplitude shape median/p90 is **5.7267/10.6449 → 4.2817/10.8398 dB**;
the upper tail remains limited by gate transients, interface noise and behavior.
Envelope amplitude-decay log2-ratio median/p90 improves
**0.2496/1.0066 → 0.1165/0.2146**. Accent level-increase p90 remains
**2.9384 dB**, versus 2.8843 dB initially; no unsupported extra OTA
compression or circuit-wide gain correction was added to hide it.

Reports: `/tmp/env-score.json` (requested subset, 2.0896) and
`/tmp/env-full-score.json` (full session, 3.7446); subset and full scores have
different weight normalizations and must not be compared directly.

All ten workspace packages were tested with explicit offline `-p` selections:
**110 tests passed, zero failures, four manual tests ignored**. ENV has
**13 passing tests**, including rate-independent RC recharge, gate removal,
Q31 junction residual/monotone tail, AC input response, OTA law and the existing
million-sample extreme/sanitization coverage. Offline ENV/voice all-target
clippy with `-D warnings`, rustfmt checks and `git diff --check` pass.
No dependencies were added, and no sample-path allocation, locks or unbounded
loops were introduced.

Manual ENV release benchmark: **0.124078 s / 8 s at 192 kHz**, **1.55% of one
core**, versus the historical eight-stage 0.90–0.99%. Whole renderer held-note
check: **2.115 s / 16 s at 48 kHz host**, **13.2% of one core**, with
3,072,000 VCF samples and **zero non-convergence/sanitization**. Measurements
were repeated after the batch rendering workers completed; contended runs
while twelve replay workers were active took much longer and are not used as
standalone CPU evidence.


Additional all-maximum CUTOFF/RES/ENV MOD/ACCENT/VOLUME factory-pattern
stress at 48 kHz host: 16 seconds, 3,072,000 VCF samples, maximum five
Newton iterations, **zero non-convergence/sanitization**. First renderer
wall measurement was **3.157 s (19.7%)**. A contended repeat measured
5.889 s wall; `/usr/bin/time -p` measured **3.60 s user + 0.06 s system**
(**22.9% of one core**, including setup and WAV output). This extreme case
exceeds the approximate 18% target and remains a performance limitation.
The default held-note check above meets the target. ENV alone costs 1.55%;
the permitted fitting scope does not include changing the VCF solver.

Commit attempt was blocked by the session filesystem policy: both `git add`
and `git commit` failed creating
`/Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/.git/worktrees/dw303-wt-env/index.lock`
with `Operation not permitted`. The external Git directory is outside the
writable roots. The four scoped files remain uncommitted on `fit1/env`;
no other branch or Git history was changed.
