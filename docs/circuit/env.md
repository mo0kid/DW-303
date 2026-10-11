# Revision 2: trigger, MEG, accent, converter, VEG, OTA and output

Authority: [contract](contracts.md), [netlist](../../reference/schematic/netlist.md),
and the complete [ENV](../../reference/schematic/transcription/env.md),
[VCF](../../reference/schematic/transcription/vcf.md) and
[VCA](../../reference/schematic/transcription/vca.md) transcriptions.
Printed RC and pot values replace the provisional estimates. `Components`
comments identify their designators. Device estimates remain explicitly flagged.

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

**MEG:** C62 charges through D37/R152 (100 ohms), approximately 100 us,
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

**Sweep:** D24 feeds R46, then VR4B terminal 1; C13 is at terminal 3.
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

**Integrator calibration:** TM3 is **62,917.60490068002 ohms**, within its
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
C41 stretching the reset. C42 recharges toward 12 V through R123, tau 1.5 s.
Q31 supplies `gate ? max((12−Vsat−V_D35−Vbe−V_C42)/(R131+R133),0) : 0`
amperes. Peak is approximately 47 uA; the current reaches zero around 3.1 s.

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
| `expo_is_ratio` | Is(Q11)/Is(Q10) = 1; complementary-pair matching is not printed. |
| `ota_control_voltage` | BA662A pin 1 = 0.65 V; determines accent diode current and source headroom. |
| `ota_mirror_ratio` | BA662A current transfer factor = 1; gm, offsets, feedthrough and buffer characteristics are not printed. |
| `tm3` | 62.918K analytical trim; nonlinear assembled-VCF page-7 ring validation remains. |
| A-pot law | VR3/VR5/VR6/VR8 use `(81^rotation−1)/80`, 10% at half rotation. A marking is printed; exact taper is not. |
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
