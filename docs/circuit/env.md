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
to expose decay. A low gate lets C72/Q36 discharge; Q31 current falls over a few milliseconds
while C42 keeps recharging.
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

**Accent:** IC12 gates ENV_OUT onto linear VR7. Its unloaded voltage is
`ENV_OUT*rotation`, with source resistance `VR7*rotation*(1−rotation)`.
When that voltage exceeds `T = V_D27 + V_pin1`, the fast VCA accent branch
loads the wiper: `Vw = (Vunloaded*(R119+R120) + T*Rsource) /
(R119+R120+Rsource)`. Below T the steady branch is open; an inactive accent
returns zero. This retains the printed 50 kΩ track rather than an arbitrary
accent gain. VERIFY: C36 transient back-current and D24 branch cross-loading
remain omitted at the voltage-only stage boundary. See the
[round-7 fit](vcf-env-fitting-round7-2026-10-12.md).

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

**VEG:** physical state is C72 across Q36 base–emitter, C41 at Q32's base
and C42 at Q31's base. D36/R134
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
| `r61` | VERIFY: printed 10 kΩ → 9.5 kΩ (−5% assumed tolerance) in the 2026-10-12 interaction fit. Output audio does not separate R61, VR5 track, Q9 bias and expo-ratio calibration. See the final investigation below. |
| `r122` | VERIFY: printed 100 kΩ → 95 kΩ (−5% assumed tolerance), round-14 high-drive fit. OUTPUT cannot separate this resistor from C22, the resonance-tap source impedance and BA662A input/current transfer. See the round-14 evidence below. |
| `meg_discharge_drop` | VERIFY: D27/D28 shared 0.60 V estimate → 0.45 V low-current reduction. D26 remains 0.60 V under R117 bias; effective C62 floor 1.50 V and buffered rest 0.20 V. Current-dependent diode law and buffer base loading remain unresolved. See round 9. |
| `diode_drop` | Generic Si Vf = 0.6 V; per-designator types, Is, ideality, leakage and current dependence are unprinted. Fixed clamp drops omit R117 bias dependence and the logarithmic near-floor tail. |
| `vbe` | Generic Q9/Q10/Q11/Q31/Q32/Q40/Q41 Vbe = 0.65 V; individual Is/beta/matching/temperature are unprinted. Q39's 2SK30(Y) type is printed, but individual Idss and buffer sink dynamics are omitted. |
| `saturation` | Q36/Q38/Q32 Vsat and buffer headroom = 0.2 V; output resistances and rail-dependent clipping need device validation. |
| `beta` | Q32/Q36 beta = 200; C41 reset stretch and C72 release need SPICE/device data. |
| `thermal_voltage` | VT = 25.9 mV, approximately 300 K; no measured temperature/tempco calibration. |
| `expo_is_ratio` | Nominal matching estimate 1; component fit remains 3.074, with current voice calibration about 3.088 after the R61 fit below. Matching is unprinted and inseparable from TM3 in these output captures. See vcf-calibration.md. |
| `ota_control_voltage` | BA662A pin 1 = 0.65 V; determines accent diode current and source headroom. |
| `ota_mirror_ratio` | BA662A current transfer factor = 1; gm, offsets, feedthrough and buffer characteristics are not printed. |
| `tm3` | Historical 62.918K analytical trim superseded by preceding VCF 470K/expo-ratio fit; see vcf-calibration.md for assembled ring/grid evidence. |
| A-pot law | VR3 retains the preceding VCF fit; VR5/VR6 now have separate measured electrical track tables below. VR8 retains `(81^rotation−1)/80`, 10% at half rotation. A markings are printed; exact tracks are not. |
| VR4 gang direction | Assume both move toward terminal 3 clockwise; VCF open item 3 does not establish correlation. |
| VR7 steady-load reduction | The printed 50 kΩ track now loads R119/R120/D27. C36's transient current and D24 mutual loading are not exchanged across the voltage-only API. Full-turn source impedance is zero, so this correction cannot explain the full-ACCENT residual by itself. |
| C72 / R142 / R143 | Printed 10 µF / 10 kΩ / 10 kΩ → current fit 11 µF / 10.5 kΩ / 9.5 kΩ. Earlier C72 fits were 8 and 12 µF. Resistors are at assumed ±5% tolerance bounds; only their combined gate response is constrained by output audio. |
| Q36 junction reference | Generic 0.65 V; previous 0.40 V at 1.18 mA → current effective 0.50 V at 1.90 mA forward-active capacity. Joint gate-shape fit with C72; not a measured transistor Vbe or Is. Constant beta 200, forward-junction base current, saturation clamp and preceding-sample Q31 emitter load remain reductions. See the v7 phrase follow-up. |

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
TM3 analytical phase, VEG recharge/C72 gate release, C36 current, OTA gm/compression,
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
not Ebers–Moll. VR7's steady VCA load is retained, but its C36 transient current,
cross-loading between the VCA/VCF accent branches, and modulation-induced
movement of EXP_B at the sweep termination are not fed back across voltage-only
block boundaries. Q9 is stiff.
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

At this historical round, Q31 base-current feedback into C42 and temperature
dependence were omitted. Round 3 below adds the base-current term; temperature
dependence remains omitted.
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


## Session-1 fitting round 2 (2026-10-12)

The starting worktree reproduces round1.json: **1.7143** overall. Changes are
confined to ENV, its circuit tests, the voice full-scale constant and these
stage/sequencer notes. The VCF-owned cutoff law is unchanged.

### Gate circuit and MIDI evidence

The reconciled ENV transcription identifies R142 10 kΩ in parallel with C72
10 µF from +12 V to Q36's base, R143 10 kΩ to Q37, and R145 10 kΩ at the
collector. The previous direct Boolean gate omitted this stored charge. It
ended Q31 current instantly, while the TD-3 gate captures release their audible
20%-peak crossings about **3–6 ms** later than the model. Pattern gaps also
retained inappropriate output-capacitor transients.

VEG now stores the positive voltage `v` across C72. The reduced Q36 circuit is:

```text
Ic = Iref * exp((v - Vbe_ref)/VT)
Vgate = min(R145*Ic, 12 - Vsat)
C72 * dv/dt = gate ? (12 - Vsat - v)/R143 : 0
             - v/R142 - Ic/beta
```

The capacitor is advanced explicitly at the prepared circuit clock and clamped
to the forward-junction voltage in saturation. Q31's existing bounded junction
solver uses `Vgate`, rather than a Boolean current switch. Reset/default gate
is high and C72 charged; the voice's silent pre-roll discharges it. The gate-on
and gate-off paths share the same capacitor and device law. There is no added
MIDI delay or audio fade envelope. Rate tests check monotone current release,
20% crossing within 3–6 ms and agreement within two 48 kHz samples at
48/96/192 kHz. Existing hard-zero note-off assertions now wait for this physical
release.

The classic, accent-slide and rests-ties sidecars were examined note by note.
Their MIDI gates are reproduced by `MonoNoteInput`; overlaps stay legato and
suppress retriggers. The rests-ties capture schedules actual rests/gates, not
internal-sequencer tie commands. No evidence warrants changing MIDI priority,
accent threshold, retrigger or slide semantics. The pattern-classic brightness
fit penalty disappears after adding the analog gate circuit. That improvement
comes from the release spectrum rather than a MIDI rule change.

### New/updated VERIFY register

| VERIFY item | Nominal / previous → fitted | Evidence and residual |
|---|---|---|
| C72 | printed **10 → 8 µF** | Assumed −20% electrolytic tolerance. A 10 µF trial improves release but adds excessive attack delay; 8 µF is the fitted tolerance endpoint. Gate audio crossings include 3 ms measurement quantization, capture latency and Q32/VCF/VCA response. Internal C72 charge was not measured. |
| Q36 junction | generic **0.65 → 0.60 V at 1.18 mA**, ideality 1 | Separate PNP junction estimate; changing shared Q31/Q32 Vbe would disturb their fitted laws. 1.18 mA is the 11.8 V / 10 kΩ saturated-load reference, not a measured transistor Is. Constant **beta=200** reuses the generic device estimate. Full saturation/base-charge dynamics, Q31 collector loading of GATE_HI and R147 emitter sag remain reduced. These device assumptions are not uniquely identifiable from output audio. |
| R119 | printed **47 → 49.35 kΩ** | +5% assumed resistor tolerance reduces excess high-accent Iabc. |
| R120 | printed **22 → 23.1 kΩ** | +5% assumed resistor tolerance; total accent path **69 → 72.45 kΩ**. C36 remains 33 nF. The conducting/off time constants become **0.519/1.629 ms**, versus 0.495/1.551 ms. A targeted 88-capture trial reduces accent level-increase p90 from about 2.98 to 2.73 dB; several middle-knob captures trade a small amount of error. Neither resistor is independently identified. |
| FULL_SCALE_VOLTS | original 5.12; round-1 **11.595 → 10.334 V** | A **+1.00 dB** recording-chain adjustment after the source/filter fits. Selection uses aggregate absolute-level errors, not per-capture gains. Absolute dBFS per volt remains unknown. |

All earlier VERIFY items remain applicable: diode drops, generic junction Vbe,
saturation/headroom, Q32/Q36 beta, thermal voltage, Q31 reference current/law,
BA662A pin-1 voltage and mirror gain, VR4 gang direction, fitted C13/C42/C62,
R138/VR6 tolerance and VR5/VR6/VR8 tracks, and the VCF-owned VR3/TM3/expo
calibration. This round does not reinterpret their values as measured parts.

### Low-note amplitude mechanism and remaining limits

On the starting executable, `wave-saw-k24` normalized RMS distance over the
3.5-second gate is **16.26 dB** with 10 ms windows, **2.38 dB** with 50 ms
windows and **1.40 dB** with 100 ms windows. `pitch-k24-t0` gives
**23.95 / 4.27 / 3.15 dB** respectively; key 48 gives **1.26 / 1.16 / 1.12 dB**.
The harness's 10 ms window resolves portions of individual bass cycles:
key 24 has a roughly 30 ms period, and tuning fully down lengthens it further.
Its saw is sharpened by the existing coupling high-pass network, with deep
inter-pulse troughs. The waveform need not be wrong by 15 dB for those troughs
to land in different windows.

A read-only phase diagnostic over 0.1–3.0 s lowers key-24 saw's 10 ms RMS
error from **16.44 to 4.13 dB** with an **11.77 ms** cycle shift; key 25 lowers
**15.45 to 4.08 dB** with **10.31 ms**. These are diagnostic comparisons only:
no shifted audio is used in fitting/scoring. Residual pitch drift, source shape
and coupling still matter; this is evidence for oscillator phase as the main
extreme bass metric mechanism, not proof that all bass discrepancy is VCO.
The oscillator free-running phase/initial state belongs to the oscillator
agent. No phase reset, coupling corner change or pitch-dependent gain was added.

High-accent level error cannot be removed by ordinary R119/R120 tolerance.
BA662A transfer/headroom, accent-source impedance/cross-loading and the TD-3's
output circuitry remain unmeasured. Gate attack latency, output-capacitor
tails, capture MIDI jitter and noise also limit the short-pattern metrics.


### Round-2 final scoring and validation

Full-session overall error **1.7143 → 1.2297** (**28.3% reduction**),
**346/346** analysed, **zero analysis failures**. Requested subset error is
**3.1588 → 1.8338**; it uses a different weight normalization from the full
score. Category contribution below means `0.75*median + 0.25*p90` before
multiplying the category weight.

| Category | Median before → after | p90 before → after | Contribution before → after | Change |
|---|---:|---:|---:|---:|
| amplitude | 2.1973 → 1.9446 | 5.6355 → 5.4646 | 3.0568 → 2.8246 | -7.60% |
| behavior | 5.1122 → 3.1127 | 13.5298 → 3.1861 | 7.2166 → 3.1310 | -56.61% |
| accent | 0.2385 → 0.2452 | 0.9681 → 0.9055 | 0.4209 → 0.4103 | -2.52% |
| gate | 0.6567 → 0.5174 | 0.7192 → 0.8480 | 0.6723 → 0.6001 | -10.75% |

No category contribution worsens by >2%. Envelopes improve **4.83%**;
pitch, tuning, waveform, VCF and slide are unchanged within rounding.
Gate off-timing median/p90 improve **3.969/3.969 → 1.500/2.379 ms**;
on-timing worsens **2.598/3.459 → 3.674/6.101 ms**, a remaining physical
attack/alignment tradeoff. The combined gate contribution improves 10.75%.
Behavior band/RMS-envelope medians improve **6.580/8.111 → 3.287/6.093 dB**.
Accent level-increase p90 improves **2.978 → 2.735 dB**; its category median
rises slightly while the p90 and specified contribution improve.

Reports: `/tmp/env-score.json`, `/tmp/env-full-score.json` (plus their generated
Markdown companions). Final renderer SHA-256:
`0f83e48d305760a1d4e973b9c2a7342cd2d8d5cc3a23eb46a611118443ac9420`.
Analysis SHA-256:
`40c5b9e78b7da6fa0015099413f1cb4ed950867633254a0bfa2175cbbd2f1cc4`.

All ten workspace packages were tested offline with explicit `-p` selections:
**115 passed, zero failures, four manual tests ignored**. ENV has **14 passing
tests**, including the new C72 release/rate check, updated physical release
assertions, Q31 residual/tail, OTA/network laws and million-sample extreme
coverage. ENV/voice all-target Clippy with `-D warnings`, scoped rustfmt checks
and `git diff --check` pass. No dependencies or tick allocations/locks were
introduced. The existing Q31 solver retains its two/four bounded iterations.

Sequential warmed eight-second renderer measurements: **16.1%** of one core
at 48 kHz; **17.1/16.2%** at 96 kHz for the default pattern; **13.9%** at
96 kHz for a held key-48 full-resonance/full-envelope/full-decay case. Each
processed 1,536,000 VCF ticks with **zero non-convergence or sanitized inputs**,
and maximum Newton count 3. Wall-clock timing varies: an initial 48 kHz run
measured 27.9%, and an earlier 96 kHz run 18.9%; these results do not establish
a hard scheduling bound. Measurements made during batch rendering are excluded
from the warmed CPU figures. The audio path remains deterministic and contains
no added allocation, lock or unbounded iteration. No commit was created.

## Session-1 fitting round 3 (Harness v2, 2026-10-12)

The isolated ENV worktree reproduces the merged round-2 executable exactly:
SHA-256 `82e0b02b781580c38e06b3d3eb77cafff59e203e0d86b830daf9e5862db3cc84`,
overall **1.0198058937**. All 346 hardware WAV/JSON pairs were read from the
absolute, read-only session path supplied for this round. No filter cutoff-law,
oscillator, engine, manifest, dependency, or sequencer implementation changed.

### Circuit correction and fitted values: VERIFY register

Q31 is a PNP whose base is the C42 storage node. Its base current flows **into**
C42, in addition to R123's recharge current. Previously it was omitted. The
unsaturated reset-free charge equation is now

```text
C42 dV_C42/dt = (12 - V_C42)/R123 + I_Q31/beta_Q31
```

The existing Q32 discharge and bounded Q31 junction solver remain in place.
The previous circuit sample's collector current supplies the base-current
term; this is a single explicit integration step, with no extra nonlinear
solve. The 48/96/192 kHz tests check the independent linear-junction solution
and the logarithmic-junction residual. At high control current, the effective
recharge time constant is about **1.524 s**, instead of the unloaded **1.575 s**;
C42 and R123 themselves retain their round-2 values. The approximation neglects
reverse-active Q31 behavior and detailed saturation/base storage.

| VERIFY item | Nominal / round 2 → final | Evidence and residual |
|---|---|---|
| Q31 beta | Base loading omitted → **beta = 200** | Generic 2SA733-class PNP estimate, not printed or independently measured. At 47 µA collector current, about 0.235 µA additionally charges C42. Keeping this physical current reduces the full envelope contribution from 0.6620 to 0.4876 before gain adjustment. R123/C42/Is/beta are not individually identified by output audio. |
| Q36 junction | Generic 0.65; round-2 **0.60 → 0.55 V at 1.18 mA** | Unprinted effective junction parameter; C72 remains its earlier fitted 8 µF, both 10 kΩ gate resistors and beta=200 remain unchanged. Gate on-timing p90 improves 6.101 → 5.382 ms; off-timing is unchanged. First-onset latency already includes hardware attack, and the harness adds the model's own attack after that offset, so this is not an independent measurement of Q36 Vbe. |
| C38 | Printed **1 → 0.8 µF** | Assumed −20% electrolytic tolerance. Coupling into the modeled 50 kΩ VR8 track has tau 40 ms, about 3.98 Hz. Fitted jointly with C59/C56 to inter-note output tails; individual capacitances and omitted inter-stage loading are not separately identifiable. |
| C59 | Printed **1 → 0.8 µF** | Assumed −20% electrolytic tolerance. At full volume, R155/C59 tau is 37.6 ms, about 4.233 Hz. The all-0.8 µF trial reduces targeted amplitude contribution 2.8442 → 2.7301 and behavior 2.7604 → 2.7278 before Q31 loading/gain. |
| C56 | Printed **1 → 0.8 µF** | Assumed −20% electrolytic tolerance. R161+R165 remains 101 kΩ, giving unloaded tau 80.8 ms / 1.970 Hz. No external interface load was added. Quiet tails still differ, including measured interface noise. |
| FULL_SCALE_VOLTS | Original 5.12; round-2 **10.334 → 10.041 V** | One session-wide **+0.25 dB** recording-chain adjustment after the physical fit. A bounded gain-only evaluation of the full v2 objective selects approximately +0.25 dB; the rebuilt executable is then rescored. No per-note/capture gain. Absolute dBFS/volt remains uncalibrated. |

All prior VERIFY items remain: generic diode drops/clamp and IC12 Ron/supply;
Q9/Q10/Q11/Q31/Q32/Q40/Q41 Vbe, Is, matching, beta and temperature; Q39 Idss;
Q36/Q38/Q32 saturation and buffer headroom; Q31 reference current/ideality;
BA662A pin-1 voltage, mirror gain, offset/feedthrough and buffer transfer;
VR4 gang correlation; fitted C13/C42/C62, R138/VR6, R119/R120 tolerances;
VR5/VR6/VR8 pot tracks; VCF-owned VR3/TM3/expo calibration; omitted mutual
VR7/accent-branch loading and C38/VR8/C59 loading; output-interface response,
noise and voltage calibration; capture latency/MIDI jitter. The original
transcription's duplicate C22/D27, R147 notation, CPU strobe interpretation,
TP6 placement and headphone ambiguities also remain unresolved. Earlier round
registers document their values and evidence; this round does not promote
estimates into measured TB-303 parts.

### Mechanism audit and rejected trials

The worst starting amplitude captures were inspected first: `pitch-k24-t0`,
then repeated accents (`a10/a09-n8-r0`, low ACCENT `n8-r10/r0`) and
`pattern-classic`. Normal-note level and brightness generally track the hardware
much more closely than the quiet gaps. For `accent-a00-n8-r0`, first-note
70–110 ms windows have roughly −78…−92 dBFS hardware tails versus −69…−70 dBFS
model tails before fitting. The minimum tuned key is below one cycle per 40 ms
harness window; its remaining phase/noise discrepancy is not an ENV time constant.

TriggerPulse correctly generates one R148/C54 one-shot per non-overlapped note;
a held trigger does not restart it. C62 already integrates the finite D37/R152
charge from its previous voltage, rather than resetting to a chosen envelope
peak. C41/Q32 similarly discharges partially recharged C42. A new test exercises
four real Q38 retriggers at 125 ms intervals at 48/96/192 kHz: both capacitors
retain charge between notes, the C62 attack recovers, and VEG peak current stays
within 1% of the first attack. No hard capacitor reset or overlap retrigger was
introduced. The R119/C36/R120/D27 conducting/off network is retained; its
analytical current and threshold-crossing tests still pass.

An IC13-inspired trial holding MIDI accent after the final note-off was
**reverted**. At full ACCENT/minimum RES, hardware falls to about −42 dBFS at
70–80 ms and −60 dBFS at 80–90 ms; the held-accent trial remains about
−29/−30 dBFS. Its full behavior contribution worsens 2.8547 → 3.2785 and
amplitude 1.5157 → 1.8735. The schematic latch alone does not establish the
TD-3 MIDI adapter's strobe/reset behavior. The recorded audio supports retaining
current note-off accent clearing, last-note priority and overlap slide semantics.
The `rests-ties` sidecar schedules short gates/rests, not internal tie commands.

An estimated external 20 kΩ line load was also **reverted**: amplitude
contribution 1.5157 → 1.5338 and behavior 2.8547 → 2.8924. Interface impedance
was not measured, and the trial did not justify changing the output boundary.
The coupling search compared 0.8/1.0/1.2 µF within assumed electrolytic
±20% tolerance, including a linear frequency-domain diagnostic of the existing
renders; decisions and final metrics use newly rendered audio. The all-1.2 µF
trial was worse than all-0.8 µF on the targeted set. A 0.40 V Q36 trial was
rejected as an implausible junction fit; the retained value is 0.55 V, flagged
above. No synthetic noise floor, audio fade, MIDI time shift or arbitrary accent
compression was fitted.

### Pattern audit: every scheduled note

All comparisons use the sidecar timeline and one first-note latency shift; no
per-note alignment or gain matching. `L` is 15–60 ms RMS in dBFS; `B` is the
harness band centroid in Hz; `P` is the largest 3 ms RMS within the first
62.5 ms, in dBFS; `D` is late (45–62.5 ms) minus early (15–35 ms) RMS in dB.
Triples are **TD-3 / round-2 / final**. `on` is the final-minus-TD local 20%-peak
crossing in ms, using 3 ms windows from −15 to +45 ms; it is an audio proxy
with capture jitter and frame quantization. Legato pitch changes have no new
gate edge, so their `on` is omitted.

#### pattern-classic

| Note/key/velocity | L (dBFS) | B (Hz) | P (dBFS) | D (dB) | on (ms) |
|---|---:|---:|---:|---:|---:|
| 1/48/120 | -26.8 / -26.4 / -26.2 | 418 / 476 / 476 | -22.5 / -21.4 / -21.1 | -2.8 / -3.1 / -3.2 | 0 |
| 2/48/80 | -31.8 / -32.1 / -31.8 | 463 / 464 / 464 | -27.7 / -28.6 / -28.4 | 1.6 / -1.0 / -1.0 | -3 |
| 3/60/80 | -29.3 / -30.5 / -30.2 | 577 / 578 / 577 | -28.3 / -29.5 / -29.3 | -0.1 / -0.1 / -0.1 | 0 |
| 4/55/80 | -30.0 / -31.1 / -30.8 | 543 / 536 / 536 | -28.3 / -29.1 / -28.9 | 0.4 / -0.6 / -0.6 | 3 |
| 5/48/120 | -27.3 / -26.9 / -26.6 | 436 / 478 / 478 | -22.5 / -20.5 / -20.2 | -1.4 / -0.8 / -0.8 | 3 |
| 6/51/80 | -30.7 / -31.7 / -31.5 | 511 / 517 / 517 | -28.0 / -28.8 / -28.5 | 0.3 / -0.4 / -0.4 | 3 |
| 7/48/80 | -31.2 / -32.1 / -31.9 | 464 / 449 / 449 | -27.7 / -28.7 / -28.5 | -1.1 / -1.0 / -1.0 | 0 |
| 8/60/80 | -29.3 / -30.4 / -30.2 | 579 / 572 / 572 | -28.3 / -29.5 / -29.3 | 0.3 / -0.0 / -0.1 | 0 |
| 9/48/120 | -26.8 / -26.4 / -26.1 | 439 / 478 / 478 | -22.5 / -20.8 / -20.6 | -2.7 / -3.2 / -3.2 | 0 |
| 10/55/80 | -30.0 / -31.1 / -30.9 | 548 / 564 / 564 | -28.3 / -29.1 / -28.9 | 0.2 / -0.0 / -0.0 | 3 |
| 11/51/80 | -30.7 / -31.7 / -31.5 | 508 / 497 / 497 | -28.0 / -28.8 / -28.5 | -0.9 / -1.0 / -1.0 | 0 |
| 12/48/80 | -31.2 / -32.1 / -31.9 | 463 / 445 / 445 | -27.7 / -28.7 / -28.5 | -1.0 / -1.1 / -1.1 | -3 |
| 13/60/120 | -24.8 / -24.7 / -24.5 | 537 / 620 / 620 | -22.6 / -21.4 / -21.1 | -2.2 / -2.1 / -2.1 | 3 |
| 14/55/80 | -30.0 / -31.1 / -30.8 | 550 / 565 / 565 | -28.3 / -29.1 / -28.9 | -0.5 / -0.7 / -0.7 | 0 |
| 15/48/80 | -31.2 / -32.1 / -31.9 | 464 / 447 / 447 | -27.7 / -28.7 / -28.4 | -1.1 / 0.3 / 0.3 | 3 |
| 16/48/80 | -31.2 / -32.4 / -32.1 | 461 / 446 / 446 | -27.7 / -28.5 / -28.3 | 0.4 / 0.2 / 0.2 | 0 |

#### pattern-accent-slide

| Note/key/velocity | L (dBFS) | B (Hz) | P (dBFS) | D (dB) | on (ms) |
|---|---:|---:|---:|---:|---:|
| 1/48/120 | -26.9 / -26.4 / -26.2 | 417 / 476 / 476 | -22.4 / -21.4 / -21.1 | -2.7 / -3.1 / -3.2 | 3 |
| 2/60/80 | -31.3 / -32.2 / -32.0 | 358 / 373 / 373 | -28.8 / -29.8 / -29.6 | -0.6 / -0.2 / -0.2 | legato |
| 3/55/80 | -30.0 / -31.1 / -30.9 | 532 / 542 / 542 | -28.3 / -29.1 / -28.9 | 0.3 / 0.3 / 0.3 | 0 |
| 4/48/80 | -31.2 / -32.1 / -31.8 | 463 / 445 / 445 | -27.7 / -28.7 / -28.4 | 0.3 / -1.1 / -1.1 | 0 |
| 5/51/120 | -29.4 / -30.0 / -29.8 | 355 / 347 / 347 | -24.8 / -25.0 / -24.8 | -1.4 / -2.9 / -2.9 | legato |
| 6/63/80 | -29.2 / -30.3 / -30.1 | 647 / 656 / 656 | -28.5 / -29.6 / -29.4 | -0.1 / -0.4 / -0.5 | 0 |
| 7/55/80 | -30.0 / -31.4 / -31.1 | 545 / 533 / 533 | -28.3 / -29.1 / -28.9 | 0.3 / -0.7 / -0.7 | 0 |
| 8/48/80 | -31.6 / -33.1 / -32.9 | 381 / 369 / 369 | -28.8 / -29.9 / -29.6 | -1.0 / 0.3 / 0.3 | legato |
| 9/48/120 | -26.7 / -26.5 / -26.3 | 432 / 478 / 478 | -22.4 / -20.5 / -20.2 | -2.1 / -2.1 / -2.1 | 3 |
| 10/60/80 | -29.2 / -30.5 / -30.2 | 581 / 601 / 601 | -28.3 / -29.5 / -29.3 | 0.1 / -0.3 / -0.3 | 0 |
| 11/55/80 | -31.1 / -31.9 / -31.7 | 418 / 416 / 416 | -29.3 / -30.3 / -30.1 | -0.0 / -0.8 / -0.8 | legato |
| 12/51/80 | -30.7 / -31.7 / -31.5 | 502 / 493 / 493 | -28.0 / -28.8 / -28.5 | 0.4 / 0.2 / 0.2 | 3 |
| 13/48/120 | -26.7 / -26.6 / -26.3 | 442 / 479 / 479 | -22.4 / -20.7 / -20.5 | -2.6 / -2.1 / -2.1 | 3 |
| 14/55/80 | -31.8 / -32.7 / -32.5 | 335 / 348 / 348 | -29.0 / -30.0 / -29.8 | 0.5 / 0.4 / 0.4 | legato |
| 15/60/80 | -29.2 / -30.7 / -30.4 | 568 / 579 / 578 | -28.1 / -29.6 / -29.3 | -0.2 / -0.2 / -0.2 | 0 |
| 16/48/80 | -31.3 / -32.1 / -31.9 | 465 / 443 / 443 | -27.7 / -28.5 / -28.2 | 0.3 / 0.3 / 0.3 | 0 |

#### pattern-rests-ties

| Note/key/velocity | L (dBFS) | B (Hz) | P (dBFS) | D (dB) | on (ms) |
|---|---:|---:|---:|---:|---:|
| 1/48/120 | -27.3 / -26.4 / -26.2 | 418 / 476 / 476 | -22.5 / -21.4 / -21.1 | -1.9 / -3.1 / -3.2 | -3 |
| 2/48/80 | -31.2 / -32.6 / -32.4 | 455 / 449 / 449 | -27.7 / -28.6 / -28.3 | -1.1 / 1.4 / 1.4 | -3 |
| 3/48/80 | -31.3 / -32.1 / -31.9 | 459 / 445 / 445 | -27.7 / -28.6 / -28.3 | 0.4 / 0.1 / 0.1 | 0 |
| 4/60/80 | -29.2 / -30.4 / -30.2 | 570 / 571 / 571 | -28.2 / -29.5 / -29.3 | -0.2 / 0.3 / 0.2 | 0 |
| 5/55/80 | -30.0 / -31.2 / -30.9 | 536 / 534 / 535 | -28.2 / -29.2 / -28.9 | -0.5 / 0.4 / 0.4 | 0 |
| 6/48/120 | -26.7 / -26.4 / -26.1 | 436 / 478 / 478 | -22.6 / -20.4 / -20.2 | -2.7 / -3.2 / -3.2 | 3 |
| 7/48/80 | -31.2 / -32.6 / -32.4 | 455 / 449 / 449 | -27.8 / -28.6 / -28.3 | 0.3 / 1.4 / 1.4 | 0 |
| 8/60/80 | -29.2 / -30.7 / -30.4 | 577 / 573 / 573 | -28.3 / -29.5 / -29.2 | -0.0 / 0.1 / 0.1 | 3 |
| 9/55/80 | -30.0 / -31.1 / -30.8 | 537 / 535 / 535 | -28.3 / -29.0 / -28.8 | -0.5 / -0.6 / -0.7 | 0 |
| 10/48/80 | -31.2 / -32.1 / -31.9 | 459 / 442 / 442 | -27.8 / -28.7 / -28.4 | -0.5 / 0.3 / 0.3 | 3 |

### Final scoring, validation and residuals

Full Harness-v2 error **1.0198058937 → 0.9576377038** (**6.10% reduction**),
346/346 captures, **zero analysis failures**. The requested four-category subset
is **1.4467224324 → 1.3790682907**; its weights differ from the full objective.
Category contribution below is `0.75*median + 0.25*p90` before weighting.

| Category | Median before → after | p90 before → after | Contribution before → after | Change |
|---|---:|---:|---:|---:|
| pitch | 0.0957 → 0.0961 | 0.2651 → 0.2667 | 0.1380 → 0.1387 | +0.49% |
| tuning | 0.0995 → 0.0995 | 0.5593 → 0.5599 | 0.2145 → 0.2146 | +0.04% |
| waveform | 0.9161 → 0.9146 | 1.4625 → 1.4541 | 1.0527 → 1.0494 | -0.31% |
| vcf | 0.5762 → 0.5762 | 4.1235 → 4.1235 | 1.4631 → 1.4631 | +0.00% |
| envelopes | 0.6346 → 0.4007 | 0.8764 → 0.6797 | 0.6951 → 0.4705 | -32.31% |
| amplitude | 1.0204 → 0.9555 | 3.0017 → 2.9010 | 1.5157 → 1.4419 | -4.87% |
| accent | 0.2452 → 0.2484 | 0.9055 → 0.9058 | 0.4103 → 0.4128 | +0.60% |
| slide | 0.0000 → 0.0000 | 0.1565 → 0.1565 | 0.0391 → 0.0391 | +0.00% |
| gate | 0.5174 → 0.5174 | 0.8480 → 0.7761 | 0.6001 → 0.5821 | -3.00% |
| behavior | 2.8152 → 2.6707 | 2.9732 → 2.7748 | 2.8547 → 2.6968 | -5.53% |

No category worsens by more than 2%; the largest increase is accent **0.60%**.
Gate on-timing median/p90 is **3.674/6.101 → 3.674/5.382 ms**;
release median/p90 stays **1.500/2.378 ms**. Behavior absolute-level median/p90
improves **0.393/0.458 → 0.111/0.195 dB**. Envelope normalized RMS median/p90
improves **1.257/1.908 → 0.694/1.368 dB**, amplitude-decay ratio median/p90
**0.108/0.198 → 0.067/0.165 octaves**. These are output measurements, not
independent capacitor or transistor measurements.

Accent level-increase p90 remains **2.735 → 2.737 dB**. Minimum-resonance full
accents remain roughly 4 dB too loud in the 15–60 ms window; maximum-resonance
levels are closer, so a common accent-current multiplier would sacrifice those
captures. Brightness and peak/decay discrepancies also implicate the VCF/accent
sweep and OTA/device transfer. R119/R120 are already at the prior +5% tolerance
fit. Unmeasured cross-loading, OTA transfer/headroom and the filter's large-signal
response limit further physically justified fitting in this scope. No claim
is made that the high-accent residual was solved.

Quiet-gap waveform/capacitor transients, interface noise, uncertain latency and
MIDI send jitter remain significant. `pattern-classic` retains 6.483 dB normalized
RMS trajectory error despite its smaller aggregate contribution. A recording
with a calibrated interface transfer and independent gate/internal-node probes
would distinguish coupling, noise and timing from envelope components.

All ten workspace packages were tested with explicit offline `-p` selections:
**118 passed, zero failures, four manual tests ignored**. ENV has **15 passing
tests**, including the new actual-Q38 partial-charge retrigger/rate test;
its recharge test now checks independent Q31-base-loaded physics instead of
pinning the superseded unloaded RC law. The output test checks the fitted
coupling corners. Existing Q31 junction residual/tail, C72 release, OTA/accent
current, sanitization and extreme-run tests pass. ENV/voice/sequencer all-target
Clippy with `-D warnings`, package-scoped rustfmt checks and `git diff --check`
pass. No dependencies, allocation, locks or unbounded tick loops were added.

Three sequential warmed eight-second Full Circuit renders at 48 kHz measured
**18.1 / 18.0 / 17.9% of one core** (1.446 / 1.442 / 1.436 s wall time).
Each processed **1,536,000 VCF samples**, **zero non-convergence**, zero sanitized
inputs, maximum Newton count **3**. Timings are wall-clock observations, not a
hard scheduling guarantee. Existing engine allocation checks pass. No commit
or branch/history change was made.

Reports: `/tmp/env-before.json`, `/tmp/env3-score.json`,
`/tmp/env3-full-score.json`, with matching Markdown companions. Final renderer
SHA-256: `712176db16feaaba441b9770f48aa6a6f58ea1accee652b7e4dc4acc105f7ede`.
Analysis SHA-256: `c62e2c0e72028d55d33f49a37f3d2acee6917575107bd33dfe376d10a1e741bb`.

## Session-1 fitting round 4 (Harness v3, 2026-10-12)

Baseline in this isolated worktree reproduces the merged round-3 Harness-v2
score **0.8917369997**, all 346 WAV/JSON captures, zero analysis failures.
The binding contract, PLAN sections 3/4/6, schematic ENV/VCA transcription,
preceding 42-note audit and measured discrepancy document were reviewed.
The only circuit-side edit is the permitted voice output calibration constant.
FilterControl cutoff constants, all capacitor/resistor/device defaults,
MonoNoteInput, renderer replay, manifests and dependencies are unchanged.

### Timing and MIDI semantics audit

The earlier 20%-peak pattern crossings confound arrival with waveform/attack
shape. A read-only audit now uses bounded 0.5-ms RMS, 2%-peak arrivals with a
pre-attack noise guard, and records both scheduled and logged-send timestamps.
It is reproducible with `tools/analysis/harness/timing.py`; all note evidence is
in `/tmp/env4-timing-audit.json`. See [Harness v3](../harness.md#harness-v3-2026-10-12-env-fitting-round-4)
for the metric definition and unchanged-model comparison.

| Probe | Gates | Schedule→arrival mean / SD / range (ms) | Send→arrival mean / SD / range (ms) |
|---|---:|---:|---:|
| loopback | 1 | 10.500 / — / — | 7.482 / — / — |
| gate-80 | 4 | 8.750 / 1.677 / 4.500 | 6.834 / 0.822 / 2.074 |
| gate-120 | 4 | 8.000 / 0.612 / 1.500 | 5.473 / 1.168 / 3.206 |
| gate-160 | 4 | 7.438 / 1.891 / 4.750 | 5.922 / 1.190 / 3.281 |
| pattern-classic | 16 | 4.438 / 0.864 / 3.000 | 3.720 / 0.844 / 2.352 |
| pattern-accent-slide | 11 | 4.136 / 1.245 / 3.500 | 3.350 / 0.874 / 2.380 |
| pattern-rests-ties | 10 | 6.050 / 1.739 / 5.500 | 4.943 / 1.661 / 4.595 |

**VERIFY: timing attribution.** These are acoustic arrival variations,
including waveform/attack dependence and the sidecar's estimated audio clock.
Logged send subtraction removes host dispatch jitter but does not identify
TD-3 firmware, MIDI transport, interface buffering or circuit delay separately.
The single-note `loopback` probe cannot establish jitter independently. The
20%-peak audit gave larger 5–8.5-ms pattern ranges; using those as arrival
alignment worsened shape scores and was rejected. The low-threshold variations
are still significant for logarithmic quiet-gap comparisons.

The 42 scheduled pattern notes contain **37 genuine gate rises and five legato
note-ons**. Each overlap retains its gate island, consistent with the existing
last-note-priority, no-retrigger slide behavior and the round-3 note audit.
`rests-ties` sends notes/rests rather than internal tie commands. Accent velocity
120 versus normal 80 matches the existing >=100 threshold. There is no capture
evidence establishing a different threshold, fixed gate fraction, same-key
retrigger or accent hold after final release. MonoNoteInput therefore remains
unchanged; the already rejected held-accent trial is not reintroduced.

### Metric correction separated from model fitting

V3 rigidly translates each model gate island to its measured reference arrival
for trajectory shape, with changes in scheduled rests. It preserves note
pitch, within-island decay, legato and release duration. Accent windows compare
15–60 ms acoustic age in each instrument and its own `a00` baseline. Absolute
level, fitted time constants and all pitch/VCF/glide features remain measured
on original audio. Audible gate errors retain original 3-ms/20% crossings;
only the first model/reference arrival difference is subtracted globally.
Later onset jitter and release-duration discrepancies remain scored.

On the **unchanged model**, v2 **0.8917369997 → v3 0.8822330482**. This is a
change of objective, not circuit progress. Behavior contribution improves
3.23%, amplitude 5.70%, envelopes 1.86%. Gate contribution increases **18.40%**
and accent **4.76%**; these >2% increases are explicitly retained because the
corrected measurement exposes errors hidden by mismatched acoustic ages.
Gate on-error median improves **3.674 → 2.179 ms** while release-error median
increases **1.500 → 4.444 ms**: after removing duplicated first attack,
model audible releases are generally early. Accent level-increase p90 rises
**2.960 → 3.319 dB**. Weights/scales are unchanged. None of these increases
comes from a circuit edit, and no gain/time adjustment is made per capture.

The timing correction only modestly improves patterns. `classic` normalized
RMS trajectory is **6.467 → 6.026 dB**; `accent-slide` is **5.200 → 5.239 dB**;
`rests-ties` is **4.545 → 4.005 dB**. Thus residual timing alone does not explain
the remaining behavior score. A diagnostic of the largest v2 RMS errors found
frames around note boundaries, e.g. classic at age 0.48 s **−44.6 dBFS hardware
versus −85.8 dBFS model**, where a short analysis frame can include a different
amount of the following attack. Low-frequency 30–76 Hz band trajectories and
quiet coupling tails also differ; that does not independently identify a
capacitor tolerance or justify an invented audio envelope/noise source.

### Fitted calibration and VERIFY register

| VERIFY item | Previous / nominal → fitted | Evidence and residual |
|---|---|---|
| `FULL_SCALE_VOLTS` | Original 5.12; round 3 **10.041 → 10.192 V/full scale** | One session-wide **−0.12965 dB** calibration. A bounded gain-only evaluation on v3 selected about −0.125…−0.13 dB; −0.13 dB was rebuilt and rescored. This is a TD-3/UMC1820 recording-chain estimate, not a printed part value or measured absolute voltage. Full amplitude contribution improves 0.73%, envelopes 3.99%, behavior 0.29%. Absolute amplitude median worsens 0.560 → 0.623 dB while p90 improves 2.564 → 2.460 dB; the objective balances the distribution. |
| Acoustic-arrival estimate | One sidecar first offset → per-gate 2% RMS proxy | Hardware timing variations above justify explicit nuisance alignment for shape. 0.5-ms quantization, attack/phase, noise guard and rigid joins in rests remain approximations. `pitch-k24-t0` exceeds the 25-ms offset bound and keeps raw trajectory scoring with a warning. No data/notes are excluded. |

All previous VERIFY registers remain binding: generic junction drops and
D26–D28 clamp/IC12 Ron/supply; Q9/Q10/Q11/Q31/Q32/Q40/Q41 Vbe/Is/beta/matching
and temperature; Q39 Idss; Q36/Q38/Q32 saturation/headroom, Q36 gate reference
and C72 dynamics, Q31 reference/ideality/base-current reduction; BA662A pin-1,
mirror gain/offset/feedthrough/buffer transfer; VR4 gang correlation and mutual
accent-branch loading; fitted C13/C42/C62/C72/C38/C59/C56 and R138/VR6/R119/R120
tolerances; VR5/VR6/VR8 tracks; VCF-owned cutoff/expo/trim calibration; omitted
C38/VR8/C59 mutual loading; interface response/noise/voltage calibration;
transcription duplicate C22/D27, R147 notation, CPU strobe/TP6/headphone
ambiguities. Their nominal→fitted values are in the preceding round tables;
none was retuned in this round.

Remaining model error is compatible with several unmeasured mechanisms:
Q36 saturation/storage and output coupling can affect gate release; MEG/VCF
large-signal response, accent branch loading and OTA transfer affect high
accent level/brightness. Quiet-gap transients, low-frequency phase and input
noise affect behavior/amplitude. Output audio alone does not identify these
uniquely. The current electrically plausible fitted component bounds are
retained rather than extending tolerances or altering MIDI semantics to absorb
capture timing. Electrical loopback with repeated edges, timestamped MIDI,
calibrated input response and internal gate/envelope probes are needed to
separate those remaining terms.

### Final same-v3 results

Final rebuilt **overall 0.8822330482 → 0.8779866621** (**0.48% improvement**).
Requested subset **1.3469 → 1.3423**. Relative to the original v2 report the
final number is 0.8917 → 0.8780, but those different objectives must not be
reported as one circuit improvement. All **346/346** captures are analysed,
**zero failures**, one explicitly warned raw-alignment fallback.

| Category | V2 baseline median / p90 | V3 unchanged median / p90 | V3 final median / p90 | Same-v3 contribution change |
|---|---:|---:|---:|---:|
| behavior | 2.6445 / 2.7299 | 2.5360 / 2.7113 | 2.5213 / 2.7257 | −0.29% |
| amplitude | 0.9356 / 2.9636 | 0.8725 / 2.8242 | 0.8584 / 2.8266 | −0.73% |
| gate | 0.5174 / 0.7761 | 0.6624 / 0.7697 | 0.6624 / 0.7697 | 0.00% |
| accent | 0.2478 / 0.9090 | 0.2485 / 0.9856 | 0.2485 / 0.9856 | <0.00001% |

No category worsens by >2% under the same v3 metric. Envelopes improve 3.99%;
pitch, tuning, waveform, VCF and slide are unchanged to reporting precision.
The numerical effect on those categories is float-WAV rounding after the
single global gain adjustment, not a DSP topology change.

Validation: **122 workspace tests passed**, zero failures, four manual tests
ignored, using explicit offline `-p` selections for all ten workspace packages.
**17 Python metric tests pass**, including retained gain/release mismatch,
identity, silent-arrival rejection, overlaps/channel identity and gate-origin
correction. ENV/voice/sequencer/renderer all-target Clippy with `-D warnings`,
package-scoped rustfmt checks and `git diff --check` pass. The DSP change adds
no allocations, locks, loops or nonlinear work. No dependencies or commits.

Reports: `/tmp/env4-before-full.json`, `/tmp/env4-v3-unchanged-full.json`,
`/tmp/env4-score.json`, `/tmp/env4-full-score.json` and matching Markdown;
`/tmp/env4-timing-audit.json` contains the per-arrival reference measurements.
Final renderer SHA-256:
`b7e91a487996431c93051d44a17655be55c95876e5ef96f5f51be55e1508c48f`.
Final analysis SHA-256:
`7164879656a7304f1483c53fc542186545414c1abc0218f7373eb743125c90d1`.

Three sequential warmed eight-second Full Circuit renders at 48 kHz measured
**1.349 / 1.347 / 1.345 s**, **16.9 / 16.8 / 16.8% of one core**.

## Session-1 fitting round 7 (Harness v4, 2026-10-12)

Round 7 added the printed VR7 source resistance to the VCA accent load
and jointly refits Q36/C72/R142/R143 for attack and release. All earlier VERIFY
items remain applicable; the three new register rows above describe the added
reduction and nominal→fitted values. Full category, solver and CPU evidence,
rejected trials and the noise-sensitive pattern centroid audit are in the
[round-7 report](vcf-env-fitting-round7-2026-10-12.md). The VCF topology, resonance
gang direction, cutoff/ENV MOD/DECAY tracks, OTA transfer and output calibration
retain their preceding fits.
Each processed **1,536,000** VCF samples, maximum Newton count **3**,
**zero non-convergence** and zero sanitized inputs. These are isolated wall-clock
observations after scoring completed, not a hard scheduling guarantee.
Reproduce with `target/release/dw303-render /tmp/env4-cpu.wav --seconds 8 --rate 48000`.


## Session-1+2 interaction fit (round 9)

The accepted current fit and complete scores/VERIFY inventory are in
[round 9](vcf-env-fitting-round9-2026-10-12.md). C72 is **12 µF** (+20% assumed
tolerance), effective Q36 reference is **0.40 V**, and the two low-current
MEG discharge drops are separately **0.45 V**. D26 remains **0.60 V** under
R117 bias; C62 floor is **1.50 V**, buffered MEG rest **0.20 V**. The voice's
rest calibration is derived from those same component fields. The earlier
transcribed-nominal equations and preceding round tables are historical
evidence, rather than the current fitted defaults.

The gate electrical 20%-current release is about **10 ms** at 48/96/192 kHz.
C62's decay times and D37 charge peak are retained. All fitted values are
output-audio identifications; low Q36 reference voltage, fixed diode drops,
MIDI off-latency and saturation/storage reduction prevent a device-authenticity
claim. D24/C36 cross-loading and the excess accented peaks remain open.

## 2026-10-12: interaction repeat floor and buffer investigation

This work uses the unchanged v5 harness, both read-only capture sessions, and
the worktree's renderer. The initial full score reproduces `round8.json` exactly:
**0.821752766751**, zero analysis failures, renderer SHA-256
`54db76b4e79c206b4c56d4250306f2a2e65cd3763fabf50f5ed167dc73b94b93`.
The preceding [round-9 investigation](vcf-env-fitting-round9-2026-10-12.md)
remains the record of its rejected trials.

### Interaction metric's own hardware repeat floor

`crates/dw303-env/analysis/interaction_noise_floor.py` calls the actual harness
`analyse(..., category_override='interaction')` on all 18 unordered session-2
repeat pairs. It preserves the common first-take timeline, per-gate acoustic
alignment, raw absolute level, decay-fit penalties and the additional normalized
full-window spectrum. It neither changes scoring nor subtracts the floor.

| Hardware stimulus | Pairs | Normalized median | p90 |
|---|---:|---:|---:|
| All three repeat patterns | 18 | 1.051804 | 1.349860 |
| Accent/slide | 6 | 0.769816 | 1.059668 |
| Classic | 6 | 0.932946 | 1.102300 |
| Rests/ties | 6 | 1.306435 | 1.457640 |

The corresponding aggregate is **1.126318**. Native-unit repeat median/p90:
bands **1.528309/1.715440 dB**, RMS shape **1.963017/2.544366 dB**,
normalized spectrum **1.050181/1.355091 dB**, absolute level
**0.016815/0.076419 dB**. The existing behavior floor stays
**1.016496/1.407178**; adding the interaction spectrum changes the metric mean.

There are **zero exact duplicate combo stimuli** after checking every setting,
tempo, MIDI mapping and relative raw MIDI schedule, including note key and
velocity. Single/phrase pairs share controls but have different gate durations
and cannot provide an interaction repeat floor. VERIFY: these accented,
mixed-key pattern repeats are a variability proxy, not an independently
measured lower bound for the unaccented, repeated-key combo phrases or singles.
Four takes and three patterns do not identify the source of this variability.

Matched session-1 patterns versus their four session-2 repetitions give an
interaction median/p90 **1.241567/1.351811**. Session-2 signed level increments
are positive in all 12 comparisons: median approximately **+0.095 dB**, range
**+0.029 to +0.126 dB**. Classic/accent-slide/rests-ties family medians are about
**+0.079/+0.097/+0.079 dB**. VERIFY: temperature, supply, interface gain,
carrier phase and MIDI arrival are not separately identified. This small session
offset does not justify a session-selective thermal correction in the synth.

### Ranked initial combo residuals

Each family below pairs keys 36/48 at identical controls and stimulus. Mean
normalized error retains the harness's 10-unit missing-fit penalties; the
feature-only column exposes the residual underneath. Updated rankings are
emitted by `analysis/interaction_audit.py`, without changing the harness.

| Family | Mean error | Feature-only mean | Penalized captures / 2 |
|---|---:|---:|---:|
| CUTOFF .6, ENV 1, RES 1, phrase | 12.7553 | 2.7553 | 2 |
| DECAY 1, RES .8, phrase | 8.1058 | 3.1058 | 1 |
| CUTOFF .2, ENV 1, RES 1, phrase | 7.4966 | 2.4966 | 1 |
| CUTOFF .2, ENV .6, RES .8, phrase | 7.3905 | 2.3905 | 1 |
| DECAY 0, RES .5, phrase | 7.3580 | 2.3580 | 1 |
| CUTOFF .6, ENV 0, RES .8, phrase | 7.3514 | 2.3514 | 1 |

Ranking only the measured features instead puts DECAY 1 / RES 0 first
(**4.0802**), followed by DECAY 1 / RES .8 (**3.1058**) and
CUTOFF .6 / ENV 0 / RES 0 (**2.8699**). ACCENT is zero and all phrase
velocities are 80: D24 and the VCA accent branch are disconnected from MEG
for these stimuli. Accent mutual loading cannot explain their main discrepancy.
The high-RES penalties often compare an exponential fitted to only seven
reference brightness windows with a whole repeated-note trajectory. They remain
in the score; fitting them is not an independent measurement of C62 decay.

### Loading and identifiability limits

R61 already supplies the physical ENV MOD residual at CCW; it is not an omitted
zero-envelope switch. Q41/Q40's source impedance is missing. Assuming Q39 Idss
**1.2 mA** gives a two-junction incremental follower resistance about **43 Ω**
at 25.9 mV VT. The standing VR5/R61 load at peak changes the two Vbe drops by
only about **4.5 mV**; even approximately 0.5 mA additional accent loading changes
them by less than **20 mV**. VERIFY: Q39's individual grade/Idss and Q40/Q41 beta,
Early effect, cutoff behavior and base loading are unmeasured. These estimates
are not accepted fitted device parameters or a complete buffer solve. They
cannot account for the multi-dB ordinary combo residual.

C36 and D24 currents share the real VR7/MEG source. The existing voltage-only
stage boundary supplies neither the pot/source resistance nor both capacitor
currents to a common solve; the assigned voice scope permits constants only.
An exact shared-node model requires coordinated integration work. A guessed
midpoint RES load inside Accent would not implement the actual gang setting.
At full ACCENT the ideal VR7 wiper resistance is zero, so its loading alone
cannot explain the full-turn accent excess. The unknown MEG buffer/device
impedance and OTA transfer remain explicit VERIFY items.

### Accepted fit and VERIFY register

Only **R61 changes at runtime**. The printed 10 kΩ resistor becomes
**9.5 kΩ**, the assumed −5% tolerance endpoint. The CCW VR5 source fraction
changes **0.166667 → 0.159664**, and its Thevenin resistance
**8.333333 → 7.983193 kΩ**. The retained CCW peak/rest ladder-current ratio
is about **1.749**, or **0.807 octaves**. C62/C72/C42 and all pot tracks retain
their preceding fits; their physical charges, decay and gate timing are not
replaced by a new envelope law.

The existing voice calibration still holds the settled centre ring target.
Its computed expo ratio changes **3.111363443 → 3.087816429** as a consequence
of R61, without an additional manual fit or a voice runtime edit. VERIFY:
neither this ratio nor R61 is independently identified from the audio. TM3,
the Q9 reference and device matching remain coupled to that calibration.

| VERIFY item | Nominal / preceding → retained | Evidence and residual |
|---|---|---|
| R61 | Printed **10 kΩ → 9.5 kΩ** | −5% assumed resistor tolerance. Both tolerance endpoints were replayed and fully scored. Lower endpoint gives the smaller full score and improves behavior/amplitude. This does not measure the hardware resistor; VR5 taper, Q9 bias, transistor laws and panel placement remain confounded. |
| Computed expo normalization | **3.111363443 → 3.087816429** | Existing centre-ring normalization compensates the different resting CCW feed. It is a derived calibration, not a separate component/device fit. |
| Interaction floor | No previous interaction-specific repeat result → **1.051804 / 1.349860 median/p90** | 18 within-session pairs, exact v5 interaction metrics. Pattern settings differ from combo stimuli, and there are no exact combo duplicates. This is a variability proxy, not a universal noise bound or a score subtraction. |
| Session offset | No thermal model → retain no session-dependent correction | Hardware session-2 pattern levels are about +0.095 dB; attribution to temperature rather than interface/supply/phase remains unresolved. |
| MEG / accent loading bounds | No accepted new device value | Estimated Q39 current makes ordinary buffer voltage droop too small to explain the main combo residual. Exact D24/C36 mutual loading still requires shared-node integration outside the assigned voice-constants scope. |

All preceding VERIFY registers remain applicable, including fixed MEG diode
drops, Q36's unusually low effective junction reference, Q31/buffer beta and
current laws, Q39 Idss, IC12 Ron/supply, VR4 gang correlation, OTA control voltage,
mirror gain/offset/output resistance, output-loading reductions, capture latency,
absolute recording volts and the earlier capacitor/pot/resistor tolerance fits.
No authenticity claim is made for the fitted TD-3 unit or its internal values.

### Full v5 scores

Category aggregate is `0.75*median + 0.25*p90`. The complete overall score is
**0.821752766751 → 0.820619690550 (−0.137885%)**. This is a small improvement;
the main phrase discrepancy remains. Both reports have zero analysis failures.
No category worsens by more than 2%; the largest increase is envelopes at
**0.672510%**.

| Category | Before | After | Change |
|---|---:|---:|---:|
| interaction | 1.832589761 | 1.831926690 | −0.036182% |
| behavior | 1.891781609 | 1.881799076 | −0.527679% |
| amplitude | 1.232026556 | 1.223434434 | −0.697397% |
| envelopes | 0.321122628 | 0.323282209 | +0.672510% |
| accent | 0.346873717 | 0.349160101 | +0.659140% |
| gate | 0.510936207 | 0.510936207 | 0.000000% |
| VCF | 0.704232636 | 0.704231428 | −0.000171% |
| waveform | 0.739877549 | 0.739877754 | +0.000028% |
| pitch | 0.139133897 | 0.139133567 | −0.000237% |
| tuning | 0.199086757 | 0.199086874 | +0.000059% |
| slide | 0.039124674 | 0.039124674 | 0.000000% |

Interaction median/p90 **1.491036/2.857250 → 1.498367/2.832607**: the
median worsens slightly while p90 improves. Behavior median/p90
**1.752903/2.308418 → 1.745888/2.289533**. The 17 interaction missing-fit
penalties remain 17; no failed row, quiet gap or inconvenient metric is removed.

| Initially worst family | Before mean | After mean | Penalties before → after |
|---|---:|---:|---:|
| CUTOFF .6, ENV 1, RES 1, phrase | 12.755298 | 12.731357 | 2 → 2 |
| DECAY 1, RES .8, phrase | 8.105778 | 8.126855 | 1 → 1 |
| CUTOFF .2, ENV 1, RES 1, phrase | 7.496594 | 7.459070 | 1 → 1 |
| CUTOFF .2, ENV .6, RES .8, phrase | 7.390517 | 7.363566 | 1 → 1 |
| DECAY 0, RES .5, phrase | 7.358032 | 7.297544 | 1 → 1 |
| CUTOFF .6, ENV 0, RES .8, phrase | 7.351380 | 7.324961 | 1 → 1 |

Three-way single/phrase medians **0.798194/2.207556 → 0.795967/2.216075**;
DECAY-extension single/phrase medians **0.542537/2.101521 →
0.518387/2.077449**. Phrase brightness fits, independent carrier phase,
per-note timing variation and capture background limit attribution. The ordinary
ACCENT=0 combo residual is not evidence for missing accent loading. Source
waveform, VCF downstream loading and shared accent/OTA nodes require coordinated
work rather than larger unexplained envelope multipliers.

### Rejected new trials

The fixed diagnostic set contains 136 captures: 64 selected interaction rows,
all 22 envelopes and 3 gates, 32 endpoint-accent rows, 3 patterns and 12 repeat
takes. Repeat means use the harness's canonical first-take order. Its interaction
aggregate **4.399088** deliberately emphasizes bad phrases and is not the full
interaction aggregate. Rows below label subset versus complete comparisons.

| Trial | Evidence / rejection |
|---|---|
| OTA input offset +1 mV, no output-resistance change | Diagnostic behavior **1.891782 → 6.347689**, interaction **4.399088 → 6.675697**. Strong quiet-gap feedthrough is contradicted by the audio. Reverted. |
| OTA offset +10 µV | Complete related-category interaction **−0.0720%**, behavior **+0.1857%**; the small apparent subset behavior gain disappeared after canonical repeat ordering. A separate <20-Hz waveform residual fit suggested about +6 µV for selected combos but mixed signs across repeated patterns. Offset is not uniquely identified; retain zero. |
| OTA Early voltage 100 V, mirror 1.024687 (ordinary 47-µA peak normalized), C42 1.05 µF | Diagnostic envelopes **0.321123 → 0.338927 (+5.54%)**; reject. This isolates output resistance while retaining C42, unlike the preceding round's joint 22-V trial. |
| OTA 100 V, mirror 1.02, C42 1.04 µF, zero offset | Diagnostic envelopes **0.341732 (+6.42%)**; reject. Modest capacitor/mirror refit does not rescue the loading reduction. No Early voltage/mirror/capacitor trial value is retained. |
| Q41 drop .65 → .55 V, Q40 retained .65 V; rest calibration derived from both | Diagnostic interaction **4.399088 → 4.462403 (+1.44%)**, despite small behavior/accent gains. Raises the buffered node but worsens the targeted high-ENV spectral residual. Reverted; constant-drop versus source-impedance/base-loading reductions remain unverified. |
| R61 10.5 kΩ (+5%) | Complete overall **0.821126500932**; envelopes improve 3.70%, behavior worsens 0.67%. All categories pass the regression limit, but 9.5 kΩ gives the lower overall score once amplitude is included. |

The earlier round's rejected VR5 .6-knot increase, passive VCA/VCF loading,
and joint 22-V OTA/C42 trial were not repeated. None of the new diagnostic
device estimates establishes a measured BA662A or transistor parameter.

### Final validation and reproduction

**135 workspace tests pass**, zero failures, five manual benchmarks ignored,
across all ten explicitly selected packages. Scoped ENV/voice all-target Clippy
with warnings denied, scoped rustfmt and diff whitespace checks pass. The
superseded raw-control-current fixture is updated with rounded tolerances, and
a new 48/96/192-kHz test checks the retained CCW feed and its reduction from
printed R61. Public stage signatures, dependency/manifests and other crates
are not changed. The runtime change is one resistor constant: no allocation,
lock, extra solve, unbounded loop or new envelope state is introduced.

Exact-replay solver audits cover **346 + 274 = 620 captures** with **zero
non-convergence**, **zero sanitized inputs** and **maximum four evaluations**.
The final full run, without category selection, exactly reproduces the retained
trial's overall and every category. The physical mutual-loading reductions and
capture limitations above remain; passing these checks does not close them.

Warm sequential Full Circuit holds at 48-kHz host / 192-kHz circuit, three
16-second runs per setting, satisfy the 13% CPU limit:

| Setting | DSP wall median | DSP wall max | Whole-process CPU median | CPU max |
|---|---:|---:|---:|---:|
| C1, default controls | 10.306% | 10.906% | 10.220% | 10.250% |
| C3, default controls | 9.706% | 9.744% | 9.645% | 9.698% |
| C3, CUTOFF/RES max, ENV MOD min | 10.394% | 11.063% | 10.373% | 10.465% |

DSP wall time excludes WAV writing; whole-process user+system CPU includes
setup and writing. This is evidence for these measured workloads, not a claim
about every automation sequence. Raw runs and diagnostics are retained in
`/tmp/envfit-cpu.json`. All holds report zero solver non-convergence/sanitization.

Final renderer SHA-256:
`1aad0de3249a63fc2c7cedceb1f0f4dcc8f7b666de667e5abb5f41d5d9e437b4`.
Unchanged harness analysis SHA-256:
`7fbc7305a654c4fa061c7434777f57539e5f3becd2f36dd36a2d026c44bc4d83`.
Full reports, noise-floor pairs with WAV/sidecar hashes, ranked families,
pattern diagnostics, solver audits and test logs are under `/tmp/envfit-*`.
No commit, branch/history change or push was performed.

```sh
cargo build --offline --release -p dw303-render
python3 crates/dw303-env/analysis/interaction_noise_floor.py \
  /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session1 \
  /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session2 \
  --json /tmp/envfit-noise.json
python3 tools/analysis/harness/score.py \
  /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session1 \
  /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session2 \
  --categories interaction,behavior,envelopes,accent,gate,vcf \
  --cache /tmp/envfit-cache --jobs 4 --json /tmp/envfit-related.json
# Final run omits --categories and writes /tmp/envfit-score.json.
cargo test --offline --release -p dw303-osc -p dw303-vcf -p dw303-env \
  -p dw303-sequencer -p dw303-voice -p dw303-engine -p dw303-clap \
  -p dw303-standalone -p dw303-render -p dw303-capture
cargo clippy --offline -p dw303-env -p dw303-voice --all-targets -- -D warnings
cargo --offline fmt -p dw303-env -p dw303-voice -- --check
```


## Phrase dynamics follow-up (Harness v7, 2026-10-12)

The starting renderer is the already merged round-10 implementation, including
reciprocal C38/C59 loading and the previous retrigger work. Its v7 overall
score is **0.752349791399**, SHA-256
`2d7e313cea259a7c1d355f95aadcb55f9a8f3d71fab02ef5b4f575fd77bb0c17`.
The rebuilt related-category baseline reproduces all six published category
statistics exactly. Neither the scoring code nor capture data was changed.
PLAN sections 3/4/6, the binding contract, v7 harness rules and the reconciled
ENV/netlist authority were read before fitting.

### Time-resolved diagnosis

`crates/dw303-env/analysis/phrase_dynamics.py` now records 20-ms RMS windows at
10-ms age increments, bin means, RMS after subtracting each bin's mean, the
8-ms attack peak's age, and the next-onset interval. Bin-mean subtraction is
an offline diagnostic, not a 30-Hz high-pass or a changed scoring metric.
The audit retains absolute output levels, each gate island's independently
measured acoustic arrival, every scheduled note and the original release age.
No gain fit, release alignment or MIDI schedule adjustment is applied.

All **119** combo phrases, patterns and repeat takes contain **1,042 notes**.
Baseline medians below are signed model minus TD-3, in dB. Held decay means
35–55-ms level minus 15–35-ms level. Raw gap bins are intentionally retained
but can lie near/below capture noise; they are not a replacement for the
harness's common floor and normalized trajectory distances.

| Family | Notes | Attack peak | 15–35 ms | 35–55 ms | Held decay | Off 0–20 ms | Off 20–40 ms | Pre-next 20 ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| combo phrases | 832 | −0.276 | −0.245 | −0.259 | −0.090 | −3.237 | −0.541 | −5.388 |
| session-1 patterns | 42 | −0.357 | −0.456 | −0.490 | −0.075 | −3.496 | +2.467 | −11.689 |
| session-2 repeats | 168 | −0.340 | −0.425 | −0.535 | −0.114 | −4.260 | +2.029 | −4.568 |

Combo first-note peak error is **−0.303 dB**, versus **−0.273 dB** on later
notes. First/later held-decay errors are **−0.506/−0.061 dB**. Repeated normal
attacks do not progressively lose their peak. The existing 48/96/192-kHz test
also demonstrates that real Q38 pulses reset partly charged C42 and recharge
C62 with <1% peak variation. This does not support weakening retriggers or
resetting either storage node artificially. C54's printed 0.47-ms time constant
is much shorter than the ordinary 62.5-ms phrase gap; omitted gate-edge recovery
and R150/C55 sag remain VERIFY items, not evidence for a new long recovery law.

The most consistent ordinary-note discrepancy is early release, not repeated
attack height: output is several dB too quiet immediately after gate-off. On
`pattern-classic`, median 10-ms release bins start at **−33.6/−66.6 dBFS** in
hardware, versus **−37.5/−59.9 dBFS** in the model. Its later model tail is often
too loud. Raw gaps change sign substantially between notes and carrier phases.
At maximum ENV/RES there are also substantial held spectral/level errors;
these cannot be uniquely assigned to VEG from output RMS alone.

**VERIFY: timing attribution.** Release ages reuse the gate island's attack
arrival offset. Host MIDI send/schedule times do not timestamp the hardware
GATE_HI transition. Firmware note-off latency, Q36/C72, OTA behavior, coupling
charge and carrier phase therefore remain confounded. Eight-ms peak windows
are phase-sensitive at key 36, and raw quiet-gap values include background.

### Rejected tolerance/device fits

The fixed diagnostic set contains 120 captures, including 56 interactions,
all 22 envelopes, all 3 gates, 3 patterns, 12 repeat takes and endpoint accent
controls. Its category distribution differs from the full 620-capture matrix;
its baseline weighted subset score is **0.974562051391**. Subset improvements
were checked against the complete matrix rather than treated as acceptance.

The examined Q36 reduction distinguishes its forward-active current capacity
from collector load, includes Q31 emitter current drawn through D35/R133/R131,
and lets C72 lose saturation headroom before GATE_HI falls. The load uses the
preceding sample's Q31 current to avoid an extra real-time solve.

| Trial | Previous → trial values / result |
|---|---|
| Extra headroom only | Q36 current reference **1.18 → 3 mA**, C72 **12 µF**, Vbe **0.40 V**, base current from collector load/β. Diagnostic score **1.387846**; release is much too long. Rejected. |
| Nominal C72 / generic junction | C72 **12 → 10 µF** (printed 10), Q36 Vbe **0.40 → 0.65 V**, reference **1.18 → 5 mA**, collector-load/β base reduction. Diagnostic score improves, but complete interaction aggregate rises about **5.8%**, and accent about **2.5%**. Rejected. |
| Preserve more of the old attack | C72 **12 → 9.5 µF** (printed 10), Vbe **0.40 → 0.60 V**, reference **1.18 → 5 mA**, collector-load/β. Complete overall **0.748682631491**, but interaction **+5.21%**. Rejected despite the smaller overall score. |
| Lower C72 endpoint | C72 **8 µF** (printed 10, −20%), Vbe **0.60 or 0.65 V**, reference **5 mA**. Diagnostic scores **0.967034/1.018064**; ordinary phrase interaction/behavior regressions remain. Rejected. |
| Lower R142 endpoint | R142 **10.5 → 9.5 kΩ** (printed 10), C72 **10 µF**, Vbe **0.65 V**, reference **5 mA**. Diagnostic score **0.9469**; worse than the joint nominal trial. Rejected. |
| Modest load headroom | Earlier C72/Vbe retained; reference **1.18 → 1.40 mA**, Q31 gate loading and collector-load/β. Complete **0.752791839587**; behavior **+1.958%**, interaction **+0.471%**, despite gate **−17.21%**. Overall worsens. Rejected. |
| Smaller load headroom | Reference **1.18 → 1.25 mA**, otherwise preceding row. Complete **0.756661878441**; interaction **+1.824%**, overall worse. Rejected. |
| Additional junction-base trial | C72 **10 µF**, Vbe **0.65 V**, capacity **5 mA**, Q31 emitter load included; retain forward-junction capacity/β for C72 base discharge instead of collector-load/β. Complete overall **0.744185453526**, but interaction **+5.153%**, accent **+2.391%**. Rejected. |

**VERIFY:** these device references are effective junction/saturation estimates,
not measured transistor currents. The fixed-beta saturated reduction omits
base-collector recombination and forced-beta storage. The prior unusually low
0.40-V reference is not authenticated by the output captures. The previous
C72/R142/R143 estimates, unprinted Q31/Q36 laws and all earlier VERIFY registers
remain applicable. None of these trial values revises the transcription.

A read-only common-gain diagnostic on the 1.40-mA trial also fails to rescue
its full objective while keeping every category within 2%. The existing
`FULL_SCALE_VOLTS` calibration and OTA/output topology are retained.


### Retained gate-load correction and VERIFY register

The retained fit keeps the forward-junction base-current reduction, including
its larger base current during saturation. The load/β-only trials above are
not retained. GATE_HI now supplies Q31's emitter current as well as R145:

```text
U = voltage across C72 (Q36 emitter minus base)
capacity = I36_ref * exp((U - V36_ref) / VT)
Q31_load = previous_I31 * (1 + 1/beta31)
GATE_HI = clamp(R145 * (capacity - Q31_load), 0, max(12 - Vsat, 0))
I36_base = capacity / beta36
C72 dU/dt = gate*(12 - Vsat - U)/R143 - U/R142 - I36_base
```

U remains bounded by the existing junction/saturation drive clamp. Capacity
is the transistor's forward-active capacity, not current known to flow through
R145. At full gate, Q31 draws roughly 47 µA in addition to R145's 1.18 mA.
Neglecting that load changes the falling gate supply and the control-current
shape. The preceding circuit sample supplies the load, keeping the explicit
state update bounded without adding a nonlinear solver. A supply with no
positive output headroom closes the gate rather than forming an invalid clamp.
C41/C42/C62 and all existing coupling-capacitor charges remain independent
physical states. No fixed retrigger reset, extra MIDI delay or release envelope
was introduced.

| VERIFY item | Printed / previous → retained | Evidence and limitation |
|---|---|---|
| C72 | Printed **10 µF**, previous **12 → 11 µF** | +10% from transcription, within the assumed ±20% electrolytic tolerance. Joint gate-shape fit against both sessions; C72, R142/R143 and Q36's device law are not individually identified. |
| Q36 junction reference | Generic **0.65 V**, previous **0.40 → 0.50 V** | Effective forward-junction reference and saturated-drive clamp, fitted jointly with C72 and current capacity. Not an internally measured Vbe or authenticated transistor Is. |
| Q36 reference capacity | Unprinted; earlier gate-load estimate **1.18 → 1.90 mA** | Modest saturation margin above the combined R145/Q31 load, instead of the rejected 5-mA trials. Output audio constrains the combined gate response, not this device parameter separately. |
| Q31 gate loading | Omitted → **I_Q31*(1+1/beta31)** | D35/R133/R131 supply path is printed. Emitter/collector relation retains the earlier unprinted beta31=200. The one-sample explicit load is an approximation, checked across 48/96/192 kHz, not a simultaneous transistor solve. |
| Q36 saturated base current | Retain **forward capacity/beta36**, beta36=200 | Forward-junction base current remains present when collector voltage clips. Base-collector recombination, reverse beta and detailed forced-beta storage are omitted; the load/β-only simplification was rejected. |
| Remaining attribution | No new MIDI law or independent timing parameter | C54 recovery/C55 sag, firmware note-off latency, capture clock uncertainty, carrier phase, accent mutual loading and OTA transfer remain unresolved. These cannot be authenticated from OUTPUT alone. |

R142 **10.5 kΩ** and R143 **9.5 kΩ** retain their preceding ±5% fits.
All other component/pot/device defaults and FULL_SCALE_VOLTS **10.192 V** are
unchanged. The existing VCF/oscillator, OTA/output loading and MonoNoteInput
implementations were not edited. All earlier diode, JFET, beta, temperature,
IC12, VR4 gang, OTA, output-load and recording-chain VERIFY items remain open.

The retained per-note audit reduces median off-0–20-ms error from
**−3.237 → −1.687 dB** for combos, **−3.496 → −1.814 dB** for patterns and
**−4.260 → −1.785 dB** for repeats. Peak medians stay within 0.005 dB of the
baseline audit. Later release/gap errors still vary with carrier phase and
coupling charge. For pattern-classic, the median 10–20-ms release bin moves
**−59.9 → −66.1 dBFS**, versus hardware **−66.6 dBFS**. The earlier 0–10-ms
bin still differs; the whole release is not claimed solved.

### Complete v7 objective and remaining error

Category aggregate is `0.75*median + 0.25*p90`, before category weighting.
Both complete runs have **620 sidecars, 629 analysis rows, zero failures**.
Overall **0.752349791399 → 0.715179553686**, **−4.94055%**.

| Category | Before aggregate | After aggregate | Change |
|---|---:|---:|---:|
| envelopes | 0.318553072 | 0.307959740 | −3.325% |
| amplitude | 1.107211725 | 1.047605645 | −5.383% |
| accent | 0.347337338 | 0.350187815 | +0.821% |
| gate | 0.510936207 | 0.467186265 | −8.563% |
| behavior | 1.744754881 | 1.551604681 | −11.070% |
| interaction | 1.636103088 | 1.557757110 | −4.789% |

No category worsens by >2%. Pitch, tuning, waveform, VCF and slide aggregates
are unchanged within 0.00008%. The largest increase is accent at **0.821%**.
The hardware repeatability floor remains **1.202458/1.407178** median/p90,
zero score weight, with no floor subtraction or modified metric.

Behavior RMS-trajectory median improves **2.220209 → 1.852800 dB**; band
trajectory median **2.839835 → 2.7232 dB**. Interaction RMS median improves
**1.872375 → 1.6851 dB**, band median **1.460665 → 1.3226 dB**. These are
output feature distances, not independently measured capacitor errors.
Remaining high-ENV/RES spectral mismatch, phase-dependent quiet gaps, accent
cross-loading, OTA/device transfer and uncertain hardware gate-off timing
limit further attribution. A more complete Q36 base-collector model and shared
analog-node integration need device/internal-node evidence; replacing the
remaining error with arbitrary timing or envelope factors is not justified.


### Final validation and reproduction

All ten workspace packages were tested offline with explicit `-p` selections:
**138 passed, zero failures, five manual benchmarks ignored**. ENV has
**20 passing tests**, including independent limiting RC bounds on C72's
forward-junction discharge at 48/96/192 kHz and the zero-output-headroom guard.
Existing real-Q38 partial-charge retrigger, Q31 junction residual, rate,
OTA/network, allocation-counter and extreme-input tests pass. Scoped ENV
all-target Clippy with `-D warnings`, rustfmt and diff whitespace checks pass.
The sample path adds no allocation, lock, solver or unbounded loop.

Exact replay audits cover **346 + 274 = 620 captures**, **zero VCF
non-convergence**, **zero sanitized inputs**, maximum **four Newton evaluations**.
The unchanged bounded Q31 solver is checked by its current-law/rate tests.
The final rebuilt renderer exactly reproduces the retained complete score and
every category. Requested related-category score is
**0.977592537128 → 0.915146421949**; it renormalizes category weights and is not the
full-session objective.

After replay workers finished, warm sequential Full Circuit measurements used
48-kHz host / 192-kHz circuit, a 2-second warmup and three 16-second holds per
setting. DSP wall time excludes WAV writing. Each hold reports zero solver
non-convergence/sanitization. These workloads satisfy the approximate 18% CPU
limit; they do not establish a hard scheduling bound.

| Workload | Median CPU / one core | Maximum |
|---|---:|---:|
| C1, default controls | 10.2% | 10.4% |
| C3, default controls | 9.8% | 9.8% |
| C3, CUTOFF/RES max, ENV MOD min | 10.6% | 10.6% |

Final renderer SHA-256:
`d81572a57ef0674301e087e0b1101aee2da105e04c504882479e2e4f20dffd3a`.
Unchanged harness analysis SHA-256:
`30ff76c5c5047125bebd78c932d8b493ba0583429a513612bc25d548a6cf2649`.
Full/related reports are `/tmp/env-i-full-score.json` and
`/tmp/env-i-score.json`; all-note evidence is
`/tmp/env-i-phrase-before-all.json` and `/tmp/env-i-phrase-after-all.json`.
Solver evidence is `/tmp/env-i-solver-session1.json` and
`/tmp/env-i-solver-session2.json`; CPU runs are `/tmp/env-i-cpu.json`.
Only small reports/logs were written under `/tmp`. All render WAVs used the
harness default cache; CPU WAVs and audit-driver files used one ENV scratch
directory. Both generated directories were deleted after validation.
Changes are left in the assigned worktree, without a commit or history change.

```sh
cargo build --offline --release -p dw303-render
python3 -B tools/analysis/harness/score.py \
  /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session1 \
  /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session2 \
  --categories behavior,interaction,amplitude,envelopes,accent,gate \
  --json /tmp/env-i-score.json
# Repeat without --categories for /tmp/env-i-full-score.json.
cargo test --offline -p dw303-osc -p dw303-vcf -p dw303-env \
  -p dw303-sequencer -p dw303-voice -p dw303-engine -p dw303-clap \
  -p dw303-standalone -p dw303-render -p dw303-capture
cargo clippy --offline -p dw303-env --all-targets -- -D warnings
cargo --offline fmt -p dw303-env -- --check
```


## High-ENV / high-RES tail fit (round 14, Harness v7)

The unmodified worktree rebuild reproduces round11.json exactly: renderer
`dcb5e3c5ad9f207ea12590fb044469f8b3377ee7a50e78e5059286afb816c5d8`,
overall **0.707084588011**, identical category distributions and zero failures.
Both read-only capture sessions contain 620 sidecars. The scoring code, capture
settings, VCF/oscillator and voice constants are unchanged by this round.
Only files within ENV and this stage document are edited.

### Ranked tail and physical drive bounds

The full interaction tail has more than one cause. Its highest round-11 rows
are `combo-c02-e00-r00-k36-phrase` **2.9460**, followed by
`combo-c02-e00-r08-k48-phrase` **2.5830** and
`combo-c02-e06-r00-k36-phrase` **2.5609**. A high-ENV-only fit cannot remove
those ordinary low-ENV phrase distances.

The worst maximum-ENV rows, ranked before any trial, are:

| Capture | Before error | Before signed absolute level, model − TD-3 |
|---|---:|---:|
| combo-c06-e10-r00-k36-phrase | 2.5220 | +1.168 dB |
| combo-c06-e10-r10-k36-phrase | 2.5153 | −2.605 dB |
| combo-c02-e10-r08-k36-phrase | 2.2325 | +0.105 dB |
| combo-c04-e10-r10-k36-phrase | 2.2183 | −2.231 dB |
| combo-c06-e10-r10-k48-phrase | 2.2003 | −1.875 dB |
| combo-c02-e10-r10-k36-phrase | 2.1558 | −1.182 dB |

An offline, 192-kHz circuit-order probe ran the actual stages for eight normal
62.5-ms gates at 125-ms intervals, keys 36/48, CUTOFF .2/.4/.6, ENV MOD 1,
DECAY .5 and RES 0/.8/1. It reconstructed the two input capacitor companions
at the OTA node without changing the renderer or scoring law:

- C62 peaks at **11.19517 V**. Under the existing 0.65-V junction estimates,
  Q41's collector is **0.80483 V above its base** and Q40 has still more
  headroom. Those pulses do not reach Darlington saturation. Adding a guessed
  rail clamp to suppress this tail is unsupported by this model's nodes.
- VR5's loaded wiper already uses its complete Thevenin resistance, R61/R63,
  R71 and R72. Its high-turn loading is not an omitted ideal pot divider.
- Q11 peak current spans **74.677–119.448 µA**, well below the estimated
  **530.227 µA** hard compliance limit. The fixed-Q12-bias reduction gives
  minimum Q11 Vce **1.104–1.202 V**. Changing R69 compliance alone cannot
  affect these unaccented combo rows.
- High-RES peak OTA differential input is **13.856–15.448 mV**. The existing
  ideal pair law's worst instantaneous gain ratio `tanh(z)/z` is about
  **0.971–0.977**, with `z=Vdiff/(2*VT)`. At zero RES the larger source output
  reaches **32.351 mV**, giving ratio **0.8875**. The model does not show a
  uniquely stronger high-RES input overload.

**VERIFY: these are model-derived bounds, not internal TD-3 measurements.**
Q12 bias motion, transistor Vbe/current gain/Early effect, Q39 Idss, actual
BA662A internals and reciprocal external loading remain uncertain. They may
move the bounds. The probe uses the existing ideal boundary between stages;
it does not resolve the missing shared analog nodes.

### Retained change and rejected trials

**R122 alone changes physically: printed 100 kΩ → fitted 95 kΩ.** This is the
assumed −5% resistor-tolerance endpoint. R121 remains 220 kΩ and both input
capacitors remain 10 nF. At high frequency the isolated resonance-tap divider
changes **0.0213178 → 0.0224147**; with `R124 || R121` held as the shunt, its
approximate coupling corner changes **155.762 → 163.776 Hz**. The implementation
still solves both loaded capacitor branches together; those single-branch
figures describe the direction of the fit rather than replace the network.

**VERIFY: tolerance attribution.** Neither the hardware resistor nor its
individual tolerance was measured. Audio constrains the combined tap gain,
source impedance, C22 corner and OTA transfer. This is a modest TD-3 output fit
within an assumed tolerance, not authentication of an original TB-303 resistor.
The circuit transcription retains 100 kΩ as its nominal authority.

The fixed diagnostic matrix contains 141 captures, including 68 interactions,
all 22 envelopes, all gates and patterns, twelve repeat takes and endpoint
accent references. It uses the unchanged v7 metrics. Its baseline weighted
selected-category score is **0.887430551**; its distribution deliberately
emphasizes maximum ENV/RES and is not the full-session objective.

| Trial | Diagnostic evidence and disposition |
|---|---|
| Q11 finite base current | Unprinted beta **400**, plus EXP_E's ΔV reducing R73/TM3 headroom. Current reduction uses `ratio = Is_ratio*exp(ΔV/VT)`, `I11 = I_cutoff*ratio*(1−ΔV/headroom)/(1+ratio/beta11)`. Interaction aggregate improves 1.58%, but envelopes worsen **6.75%** and accent **9.64%**. Reverted both reductions; no beta is fitted or retained. Q10's current-dependent Vbe and base feedback were still approximate. |
| R122 95 kΩ only | Diagnostic **0.875580679**, amplitude **−3.05%**, behavior **−1.00%**, interaction **−1.48%**; full check below passes the regression limit. Retained. |
| R122 95 kΩ + C22 10.5 nF | C22 printed 10 nF → +5% trial. Diagnostic **0.878125449**, weaker behavior/interaction improvement than R122 alone. Reverted capacitor. |
| R122 95 kΩ + C21 9.5 nF | C21 printed 10 nF → −5% trial. Diagnostic **0.878069204**, envelopes **+3.35%**. Reverted capacitor. |
| R122 95 kΩ + R121 231 kΩ | R121 printed 220 kΩ → +5% trial. Diagnostic **0.883086348**, envelopes **+7.92%** despite interaction −2.15%. Reverted R121. |

### OTA evaluation cost

The retained OTA remains `Iout = Iabc*mirror_ratio*tanh(Vdiff/(2*VT))`.
A bounded seventh Lambert continued-fraction convergent evaluates `tanh(z)`
for `|z| <= 0.5`; the prior tiny-tail cubic remains below `1e-4`, and libm
handles larger inputs. This is a numerical evaluation change, not a wider OTA
linear range or fitted circuit coefficient. The seventh/eighth convergents
bracket the exact real transfer within
`|z|^15 / (135135*2027025) < 1.12e-16`; including f64 rounding, the dense test
requires **<= 5e-16 absolute error** against libm. It checks 100,001 points,
odd symmetry, monotonicity, both branch boundaries and large-signal fallback.
There is no new state, allocation, lock, solver or unbounded iteration.

Q31's positive Lambert-W reduction additionally uses a three-term asymptotic
seed for `x >= 32`, followed by one Newton evaluation instead of two. The
low-current branch and at-most-four-step fallback are unchanged. A 100,001-point
independent junction-residual check covers x=32…1032 with error <=1e-10 in
`y+ln(y)=x`, and the old four-step solve checks the branch boundary. At nominal
VT this corresponds to <=2.59e-12 V junction residual. This saves one logarithm
on strong retriggered notes without changing any device parameter or current law.

### Consolidated remaining VERIFY register

All previous registers remain applicable. For the current source, the complete
set of retained fitted/unprinted component and device assumptions is:

- **Pot/trim calibration:** VR3/VR5/VR6 electrical track tables, TM3/expo-ratio
  identifiability, the voice's centre-ring/rest-voltage calibration, fitted R47
  and VR3 track, VR8's generic A law and VR4 gang direction correlation.
- **MEG/sweep tolerances:** C62 **1 → .95 µF**, R138 **68 → 71.4 kΩ**, VR6
  **1 → .8 MΩ**, C13 **1 → .8 µF**, R61 **10 → 9.5 kΩ**, D27/D28 effective
  **.60 → .45 V**. Current-dependent diode behavior, R117 clamp bias,
  Q39's individual Idss, Darlington gain/Vbe/source and sink impedance,
  base-current loading, saturation and IC12 supply/Ron remain unmeasured.
- **VEG/gate tolerances and devices:** R142 **10 → 10.5 kΩ**, R143
  **10 → 9.5 kΩ**, C72 **10 → 11 µF**, C42 **1 → 1.05 µF**; Q36 effective
  **.50 V / 1.90 mA**, Q32/Q36 beta **200**, Q31 beta **200** and reference
  **47 µA**. Fixed-beta storage, base-collector recombination, Q31/Q36
  current laws and preceding-sample gate/base loads remain reductions.
- **OTA/accent/output:** R119 **47 → 49.35 kΩ**, R120 **22 → 23.1 kΩ**, new
  R122 **100 → 95 kΩ**, C38/C59/C56 each **1 → .8 µF**; BA662A control
  voltage **.65 V**, mirror ratio **1**, zero input offset, ideal tanh
  differential pair and clipped unity output buffer. Pin currents, gain,
  feedthrough, output impedance/headroom, C36/D24 mutual accent loading,
  VCF/tap reciprocal loading and the VCA's full-VOLUME load reduction remain
  open. R121/C21/C22 are retained at their printed values.
- **Shared estimates:** silicon diode drop **.60 V**, generic junction drop
  **.65 V**, saturation drop **.20 V**, thermal voltage **25.9 mV**,
  expo saturation-current matching/calibration and Q11 compliance. Device
  assignments, temperature, Early effect, high-current gain and bias-rail
  motion are not established by OUTPUT captures.
- **Integration/measurement:** C54 gate-edge recovery and R150/C55 pulse
  reservoir loading, accent source cross-loading, physical panel placement,
  firmware/acoustic note-off timing, carrier-phase/quiet-gap sensitivity,
  recording volts/gain and session offsets. No new MIDI delay or release law
  follows from these trials. The transcription's duplicate D27/C22, optional
  R147, TP6/CPU-strobe ambiguity and headphone/open-rail items remain unresolved.

No rejected device or tolerance trial is an additional retained parameter.
The remaining large high-ENV/high-RES phrase level/spectral differences exceed
what this resistor fit explains. They remain coupled to ladder/output
large-signal response, phase, shared-node loading and unmeasured device laws.


### Final complete score and validation

Both complete v7 runs contain **620 sidecars, 629 analysis rows, zero failures**.
Overall **0.707084588011 → 0.699090882794 (−1.13052%)**. The final selected
ENV-related objective is **0.913624307618 → 0.905622896758**. The two numerical
fast paths preserve the R122-only category scores to floating-point rounding;
no circuit transfer was relaxed to gain CPU.

Category aggregate is `0.75*median + 0.25*p90`, before category weighting.

| Category | Before aggregate | After aggregate | Change |
|---|---:|---:|---:|
| envelopes | 0.313170596 | 0.312998222 | -0.0550% |
| amplitude | 1.048408111 | 1.028930141 | -1.8579% |
| accent | 0.351948404 | 0.352001126 | +0.0150% |
| gate | 0.467186265 | 0.467186265 | +0.0000% |
| behavior | 1.554874527 | 1.539373561 | -0.9969% |
| interaction | 1.542449255 | 1.529320231 | -0.8512% |

No category worsens by >2%. The largest increase is waveform **+0.2075%**;
VCF improves **3.7686%**. Interaction median/p90 moves
**1.297507/2.277275 → 1.282496/2.269793**. The maximum-ENV/high-RES subset
(ENV=1, RES>=.8; 24 captures) improves **1.386054/2.212932 →
1.358875/2.170133**. Amplitude p90 moves **1.687182 → 1.683274**;
behavior **1.523814/1.648055 → 1.504230/1.644803**. The take-to-take
floor remains **1.202458/1.407178**, with zero score weight and no subtraction.

All ten packages were tested with explicit offline `-p` selections:
**141 passed, zero failed, five manual benchmarks ignored**. ENV has **22
passing tests**, including the two new numerical/current-law checks and all
existing 48/96/192-kHz physical, extreme-input and lifecycle checks. ENV/VOICE
all-target Clippy with warnings denied, scoped rustfmt and diff whitespace
checks pass. The sample path has no added allocation or lock, and all iterations
remain bounded.

A final-binary replay diagnostic uses the renderer's exact sidecar parser,
event delivery and latency logic, with no WAV output. It audits **346+274=620
captures**, with **zero VCF non-convergence**, **zero sanitized inputs**, and
at most **four Newton evaluations**. This includes the retained fast paths;
Q31's residual has its separate stage tests. No scoring or renderer source
was edited to obtain the diagnostics.

CPU measurements use 48-kHz host / 192-kHz Full Circuit. The actual renderer
was run sequentially after replay workers completed, three 16-second MIDI holds
per setting; its reported DSP wall time excludes WAV writing. The focused
voice phrase measurements warm up for 2 seconds, then time 16 seconds, with
normal 62.5-ms gates and 125-ms trigger intervals. All have zero solver faults.

| Workload | Median CPU / one core | Maximum |
|---|---:|---:|
| Renderer, C1 default hold | 10.4% | 10.4% |
| Renderer, C3 default hold | 10.3% | 10.3% |
| Renderer, C3 CUTOFF/RES max, ENV min | 11.3% | 11.4% |
| Voice, C2 CUTOFF=.6 / ENV=1 / RES=1 phrase | 12.980% | 13.068% |
| Voice, C3 same phrase controls | 11.813% | 12.176% |

**CPU limitation:** renderer holds meet 13%, and focused phrase medians meet
13%, but one phrase measurement exceeds it by **0.068 percentage points**.
Earlier shared-machine voice phrase passes varied as high as **18.28%**;
those wall-time measurements do not establish a strict 13% worst-case ceiling.
A renderer timing pass overlapping eight replay-audit workers was discarded
and rerun after they finished. No CPU reduction is attributed solely to a
noisy before/after wall-time comparison. The two retained numerical changes
remove library evaluations with independently checked transfer accuracy;
filter/oscillator runtime changes remain outside this assigned scope.

Final renderer SHA-256:
`c4642bf3126ff6126fe353bb9b08f0ced05eb11ca42e14d89289bfdcff7ea5f8`.
Unchanged harness analysis SHA-256:
`30ff76c5c5047125bebd78c932d8b493ba0583429a513612bc25d548a6cf2649`.
Reports are `/tmp/env14-baseline.json`, `/tmp/env14-final-score.json` and
`/tmp/env14-full-score.json`. Small diagnostic logs are
`/tmp/env14-headroom.log`, `/tmp/env14-solver-final.log`,
`/tmp/env14-cpu-focus.log` and `/tmp/env14-render-cpu-final.log`.
All WAVs used the harness default cache or the single ENV scratch directory.
Both generated directories and Python bytecode created by this round are
removed before delivery. Changes remain in this worktree without a commit.

```sh
cargo build --offline --release -p dw303-render
python3 -B tools/analysis/harness/score.py \
  /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session1 \
  /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session2 \
  --categories interaction,amplitude,behavior,envelopes,accent,gate \
  --json /tmp/env14-final-score.json
# Repeat without --categories for /tmp/env14-full-score.json.
cargo test --offline -p dw303-osc -p dw303-vcf -p dw303-env \
  -p dw303-sequencer -p dw303-voice -p dw303-engine -p dw303-clap \
  -p dw303-standalone -p dw303-render -p dw303-capture
cargo clippy --offline -p dw303-env -p dw303-voice --all-targets -- -D warnings
cargo --offline fmt -p dw303-env -p dw303-voice -- --check
```
