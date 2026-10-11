# TB-303 page 5: pitch CV path and VCO (reader vcoB)

Source: p5-05.png (300 dpi), crops /tmp/vcoB-*.png; errata p-09.png; adjustments p-07.png; parts list p-08.png.

## Symbol conventions on this page
- **⊕ (a circle with a cross bar)** = **5.333V**. The boxed note on page 5 says: "5.333V ⊕ connects to pin 7 of IC16 (POWER SUPPLY)". IC16 is an AN6562 with TP5, and it is annotated 5.333V.
- **Filled arrow pointing up** = positive supply rail. It is unlabeled at each use in this block. The nearest labeled arrow, at IC16, reads "12V". In this file it is written **+V (arrow)**. It is assumed to be +12V, but that is not printed at these locations.
- **Hatched ground** = GND.
- A dot next to a resistor (R74–R90) marks it as one of the matched 0.1% parts.

## 1. Component table

| Ref | Value as printed | Type | Conf |
|---|---|---|---|
| IC9 | — | "4174" (hex D flip-flop; parts list HD14174BP) | ✓ |
| IC10 | — | "4050" (hex buffer HD14050BP) | ✓ |
| R74, R75, R76, R77, R78, R79, R80, R81, R82, R83, R84, R85, R87, R88, R89, R90 | "200K" (group note "(R74—R90) ↓ 200K") | resistor, matched pair, 0.1% selected | ✓ |
| R96 (as printed; probably R86) | 200K (group) | resistor | ? label reads "R96" on page 5 and on the p-09 erratum copy. The R74–R90 sequence has no other R86. |
| IC11a, IC11b | — | AN6562 dual op amp | ✓ |
| R107 | 2.2K | resistor | ✓ |
| D30, D31 | — | diode (type not printed) | ✓ |
| IC12 | — | 4066-type quad analog switch (parts list HD14066BP). Pins 8/9 ctl 6, 10/11 ctl 12 | ✓ |
| C35 | .22 (µF) | capacitor | ✓ |
| Q29, Q30 | — | NPN (emitter arrows point outward), type not printed | ✓ symbol |
| R91 | 1M | resistor | ✓ |
| R92 | 100K | resistor | ✓ |
| R93 | 100K | resistor | ✓ |
| R59 | 100K | resistor | ✓ |
| TM4 | "4 K (B)": a "4", a gap, then "K", then "(B)" | trimmer "TUNE" | ? the digit after 4 is missing. The p-08 trimmer list has 47kB / 220kB / 470kB / 4.7kB, so this is likely 47K(B) or 4.7K(B). |
| R102 | 100 | resistor | ✓ |
| R103 | 220K | resistor | ✓ |
| Q26 | 2SC1583 | dual NPN matched pair (p-09: 2SC1583(F) Dual Tr) | ✓ |
| R104 | 2.2K | resistor | ✓ |
| C34 | .001 | capacitor | ✓ |
| TM5 | 4.7K | trimmer "WIDTH", wired as a variable resistor (wiper tied to one end) | ✓ |
| R106 | 27K | resistor (matched-dot marked) | ✓ |
| R118 | 100K | resistor | ✓ ("Q100K": the Q belongs to the Q25 label) |
| R100 | 560 | posistor (PTC); note "POSISTOR: R100, MFR: ■" | ✓ |
| Q25 | — | NPN (emitter arrow outward), type not printed | ? arrow small |
| Q24 | — | NPN, diode-connected, type not printed | ✓ |
| Q27 | — | PNP (emitter arrow inward), type not printed | ✓ |
| D25 | — | diode | ✓ |
| R101 | 10K | resistor | ✓ |
| C33 | .01 | capacitor | ✓ |
| Q28 | 2SK30(O) | N-JFET source follower | ✓ |
| R105 | 10K | resistor | ✓ |
| C32 | 10/16 | electrolytic (rail decoupling) | ✓ |
| C31 | 10/16 | electrolytic (5.333V decoupling) | ✓ |
| C10 | .01 | capacitor | ✓ |
| R34 | 10K | resistor | ✓ |
| R35 | 100K | resistor | ? the "100K" sits above R35 under the C33 ground; no other value is nearby |
| Q8 | — | PNP (emitter arrow inward), type not printed | ✓ |
| R45 | 22K | resistor | ✓ |
| C11 | 1/50 | electrolytic 1µF/50V | ✓ |
| R36 | 10K | resistor | ✓ |
| S1 | — | slide switch "WAVEFORM" | ✓ |
| VR2 | 50K(B) | pot "TUNING", pins 1/2/3 | ✓ |
| J3 | — | jack "CV" | ✓ |

## 2. Netlist

### DAC (IC9 → IC10 → R-2R ladder)
- IC9 pins 1 and 16 → 5.333V. Pin 8 → GND. Pin 9 → net **SLIDE** (dot junction). That same vertical net continues downward off-block, and the "SLIDE ON/OFF" line branches right from it to R59.
- IC9 inputs (pins 4, 13, 3, 14, 6, 11) come from the left, outside this block.
- **IC9 pin 10** → IC10 inputs 5, 7, 9, 11 (four buffers in parallel). Outputs 4, 6, 10, 12 → R90 → **DAC_OUT**.
- **IC9 pin 7** → IC10 (two buffers in parallel) → R87 → node N2.
  - **Errata p-09:** this buffer's pins were corrected to inputs **3, 14** and outputs **2, 15**. Page 5 originally printed inputs 2, 15 and outputs 3, 14.
- IC9 pin 15 → R79 → N3.
- IC9 pin 2 → R80 → N4.
- IC9 pin 12 → R81 → N5.
- IC9 pin 5 → R82 → N6.
- Rungs (each is two 200K in parallel):
  - DAC_OUT–N2: R88 ∥ R89
  - N2–N3: R85 ∥ R96(R86?)
  - N3–N4: R83 ∥ R84
  - N4–N5: R76 ∥ R77
  - N5–N6: R74 ∥ R75
- Termination: N6 → R78 → **5.333V** (⊕).

### CV buffer and slide
- **IC11b**:
  - pin 5 (+) = DAC_OUT, also IC12 pin 8.
  - pin 6 (−) → R107 2.2K → pin 7.
  - pin 7 = **CV** (marked "(TP4)").
- **IC12**:
  - Switch A: pin 8 = DAC_OUT, pin 9 = SLIDE_CAP, control pin 6 = SLIDE_CTL_A.
  - Switch B: pin 10 = SLIDE_CAP, pin 11 = CV (IC11b pin 7), control pin 12 = SLIDE_CTL_B.
- C35 .22: SLIDE_CAP ↔ GND.
- **Q29** (NPN):
  - B → R59 100K → SLIDE (the SLIDE ON/OFF line).
  - E → GND.
  - C = SLIDE_CTL_B: goes to IC12 pin 12, R92 100K → +V (arrow), and R91.
- **Q30** (NPN):
  - B → R91 1M → SLIDE_CTL_B.
  - E → GND (it ties to Q29's emitter/GND node).
  - C = SLIDE_CTL_A: goes to IC12 pin 6 and R93 100K → +V (arrow).
- **D31**: anode = CV, cathode → ⊕ (5.333V). The TP4 label is printed beside this symbol.
- **D30**: anode = GND, cathode = CV.
- CV runs right along the top bus to a node above TM5. From there it continues right (about x≈2545 at 300 dpi), down to the "CV" bus line, and right to the **J3 "CV"** jack tip.
  - No series parts are seen on that run.
  - J3 sleeve → GND.
  - (IC16 is the 5.333V supply regulator with TP5, not part of the CV OUT path.)

### Exponential converter
- **IC11a**:
  - pin 3 (+) → 5.333V.
  - pin 2 (−) = **EXP_REF**.
  - pin 1 = **EXP_DRV**.
- EXP_REF connects to:
  - R103 220K → +V (arrow)
  - R102 100 → TM4 wiper
  - Q26 left collector
  - C34 .001 → EXP_DRV
- TM4: one end → 5.333V, other end → +V (arrow), wiper → R102.
- R104 2.2K: EXP_DRV ↔ Q26 common emitters.
- **Q26 left half**: B → 5.333V, C = EXP_REF, E = common emitter.
- **Q26 right half**: B = **EXP_IN**, C = **SAW_CAP**, E = common emitter.
- EXP_IN connects to:
  - R106 27K → TM5 lower end. TM5 4.7K (wiper strapped to its lower end) → CV.
  - R118 100K → **TUNE** line.
  - R100 560 posistor → 5.333V.
- TUNE line runs right as "TUNE" to **VR2** pin 2 (wiper). VR2 pin 1 → GND. VR2 pin 3 → a short stub with a dot ("-•-"); its destination is unclear (see uncertainties).

### Sawtooth core and reset
- C33 .01: SAW_CAP ↔ GND.
- **Q25** (NPN): E = SAW_CAP, B = RST_B, C → Q27 base.
- **Q27** (PNP): E → +V (arrow), B = Q25 collector, C → D25 anode.
- **D25**: cathode = RST_B.
- **Q24** (NPN): B and C both = RST_B. E → R101 10K → 5.333V.

### Saw output
- **Q28** 2SK30(O): G = SAW_CAP, D → +V (arrow), S = **VCO_SAW**.
- R105 10K: VCO_SAW → GND.
- VCO_SAW → C10 top, R35 right end, and the saw line → S1 saw contact (upper).
- Supply decoupling next to the block:
  - C32 10/16: + → +V (arrow), − → GND.
  - C31 10/16: + → ⊕ 5.333V, − → GND.

### Square shaper
- C10 .01: VCO_SAW ↔ node X.
- R34 10K: X ↔ Q8 base.
- R35 100K: VCO_SAW ↔ Q8 base. R35 is parallel to the C10+R34 series path.
- **Q8** (PNP):
  - B as above.
  - E → R45 22K → +V (arrow). E also → C11 1/50 (+ on the emitter side) → GND.
  - C = **VCO_SQR**.
- R36 10K: VCO_SQR → 5.333V.
- VCO_SQR → S1 square contact (lower).

### Waveform switch and VCF input
- **S1 "WAVEFORM"**: upper contact = VCO_SAW, middle (common) = **VCF_IN** ("saw/square" label), lower contact = VCO_SQR. As drawn, the slider bridges the upper and middle contacts (the saw position).
- VCF_IN runs back left along the bus, then down at about x≈2665 into the VCF block. There it reaches **R62 220K** (top), then C17 1/50 (+ on the R62 side), then the Q12 left base and R70. Those parts belong to the VCF reader.

## 3. Annotations
- Page 5:
  - "TUNE: 110Hz (SET TUNING CV OUT: 2.75V (A KEY))"
  - "WIDTH: 1 OCT./VOLT"
  - "CV KEY C=1.000V, C̄=4.000V" (high C)
- Saw waveform at Q28 S: falling ramp with a vertical reset, from 12V down to 5.5V (scale: GND, 5.5V, 12V).
- Square at Q8 C: 5V low, 8V high, period "(9ms KEY A)".
- "(R74—R90) 200K (MATCHED PAIR .1% SELECTED MFR: R74-R90)". "POSISTOR: R100 MFR: ■".
- "(TP4)" arrow at IC11b pin 7. TP4 is also printed next to the ⊕ at the D31 cathode.
- p-07 VCO adjustment:
  - Check point is the Q28 source or the S1 WAVEFORM terminal.
  - Set TUNE (VR2) to its middle position.
  - In PITCH mode, tap CL and CH alternately and adjust TM5 WIDTH for 2:1 waveforms.
  - Press the A key and adjust TM4 for 110Hz.
  - Confirm 4:1 ±0.5% with CL and CH + TRANSPOSE UP.
- p-08 parts:
  - TUNING pot K161B0-50KB.
  - Trimmers 47kB / 220kB / 470kB / 4.7kB.
  - 2SK30TM(Y)/(O). AN6562 dual op amp. HD14066BP. HD14174BP. HD14050BP.
- p-09 errata:
  - IC10 second buffer pins corrected to in 3/14, out 2/15.
  - 2SC1583(F) is a dual transistor.

## 4. Functional description (from this transcription only)
- **DAC:** IC9 latches 6 bits. Its two MSBs drive the ladder through paralleled 4050 buffers. The 200K/100K R-2R ladder, terminated to 5.333V, gives DAC_OUT. The supply is 5.333V, so one LSB is about 5.333/64 ≈ 1/12 V, consistent with "C=1.000V, C̄=4.000V".
- **CV buffer:** IC11b buffers DAC_OUT into CV. CV is clamped by D30 (to GND) and D31 (to 5.333V), appears at TP4, and goes to the J3 CV jack.
- **Slide:** Q29/Q30 drive IC12's two control pins in complementary fashion. With SLIDE low, Q29 is off and Q30 is on, so switch B is on and switch A is off: C35 sits on CV (pin 11). With SLIDE high, the states reverse: switch A connects C35 to DAC_OUT, so it no longer sits on CV. This exact topology looks odd; see uncertainties.
- **Exponential converter:** CV goes through TM5 (scale, "1 oct/volt") and R106 27K. Together with TUNE (VR2 via R118 100K), it is summed at the Q26 right base against R100 (PTC) to 5.333V. IC11a holds the left collector at 5.333V, with reference current from R103 and TM4/R102. The right collector draws an exponential current from C33.
- **Saw core:** C33 ramps down from about 12V to about 5.5V. Reset is a regenerative Q25/Q27/D25/Q24 loop that recharges C33.
- **Outputs:** Q28 buffers the saw (VCO_SAW). Q8 (PNP) with the C10/R34/R35 network makes a square of about 5–8V across R36. S1 selects saw or square as VCF_IN, which then goes to R62 in the VCF.

## 5. Uncertainties
1. **R96 vs R86:** printed "R96" in both page 5 and the erratum, but it falls inside the R74–R90 sequence.
2. **TM4 value:** printed "4 K (B)". The missing digit could make it 47K(B) or 4.7K(B).
3. **IC12 topology:** switch A runs DAC_OUT → C35, switch B runs C35 → IC11b output (pin 11 is joined to pin 7 with a dot). So C35 is never placed on IC11b's input, which seems functionally odd. This is transcribed as drawn and should be verified.
4. **SLIDE net:** IC9 pin 9 is drawn joined (dot) to the SLIDE ON/OFF net.
5. **Supply arrows** (R92, R93, R103, R45, TM4 end, Q27 E, Q28 D, C32 +): the rail is not labeled; assumed +12V.
6. **VR2 pin 3:** goes to a short "-•-" stub, not a clear ⊕. Its destination (5.333V?) is unconfirmed.
7. **⊕ at the D31 cathode:** it may be the TP4 pad symbol rather than 5.333V. It is the same glyph as 5.333V.
8. **Q25 polarity:** read as NPN from a small arrow; moderate confidence.
9. **R35 value:** the "100K" placement is slightly ambiguous.
10. **Device types:** Q8, Q24, Q25, Q27, Q29, Q30, D25, D30 and D31 have no type printed.

## Memory hints (unverified)
- Standard 40174 pinout: Q outputs 2, 5, 7, 10, 12, 15; D inputs 3, 4, 6, 11, 13, 14; pin 9 = CLK, pin 1 = CLR. That would make IC9 pin 9 a clock input fed by the SLIDE net, which is odd.
- Standard 4050: in 3→out 2, 5→4, 7→6, 9→10, 11→12, 14→15. This matches the p-09 correction.
