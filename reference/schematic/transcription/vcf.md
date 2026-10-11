# TB-303 VCF — reconciled transcription (from vcf-A.md + vcf-B.md)

Scope: VCF ladder, output stage, resonance feedback, cutoff control / exponential converter, accent-sweep input.
Source: p5-05.png (4961x3505, 300 dpi); p-07 (adjustment), p-08 (VCF envelope note + parts list), p-09 (errata).
Coordinates are p5-05 source pixels (x0,y0)-(x1,y1). Reconciliation crops were saved as /tmp/vcfR/*.png.

Supply symbols, confirmed at IC16 (crop (2300,400)-(3000,800)): **filled up-arrow = 12V**, **⊕ = 5.333V**.
Both labels are printed beside IC16. All filled arrows in the VCF (R95, VR3-3, Q18 C) use the same filled style.

Confidence: ✓ clear, ? partly legible/inferred, ✗ illegible.

## 1. Component table (reconciled)

| Ref | Value as printed | Type / role | Conf. |
|---|---|---|---|
| R62 | 220K | input series resistor (S1 common → C17) | ✓ |
| C17 | 1/50 | electrolytic 1µF/50V; + on R62 side, − on Q12L base | ✓ |
| R70 | 2.2K | Q12L base → BIAS_IN | ✓ |
| R68 | 2.2K | ⊕5.333V → BIAS_IN | ✓ |
| C16 | 10/16 | electrolytic; + BIAS_IN, − GND | ✓ |
| R98 | 2.2K | BIAS_IN → Q12R base | ✓ |
| R97 | 10K | Q12R base → C23 + (resonance injection) | ✓ |
| Q12 | 2SC1583 | dual NPN, bottom differential pair of the ladder | ✓ |
| R69 | 2.2K | Q12 common emitters (IEXP) → Q11 C | ✓ |
| Q13, Q15 | (no type) | NPN, diode-connected (B=C), lowest diode rung L3→L4 / R3→R4 | ✓ (type ✗) |
| Q14, Q17 | (no type) | NPN, diode-connected, L2→L3 / R2→R3 | ✓ (type ✗) |
| Q16, Q23 | (no type) | NPN, diode-connected, L1→L2 / R1→R2 | ✓ (type ✗) |
| Q22 | 2SC2291 | dual NPN, top pair; C to VCF_RAIL, bases tied, E = L1/R1 | ✓ |
| C26 | .033 | rung L1–R1 (Q22 emitters, top) | ✓ |
| C24 | .033 | rung L2–R2 | ✓ |
| C19 | .033 | rung L3–R3 | ✓ |
| C18 | .018 | rung L4–R4 (Q12 collectors, bottom) | ✓ |
| R115 | 10K | VCF_RAIL → BIAS_Q22 | ✓ |
| C30 | 10/16 | + BIAS_Q22, − GND | ✓ |
| R95 | 100 | 12V → VCF_RAIL | ✓ |
| C28 | 47/16 | + VCF_RAIL, − GND | ✓ |
| C25 | .1 | L1 (left top) → Q21R base (crossed) | ✓ |
| C27 | .1 | R1 (right top) → Q21L base (crossed) | ✓ |
| Q21 | 2SC1583 | dual NPN output differential amplifier | ✓ |
| R111 | 2.2K | VCF_RAIL → Q21L C (no other connection) | ✓ |
| R110 | 2.2K | VCF_RAIL → Q21R C (= DIFF_OUT) | ✓ |
| R113 | 100K | Q21L B → BIAS_DIFF | ✓ |
| R114 | 100K | Q21R B → BIAS_DIFF | ✓ |
| R116 | 10K | ⊕5.333V → BIAS_DIFF | ✓ |
| C29 | 1/50 | + BIAS_DIFF, − GND | ✓ |
| R112 | 10K | BIAS_DIFF → Q20 B | ✓ |
| R109 | 10K | Q20 B → GND | ✓ |
| Q20 | (no type) | NPN tail current sink for Q21 | ✓ (type ✗) |
| R108 | 2.2K | Q20 E → GND | ✓ |
| Q19 | (no type) | NPN emitter follower; C to VCF_RAIL | ✓ (type ✗) |
| R96 | 10K | Q19 E → GND | ✓ |
| C14 | 1/50 | + Q19 E, − VCF_OUT | ✓ |
| VR4 gang A | 50K(B) | RESONANCE; 3=VCF_OUT, 2=RES_FB, 1=GND | ✓ |
| VR4 gang B | (value not printed; parts list K162T00W-50kB x2) | RESONANCE second gang; 3=C13, 2=ACC_SWEEP, 1=R46 | ✓ |
| C15 | 1/50 | − RES_FB, + R67 | ✓ |
| R67 | 2.2K | C15 + → Q18 B | ✓ |
| R99 | 100K | Q18 B → BIAS_FB | ✓ |
| R66 | 100K | ⊕5.333V → BIAS_FB | ✓ |
| C22 (VCF) | 1/50 | + BIAS_FB, − GND. **Duplicate designator**: the VCA input also has "C22 .01" | ✓ |
| Q18 | (no type) | NPN emitter follower; C to 12V (filled arrow) | ✓ (type ✗) |
| R94 | 10K | Q18 E → GND | ✓ |
| C23 | 1/50 | + R97 side, − Q18 E | ✓ |
| Q9 | (no type) | **PNP** emitter follower; E=VQ9, C=GND | ✓ (type ✗) |
| R65 | 10K | ⊕5.333V → Q9 B | ✓ |
| R64 | 10K | Q9 B → GND | ✓ |
| Q10 | (no type) | **PNP**; E=EXP_E, B=EXP_B, C=VQ9 | ✓ (type ✗) |
| Q11 | (no type) | **NPN**; C=R69, B=EXP_E, E=VQ9 | ✓ (type ✗) |
| R71 | 2.2K | EXP_B → VQ9 | ✓ |
| R73 | 100K | EXP_E → TM3 (strapped end) | ✓ |
| TM3 | 470K | trimmer as rheostat (wiper strapped to R73 end); boxed "FREQ" | ✓ |
| R63 | 220K | VR5 wiper → EXP_B | ✓ |
| R72 | 100K | VR4B wiper → EXP_B | ✓ |
| VR3 | (not printed; parts list K161B0-50kA) | CUTOFF FREQ; 3=12V, 2=CUTOFF_W, 1=R47 | ✓ |
| R47 | 10K | VR3-1 → VQ9 | ✓ |
| VR5 | 50K(A) | ENV.MOD; 3=ENV_IN, 2=R63, 1=R61 | ✓ |
| R61 | 10K | VR5-1 → VQ9 | ✓ |
| VR7 | 50K(B) | ACCENT (other block); 2=ACCENT_W, 1=GND, 3=accent source | ✓ |
| D24 | (no type) | anode ACCENT_W, cathode R46 | ✓ (type ✗) |
| R46 | 47K | D24 K → VR4B-1 | ✓ |
| C13 | 1/50 | + VR4B-3, − GND | ✓ |
| TP6 | — | test-point badge at ~(4031,1803), no drawn wire | ✓ (attachment ?) |

There is no op-amp in the VCF. The output stage is discrete (Q21 diff pair, Q20 tail, Q19 follower).

## 2. Reconciled netlist

Net names:
- `VCF_RAIL`: R95/C28-filtered 12V.
- `L1..L4`, `R1..R4`: left/right ladder columns, 1 = top (Q22 emitters), 4 = bottom (Q12 collectors).

### Supply and top of ladder
- R95: 12V ↔ VCF_RAIL. C28: + VCF_RAIL, − GND.
- VCF_RAIL also connects to: R115, Q22L C, Q22R C, R111, R110, Q19 C.
- R115: VCF_RAIL ↔ BIAS_Q22. C30: + BIAS_Q22, − GND.
- Q22L: C VCF_RAIL, B BIAS_Q22, E L1.
- Q22R: C VCF_RAIL, B BIAS_Q22, E R1.

### Ladder (current flows top → bottom; each diode is C+B on the upper node, E on the lower node)
- C26 .033: L1 ↔ R1
- Q16: C+B L1, E L2. Q23: C+B R1, E R2.
- C24 .033: L2 ↔ R2
- Q14: C+B L2, E L3. Q17: C+B R2, E R3.
- C19 .033: L3 ↔ R3
- Q13: C+B L3, E L4. Q15: C+B R3, E R4.
- C18 .018: L4 ↔ R4
- Q12L: C L4, B Q12L_B, E IEXP.
- Q12R: C R4, B Q12R_B, E IEXP.
- R69: IEXP ↔ Q11_C. The R69 vertical crosses the R68–R98 bias line with no dot.

### Input and Q12 bias
- VCF_IN = S1 WAVEFORM common (middle contact). The line runs along y≈1277 to x≈2665, then down to R62. It crosses the TUNE line (y≈1317) with no dot.
- R62: VCF_IN ↔ C17+. C17: + R62, − Q12L_B.
- R70: Q12L_B ↔ BIAS_IN.
- R68: 5.333V ↔ BIAS_IN. C16: + BIAS_IN, − GND.
- R98: BIAS_IN ↔ Q12R_B.
- R97: Q12R_B ↔ C23+. C23: + R97, − FB_E.

### Output stage
- C25 .1: L1 ↔ Q21R_B. The y≈1554 line has a dot at L1 and crosses the R1 column with no dot.
- C27 .1: R1 ↔ Q21L_B. The y≈1594 line is C26's rung line, with a dot at R1.
- R113: Q21L_B ↔ BIAS_DIFF. R114: Q21R_B ↔ BIAS_DIFF.
- R116: 5.333V ↔ BIAS_DIFF. C29: + BIAS_DIFF, − GND.
- R112: BIAS_DIFF ↔ Q20_B. R109: Q20_B ↔ GND.
- Q21L: C → R111 → VCF_RAIL; B Q21L_B; E TAIL21.
- Q21R: C DIFF_OUT (→ R110 → VCF_RAIL); B Q21R_B; E TAIL21.
- Q20: C TAIL21, B Q20_B, E → R108 → GND.
- Q19: C VCF_RAIL, B DIFF_OUT, E Q19_E. R96: Q19_E ↔ GND.
- C14: + Q19_E, − VCF_OUT.
- VCF_OUT (x≈3540):
  - → VR4A-3 (dot at (3540,1840), line y≈1837)
  - → down to y≈2467, then left to x≈2462, then to the VCA: **C21 .01 → R121 220K → IC15 (BA662A) input**

### Resonance (gang A) feedback
- VR4A: 3 VCF_OUT, 2 RES_FB (line y≈1890), 1 GND.
- RES_FB:
  - → C15 −
  - → dot at x≈3584 → down to y≈2507 → left to x≈2420 → VCA: **C22 .01 → R122 100K → IC15 input**
- C15: − RES_FB, + C15P. R67: C15P ↔ Q18_B.
- R99: Q18_B ↔ BIAS_FB. R66: 5.333V ↔ BIAS_FB. C22(VCF) 1/50: + BIAS_FB, − GND.
- Q18 (NPN): C 12V, B Q18_B, E FB_E. R94: FB_E ↔ GND.
- C23: − FB_E, + → R97 → Q12R_B.
- Chain: VCF_OUT → VR4A → C15 → R67 → Q18 follower → C23 → R97 → Q12R base.

### Exponential converter / cutoff
- Q9 (PNP): E VQ9, B Q9_B, C GND. R65: 5.333V ↔ Q9_B. R64: Q9_B ↔ GND.
- Q11 (NPN): C Q11_C (R69), B EXP_E, E VQ9.
- Q10 (PNP): E EXP_E, B EXP_B, C VQ9.
- R71: EXP_B ↔ VQ9.
- R73: EXP_E ↔ TM3_S. TM3: wiper + one end = TM3_S; other end = CUTOFF_W.
- CUTOFF_W: TM3 → y≈2203 → x≈3505 → up to y≈1697 → VR3 wiper.
- VR3: 3 = 12V, 2 = CUTOFF_W, 1 → R47 → VQ9.
- VR5: 3 = ENV_IN (x≈3903 riser, from the ENV block), 2 → R63 → EXP_B, 1 → R61 → VQ9.
- VQ9 bus connects: Q9 E, Q11 E, Q10 C, R71, R47 and R61.
  - Route: R47 riser x≈4100 meets R61 at a dot at y≈2219 → left to x≈3700 → down to y≈2390 → left.
  - The x≈4100 riser crosses VCF_OUT, RES_FB and the gang-B lines with no dots.

### Accent sweep (gang B)
- ACCENT_W = VR7 wiper (line y≈2547). D24 anode joins it with a dot at x≈3740.
- D24: A ACCENT_W, K → R46.
  - The D24 vertical crosses the VR5-wiper line (y≈2165) with no dot.
- R46 47K: D24 K ↔ VR4B-1 (line y≈2009).
- VR4B: 3 → C13+ (y≈1966); 2 → ACC_SWEEP (→ x≈3817 → down to y≈2333 → R72); 1 → R46.
- C13: + VR4B-3, − GND.
- R72: ACC_SWEEP ↔ EXP_B.

### Block interface
- In: VCF_IN (S1), ENV_IN (VR5-3, from ENV), ACCENT_W (VR7 wiper), 12V, 5.333V, GND.
- Out: VCF_OUT (→ VCA C21/R121, TP6), RES_FB (→ VCA C22 .01/R122).

## 3. Annotations, pages 7–9, errata

### Page 5 waveform sketch (crop (3640,1350)-(4100,1650))
- Ringing bursts with ring period "2ms ±.5ms" (marked between the first two peaks).
- Peak "5.8V", baseline between bursts "5.2V", and a "0" line.
- Burst spacing is marked "(15 ms KEY C : see ADJ. VCF)". A leader arrow points to the VCF_OUT line at y≈1837, just left of VR4A-3.
- "FREQ" is printed in a box beside TM3.

### p-07 adjustment
Check point TP6. Panel settings:
- CUTOFF FREQ: center
- WAVEFORM: SAWTHOOTH [sic]
- RES: full CW
- ENV MOD, DECAY, ACC: full CCW

"Afdjust [sic] TM3 for: 2ms ± 0.5ms".

### p-08 text and figures
The "VCF ENVELOPE MODULATION" text and Figs. 1–3 are quoted verbatim in vcf-A/vcf-B; the two readers agree. The relevant points:
- Q9 sets the "VQ9" bias for Q10/Q11.
- The VR5 wiper moving toward ③ lowers the cutoff.
- Fig. 2 topology matches this netlist: VR5-2 → R63 → Q10 B, VR5-1 → R61 → VQ9, Q10 C and Q11 E on VQ9.

### p-08 parts list
- Pots:
  - CUT OFF FREQ, ENV MOD: K161B0-50kA
  - RESONANCE: K162T00W-50kB x2
  - TUNING, ACCENT: K161B0-50KB
  - DECAY: K161B0-1MA
- Trimmers: 47kB, 220kB, 470kB (= TM3) and 4.7kB.
- Transistors: 2SC1583(F) Dual Tr, 2SC2291(F) Dual Tr, 2SA733(P), 2SB647(C), 2SB596(O), 2SD667(C), 2SC2021(R).

### p-09 errata (applied)
- RESONANCE pot part number 13219774 → **13219775** (K162T00W-50kB x2). Values are unchanged.
- Transistor list additions: 2SC2021(R) "or 2SD637(R)", **2SC536(F)**, **2SA937(Q) "or 2SB642(R)"**.
- Page-5 circuit corrections cover IC10 pins, C66 0.0022, C67 10/16 and Q42 2SC536. **None touch the VCF.**

## 4. Resolution log

| # | Item | A | B | Decision | Evidence (crop) |
|---|---|---|---|---|---|
| 1 | Ladder diode orientation/order | Q13/Q15 C+B on L4, E on L3 … Q16/Q23 E on L1 (C at the lower node) | Q16/Q23 C+B L1, E L2; Q14/Q17 L2→L3; Q13/Q15 L3→L4 | **B.** Each strapped C+B is on the upper node and the emitter arrow on the lower node. Q22 E, C26 and the Q16 strap are one continuous column node. | (2720,1410)-(3000,1750), (2720,1700)-(3000,2100) |
| 2 | Rung caps | C26 top, C24, C19, C18 bottom | same | **Confirmed.** C26 .033 = L1/R1 (Q22 E); C24 .033 = L2/R2; C19 .033 = L3/R3; C18 .018 = L4/R4 (Q12 C). | same |
| 3 | C25/C27 crossing | L1→C25→Q21R, R1→C27→Q21L | same | **Confirmed.** The y≈1554 line has a dot at the left column only and crosses the right column (x≈2928) with no dot. The y≈1594 C26 line has dots on both columns and continues to C27. Sign check: input on Q12L gives output in phase; the feedback on Q12R subtracts (negative feedback, as a ladder needs). | (2780,1530)-(2960,1640), (2900,1500)-(3330,1700) |
| 4 | Input target | C17 → Q12 left base | same | **Q12L base** (R70 to BIAS_IN). C17 + is on the R62 side. | (2560,1900)-(3000,2230) |
| 5 | TUNE line vs VCF_IN | not mentioned | possible T-junction | **Crossing, no dot.** It is a clean 4-way cross at (2665,1317). TUNE is a separate net. | (2640,1290)-(2690,1340) |
| 6 | Supply arrows | filled = 12V | "unlabelled V+" | **12V.** Labelled at IC16. Q18's collector arrow is filled (B said open). | (2300,400)-(3000,800), (3180,1960)-(3240,2110) |
| 7 | Q9 polarity | PNP ? | PNP | **PNP.** The arrow base edge is away from the bar and the apex points into it. | (2780,2280)-(2840,2360) |
| 8 | Q10 / Q11 / Q18 polarity | PNP / NPN / NPN | same | **Confirmed.** Q10 arrow points into the bar; Q11 and Q18 arrows point out. | (2920,2230)-(2970,2300), (2850,2220)-(2900,2290), (3180,1960)-(3240,2110) |
| 9 | C23 polarity | + R97 side ? | + R97 side | **+ on R97 side.** | (3040,2060)-(3200,2160) |
| 10 | VCA input order | VCF_OUT → R121 → C21; RES_FB → R122 → C22 | C21 → R121; C22 → R122 | **B.** From the VCF side: C21 .01 then R121 220K; C22 .01 then R122 100K. | (1950,2050)-(2600,2550) |
| 11 | D24 vs VR5 wiper line | no junction | no junction | **No dot** at (3740,2165). The dot is at (3740,2547) only. D24 anode is at the bottom. | (3700,2100)-(3780,2200), (3700,2510)-(3780,2580) |
| 12 | Gang B terminals | 1 → R46, 3 → C13 | same | **Confirmed.** R46 line y≈2009 (B said 1977). | (3550,1600)-(4450,2100) |
| 13 | TM3 strap | wiper to R73 end | same | **Confirmed.** Other end → CUTOFF_W. | (2900,2120)-(3600,2420) |
| 14 | Duplicate C22 | noted | noted | **Kept as printed.** Netlist uses "C22 (VCF) 1/50" and "C22 (VCA) .01". | — |
| 15 | VR7 terminal 1 | — | GND | **GND** (ground symbol at VR7-1). | overview (2500,1200)-(4500,2700) |

## 5. Open items

1. **TP6 location / DC levels.**
   - The badge has no wire. The only attachment evidence is the annotation arrow to VCF_OUT at VR4A-3.
   - The sketch gives a 5.2V baseline and 5.8V peak, which is inconsistent with VCF_OUT. VCF_OUT is after C14, and VR4A-1 is grounded, so its DC level is 0V.
   - Q19_E is estimated at about 10.7V DC (§6), which does not match 5.2V either.
   - Interpretations:
     - (a) TP6 = VCF_OUT, and the volt labels are relative or a drafting error.
     - (b) TP6 is a different node.
   - Consequence: only the 2 ms ring period (TM3 calibration target) is safe to use. Do not use the 5.2/5.8V levels for model calibration.
2. **Transistor types** for Q9, Q10, Q11, Q13–Q20 and Q23 are not printed.
   - The parts list offers 2SA733(P)/2SA937(Q) PNP and 2SC536(F)/2SC2021(R) NPN, plus the dual 2SC1583/2SC2291.
   - The mapping is not on the schematic. It affects Is/β matching of the ladder diodes and expo pair (tempco, offset of the expo law).
3. **Gang A / gang B wiper direction correlation.**
   - Both gangs are numbered 3/2/1 and linked by a dashed line. The drawing does not prove that the wipers move toward the same-numbered terminals together.
   - If they move together (assumed), RES full CW sends the gang-B wiper to terminal 3: the C13-smoothed accent ("sweep").
   - If reversed, RES max would deliver the unsmoothed accent via R46. The accent behaviour flips.
4. **ENV_IN and ACCENT_W sources** are outside this block (ENV/VCA transcription). Their DC rest levels determine the absolute VR5/VR4B offsets.

## 6. Model-relevant values (derived from the reconciled netlist)

DC estimates assume Vbe ≈ 0.65V, VT ≈ 25.9 mV (≈27 °C) and neglect base currents unless stated. They are estimates, not printed values.

**Supplies / references**
- VCF_RAIL = 12V − R95·I. The R95 (100 Ω) / C28 (47µ) filter corner is 1/(2π·100·47e-6) ≈ **33.9 Hz**.
- BIAS_Q22 ≈ VCF_RAIL (R115 10K carries only Q22 base current). R115·C30 = **100 ms**.
- BIAS_IN = 5.333V. The R68 2.2K / C16 10µ filter has τ = **22 ms** (7.2 Hz).
- Q9 base = 5.333·R64/(R65+R64) = **2.667V** (Thevenin 5K). Q9 is a PNP follower, so **VQ9 ≈ 2.667 + 0.65 ≈ 3.3V**. The PNP can only sink current, and all VQ9 loads (Q11 E, Q10 C, R71, R47, R61) feed current in.
- BIAS_DIFF and Q20_B: the 5.333V divider is R116+R112 = 20K over R109 = 10K.
  - Q20_B ≈ 5.333·10/30 = **1.78V**; BIAS_DIFF ≈ **3.56V**.
  - Q21 tail I ≈ (1.78−0.65)/R108 2.2K ≈ **0.51 mA** (≈0.26 mA per side).
- Q21R collector ≈ VCF_RAIL − 0.26 mA·2.2K ≈ rail − 0.56V. Q19_E ≈ rail − 1.2V ≈ **10.7V**; I(R96) ≈ 1.07 mA.
- Q18 bias:
  - BIAS_FB = 5.333V, through R66+R99 = 200K to Q18 B.
  - Q18 Ie ≈ (V_B − 0.65)/R94 10K ≈ 0.45 mA.
  - With β ≈ 200, the base drop is ≈ 0.45V, so Q18_B ≈ 4.9V and FB_E ≈ 4.2V (β-dependent).
- Ladder DC: the nodes step down by ≈1 Vbe per rung from BIAS_Q22. L1 ≈ rail − 0.65 … L4 ≈ rail − 2.6 ≈ 9.3V.
- Q12 E (IEXP) ≈ 5.333 − 0.65 ≈ **4.7V**.
  - Q11 collector = IEXP − I_ladder·R69 (2.2K), and Q11 E = VQ9 ≈ 3.3V.
  - Q11 stays out of saturation only for I_ladder ≲ (4.7 − 3.3 − 0.2)/2.2K ≈ **0.55 mA** (estimate).

**Signal-path gains and corners**
- **Input attenuation:** R70/(R62+R70) = 2.2K/222.2K = **0.0099 (−40.1 dB)** into Q12L base. BIAS_IN is AC-ground via C16.
- Input coupling C17 1µ·(220K+2.2K) → τ = 222 ms, **fc ≈ 0.72 Hz**.
- **Feedback injection:** R98/(R97+R98) = 2.2K/12.2K = **0.180** into Q12R base. This is 18.2× the input-path attenuation.
  - C23 1µ·12.2K → τ = 12.2 ms, **fc ≈ 13 Hz** (high-passes the resonance feedback).
- Q18 input coupling: C15 1µ into R67 + R99 = 102.2K (BIAS_FB AC-ground via C22) → τ ≈ 102 ms, fc ≈ 1.6 Hz. The VR4A wiper source resistance adds to this.
- **Resonance loop gain factor:** k_A (VR4A wiper fraction from terminal 1, 50K linear "B") × Q18 follower (≈1) × 0.180, relative to the 0.0099 signal input.
- **Ladder rung caps:** C26 = C24 = C19 = **33 nF**, C18 = **18 nF** (bottom, at Q12 collectors). Four poles in total. The Q22 emitters act as the top diode pair.
  - Small-signal per-pole formula (standard differential-ladder result, applied, not printed): ω_n ≈ I_ladder/(4·VT·C_n).
  - With I = Q11 collector current, f(33 nF) ≈ I/(2π·4·0.0259·33e-9) ≈ **46.5 kHz/mA**, and the C18 pole is ≈ **85.4 kHz/mA** (×1.83).
- Output coupling C25/C27 0.1µ into R113/R114 100K → τ = 10 ms, **fc ≈ 15.9 Hz**.
- Q21 single-ended gain ≈ gm·R110/2 with gm ≈ 0.26 mA/VT ≈ 10 mS → **≈ 11×** (estimate).
- C14 1µ into VR4A 50K ∥ R121 220K (via C21) ≈ 40.7K → fc ≈ 3.9 Hz.
- VCA-side couplings: C21 .01/R121 220K → 72 Hz corner; C22(VCA) .01/R122 100K → 159 Hz corner (load conditions per the VCA transcription).

**Exponential converter law**
- Q10 (PNP) is a follower from EXP_B to EXP_E, so Vbe(Q11) = V(EXP_B) − VQ9 + Vbe(Q10).
- Q10's emitter current ≈ I_R73 (the cutoff current), therefore:
  **I_ladder ≈ (Is11/Is10) · I_R73 · exp(ΔV/VT)**, with ΔV = V(EXP_B) − VQ9.
- Cutoff current:
  - I_R73 = (V_CUTOFF_W − VQ9 − Vbe11)/(R73 + R_TM3).
  - R73 = 100K. R_TM3 = 0…470K (TM3 = "FREQ" calibration).
  - V_CUTOFF_W is the VR3 wiper (50K, A/log taper) between 12V and VQ9 + R47 10K.
  - The cutoff knob therefore acts *linearly* on current (the log taper gives the quasi-exponential feel).
- ΔV is the sum of currents into EXP_B times R71 = 2.2K (R63/R72 ≫ R71):
  - ENV MOD: R71/R63 = 2.2K/220K = **0.0100** × (VR5 wiper − VQ9).
  - Accent sweep: R71/R72 = 2.2K/100K = **0.0220** × (VR4B wiper − VQ9).
  - Sensitivity: 1 octave per VT·ln2 ≈ 17.9 mV at EXP_B. That is ≈ 1.79V per octave at the VR5 wiper and ≈ 0.81V per octave at the VR4B wiper.
- The VR5 track runs from ENV_IN (③) to VQ9 via R61 (①).
  - With the wiper at ①, ENV MOD contributes ≈0.
  - Moving toward ③ pulls EXP_B toward the envelope's rest level. If that is below VQ9 (~3.3V), the cutoff drops, which agrees with p-08 Fig. 3.
- VR3 range: terminal 1 sits at VQ9 + R47 drop; terminal 3 at 12V.

**Accent sweep network**
- Charge: ACCENT_W → D24 → R46 47K → VR4B track (50K, from 1 to 3) → C13 1µ. τ_charge ≈ (47K + 50K)·1µ ≈ **97 ms**, plus the ACCENT pot source impedance and the D24 drop.
- Discharge: D24 blocks the reverse path. C13 discharges through the VR4B track (terminal 3 → wiper, (1−α)·50K) → R72 100K → R71 2.2K → VQ9. τ_discharge ≈ (102.2K + (1−α)·50K)·1µ ≈ **102–152 ms**, where α is the wiper fraction from terminal 3 to terminal 1.
- The tap point moves with RESONANCE (assumed gang correlation, Open item 3): RES max puts the wiper at C13 (smoothed sweep); RES min puts it at the R46/D24 node (direct, diode-gated accent).

**Calibration target (p-07)**
- Set CUTOFF center, RES max, ENV MOD/DECAY/ACC min. Adjust TM3 for a ring period of 2 ms ± 0.5 ms → **ring frequency ≈ 500 Hz (400–667 Hz)**.
- The 15 ms burst spacing matches the period of KEY C (C2 ≈ 65.4 Hz → 15.3 ms) with sawtooth.

**Pots (parts list + p-09)**
- VR3 CUTOFF 50kA (log)
- VR4 RESONANCE 50kB x2 (linear, part 13219775)
- VR5 ENV MOD 50kA (log)
- VR7 ACCENT 50kB (linear)
- TM3 470kB trimmer

## Memory hints (unverified)
Not used for any decision above. Reader A recalled published references listing the ladder devices as 2SC1583 pairs and Q9/Q10 as 2SA733 with a 2SC1815-class Q11. This page does not confirm the Q-number mapping.
