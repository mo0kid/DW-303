# TB-303 VCF block — transcription (reader vcfB)

Source: p5-05.png (300 dpi). Coordinates given as source-pixel (x,y) where useful for cross-checking.
Also used: p-07.png (VCF adjustment), p-08.png (VCF envelope modulation text + parts list), p-09.png (errata).
Symbol conventions observed on the page:
- `⊕` (circle with cross/dot) = a supply connection point. The same symbol is labelled "5.333V" at IC9 (top-left
  of page 5), so `⊕` is taken as **+5.333V** here (inference from the symbol, not labelled locally).
- Filled/open up-arrow = a supply rail, voltage NOT labelled at the VCF (written below as `V+`, `V+_VR3`, `V+_Q18`).
- Ground = hatched ground symbol.

## 1. Component table

| Ref | Value as printed | Type | Conf. |
|---|---|---|---|
| Q22 | — | 2SC2291 dual (pair drawn in one envelope: Q22L, Q22R) NPN | ✓ |
| Q16 | — | NPN, single symbol, no type printed | ✓ |
| Q23 | — | NPN, single symbol, no type printed | ✓ |
| Q14 | — | NPN, no type printed | ✓ |
| Q17 | — | NPN, no type printed | ✓ |
| Q13 | — | NPN, no type printed | ✓ |
| Q15 | — | NPN, no type printed | ✓ |
| Q12 | — | 2SC1583 dual (Q12L, Q12R) NPN | ✓ |
| Q21 | — | 2SC1583 dual (Q21L, Q21R) NPN | ✓ |
| Q20 | — | NPN, no type printed | ✓ |
| Q19 | — | NPN, no type printed | ✓ |
| Q18 | — | NPN, no type printed | ✓ |
| Q11 | — | NPN, no type printed | ✓ |
| Q10 | — | **PNP** (arrow into bar on emitter), no type printed | ✓ (symbol) |
| Q9 | — | **PNP**, no type printed | ✓ |
| C26 | .033 | film (µF) | ✓ |
| C24 | .033 | film | ✓ |
| C19 | .033 | film | ✓ |
| C18 | .018 | film | ✓ |
| C27 | .1 | film | ✓ |
| C25 | .1 | film | ✓ |
| C30 | 10/16 | electrolytic 10µF/16V | ✓ |
| C28 | 47/16 | electrolytic 47µF/16V | ✓ |
| C29 | 1/50 | electrolytic 1µF/50V | ✓ |
| C14 | 1/50 | electrolytic | ✓ |
| C17 | 1/50 | electrolytic | ✓ |
| C16 | 10/16 | electrolytic | ✓ |
| C23 | 1/50 | electrolytic | ✓ |
| C15 | 1/50 | electrolytic | ✓ |
| C22 (VCF, near R66) | 1/50 | electrolytic | ✓ (see uncertainty: designator duplicated) |
| C13 | 1/50 | electrolytic | ✓ |
| R115 | 10K | | ✓ |
| R111 | 2.2K | | ✓ |
| R110 | 2.2K | | ✓ |
| R95 | 100 | | ✓ |
| R96 | 10K | | ✓ |
| R113 | 100K | | ✓ |
| R114 | 100K | | ✓ |
| R116 | 10K | | ✓ |
| R112 | 10K | | ✓ |
| R109 | 10K | | ✓ |
| R108 | 2.2K | | ✓ |
| R62 | 220K | | ✓ |
| R70 | 2.2K | | ✓ |
| R68 | 2.2K | | ✓ |
| R98 | 2.2K | | ✓ |
| R69 | 2.2K | | ✓ |
| R97 | 10K | | ✓ |
| R94 | 10K | | ✓ |
| R67 | 2.2K | | ✓ |
| R99 | 100K | | ✓ |
| R66 | 100K | | ✓ |
| R73 | 100K | | ✓ |
| R63 | 220K | | ✓ |
| R72 | 100K | | ✓ |
| R71 | 2.2K | | ✓ |
| R65 | 10K | | ✓ |
| R64 | 10K | | ✓ |
| R47 | 10K | | ✓ |
| R61 | 10K | | ✓ |
| R46 | 47K | | ✓ |
| D24 | — | diode, type not printed | ✓ |
| TM3 | 470K | trimmer, wired as rheostat (wiper tied to one end); box label "FREQ" | ✓ |
| VR3 | (no value printed on p5) | CUTOFF FREQ pot; parts list: K161B0-50kA | ✓ |
| VR4 | 50K(B) | RESONANCE, dual-gang (gang A + gang B, dashed mechanical link); parts list K162T00W-50kB x2 | ✓ |
| VR5 | 50K(A) | ENV.MOD | ✓ |
| VR7 | 50K(B) | ACCENT (only wiper/D24 relevant here) | ✓ |
| TP6 | — | test point label | ✓ |

## 2. Netlist

Net names: `RAIL_VCF` = filtered supply at top of ladder; `LAD_Ln`/`LAD_Rn` = ladder left/right column nodes,
n=1 top … 4 bottom; `BIAS_Q22` = Q22 base node; `BIAS_IN` = Q12 bias node; `BIAS_DIFF` = Q21 bias node;
`IEXP` = Q12 tail; `VQ9` = Q9 emitter node ("VQ9" in p8 Fig. 2); `EXP_E` = Q10 emitter/Q11 base;
`EXP_B` = Q10 base.

### Supply / decoupling
- R95 100: one end `V+` (filled supply arrow, voltage unlabelled, ~(3585,1430)); other end `RAIL_VCF`.
- C28 47/16: + `RAIL_VCF`, − GND.
- `RAIL_VCF` (long horizontal at y≈1434) also connects: R115 top, Q22L C, Q22R C, R111 top, R110 top, Q19 C.

### Ladder
- R115 10K: `RAIL_VCF` — `BIAS_Q22`.
- C30 10/16: + `BIAS_Q22`, − GND.
- Q22L (2SC2291): C `RAIL_VCF`, B `BIAS_Q22`, E `LAD_L1`.
- Q22R (2SC2291): C `RAIL_VCF`, B `BIAS_Q22`, E `LAD_R1`.  (Both bases tied together.)
- C26 .033: `LAD_L1` — `LAD_R1`.
- Q16: C+B (base strapped to own collector, i.e. diode-connected as drawn) `LAD_L1`, E `LAD_L2`.
- Q23: C+B `LAD_R1`, E `LAD_R2`.
- C24 .033: `LAD_L2` — `LAD_R2`.
- Q14: C+B `LAD_L2`, E `LAD_L3`.
- Q17: C+B `LAD_R2`, E `LAD_R3`.
- C19 .033: `LAD_L3` — `LAD_R3`.
- Q13: C+B `LAD_L3`, E `LAD_L4`.
- Q15: C+B `LAD_R3`, E `LAD_R4`.
- C18 .018: `LAD_L4` — `LAD_R4`.
- Q12L (2SC1583): C `LAD_L4`, B `VCF_IN_AC` (= Q12L base node), E `IEXP`.
- Q12R (2SC1583): C `LAD_R4`, B `Q12R_B`, E `IEXP`.
- R69 2.2K: `IEXP` — Q11 C.  (R69's vertical crosses the R98/R68 line without a dot.)

### Input (from WAVEFORM switch) and input bias
- S1 WAVEFORM common terminal (labelled with saw/square "⋀/⎍" symbol) → horizontal line y≈1275 → corner at
  x≈2665 → down to R62 top. A branch at the same corner runs left along y≈1317 to x≈2505 then up (destination
  outside block, not traced; the word "TUNE" is printed just right of this corner — see uncertainties).
- R62 220K: `WAVE_IN` — C17 +.
- C17 1/50: + R62, − `VCF_IN_AC` (Q12L base).
- R70 2.2K: `VCF_IN_AC` — `BIAS_IN`.
- R68 2.2K: `⊕` (+5.333V) — `BIAS_IN`.
- C16 10/16: + `BIAS_IN`, − GND.
- R98 2.2K: `BIAS_IN` — `Q12R_B`.
- R97 10K: `Q12R_B` — C23 +.
- C23 1/50: + R97, − `FB_E` (Q18 emitter).
- R94 10K: `FB_E` — GND.

### Output differential amplifier and buffer
- C25 .1: `LAD_L1` (via horizontal y≈1555, crossing `LAD_R1` column without dot) — Q21R B.
- C27 .1: `LAD_R1` (horizontal y≈1595 from right column, dot) — Q21L B.
- Q21L (2SC1583): C R111 bottom (`Q21L_C`), B `Q21L_B`, E `TAIL21`.
- Q21R (2SC1583): C `DIFF_OUT` (R110 bottom), B `Q21R_B`, E `TAIL21`.
- R111 2.2K: `RAIL_VCF` — `Q21L_C` (no other connection seen).
- R110 2.2K: `RAIL_VCF` — `DIFF_OUT`.
- R113 100K: `Q21L_B` — `BIAS_DIFF`.
- R114 100K: `Q21R_B` — `BIAS_DIFF`.
- R116 10K: `⊕` (+5.333V) — `BIAS_DIFF`.
- C29 1/50: + `BIAS_DIFF`, − GND.
- R112 10K: `BIAS_DIFF` — `Q20_B`.
- R109 10K: `Q20_B` — GND.
- Q20: C `TAIL21`, B `Q20_B`, E R108.
- R108 2.2K: Q20 E — GND.
- Q19: C `RAIL_VCF`, B `DIFF_OUT`, E `Q19_E`.
- R96 10K: `Q19_E` — GND.
- C14 1/50: + `Q19_E`, − `VCF_OUT`.
- `VCF_OUT`: vertical at x≈3540 down to y≈2467, then left to VCA block: → C21 .01 → R121 220K → IC15 (BA662A)
  input network (VCA block, not mine). Also branch (dot at (3540,1840)) right along y≈1840 → VR4 gang A
  terminal 3. TP6 label sits on this y≈1840 line.

### RESONANCE VR4 50K(B) dual gang and feedback path
- VR4 gang A: 3 = `VCF_OUT`; 1 = GND; 2 (wiper) = `RES_FB`.
- `RES_FB`: line y≈1895 → C15 − ; also dot at x≈3584 → down to y≈2507 → left to VCA block: C22 .01 → R122 100K →
  joins R121 end at IC15 input network (VCA block).
- C15 1/50: + `Q18_BASE_TOP` (R67 top), − `RES_FB`.
- R67 2.2K: C15 + — `Q18_B`.
- Q18: C `V+_Q18` (open up-arrow, unlabelled), B `Q18_B`, E `FB_E`.
- R99 100K: `Q18_B` — `BIAS_FB`.
- R66 100K: `⊕` (+5.333V) — `BIAS_FB`.
- C22 1/50: + `BIAS_FB`, − GND.
- So feedback: VCF_OUT → VR4A → C15 → R67 → Q18 (emitter follower) → C23 → R97 → Q12R base.
- VR4 gang B: 3 = `ACC_C13` (line y≈1968 → C13 +); 1 = R46 top (line y≈1977); 2 (wiper) = `RES_B_WIPER`
  (line down x≈3817 → y≈2333 → left → R72 → `EXP_B`).
- C13 1/50: + `ACC_C13`, − GND.
- R46 47K: VR4B terminal 1 — D24 cathode.
- D24: cathode R46, anode `ACCENT_WIPER` (vertical x≈3740 → dot y≈2547 → right to VR7 wiper 2, and left along
  y≈2547 toward ENV/VCA area, not traced).
- VR7 ACCENT 50K(B): 2 = `ACCENT_WIPER`, 1 = GND, 3 = line leaving left (accent source, other block).

### Cutoff control / exponential converter
- VR3 CUTOFF FREQ: 3 = `V+_VR3` (filled up-arrow, unlabelled); 2 (wiper) = `CUTOFF_W` (line y≈1697 left → x≈3505
  down → y≈2203 left) → TM3; 1 = R47.
- R47 10K: VR3 terminal 1 — `VQ9` (vertical x≈3700, dot at y≈2218, crosses VCF_OUT line without dot).
- TM3 470K ("FREQ"): one end + wiper (strapped) = R73 right end; other end = `CUTOFF_W`.
- R73 100K: TM3 — `EXP_E`.
- VR5 ENV.MOD 50K(A): 3 = `ENV` (vertical x≈3900 down past bottom, from envelope generator, other block);
  2 (wiper) = line → R63; 1 = R61.
- R61 10K: VR5 terminal 1 — `VQ9` (same node as R47 bottom).
- R63 220K: VR5 wiper — `EXP_B`.
- R72 100K: VR4B wiper — `EXP_B`.
- R71 2.2K: `EXP_B` — `VQ9`.
- Q10 (PNP): E `EXP_E`, B `EXP_B`, C `VQ9`.
- Q11 (NPN): C R69 bottom (ladder tail current), B `EXP_E`, E `VQ9`.
- Q9 (PNP): E `VQ9`, B `Q9_B`, C GND.
- R65 10K: `⊕` (+5.333V) — `Q9_B`.
- R64 10K: `Q9_B` — GND.
- `VQ9` node: Q9 E, Q11 E, Q10 C, R71, R47, R61 (bottom bus y≈2390 → right x≈3703 → up to y≈2218).

## 3. Annotations

On page 5 near VCF:
- Waveform sketch (top right of VCF): decaying ringing bursts; "2ms±.5ms" (period of ringing), levels
  "5.8V" (peak), "5.2V" (baseline), "0"; burst repetition marked "(15 ms KEY C : see ADJ. VCF)". An arrow from
  this note points to the VCF_OUT line at y≈1840 where TP6 label sits.
- Box "FREQ" next to TM3; "TM3" label.
- Block label "VCF".

Page 7 (adjustment), VCF:
"VCF / Check point: TP6 / Panel settings: / CUTOFF FREQ : center / WAVEFORM : SAWTHOOTH / RES : Full clockwise /
ENV MOD, DECAY, ACC : Full counterclockwise" — "Afdjust TM3 for:" [ringing waveform sketch] "2ms ± 0.5ms".
Page 7 package legend (no Q mapping): 2SC536F, 2SA733P, 2SC2021R, 2SA937Q.

Page 8 "VCF ENVELOPE MODULATION" (verbatim):
"In the conventional VCF configuration, envelope modulation voltages applied to modulation control voltage input of
VCF rise from a fixed potential to their values regardless of shape and amount of envelope and fall back to the
same potential. Fig. 1. In this setup, an attempt to deepen envelope modulation is increment of control voltage in
one direction, only to open up filter. Resultants are; higher the control voltage input, longer the period of time
cutoff point travels through higher frequency range in which significant aural sound characteristic changes do not
occur, whereas notes are brightened.
To compensate for the limitation this impose, there is a gimmick in the TB-303 VCF circuit. The Q9 has a key role
in this arrangement. See Fig. 2. It develops bias for Q10 and Q11 to set initial cutoff point at the center of
frequency range. Within the range even a small amount of voltage change can create a appreciable modulation."
"Rotating wiper ② of VR5 closer to terminal ③ increases ENV MOD voltage being fed to base of Q10 while changing the
bias which in turn lowers VCF cutoff frequency. See Fig. 3. This is equal to turning CUTOFF knob counterclockwise
and, since Q10 and Q11 in combination convert control voltage to the current in anti-log fashion, the cutoff point
stays in an effective frequency range for the most portion of envelope sweeping duration."
- Fig. 1: two decaying envelopes (large and small) starting from the same baseline.
- Fig. 2: Q11 NPN (collector up, emitter down), base → Q10 PNP emitter; Q10 collector and Q11 emitter tied →
  node → battery "VQ9" → ground; Q10 base → a resistor labelled "VR5" → VR5 wiper ②; VR5 ③ = envelope input
  (arrow from Fig. 1 into a terminal circle); VR5 ① → VQ9 node. (Matches p5 netlist: R63 between wiper and Q10 B,
  R61 between ① and VQ9.)
- Fig. 3: large envelope and baseline lowered, dashed level "VQ9" below; with deeper ENV MOD the starting level
  drops below the original so the envelope sweeps around the middle.

Parts list p8 / errata p9:
- CUT OFF FREQ, ENV MOD: K161B0-50kA (13219340). RESONANCE: K162T00W-50kB x2, part no. 13219774 corrected to
  13219775 (p9). TUNING, ACCENT: K161B0-50KB. DECAY: K161B0-1MA.
- Trimmers: H1051A016-47kB, H1051A021-220kB, H1051A023-470kB (= TM3 470K), H1021A011-4.7kB.
- Transistors: 2SC1583(F) Dual Tr, 2SC2291(F) Dual Tr; 2SA733(P), 2SB647(C), 2SB596(O), 2SD667(C), 2SC2021(R)
  (errata adds "or 2SD637(R)"), errata adds 2SC536(F) and 2SA937(Q) (or 2SB642(R)).
- p9 errata contains no correction to the VCF section of page 5.

## 4. Functional description (from this transcription)
Saw/square from S1 WAVEFORM is AC-coupled (R62 220K, C17) into Q12L base, biased from +5.333V via R68/R70 with
C16 decoupling. Q12 (2SC1583 pair) is the ladder's bottom differential pair, tail current set by R69 2.2K into
Q11 collector. Ladder: four capacitor rungs C18 .018 (bottom), C19 .033, C24 .033, C26 .033 (top) between left/
right columns of diode-connected (as drawn) NPN transistors Q13/Q15, Q14/Q17, Q16/Q23, topped by Q22 (2SC2291
pair) with bases at R115/C30 from RAIL_VCF (R95 100/C28 filtered). Top ladder nodes are AC-coupled by C25/C27 (.1)
crosswise to the Q21 differential amp (2SC1583, tail Q20 + R108, bases biased via R113/R114 100K from BIAS_DIFF),
single-ended output across R110 to emitter follower Q19, out via C14 to VCF_OUT (→ VCA via C21/R121, and TP6).
Resonance: VR4 gang A attenuates VCF_OUT; wiper → C15 → R67 → Q18 follower → C23 → R97 → Q12R base (with
R98 to BIAS_IN, R94 to GND). VR4A wiper also feeds VCA via C22 .01/R122. VR4 gang B (ganged) scales the accent
signal (VR7 wiper → D24 → R46 → VR4B, smoothed by C13) and feeds it via R72 100K into Q10 base (accent sweep).
Cutoff: VR3 wiper → TM3 470K → R73 100K → Q10E/Q11B; ENV MOD wiper → R63 220K → Q10 base; Q10 base returned
to VQ9 via R71 2.2K. Q9 (PNP follower from R65/R64 divider off +5.333V) sets VQ9, which is also the reference end
(terminal 1) of both VR3 (via R47) and VR5 (via R61). Q10/Q11 form the "anti-log" converter whose Q11 collector
current is the ladder tail current.

## 5. Uncertainties
1. Two different capacitors are both printed "C22": C22 1/50 (VCF, R66/R99 node, ~(3420,2090)) and C22 .01
   (VCA, with R122, ~(2120,2210)). Both readings are clear; schematic designator duplication.
2. Supply voltages for `V+` at R95, `V+_VR3` (VR3 terminal 3) and `V+_Q18` (Q18 collector) are not printed in the
   block. `⊕` = +5.333V is inferred from the identical symbol labelled 5.333V at IC9.
3. Ladder transistors Q13–Q17, Q23 and Q9, Q10, Q11, Q18, Q19, Q20 have no part type printed on page 5.
4. Ladder transistors are drawn with base strapped to collector (diode-connected) for every rung Q13–Q17/Q23;
   this is what the drawing shows — confirm against other reader.
5. TP6 is a floating label above the VCF_OUT line (after C14). The annotation waveform gives 5.2V baseline, which
   is inconsistent with a DC level after C14 with VR4A terminal 1 at GND — either TP6 is physically elsewhere
   (e.g. before C14 at Q19 emitter) or the voltage refers to something else. Unresolved.
6. WAVEFORM input net: a branch at the x≈2665 corner runs left (y≈1317) to x≈2505 and up; whether this is part of
   the same net (T-junction without dot) or a separate "TUNE" line drawn touching it is ambiguous. The "TUNE" text
   lies just right of that corner.
7. VR3 value not printed on page 5 (50kA from parts list).
8. D24 type not printed.

## Memory hints (unverified)
- Common TB-303 documentation describes the ladder transistors as 2SC1583 pairs and the bottom-rung cap as .018 vs
  .033 others — consistent with what is read here; nothing here was taken from memory.
