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
| `diode_drop` | Generic Si Vf = 0.6 V; per-designator types, Is, ideality, leakage and current dependence are unprinted. Fixed clamp drops omit R117 bias dependence and the logarithmic near-floor tail. |
| `vbe` | Generic Q9/Q10/Q11/Q31/Q32/Q40/Q41 Vbe = 0.65 V; individual Is/beta/matching/temperature are unprinted. Q39's 2SK30(Y) type is printed, but individual Idss and buffer sink dynamics are omitted. |
| `saturation` | Q36/Q38/Q32 Vsat and buffer headroom = 0.2 V; output resistances and rail-dependent clipping need device validation. |
| `beta` | Q32/Q36 beta = 200; C41 reset stretch and C72 release need SPICE/device data. |
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
Each processed **1,536,000** VCF samples, maximum Newton count **3**,
**zero non-convergence** and zero sanitized inputs. These are isolated wall-clock
observations after scoring completed, not a hard scheduling guarantee.
Reproduce with `target/release/dw303-render /tmp/env4-cpu.wav --seconds 8 --rate 48000`.
