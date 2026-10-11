# TB-303 VCF: transcription by reader vcfA

Source: p5-05.png (300 dpi main-board page). Pixel coordinates below are p5-05 coordinates (x,y).
VCF block is roughly x 2560–3560, y 1380–2400. Front-panel pots are at x ≈ 4280–4400.
Also used: p-07 (VCF adjustment), p-08 (VCF envelope modulation text and parts list), p-09 (errata).

Supply symbols, identified from the IC16/CV area of the same page:
- **Filled arrow = 12V.** The "12V" label sits on a filled arrow at IC16.
- **⊕ = 5.333V.** The "5.333V" label sits on a ⊕ at IC16. Errata p-09 shows the same ⊕ = 5.333V at IC10.

Confidence key: ✓ clear, ? partly legible or inferred, ✗ illegible.

## 1. Component table

| Designator | Value as printed | Type / notes | Conf. |
|---|---|---|---|
| R62 | 220K | resistor, input series | ✓ |
| C17 | 1/50 | electrolytic 1µF/50V, + toward R62 | ✓ |
| R70 | 2.2K | Q12 left base to bias node | ✓ |
| R68 | 2.2K | ⊕5.333V to bias node | ✓ |
| C16 | 10/16 | electrolytic 10µF/16V, + to bias node, − to GND | ✓ |
| R98 | 2.2K | bias node to Q12 right base | ✓ |
| R97 | 10K | Q12 right base to C23 (resonance feedback injection) | ✓ |
| Q12 | 2SC1583 | dual NPN, bottom pair of the ladder (label "2SC1583" printed beside it) | ✓ |
| Q13, Q15 | (no type printed) | ladder, row 4. NPN symbol, diode-connected (B tied to C) | ✓ (type ✗) |
| Q14, Q17 | (no type printed) | ladder, row 3. NPN, diode-connected | ✓ (type ✗) |
| Q16, Q23 | (no type printed) | ladder, row 2. NPN, diode-connected | ✓ (type ✗) |
| Q22 | 2SC2291 | dual NPN, top pair of the ladder, bases tied together | ✓ |
| C18 | .018 | ladder cap, bottom rung (Q13 E / Q15 E level) | ✓ |
| C19 | .033 | ladder cap (Q14 E / Q17 E level) | ✓ |
| C24 | .033 | ladder cap (Q16 E / Q23 E level) | ✓ |
| C26 | .033 | ladder cap, top rung (Q22 E level) | ✓ |
| R115 | 10K | top rail to Q22 base node | ✓ |
| C30 | 10/16 | electrolytic, Q22 base node to GND, + up | ✓ |
| R95 | 100 | 12V to VCF top rail | ✓ |
| C28 | 47/16 | electrolytic, top rail to GND | ✓ |
| C27 | .1 | ladder top-right node (Q22R E) to Q21 left base | ✓ |
| C25 | .1 | ladder top-left node (Q22L E) to Q21 right base | ✓ |
| Q21 | 2SC1583 | dual NPN differential output amplifier ("2SC1583" printed above it) | ✓ |
| R111 | 2.2K | Q21 left collector load | ✓ ("2.2K" partly cropped by the rail label, still legible) |
| R110 | 2.2K | Q21 right collector load | ✓ |
| R113 | 100K | Q21 left base bias | ✓ |
| R114 | 100K | Q21 right base bias | ✓ |
| R116 | 10K | ⊕5.333V to Q21 bias node | ✓ |
| C29 | 1/50 | electrolytic, Q21 bias node to GND | ✓ |
| R112 | 10K | Q21 bias node to Q20 base | ✓ |
| R109 | 10K | Q20 base to GND | ✓ |
| Q20 | (no type printed) | NPN tail current sink for Q21 | ✓ (type ✗) |
| R108 | 2.2K | Q20 emitter to GND | ✓ |
| Q19 | (no type printed) | NPN emitter follower, output buffer | ✓ (type ✗) |
| R96 | 10K | Q19 emitter to GND | ✓ |
| C14 | 1/50 | electrolytic output coupling, + on Q19 side | ✓ |
| VR4 (gang A) | 50K(B) | RESONANCE. Dual pot, mechanical link drawn dashed | ✓ |
| VR4 (gang B) | (no separate value printed) | second gang of RESONANCE, drawn to the right of gang A | ✓ |
| C15 | 1/50 | electrolytic, + on R67 side | ✓ |
| R67 | 2.2K | C15 to Q18 base | ✓ |
| R99 | 100K | Q18 base to bias node | ✓ |
| R66 | 100K | ⊕5.333V to Q18 bias node | ✓ |
| C22 | 1/50 | electrolytic, Q18 bias node to GND. **The VCA block also has a "C22 .01" (duplicate designator)** | ✓ |
| Q18 | (no type printed) | NPN emitter follower, collector to 12V | ✓ (type ✗) |
| R94 | 10K | Q18 emitter to GND | ✓ |
| C23 | 1/50 | electrolytic, + toward R97 | ✓ (polarity ?) |
| R69 | 2.2K | Q12 common emitters to Q11 collector | ✓ |
| Q11 | (no type printed) | NPN, expo-converter output transistor | ✓ (type ✗) |
| Q10 | (no type printed) | PNP (emitter arrow points into the bar, as in p-08 Fig. 2) | ✓ (type ✗) |
| Q9 | (no type printed) | Collector to GND. Emitter arrow is ambiguous at this scan resolution; PNP is the only reading consistent with the wiring | ? |
| R65 | 10K | ⊕5.333V to Q9 base | ✓ |
| R64 | 10K | Q9 base to GND | ✓ |
| R71 | 2.2K | Q10 base to VQ9 rail | ✓ |
| R73 | 100K | Q10 emitter / Q11 base to TM3 | ✓ |
| TM3 | 470K | trimmer wired as a rheostat (wiper tied to one end), boxed label "FREQ" | ✓ |
| R63 | 220K | VR5 wiper (ENV MOD) to Q10 base | ✓ |
| R72 | 100K | VR4 gang B wiper (accent sweep) to Q10 base | ✓ |
| VR3 | (no value printed on schematic; parts list gives K161B0-50kA) | CUTOFF FREQ | ✓ |
| R47 | 10K | VR3 terminal 1 to VQ9 rail | ✓ |
| VR5 | 50K(A) | ENV.MOD | ✓ |
| R61 | 10K | VR5 terminal 1 to VQ9 rail | ✓ |
| C13 | 1/50 | electrolytic, VR4 gang B terminal 3 to GND, + up | ✓ |
| R46 | 47K | D24 cathode to VR4 gang B terminal 1 | ✓ |
| D24 | (no type printed) | diode, anode = ACCENT pot wiper, cathode = R46 | ✓ |
| TP6 | — | test-point badge printed at about (3700,1770), near the waveform sketch | ✓ (attachment ?) |

No AN6562 op-amp appears in the VCF. The output stage is discrete: Q21 differential pair, Q20 tail, Q19 follower. The AN6562 on this page is IC16 in the CV area.

## 2. Netlist

Net names: `VCF_RAIL` = filtered 12V top rail; `VQ9` = Q9 emitter rail; `NL1..NL4` / `NR1..NR4` = left/right ladder nodes, top to bottom.

### Input and bottom pair
- `VCF_IN` = S1 WAVEFORM common terminal (middle contact; the saw line and square line go to the other two contacts). Runs along y≈1275, down x≈2665 → R62 top.
- R62: `VCF_IN` ↔ `C17P`
- C17: + `C17P`, − `Q12_BL`
- `Q12_BL` = Q12 left base = R70 top
- R70: `Q12_BL` ↔ `BIAS12`
- R68: ⊕5.333V ↔ `BIAS12`
- C16: + `BIAS12`, − GND
- R98: `BIAS12` ↔ `Q12_BR`
- `Q12_BR` = Q12 right base = R97 left
- R97: `Q12_BR` ↔ `C23P`
- Q12 (2SC1583 dual):
  - left C → `NL4`; right C → `NR4`
  - both E → `Q12_E`
  - B as above
- R69: `Q12_E` ↔ `Q11_C`

### Ladder
Every ladder transistor is drawn with its base strapped to its own collector (diode-connected).
- Q13: C+B `NL4`, E `NL3`
- Q15: C+B `NR4`, E `NR3`
- C18 (.018): `NL4` ↔ `NR4`
- Q14: C+B `NL3`, E `NL2`
- Q17: C+B `NR3`, E `NR2`
- C19 (.033): `NL3` ↔ `NR3`
- Q16: C+B `NL2`, E `NL1`
- Q23: C+B `NR2`, E `NR1`
- C24 (.033): `NL2` ↔ `NR2`
- C26 (.033): `NL1` ↔ `NR1`
- Q22 (2SC2291 dual):
  - left E `NL1`, right E `NR1`
  - both C `VCF_RAIL`
  - both B tied → `Q22_B`
- R115: `VCF_RAIL` ↔ `Q22_B`
- C30: + `Q22_B`, − GND
- R95: 12V ↔ `VCF_RAIL`
- C28: + `VCF_RAIL`, − GND

The "C24 / C19 / C18" rungs sit between the emitters of the pair above and the collectors of the pair below. The node naming above follows that layout.

### Output differential amplifier
- C27 (.1): `NR1` ↔ `Q21_BL`. The y≈1595 line from the Q22R emitter column runs right to C27.
- C25 (.1): `NL1` ↔ `Q21_BR`. The y≈1555 line from the Q22L emitter crosses the right column with no dot and runs to C25.
- R113: `Q21_BL` ↔ `BIAS21`
- R114: `Q21_BR` ↔ `BIAS21`
- R116: ⊕5.333V ↔ `BIAS21`
- C29: + `BIAS21`, − GND
- R112: `BIAS21` ↔ `Q20_B`
- R109: `Q20_B` ↔ GND
- Q21 (2SC1583 dual):
  - left C → R111 → `VCF_RAIL`
  - right C → `Q21_CR` → R110 → `VCF_RAIL`
  - E tied → `Q21_E`
- Q20: C `Q21_E`, B `Q20_B`, E → R108 → GND
- Q19: B `Q21_CR`, C `VCF_RAIL`, E `Q19_E`
- R96: `Q19_E` ↔ GND
- C14: + `Q19_E`, − `VCF_OUT`

### Output distribution and resonance
`VCF_OUT` runs down x≈3542.
- VR4 gang A terminal 3 = `VCF_OUT` (dot at (3542,1840)).
- `VCF_OUT` also continues down to y≈2467, then left to x≈2459 and up to the VCA. There it feeds R121 220K → C21 .01 → IC15 (BA662A) pin 3.
- VR4 gang A terminal 1 → GND.
- VR4 gang A wiper (2) → `RES_FB`. Line y≈1893, dot at x≈3584.
- From `RES_FB`, a branch runs down to y≈2505, left to x≈2418 and up to the VCA. There it feeds R122 100K → C22(.01, VCA) → IC15 pin 3.
- C15: − `RES_FB`, + `C15P`
- R67: `C15P` ↔ `Q18_B`
- R99: `Q18_B` ↔ `Q18_BIAS`
- R66: ⊕5.333V ↔ `Q18_BIAS`
- C22 (1/50): + `Q18_BIAS`, − GND
- Q18: C 12V (arrow), B `Q18_B`, E `Q18_E`
- R94: `Q18_E` ↔ GND
- C23: − `Q18_E`, + `C23P`. C23P is joined to `Q12_BR` through R97.

### Exponential converter and cutoff control
- Q9: B `Q9_B`, E `VQ9`, C GND
- R65: ⊕5.333V ↔ `Q9_B`
- R64: `Q9_B` ↔ GND
- Q11: C `Q11_C` (R69), B `Q11_B`, E `VQ9`
- Q10 (PNP): E `Q11_B`, B `Q10_B`, C `VQ9`
- R71: `Q10_B` ↔ `VQ9`
- R73: `Q11_B` ↔ `TM3_A`
- TM3 470K: one end and wiper → `TM3_A`; the other end → `CUTOFF_W`
- `CUTOFF_W`: line y≈2205 → x≈3505 → up to y≈1693 → VR3 wiper (2)
- VR3 CUTOFF FREQ: terminal 3 → 12V (filled arrow); wiper → `CUTOFF_W`; terminal 1 → R47 → `VQ9` (node at x≈4100, y≈1738–2220)
- VR5 ENV.MOD:
  - terminal 3 → `ENV_IN`: line y≈2125 → x≈3900 → down to y≈2667 → left to x≈2256 → up toward the ENV block (not traced further)
  - wiper (2) → `ENVMOD_W`: y≈2163 → x≈3658 → y≈2270 → R63
  - terminal 1 → R61 → `VQ9`
- R63: `ENVMOD_W` ↔ `Q10_B`
- `VQ9` path: VR3-1/R47 and VR5-1/R61 meet at x≈4100, y≈2220. From there the line runs left to x≈3700, down to y≈2390, then left along the y≈2390 rail to R71 bottom, Q10 C and Q11/Q9 E.

### Accent sweep
- `ACCENT_W` = VR7 (ACCENT, 50K(B)) wiper, line y≈2545. Dot at x≈3740. The same line also runs left to x≈2536 and up into the ENV/VCA area.
- D24: anode `ACCENT_W`, cathode → R46 bottom
- R46 (47K): D24 cathode ↔ VR4 gang B terminal 1. Terminal 1 line is y≈2005; R46 top is at x≈3745.
- VR4 gang B terminal 3 → C13 + (y≈1966). C13 − → GND.
- VR4 gang B wiper (2) → `ACC_SWEEP`: y≈2045 → x≈3820 → down to y≈2335 → left → R72
- R72: `ACC_SWEEP` ↔ `Q10_B`

### Signals leaving the block
- `VCF_OUT` → VCA (IC15 via R121/C21)
- `RES_FB` → VCA (IC15 via R122/C22 .01)
- `ENV_IN` from the ENV block
- `ACCENT_W` from VR7
- 12V, ⊕5.333V, GND

## 3. Printed annotations and pages 7–9

### Near the VCF on page 5
- Waveform sketch near TP6, top right of the block. Two decaying ringing bursts are drawn.
  - Ring period marked "2ms ±.5ms".
  - Levels marked "5.8V" (peak) and "5.2V" (baseline), with a "0" reference line.
  - The burst-to-burst spacing has a dimension arrow and the note "(15 ms KEY C : see ADJ. VCF)".
- A leader line from that note points to the `VCF_OUT` line just left of VR4 terminal 3.
- The TP6 badge itself has no drawn wire.
- "FREQ" is printed in a box beside TM3.
- Dual-gang RESONANCE: the second resistive element sits right of VR4, with a dashed mechanical-link line. Terminals are numbered 3 / 2 / 1 on each gang.
- Pot labels on the schematic:
  - VR3 "CUTOFF FREQ." (no value printed)
  - VR4 "50K(B)" "RESONANCE"
  - VR5 "50K(A)" "ENV.MOD"
  - VR7 "50K(B)" "ACCENT"

### p-07 VCF adjustment
> VCF / Check point: TP6 / Panel settings: CUTOFF FREQ : center / WAVEFORM : SAWTHOOTH / RES : Full clockwise / ENV MOD, DECAY, ACC : Full counterclockwise. "Afdjust TM3 for:" [ringing waveform sketch] "2ms ± 0.5ms".

The typos "SAWTHOOTH" and "Afdjust" are as printed.

### p-08 "VCF ENVELOPE MODULATION" (verbatim)
> In the conventional VCF configuration, envelope modulation voltages applied to modulation control voltage input of VCF rise from a fixed potential to their values regardless of shape and amount of envelope and fall back to the same potential. Fig. 1. In this setup, an attempt to deepen envelope modulation is increment of control voltage in one direction, only to open up filter. Resultants are; higher the control voltage input, longer the period of time cutoff point travels through higher frequency range in which significant aural sound characteristic changes do not occur, whereas notes are brightened.
> To compensate for the limitation this impose, there is a gimmick in the TB-303 VCF circuit. The Q9 has a key role in this arrangement. See Fig. 2. It developes bias for Q10 and Q11 to set initial cutoff point at the center of frequency range. Within the range even a small amount of voltage change can create a appreciable modulation.
>
> Rotating wiper ② of VR5 closer to terminal ③ increases ENV MOD voltage being fed to base of Q10 while changing the bias which in turn lowers VCF cutoff frequency. See Fig. 3. This is equal to turning CUTOFF knob counterclockwise and, since Q10 and Q11 in combination convert control voltage to the current in anti-log fashion, the cutoff point stays in an effective frequency range for the most portion of envelope sweeping duration.

The figures:
- **Fig. 1:** two decaying envelopes of different size, both rising from and returning to the same baseline.
- **Fig. 2:**
  - Q11 is NPN; its emitter goes to a common node.
  - Q10 is PNP: emitter to Q11 base, collector to the common node.
  - Q10 base → resistor "VR5" (the series resistor drawn beside the label) → VR5 wiper ②.
  - The envelope (arrow from a decay curve) enters VR5 terminal ③. VR5 terminal ① goes to the common node.
  - The common node sits on a battery "VQ9" to ground.
  - This matches the page-5 wiring: VR5 → R63 → Q10 B, VR5-1 → R61 → VQ9.
- **Fig. 3:** larger and smaller envelopes cross at a pivot level. Below them the baseline is marked "VQ9", with a dashed line.

### p-08 parts list (relevant items)
Potentiometers:
- K161B0-50KB: TUNING, ACCENT
- K161B0-50kA: CUT OFF FREQ, ENV MOD
- K161B0-1MA: DECAY
- K162T00W-50kB x 2: RESONANCE
- Trimmers: H1051A016-47kB, H1051A021-220kB, H1051A023-470kB, H1021A011-4.7kB. The 470kB trimmer matches TM3 470K.

Transistors:
- 2SA733(P), 2SB647(C), 2SB596(O), 2SD667(C), 2SC2021(R)
- 2SC1583(F) "Dual Tr"
- 2SC2291(F) "Dual Tr"

Diodes: 1S2473 (Si), 1S-188FM (Ge), 1SS-133 (Si), and others.

### p-09 errata
- RESONANCE pot part number 13219774 is corrected to 13219775 (K162T00W-50kB x2).
- Transistor list adds:
  - 2SC2021(R) "or 2SD637(R)"
  - 2SC536(F)
  - 2SA937(Q) "or 2SB642(R)"
- The page-5 circuit corrections cover IC10 (DAC) and the power supply (C66 0.0022, C67 10/16, Q42 2SC536). None affect the VCF.

The p-07 package legend lists 2SC536F, 2SA733P, 2SC2021R and 2SA937Q, so these are the general-purpose types. The schematic does not assign them to the VCF Q-numbers.

## 4. Functional description (from this transcription only)
1. **Input.** The selected VCO waveform (S1 common) is AC-coupled through R62 220K and C17 into Q12 left base. Q12 right base is the feedback input. Both bases are DC-biased from ⊕5.333V via R68, C16, R70 and R98.
2. **Ladder.** Q12's collectors drive a four-stage ladder of diode-connected NPN pairs (Q13/Q15, Q14/Q17, Q16/Q23, Q22). Rung capacitors are .018 (bottom) and .033, .033, .033. The top pair Q22 (2SC2291) has a common base held by R115 and C30 from the R95/C28-filtered 12V rail.
3. **Output.** The top ladder nodes are AC-coupled crosswise (C25, C27 .1) into the 2SC1583 differential pair Q21. Q21 has 2.2K loads, a Q20 current sink, and base bias from ⊕5.333V via R116, C29, R112 and R109. Q19 buffers the result and C14 couples it out as `VCF_OUT`.
4. **Resonance.** `VCF_OUT` goes to the VCA through R121/C21 and to RESONANCE gang A. The gang A wiper returns via C15, R67, emitter follower Q18, C23 and R97 to Q12 right base. The same wiper also feeds the VCA through R122.
5. **Cutoff and envelope.** Ladder current is Q12 tail current, set by Q11's collector through R69. Q10 (PNP) and Q11 (NPN) form the "anti-log" converter referenced to `VQ9`, which emitter follower Q9 holds at about 2.67V + Vbe (R65/R64 divider). The control inputs are:
   - CUTOFF pot wiper via TM3 + R73 into Q11 base / Q10 emitter
   - ENV MOD wiper via R63 220K into Q10 base
   - accent sweep via R72 100K into Q10 base
6. **Pot references.** CUTOFF and ENV MOD pot "ground" ends return to `VQ9` through R47 and R61 (10K each), not to GND. This matches p-08 Fig. 2.
7. **Accent sweep.** The ACCENT pot wiper drives D24 → R46 47K → RESONANCE gang B (terminal 1), with C13 1/50 to GND at terminal 3. Gang B's wiper feeds R72 into Q10 base, so accent-sweep depth and smoothing depend on the RESONANCE setting.

## 5. Uncertainties
1. **Ladder transistor types.** Q13–Q17 and Q23 have no type printed, and neither do Q9, Q10, Q11, Q18, Q19 or Q20. Only Q12 and Q21 (2SC1583) and Q22 (2SC2291) are labelled.
2. **Q9 polarity.** The arrow is ambiguous in the scan. PNP is inferred from collector-to-GND wiring and p-08 ("VQ9").
3. **Ladder rung placement.** The node assignment of C24, C19 and C18 between the adjacent emitter and collector nodes depends on line-junction reading. Rung values are certain: C26 .033, C24 .033, C19 .033, C18 .018.
4. **C25/C27 crossing.** C25 connects to the left top node (Q22L E) and C27 to the right top node (Q22R E), a crosswise coupling. This rests on a no-dot crossing at about (2930,1555). Worth cross-checking.
5. **Wire crossings.** Q12_E (x≈2985) crosses the R98 line, and Q21_E (x≈3168) crosses the BIAS21 line. Both are read as crossings with no dot.
6. **Duplicate C22.** "C22" appears twice: 1/50 at the Q18 bias in the VCF, and .01 in the VCA input.
7. **TP6 attachment.** The TP6 badge has no drawn connection. The location is inferred from the annotation leader pointing at `VCF_OUT` (VR4 gang A terminal 3).
8. **Pot values.** VR3's value is not printed on page 5; 50kA comes from the parts list. Gang B of VR4 has no separate value printed; the parts list gives "50kB x 2".
9. **C23 polarity.** The "+" sign is small; read as + on the R97 side.
10. **ENV_IN source.** The upstream source of `ENV_IN` (VR5 terminal 3, x≈2256 riser) and the ACCENT line's left continuation (x≈2536) were not traced into the ENV block.

## Memory hints (unverified)
- These are memory only, not checked against this scan.
- Typical TB-303 references describe the ladder transistors as 2SC1583 dual pairs, and Q9/Q10 as 2SA733 PNP with Q11 a 2SC1815-class NPN.
- I did not use any of this in the transcription.
