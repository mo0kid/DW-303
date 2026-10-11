# TB-303 page 5: pitch CV path and VCO (reader vcoA)

Source: p5-05.png (300 dpi). Approximate full-res coordinates are given as (x,y) for re-checking.
Block area: x≈720–2700, y≈280–1500, plus the right-edge S1/VR2/J3 at x≈4100–4400, y≈950–1650.

## Symbol conventions used on this page (as observed)
- **Circle with horizontal bar through it** = the 5.333 V reference. The 5.333 V source is identified by the note box
  "5.333V [symbol] connects to pin 7 of IC16 (POWER SUPPLY)" (≈1400,1420), and IC16 pin 7 is shown at the same
  symbol labelled "5.333V" and "(TP5)" (≈2900,700). Net name used here: **+5.333V**.
- **Solid filled arrow pointing up** = a supply rail. It is unlabelled in the VCO, but the same arrow is labelled
  "12V" at IC16 (≈2665,680). Net name used here: **+12V(arrow)**. (See uncertainty U1.)
- Hatched ground symbol = **GND**.
- A small filled ■ beside a resistor = "MFR" (metal film resistor). The legend says "POSISTOR: R100, MFR: ■".

## 1. Component table

| Designator | Value as printed | Type | Conf. |
|---|---|---|---|
| IC9 | 4174 | Hex D flip-flop (HD14174BP, from the parts list). Pins 1 and 16 go to 5.333V, pin 8 to GND | ✓ |
| IC10 | 4050 | Hex buffer (HD14050BP). Two symbols are used | ✓ |
| R74, R75, R76, R77, R78, R79, R80, R81, R82, R83, R84, R85, R87, R88, R89, R90 | 200K ("(R74—R90) ↓ 200K") | MFR ■, ".1% SELECTED", "MATCHED PAIR" | ✓ |
| "R96" (the ladder resistor to the right of R85) | 200K (it falls inside the R74–R90 group) | MFR ■ | ? The label reads "R96", most likely a misprint for R86 (see U2) |
| IC11a / IC11b | AN6562 | Dual op-amp | ✓ |
| R107 | 2.2K | resistor | ✓ |
| D30, D31 | (no type printed) | diode | ✓ (type ✗) |
| IC12 | (no type printed beside it; the 4066 family is HD14066BP in the parts list) | Analog switch. Sections 8/9 (control pin 6) and 10/11 (control pin 12) are used | ✓ |
| C35 | .22 | capacitor, 0.22 µF | ✓ |
| Q29, Q30 | (no type printed) | NPN | ✓ polarity |
| R91 | 1M | resistor | ✓ |
| R92 | 100K | resistor | ✓ |
| R93 | 100K | resistor | ✓ |
| R59 | 100K | resistor | ✓ |
| TM4 "TUNE" | "4 K (B)". A gap after the "4" probably held a "7", so likely 47K(B) (see U3) | trimmer | ? |
| R102 | 100 | resistor. The unit is not printed: 100 Ω, or 100K if the nearby "K" belongs to it (U3) | ? |
| R103 | 220K | resistor | ✓ |
| TM5 "WIDTH" | 4.7K | trimmer, wired as a rheostat (the wiper is tied to the lower end) | ✓ |
| R106 | 27K | resistor, MFR ■ | ✓ |
| R118 | 100K | resistor | ✓ |
| Q26 | 2SC1583 | dual NPN, drawn in one envelope (a matched pair). The parts list gives 2SC1583(F) "Dual Tr" | ✓ |
| R104 | 2.2K | resistor | ✓ |
| C34 | .001 | capacitor, 1 nF | ✓ |
| R100 | 560 | posistor (PTC), drawn as a resistor inside a rounded box | ✓ |
| Q25 | (no type printed) | transistor. Polarity is unclear (U4) | ? |
| Q24 | (no type printed) | NPN, diode-connected (base tied to collector) | ✓ |
| Q27 | (no type printed) | PNP | ✓ |
| D25 | (no type printed) | diode | ✓ |
| R101 | 10K | resistor | ✓ |
| C33 | .01 | capacitor, 10 nF (the timing capacitor) | ✓ |
| Q28 | 2SK30(O) | N-JFET. The parts list gives 2SK30TM(O) | ✓ |
| R105 | 10K | resistor | ✓ |
| C31 | 10/16 | electrolytic, 10 µF/16 V | ✓ |
| C32 | 10/16 | electrolytic, 10 µF/16 V | ✓ |
| Q8 | (no type printed) | PNP | ✓ |
| R34 | 10K | resistor | ✓ |
| R35 | 100K | resistor | ✓ |
| R36 | 10K | resistor | ✓ |
| R45 | 22K | resistor | ✓ |
| C10 | .01 | capacitor, 10 nF | ✓ |
| C11 | 1/50 | electrolytic, 1 µF/50 V | ✓ |
| S1 "WAVEFORM" | — | 2-position slide switch with 3 terminals | ✓ |
| VR2 "TUNING" | 50K(B) | panel potentiometer, pins 1/2/3 | ✓ |
| J3 "CV" | — | jack | ✓ |

## 2. Netlist

### DAC: IC9 latch, IC10 buffers, R-2R ladder
- IC9 (4174): pins 1 and 16 go to +5.333V (the circled reference symbol labelled "5.333V"). Pin 8 goes to GND.
  - The inputs on the left side (pins 4, 13, 3, 14, 6, 11) come from the CPU bus, which is outside this block.
  - Pin 9 (CLK) runs down to IC13 pin 11 (marked "T", on the 14013). A junction dot joins this same vertical line to the horizontal line labelled **"SLIDE ON/OFF"** (see U5).
- Bit outputs, MSB to LSB:
  - IC9 pin 10 → IC10 buffer group (inputs 5, 7, 9, 11 in parallel; outputs 4, 6, 10, 12 in parallel) → R90 → node D5.
  - IC9 pin 7 → IC10 buffer group (printed on p5 as inputs "2, 15", outputs "3, 14"; corrected on p-09 to **inputs 3, 14 → outputs 2, 15**) → R87 → node D4.
  - IC9 pin 15 → R79 → node D3.
  - IC9 pin 2 → R80 → node D2.
  - IC9 pin 12 → R81 → node D1.
  - IC9 pin 5 → R82 → node D0.
- Ladder. Each series arm is two 200K resistors in parallel (100K); the shunt elements are single 200K resistors.
  - D5 (top) ← R88 ∥ R89 → D4
  - D4 ← R85 ∥ "R96" → D3
  - D3 ← R83 ∥ R84 → D2
  - D2 ← R76 ∥ R77 → D1
  - D1 ← R74 ∥ R75 → D0
  - D0 ← R78 → +5.333V (the termination goes to the reference, not to GND)
- **D5 = DAC_OUT** goes to IC11b pin 5 (+) and to IC12 pin 8.

### Buffer and slide (IC11b, IC12, C35, Q29/Q30)
- IC11b (AN6562):
  - Pin 5 (+) = DAC_OUT.
  - Pin 6 (−) → R107 2.2K → pin 7. R107 is the feedback resistor, so the stage is a follower.
  - Pin 7 = **CV_BUF**, marked "(TP4)" with an arrow. The black "TP4" badge sits next to D31 on the same node.
- D31: anode on CV_BUF, cathode on +5.333V.
- D30: anode on GND, cathode on CV_BUF. Together, D30 and D31 clamp CV_BUF to between about −0.6 V and about 5.9 V.
- IC12 switch A: pin 8 = DAC_OUT, pin 9 = SLEW, control pin 6 = Q30 collector.
- IC12 switch B: pin 10 = SLEW, pin 11 = CV_BUF (IC11b pin 7), control pin 12 = Q29 collector.
- C35 .22: SLEW to GND.
- Q29 (NPN):
  - Base → R59 100K → SLIDE ON/OFF line.
  - Emitter → GND.
  - Collector = node SW_B, which also has R92 100K to +12V(arrow), R91 1M to Q30 base, and IC12 pin 12.
- Q30 (NPN):
  - Base ← R91 1M ← SW_B.
  - Emitter → GND (joined to Q29's emitter / GND).
  - Collector = SW_A, which also has R93 100K to +12V(arrow) and IC12 pin 6.
- CV_BUF runs right along the top rail (y≈410). It feeds:
  - the upper end of TM5;
  - the outer vertical line at x≈2545. This line goes down to y≈1115, runs right under the label "CV", and reaches J3 (CV) tip. J3 sleeve is GND. No series resistor is drawn.

### Exponential converter (Q26, IC11a)
- Q26a (left half):
  - Base → +5.333V.
  - Collector = node REF_SUM (≈1770,640), which also connects to R103 220K (other end +12V(arrow)), R102 (other end TM4 wiper), IC11a pin 2 (−), and C34.
  - Emitter = EXP_E.
- TM4: one end → +5.333V, other end → +12V(arrow), wiper → R102.
- IC11a (AN6562):
  - Pin 3 (+) → +5.333V.
  - Pin 2 (−) = REF_SUM.
  - Pin 1 output → R104 2.2K → EXP_E (the joined emitters of Q26a and Q26b).
- C34 .001: between pin 2 and pin 1 of IC11a.
- Q26b (right half):
  - Base = node EXP_B, fed by R106 27K (from TM5) and R118 100K (from TUNE), with R100 560 posistor to +5.333V.
  - Collector = **SAW_CAP** node.
  - Emitter = EXP_E.
- TM5 4.7K: top end = CV_BUF; wiper tied to the lower end; lower end → R106 → EXP_B.
- R118 100K: EXP_B → TUNE line. The TUNE line runs up and right along y≈450, down the inner vertical at x≈2508, then right along y≈1315 (labelled "TUNE") to the VR2 wiper (pin 2).
- VR2 TUNING 50K(B):
  - Pin 1 → GND.
  - Pin 3 → a short-bar terminal symbol with no label (U6).
  - Pin 2 (wiper) → TUNE line.

### Sawtooth core and reset
- C33 .01: SAW_CAP to GND.
- Q28 2SK30(O):
  - Gate = SAW_CAP.
  - Drain → +12V(arrow).
  - Source = **VCO_SAW**, with R105 10K to GND.
- Q25: emitter = SAW_CAP; base = node X; collector → Q27 base.
- Q27 (PNP): emitter → +12V(arrow); base ← Q25 collector; collector → D25 anode.
- D25: anode on Q27 collector, cathode on node X.
- Q24 (NPN): base and collector both on node X; emitter → R101 10K → +5.333V.
- C32 10/16: + on +12V(arrow), − on GND.
- C31 10/16: + on +5.333V, − on GND. Both are local decoupling capacitors.

### Square shaper (Q8)
- R35 100K: VCO_SAW → Q8 base.
- C10 .01: VCO_SAW → node C10b.
- R34 10K: C10b → Q8 base.
- Q8 (PNP):
  - Emitter = node Q8E, which also has R45 22K to +12V(arrow) and C11 1/50 (+ on Q8E, − on GND).
  - Base as above.
  - Collector = **VCO_SQR**, with R36 10K to +5.333V.

### Waveform select and the VCF feed
- VCO_SAW runs from the Q28 source right/down along x≈2182 and along line y≈1197 (marked with a saw glyph) to S1, top terminal.
- VCO_SQR runs from the Q8 collector along y≈1052, down x≈2145, and along line y≈1237 (marked with a square glyph) to S1, bottom terminal. It crosses the VCF_IN line without a dot.
- S1 middle terminal (common) = **VCF_IN** (marked with a "saw/square" glyph). This line runs left along y≈1273 to x≈2664, then down into the VCF block, where it reaches the top of R62 220K (≈2640,1830). The VCF reader should confirm this endpoint.
- The switch lever is drawn bridging the top (saw) and middle terminals.

### Signals entering and leaving the block
- In:
  - IC9 D inputs (CPU)
  - CLK/SLIDE ON/OFF
  - +12V(arrow), +5.333V, GND
- Out:
  - CV_BUF → J3 CV
  - VCF_IN → VCF (R62)
  - TUNE ↔ VR2

## 3. Printed annotations
- "5.333V" at IC9 pins 1 and 16. "(R74—R90) ↓ 200K".
- "(MATCHED PAIR .1% SELECTED MFR: R74-R90)".
- "TM4 [TUNE]", "TM5 [WIDTH]".
- "TUNE: 110Hz (SET TUNING [knob drawn at 12 o'clock] CV OUT: 2.75V (A KEY))".
- "WIDTH: 1 OCT./VOLT".
- "POSISTOR: R100", "MFR: ■".
- Saw waveform at the Q28 source:
  - a falling ramp with a vertical (instant) reset;
  - top "12V", bottom "5.5V", "GND" reference.
- Square waveform at the Q8 collector: high "8V", low "5V", "GND" reference, period "(9ms KEY A)". The high level shows a droop.
- Near IC16: "CV KEY C=1.000V, C̿=4.000V" (two Cs, the second with a double bar), and TP5 = IC16 pin 7 (5.333 V).
- p-07 VCO adjustment:
  - Check point is the Q28 source or the S1 WAVEFORM terminal.
  - Set TUNE to its middle position.
  - In PITCH mode, tap CL and CH alternately and adjust TM5 WIDTH for 2:1 waveforms (the drawing shows two saw periods labelled WIDTH).
  - Press the A key and adjust TM4 for 110 Hz.
  - Confirm 4:1 ±0.5% using CL and CH + TRANSPOSE UP.
- p-08 parts list:
  - 2SC1583(F) Dual Tr; 2SK30TM(Y) and (O); HD14174BP; HD14050BP; HD14066BP; AN6562 dual op-amp.
  - Pots: K161B0-50kB "TUNING, ACCENT".
  - Trimmers: 47kB, 220kB, 470kB, 4.7kB.
- p-09 errata: the IC10 buffer driven by IC9 pin 7 is corrected to input pins 3/14 and output pins 2/15.

## 4. Functional description (from this transcription)
1. **DAC.** A 6-bit latch (IC9, clocked on pin 9) drives a 200K/100K R-2R ladder. The two MSBs go through 4050 buffers, with four buffers paralleled on the MSB. The ladder is terminated to 5.333 V, so DAC_OUT spans from about 5.333 V downward or upward around the reference, depending on the codes. It is buffered by IC11b (a follower with 2.2K feedback) to give CV_BUF (TP4), which is clamped by D30/D31 and sent straight to the CV jack J3.
2. **Slide.** Q29 and Q30 form an inverter pair, so IC12 switches A and B are driven in antiphase.
   - When switch A is closed, C35 (0.22 µF) loads the ladder output node. The ladder's Thevenin source resistance is about 100K, so CV_BUF glides with a time constant of about 22 ms.
   - When switch B is closed instead, C35 is held at CV_BUF (pre-charged), and the DAC drives IC11b directly.
   - Which state of the SLIDE line selects which switch has not been verified.
3. **Exponential converter.** The CV is scaled by TM5 + R106 against R100 (a 560 Ω PTC, for temperature compensation) to 5.333 V at the base of Q26b, with the TUNE pot added through R118 100K. Q26a's base and collector are both held at 5.333 V by IC11a, which forces Q26a's collector to sink the reference current (set by R103 from +12 V and by TM4 through R102). IC11a drives the shared emitters through R104. The collector current of Q26b is therefore exponential in the voltage at its base.
4. **Sawtooth core.** Q26b's collector current discharges C33 (0.01 µF), producing a falling ramp from about 12 V to about 5.5 V. The reset loop (Q25 / Q27 / D25 / Q24 + R101 to 5.333 V) recharges C33 rapidly when the ramp reaches a threshold of about 5.333 V + Vbe − Vbe.
5. **Outputs.** Q28 (2SK30) is a source follower giving VCO_SAW. Q8 (PNP, emitter supplied from +12 V through R45 22K and decoupled by C11) is driven by the saw through R35, plus an AC path through C10 + R34. It acts as a comparator/shaper, producing a square of about 5–8 V across R36 to 5.333 V. S1 selects saw or square onto VCF_IN.

## 5. Uncertainties
- **U1.** The filled-arrow supply symbol is unlabelled in the VCO. It is assumed to be +12 V because an identical arrow at IC16 is labelled "12V".
- **U2.** The ladder resistor beside R85 reads "R96". It is most likely R86, since R74–R90 is printed and R86 appears nowhere else in the ladder.
- **U3.** TM4's value prints as "4 K (B)" with a gap. It is probably 47K(B), which matches the 47kB trimmer in the parts list.
  - R102 prints as "100", with no unit attached.
  - A small "↙" arrow is printed under "(B)". Its meaning is unknown.
  - R102 = 100 Ω would load the 5.333 V summing node heavily from the wiper of a 47K trimmer, so 100K is plausible. This is unverified.
- **U4.** Q25 polarity: the emitter arrowhead is a blob and its direction is ambiguous.
  - The topology (emitter on SAW_CAP, base on node X, collector sinking Q27's PNP base) only works as an NPN emitter-follower reset. In that case Q25 conducts when the saw falls below X − Vbe, turning on Q27, which drives X high through D25 and recharges C33.
  - Treat it as NPN?, to be confirmed by the other reader.
- **U5.** The "SLIDE ON/OFF" line connects with a junction dot to the IC9 pin 9 / IC13 pin 11 vertical line. That would make the latch clock and the slide control the same net. This is drawn so, but it is electrically surprising, so recheck it.
- **U6.** VR2 pin 3 goes to an unlabelled short-bar terminal symbol. Its supply (a reference voltage?) is not identified in this region.
- **U7.** Diode types for D25, D30 and D31, and transistor types for Q8, Q24, Q25, Q27, Q29 and Q30, are not printed near the symbols.
- **U8.** The "CV OUT feed via IC16" mentioned in the task: on this page the CV line only passes beside IC16. IC16 is the 5.333 V / 12 V supply op-amp (TP5 = pin 7 = 5.333 V). No buffer between CV_BUF and J3 was found.

## Memory hints (unverified)
- The TB-303 slide is commonly described as having a time constant of about 60 ms. This is not derived from this page.
- Q24/Q25 are often described elsewhere as the reset comparator pair, with Q25 an NPN-type device. This is unverified here.
