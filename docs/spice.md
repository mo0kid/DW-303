# DW-303 SPICE reference

These decks follow `reference/schematic/netlist.md` and its reconciled VCO,
VCF, ENV and VCA transcriptions. The files are component-level nominal-circuit
experiments with explicitly generic semiconductors. They are reference data for
**these models**, not validated original-unit measurements. A passing simulation
alone does not establish the pitch/filter/envelope error budgets.

Run from any directory:

```sh
python3 reference/spice/run.py
# Or select another installed ngspice:
python3 reference/spice/run.py --ngspice /path/to/ngspice
```

Python 3 uses only the standard library. Every invocation executes `ngspice -b`
in isolated temporary directories, times out after 120 seconds per deck, checks
logs for errors (ngspice can return zero after a failed analysis), checks exported
vectors for finite values and matching column counts, and writes the data under
`reference/golden/`. No Rust crate is assigned to this work; Cargo checks do not
apply. The runner does not alter manifests or dependency locks.

The four primary decks also run directly with `ngspice -b vco.cir` (and likewise
`vcf.cir`, `env.cir`, `vca.cir`) from `reference/spice/`; they export `data.txt` and,
for AC decks, `ac.txt` in the current directory. The runner leaves the checked-in
decks unchanged and overrides their nominal `.param` values in temporary copies.
The supplemental `square-transfer.cir` is a standalone DC experiment.

The simulation uses ngspice's Gummel–Poon BJT and level-1 JFET models, not a
hand-coded envelope or four independent ladder poles. Behavioral sources are
limited to ideal op-amps, the imposed Q11 tail-current test input, control-current
measurement and the documented OTA substitute. The output mixer uses Q34/Q33 and
R153 feedback. ngspice syntax and vector-export semantics follow the
[official ngspice manual](https://ngspice.sourceforge.io/docs/ngspice-manual.pdf).

## Experiments and integration boundaries

| Deck | Physical state and measurements | Isolation / stimulus |
|---|---|---|
| `vco.cir` | C35 slide voltage, Q26 expo pair, C33 ramp, Q25/Q27 regenerative reset, Q28 JFET source follower, Q8/R34/R35/C10 square shaper | Thevenin DAC source, 100 kΩ; IC11 ideal servo; centre tuning; TM4 measured at A=2.75 V |
| `vcf.cir` | Both ladder columns and cross-rung C26/C24/C19/C18; Q22 top pair; crossed C25/C27 into Q21/Q20; Q19/C14; VR4A/C15/Q18/C23 feedback | Q11 current imposed at R69; VCA input load included; 1 V AC, tiny transient pulse to seed ringing |
| `env.cir` | Q38/C54 one-shot, D37/R152/C62 charge, R138/VR6 discharge, three-diode clamp, Q41/Q40/Q39 buffer; IC12 accent switches; VR7; D24/R46/VR4B/C13/R72; loaded Q9/Q10/Q11 converter | Gate-inverter falling edge and accent latch level supplied externally; cutoff current measured with Q11 collector held at 12 V |
| `vca.cir` | C41/Q32/C42 reset and recharge; Q31 current; R119/C36/R120/D27 accent; BA662A substitute; both C21/R121 and C22/R122 inputs; C38/VR8; transistor mixer and line output | 11.8 V held gate; 1.4 ms trigger; constant extra OTA current for AC sweeps; separate VCF/tap/MIX IN stimuli |

VCO goldens cover CV=1, 2, 2.75 and 4 V, a 2→3 V slide at 100 ms, and
static square transfer. TM4 is bisected for 110 Hz; TM5 remains the transcribed
hand estimate of 3.6 kΩ. Consequently octave error is measured, not calibrated
away. Slide-off switch dynamics and the full bit-by-bit DAC are not simulated:
the exact transcribed 100 kΩ Thevenin resistance and requested DAC voltage are
used at the input boundary.

VCF AC goldens cover Q11 currents 2, 10, 40 and 160 µA at RES=0, 0.5 and 1,
0.1 Hz–100 kHz, 80 points/decade. The transient runs cover 600 ms after a
1 mV, 1 ms pulse. The summary marks oscillation only if late output RMS exceeds
100 µV and remains at least half the early RMS. This deterministic perturbation
avoids an unphysical persistent input or noise used to disguise decaying ringing.
A `null` oscillation frequency means no sustained oscillation was observed.

The page-7 check sweeps TM3=0..470 kΩ with CUTOFF at centre, RES max and
ENV MOD/ACCENT min. The reported AC resonant peak is an **indicator**, not a
replacement for measuring the 2 ms transient ring period on a working calibrated
unit. TP6's uncertain attachment and inconsistent 5.2/5.8 V sketch are not used
as voltage targets.

MEG min/mid/max and accented counterparts are single-trigger 4 s records. Accent
trains contain exactly 1..8 accented sixteenths at 120 BPM (125 ms step, 62.5 ms
gate), at RES min/mid/max. Accent stays latched for the whole step. Both the C13
voltage and gang-B wiper voltage are saved: the latter contains more direct
accent at low RES and more stored charge at high RES. The converter CSV sweeps
CUTOFF=0/.25/.5/.75/1 and ENV MOD=0/.5/1 at explicit ENV levels 0.3/9.9 V.

VCA AC sweeps use 1, 10, 47 and 100 µA independent OTA control current with
separate VCF output, resonance wiper and MIX IN sources. The VEG transient uses
no added control current. The AC operating point has C42 recharged, so its Q31
current is essentially zero: the independent current sets OTA gain. The mixer
is measured at full volume. No headphone amplifier or PSU regulator model is
included; neither affects the binding line-output contract.

## Data and measurement conventions

CSV headers use circuit vector names; `time` is seconds and `frequency` hertz.
Voltages are node volts, currents amperes, `magnitude_db`/`gain_*` amplitude dB,
and `phase_*` wrapped radians. AC vectors retain the ngspice logarithmic grid.
Transient CSVs are linearly resampled at 20 µs (VCO/VCF) or 250 µs (ENV/VCA) to
keep reference size bounded. Measurements use the original adaptive-step data;
CSV sampling cannot resolve sub-microsecond reset shape. Square transfer is DC,
not an AC-coupled edge measurement.

`summary.json` records scalar measurements, input parameters, deck hashes,
include hashes and simulator version. Each saved run has a diagnostic log.
VCO frequency uses rising C33 threshold crossings after 100 ms; this works even
though the audible saw falls. RMS is time-weighted. MEG tau fits assume a 1.5 V
asymptote over 0.2..1 nominal RC constants after the peak; that is an explicitly
approximate comparison because diode forward drops vary with current. VEG fits
the residual to the 12 V recharge rail over 0.1..2 s. All scalars are deterministic
for a given simulator/model version, temperature and solver settings.

## VERIFY register

All corresponding uncertain assumptions have `* VERIFY:` comments in decks or
includes (SPICE comments serve as circuit documentation).

1. `models.inc`: generic NPN `Is=1e-14`, β=200, Early voltage 100 V; PNP
   `Is=1e-14`, β=150, Early voltage 80 V. Junction capacitances, high-current
   rolloff and transit times are estimates. Same model for all matched-pair
   devices enforces ideal matching; actual 2SC1583/2SC2291 characteristics and
   per-designator types for all unspecified Q parts remain unverified.
2. `models.inc`: Si diode `Is=2 nA`, ideality 1.7, series resistance 1 Ω and
   capacitance/transit time are substitutes, not fitted 1S2473/1SS133 data.
   Unspecified diode assignments and duplicate D27/C22 names remain qualified
   by block.
3. `models.inc`: 2SK30(Y/O) level-1 models assume Idss=2/1 mA, Vto=−1.5 V,
   lambda=.01. Actual rank curves and the follower's gate/output offset are
   unknown.
4. `models.inc`: 4066 assumes 12 V supply, 200 Ω on resistance, ideal controls
   and no charge injection. The schematic does not draw its supply.
5. `models.inc`: AN6562 has ideal high-gain, rail-clipped behavior; no measured
   offset, finite bandwidth, slew rate or recovery.
6. `models.inc`: BA662A assumes LM13700-style
   `Iout=Iabc*tanh(Vdiff/(2VT))`, VT=25.865 mV, pin-1 voltage 1.2 V and ideal
   unity clipped buffer. Actual gm, bleed, DC pin voltage, offset and internal
   output loading are unknown; this is the largest VCA model uncertainty.
7. `vco.cir`: R102 uses electrically plausible 100 kΩ; the scan prints bare
   `100`, and the reconciled transcription explicitly leaves the unit open.
8. `vco.cir`: tuning pin 3 assumed +5.333 V, with a stiff centre wiper source.
   VR2 loading and the unidentified rail are unresolved; ±700-cent end tuning
   is not validated here. TM4 is 47 kΩ as the transcription's inferred reading;
   Q25 NPN follows its topology-based interpretation.
9. `vco.cir`: 10 pF parasitic capacitances at reset node and Q27 base estimate
   wiring capacitance; used with Gear integration to make regenerative switching
   converge. They must be checked before treating reset spectra as authoritative.
10. `control-stage.inc` / `vca.cir`: A/audio pot rotation maps to electrical
    fraction `(100^rotation-1)/99` (about 9.1% at centre). Actual Roland taper
    curves, including VR6, are unknown. B pots are linear.
11. `control-stage.inc`: RES gangs move toward terminal 3 together. This is
    assumed, not proven by the drawing; reversing gang B reverses the sweep mix.
12. Q11 collector is held at 12 V for isolated control-current sweeps. Its actual
    compliance depends on R69/Q12 and saturation above approximately 0.55 mA;
    high converter currents in the CSV are available demand, not guaranteed
    deliverable ladder current. ENV rest/peak test voltages are hand-estimate
    probes; loaded MEG traces are saved separately.
13. Gate/latch stimuli use independent ideal logic. The schematic's shared
    slide/latch-strobe line, R147 fitting ambiguity and CPU protocol are outside
    these analog test boundaries. Full gate Q36/C72 edge shaping is not fitted.

## Known limitations

Do not use these datasets to silently retune the stage crates. Device uncertainty
can cause much larger errors than the numerical solver. In particular, a generic
model failing to self-oscillate is a result to investigate, not justification for
inserting extra feedback gain. The converter's available current is measured
separately from ladder compliance. The static OTA macro does not supply the
BA662A's control bleed or distortion from its actual internal buffer. Physical
capacitor tolerances, PTC temperature tracking, hardware noise, leakage and
per-unit mismatch are absent. Further validation should use manufacturer device
curves and the hardware comparison protocol.

## Measured results and disagreements (ngspice 47, 27 °C)

| Quantity | Simulation | Transcription hand estimate / target |
|---|---:|---:|
| A at 2.75 V, TM4=0.62835693 | 110.0007 Hz | 110 Hz (calibrated) |
| C1 / C2 / C4, TM5=3.6 kΩ | 33.1267 / 65.6999 / 260.0516 Hz | 32.7032 / 65.4064 / 261.6256 Hz |
| Pitch error C1 / C2 / C4 | +22.28 / +7.75 / −10.45 cents | ±1 cent budget; not met |
| Slide C35 time constant | 21.9997 ms | 22 ms |
| A saw Q28 source min/max | 5.259 / 10.748 V | annotated 5.5 / 12 V; not met |
| A square Q8 collector min/max | 5.333 / 8.619 V | annotated about 5 / 8 V |
| Q38 trigger above 6 V | 1.34 ms | about 1.4 ms |
| MEG C62 peak / ENV peak, DECAY max | 11.354 / 10.138 V | about 11.2 / 9.9 V |
| MEG fitted tau min/mid/max | 67.35 / 154.89 / 999.40 ms | 68 / 158.91 / 1068 ms with assumed taper |
| Accented MEG fitted tau, all DECAY settings | 67.34–67.54 ms | 68 ms |
| MEG C62 / ENV after 4 s, DECAY min | 0.737 / 0.0208 V | fixed-floor simplification 1.5–1.8 / 0.2–0.5 V; not met |
| VEG peak control current | 44.77 µA | about 47 µA |
| VEG recharge fitted tau | 1.602 s | R123 C42 = 1.500 s; +6.8% |
| VEG control current at 3.1 s, held gate | 3.20 µA | about zero by 3.1–3.2 s; not met |
| VCA input attenuations at 1 kHz (VCF / tap) | 0.009670 / 0.021065 | 0.00969 / 0.0213 (asymptotic estimates) |
| OTA voltage gain at 47 µA, before input attenuation | 47.720 | gm × 52.5 kΩ ≈ 47.7 (macro assumption) |
| Mixer VCA path, 1 kHz, full volume | −1.926 | −2.128; magnitude about 9.5% low |
| MIX IN to line, 1 kHz | −0.896 | −1; magnitude about 10.4% low, includes line loading |

The A tune is the only fitted pitch datum. TM5 is deliberately kept at the hand
estimate so octave errors remain visible; an authentic page-7 WIDTH adjustment
would be the next calibration experiment. The generic JFET/reset models do not
reproduce the annotated 12 V saw peak. MEG fitted tau differs from raw RC because
the fit uses a fixed floor while the physical diode clamp and buffer loading do
not. The max-decay error exceeds the 2% envelope comparison budget. These
are recorded disagreements, not grounds to rewrite schematic nominal values.

At RES max, all four tested currents yield decaying responses with late RMS
below 1 nV; **no sustained self-oscillation or threshold was observed**. Do not
interpret `null` threshold as proof that a real TB-303 cannot self-oscillate.
The complete transistor ladder is loaded by the output differential pair; its
transfer cannot be equated to four independent RC poles from the hand formula.
For RES=0, the measured high-side −3 dB frequencies relative to the AC maximum
are approximately 59.6 / 172.8 / 421.7 / 1258.9 Hz at 2 / 10 / 40 / 160 µA.
For comparison, the hand **individual 33 nF pole** estimates are 93.2 / 466.2 /
1864.6 / 7458.5 Hz; these are different observables, and their difference is not
a quantified filter-response error budget. The unexpectedly low loaded response
and insufficient resonant feedback require investigation against transistor data
and the original schematic before using this deck as a production VCF oracle.

The page-7 TM3 sweep produces Q11 demand of 12.56→2.45 µA as TM3 rises from
0 to 470 kΩ, with RES-max AC peaks 37.6→19.4 Hz. None lies in the target
400–666.7 Hz band (2 ms ±0.5 ms period). It therefore **fails the calibration
check** under these generic models and the assumed audio taper. No frequency or
feedback gain was inserted to force a 500 Hz result.

C13 accumulates in all three RES positions. At the end of the last accented
step, its voltage rises from 3.934→4.261 V (RES min), 3.855→4.065 V (mid), and
3.787→3.901 V (max), comparing one with eight accents. The corresponding final
step wiper peaks are 5.919→6.324, 4.688→5.189 and 4.351→4.665 V. The loaded
charge network therefore confirms accumulation qualitatively; it is not a
single unloaded 97 ms exponential. Static converter demand spans approximately
0.992–303.3 µA over the saved CUTOFF/ENV MOD matrix.

Validation: 66 saved runs produced 97 CSVs successfully; the runner's log/vector
checks and all eight Python regression tests passed. `python3 -m unittest discover -s reference/spice -p 'test_*.py'` checks
measurement functions, finite monotonic CSVs, the native DC sweep, model hashes,
A/slide calibration, accent bypass/accumulation and OTA current sign/gain scaling.
These checks verify the artifacts and simulation plumbing; hardware accuracy
remains limited as described above.
