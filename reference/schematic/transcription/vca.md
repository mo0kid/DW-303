# TB-303 — VCA / VCA envelope / accent / mixer / outputs / headphone amp — RECONCILED

Inputs: `vca-A.md` (reader A) and `vca-B.md` (reader B). Arbitration was done against `pages/p5-05.png` (300 dpi;
all coordinates below are p5-05 pixels unless noted), page 2 of `TB-303.pdf` rendered at 200 dpi
(`/tmp/vcaR-p2-02.png`, BA662 and LA4140 pinouts), `p-08.png` (parts list) and `p-09.png` (errata).

Symbol conventions on p5. A filled up-arrow (▲) means **+12V**: the only labelled filled arrow is "12V" at IC16 pin 1
(≈2660,700). ⊕ means **+5.333V**: the boxed note reads "5.333V ⊕ connects to pin 7 of IC16 (POWER SUPPLY)". The
hollow triangle (△) appears twice in the PSU. One is labelled "+6V" (≈3694,330). The other is unlabelled
(≈3315,330) and fed from +6V through D2 (anode on the +6V side, cathode on the △ side, C1 100/6.3 to GND).
Hatched symbol = GND.

---------------------------------------------------------------------------------------------------
## 1. Component table (reconciled)

### VCA, VCA envelope generator, accent input
| Des | Value | Type / notes | Conf |
|---|---|---|---|
| IC15 | — | BA662A ("Vari-conductance amp", p8: BA662(A)). OTA plus a separate buffer amp in one 9-pin SIP | ✓ |
| Q31 | — | PNP. Type not printed at the symbol | ✓ sym |
| Q32 | — | NPN. Type not printed at the symbol | ✓ sym |
| R119 | 47K | | ✓ |
| R120 | 22K | | ✓ |
| R121 | 220K | | ✓ |
| R122 | 100K | | ✓ |
| R123 | 1.5M | | ✓ |
| R124 | 2.2K | | ✓ |
| R125 | 2.2K | | ✓ |
| R126 | 2.2K (printed "2.?K"; the 2nd digit is half blotted, and the surviving top stroke matches a "2") | | ? → resolved 2.2K |
| R127 | 47K | | ✓ |
| R128 | 220K | | ✓ |
| R129 | 22K | | ✓ |
| R130 | 100 | | ✓ |
| R131 | 220K | | ✓ |
| R132 | 100 | | ✓ |
| R133 | 2.2K | | ✓ |
| R134 | 22K | | ✓ |
| R137 | 1K | Gate-output path, shares node with D35 | ✓ |
| R162 | 2.2K | | ✓ |
| C21 | .01 | non-polar | ✓ |
| C22 | .01 | non-polar. **Duplicate designator**: another "C22 1/50" is printed in the VCF next to R66 | ✓ |
| C36 | .033 | non-polar | ✓ |
| C37 | 10/16 | electrolytic. + on the R124/R125/R126 node, − to GND | ✓ |
| C38 | 1/50 | electrolytic. + on IC15 pin 8, − to the VR8 pin 3 line | ✓ |
| C40 | 10/16 | electrolytic. + on the VB_VCA node, − to GND | ✓ |
| C41 | .1 | non-polar | ✓ |
| C42 | 1/35 (1 µF 35 V) | electrolytic. **+ on the Q31-base / R123 node, − to GND** | ✓ (resolved) |
| D27 | — | diode. **Duplicate designator**: another D27 sits in the ENV chain D28/D27/D26 | ✓ |
| D34 | — | diode (gate-output path) | ✓ |
| D35 | — | diode | ✓ |
| D36 | — | diode | ✓ |

### Mixer and line output
| Des | Value | Type / notes | Conf |
|---|---|---|---|
| Q33 | — | PNP | ✓ sym |
| Q34 | — | NPN | ✓ sym |
| R136 | 100 | | ✓ |
| R153 | 100K | | ✓ |
| R154 | 100K | | ✓ |
| R155 | 47K | | ✓ |
| R156 | 100K | | ✓ |
| R157 | 10K | | ✓ |
| R158 | 10K | | ✓ |
| R160 | 10K | | ✓ |
| R161 | 1K | | ✓ |
| R165 | 100K | | ✓ |
| C52 | 10/16 | + on Q34 base | ✓ |
| C53 | 47/16 | + on V_MIX | ✓ |
| C56 | 1/50 | + on MIX_OUT | ✓ |
| C58 | 1/50 | + on SUM, − on R156 side | ✓ |
| C59 | 1/50 | + on SUM, − on R155 side | ✓ |

### Headphone amp
| Des | Value | Type / notes | Conf |
|---|---|---|---|
| IC14 | — | LA4140 ("AF Power amp") | ✓ |
| R135 | **4.7K** (resolved; the K is partly eroded) | | ? → resolved |
| R159 | 33K | | ✓ |
| R163 | 10 | "10 X2" printed under R163/R164 | ✓ |
| R164 | 10 | | ✓ |
| C43 | 1000/6.3 | + on IC14 pin 6 | ✓ |
| C44 | 47/16 | + on pin 8 (BOOT), − on pin 6 | ✓ |
| C45 | .068 | pin 6 to GND | ✓ |
| C46 | .01 | BOOT to HP_RAIL | ✓ |
| C47 | .0068 | **between pin 4 and pin 1** (resolved) | ✓ |
| C48 | 100/6.3 | + on pin 9, − to GND | ✓ |
| C49 | 100/6.3 | + on HP_RAIL, − to GND | ✓ |
| C50 | 10/16 | + on pin 3 | ✓ |
| C51 | 2.2/50 | + on the R159 side, − on pin 2 | ✓ |
| D32 | — | anode GND, cathode pin 6 | ✓ |
| D33 | — | anode pin 6, cathode HP_RAIL | ✓ |

### Front panel and jacks
| Des | Value | Notes | Conf |
|---|---|---|---|
| VR7 | 50K(B) ACCENT | p8: K161B0-50KB "TUNING, ACCENT" (linear) | ✓ |
| VR8 | 50K(A) VOLUME | p8: VM11R851A-5M1411-50KA "master vol. & power switch" (audio/log taper). A dashed link gangs it to POWER SW | ✓ |
| **J1** | **MIX IN** | Mono jack. A normally-closed contact on the tip grounds the input when no plug is inserted | ✓ (resolved) |
| **J6** | **OUTPUT** | Mono jack, no switch | ✓ (resolved) |
| J2 | HEAD PHONE | Stereo jack (tip, ring, sleeve = GND) plus a switch group: ▼ contact, middle contact on △, ▽ contact | ✓ drawing / ✗ function |

### Supply items used (detail belongs to the PSU block)
- IC16 AN6562. Pin 7 = +5.333V (IC16B follower, set by TM6 4.7K; TP5). Pin 1 = +12V (IC16A, gain 1+R178/R179 = 1+2.2K/1.8K = 2.222).
- +6V = switched battery / adaptor rail (POWER SW ganged with VR8).
- Page 9 errata (applied; all are in the PSU, none touch this block): **C66 .001 → 0.0022**, **Q42 (unlabelled, next to Q43) = 2SC536**, **C67 2.2/50 → 10/16**, R170 terminal numbering re-drawn (T2 pins 3/2). Parts list: RESONANCE pot 13219774 → 13219775; transistor list adds 2SC536(F), 2SD637(R) as an alternative to 2SC2021(R), and 2SA937(Q) / 2SB642(R).

---------------------------------------------------------------------------------------------------
## 2. Netlist (reconciled)

### Nets entering or leaving the block
| Net | Source |
|---|---|
| +12V | ▲ arrows: R123 top, IC15 pin 9 (and the "4" label), R136 |
| +5.333V | ⊕: R126, R162, R157 |
| GATE_HI | Q36 collector (ENV block; PNP with emitter to +12V via R147 22, collector to GND via R145 10K) |
| VEG_TRIG | Q38 collector (ENV block; PNP with emitter at the +12V side via R150 22 / C55; collector to GND via R151 22K, also D37 anode) |
| ACC_SRC | IC12 (4066) pin 2. Switch 13/1/2: pin 1 = accent envelope source, pin 13 = control |
| VCF_OUT | C14 (1/50) − plate = VR4 section A pin 3 (RESONANCE pot top), vertical line x≈3540 |
| RES_WIPER | C15 (1/50) − plate = VR4 section A pin 2 (wiper); VR4A pin 1 = GND. Junction ≈3585,1893 |
| GATE (bus) | D34 cathode → GATE bus (y≈1157) → J4 GATE |

### VCA envelope generator
| Part | Terminals |
|---|---|
| R137 1K | N_GSUM ↔ D34 anode |
| D34 | A = R137, K = GATE bus |
| D35 | A = N_GSUM (= GATE_HI, Q36 collector), K = R133 |
| R133 2.2K | D35 K ↔ R131 top |
| R131 220K | R133 ↔ Q31 E |
| Q31 PNP | E = R131; B = VENV_C; C = IABC (IC15 pin 1) |
| R123 1.5M | +12V ↔ VENV_C |
| C42 1/35 | + = VENV_C, − = GND |
| R132 100 | VENV_C ↔ Q32 C |
| Q32 NPN | C = R132; B = Q32B; E = R130 → GND |
| R130 100 | Q32 E ↔ GND |
| C41 .1 | Q32B ↔ GND |
| R134 22K | D36 K ↔ Q32B |
| D36 | A = VEG_TRIG (Q38 collector), K = R134 |

### Accent into the VCA
| Part | Terminals |
|---|---|
| VR7 50K(B) | pin 3 = ACC_SRC (IC12 pin 2; wire y≈2430 → … → y≈2707 → VR7 pin 3); pin 2 = ACC_W; pin 1 = GND |
| ACC_W | VR7 wiper, R119 right. Also junction ≈3738,2547 to the D24 / R46 47K accent path in the VCF |
| R119 47K | ACC_W ↔ N_C36 |
| C36 .033 | N_C36 ↔ GND |
| R120 22K | N_C36 ↔ D27 A |
| D27 | A = R120, K = IABC |

### IC15 BA662A (pin functions from the page 2 pinout)
Page 2 pinout: 1 = C (gain control current), 2 = −IN1, 3 = +IN1, 4 = (no label), 5 = −V, 6 = OUT1 (OTA current
output), 7 = IN2 (buffer input), 8 = OUT2 (buffer output), 9 = +V.

| Pin | Function | Net |
|---|---|---|
| 1 | control current (I_ABC) | IABC: Q31 C, D27 K |
| 2 | −IN | R125 2.2K → VB_IN |
| 3 | +IN | VCA_IN: R124 2.2K → VB_IN; R121 220K – C21 .01 → VCF_OUT; R122 100K – C22 .01 → RES_WIPER |
| 4 | no label on the pinout | printed next to the +12V stem (see Resolution log) |
| 5 | −V | GND |
| 6 | OTA output | N_67 (= pin 7), R127 bottom, R128 bottom |
| 7 | buffer input | N_67 |
| 8 | buffer output | C38 + |
| 9 | +V | +12V |

- VB_IN: R124, R125, R126 (2.2K to +5.333V), C37 + (− to GND).
- VB_VCA: R162 (2.2K to +5.333V), C40 + (− to GND), R128 top, R129 top. A stray junction dot at ≈2135,1778 has no wire below it.
- R129 22K: VB_VCA ↔ N_127. R127 47K: N_127 ↔ N_67. R128 220K: VB_VCA ↔ N_67. So R128 is in parallel with (R129 + R127).
- C38 − → VCA_OUT → wire x≈2500, y≈2747 → VR8 pin 3.

### Volume, mixer, line output
| Part | Terminals |
|---|---|
| VR8 50K(A) | pin 3 = VCA_OUT; pin 2 = VOL_W; pin 1 = GND |
| R155 47K | VOL_W ↔ C59 − |
| C59 1/50 | − = R155, + = SUM |
| J1 MIX IN | tip = MIX_IN; NC contact tip → GND; sleeve GND |
| R156 100K | MIX_IN ↔ C58 − |
| C58 1/50 | − = R156, + = SUM |
| Q34 NPN | E = SUM; B = Q34B; C = Q34C |
| R157 10K | Q34B ↔ +5.333V |
| C52 10/16 | + = Q34B, − = GND |
| R154 100K | V_MIX ↔ Q34C |
| R158 10K | Q34C ↔ Q33 B |
| Q33 PNP | E = V_MIX; B = R158; C = MIX_OUT |
| R136 100 | +12V ↔ V_MIX |
| C53 47/16 | + = V_MIX, − = GND |
| R160 10K | MIX_OUT ↔ GND |
| R153 100K | MIX_OUT ↔ SUM (feedback) |
| C56 1/50 | + = MIX_OUT, − = R161 |
| R161 1K | C56 − ↔ LINE_OUT |
| R165 100K | LINE_OUT ↔ GND |
| J6 OUTPUT | tip = LINE_OUT (wire y≈3184 → x≈4031 up), sleeve GND |
| R159 33K | MIX_OUT ↔ C51 + |

### Headphone amp IC14 LA4140 (pin functions from the page 2 internal schematic)
From page 2: pin 7 = Vcc (top rail), 5 = GND, 6 = output, 8 = bootstrap (R13), 2 = input (TR2 base), 3 = NF
(TR3 side), 9 = ripple filter (bias chain), 1 and 4 = compensation nodes (TR9 base, TR12 base).

| Pin | Net |
|---|---|
| 1 | C47 right end (an unnumbered lead on the p5 symbol, between pins 5 and 3) |
| 2 | C51 − |
| 3 | C50 + (C50 − → R135 4.7K → GND) |
| 4 | C47 left end |
| 5 | GND |
| 6 | HP_OUT: C43 +, C44 −, C45 → GND, D32 K, D33 A |
| 7 | HP_RAIL: D33 K, C49 +, C46, plus the outer loop (x≈1696, y≈3264) → node ≈4094 → R163 and R164 |
| 8 | BOOT: C44 +, C46 |
| 9 | C48 + |

- C43 − → inner loop (x≈1735, y≈3224) → x≈4071 → J2 ▼ contact.
- R163 10 → J2 tip line. R164 10 → J2 ring. J2 sleeve = GND.
- J2 switch, as drawn: ▼ (on the C43− wire) rests on a middle contact. A △ supply drops onto that middle contact.
  A ▽ hangs from the middle contact down to the tip line (R163 side). See Open item 1.

---------------------------------------------------------------------------------------------------
## 3. Resolution log
| # | Item | A | B | Decision | Evidence |
|---|---|---|---|---|---|
| 1 | MIX IN / OUTPUT jack designators | J1 = MIX IN, J6 = OUTPUT | same | **J1 = MIX IN, J6 = OUTPUT, J2 = HEAD PHONE.** Each designator sits just above-right of its own jack body, and each function label sits to the right of the body. The brief's "MIX IN J6" is not supported | crop (4150,2850)-(4550,3320) ×2: J1 at y≈2895 over the 1st body; J6 at y≈3015 over the 2nd body ("OUTPUT"); J2 at y≈3175 |
| 2 | D36 anode source | Q38 collector | "Q38 emitter" | **Q38 collector.** Q38 is PNP with the emitter arrow on the upper (+12V/R150/C55) terminal. The lower terminal (R151 22K to GND, D37) carries the dot to the x≈1092 riser | crop (920,2600)-(1120,2800) ×4 |
| 3 | C22 source | C15 right plate → VR4 area | VR4A wiper | **Same net.** The C15 − plate wire ends in VR4A pin 2 (wiper). C22's riser joins it at ≈3585,1893. C21's riser x≈3540 joins the VR4A pin 3 line at y≈1840 | crop (3350,1650)-(4450,2600) |
| 4 | IC15 supply pin 9 vs 4 | ambiguous | probably 4 | **+12V on pin 9** (page 2 BA662 pinout: 9 = +V, 5 = −V, 4 unlabelled). On p5 one ▲ stem rises from the buffer top with "9" on its left and "4" on its right. Read this as pin 9 = V+, with pin 4 (no function given) drawn on the same stem, i.e. tied to +12V or NC. No model impact | p5 crop (2120,1960)-(2260,2100) ×5; p2 crop (640,220)-(1010,580) ×3 |
| 5 | IC15 input polarity | not marked | not marked | **Pin 3 = +IN (signal), pin 2 = −IN (bias only).** The VCA is non-inverting from VCA_IN | page 2 pinout |
| 6 | R126 value | 2.?K | 2.?K | **2.2K.** Pixel dump of the 2nd digit: the upper half has a top bar and a left curl matching the leading "2". The lower half is blotted out. Not "7" (no flat top / diagonal) | (1690,2185)-(1775,2215) pixel dump |
| 7 | R127/R129 junction | series R127+R129 ∥ R128 | same, with a dot above R127 and no wire | **No connection at the dot.** The column x 2131–2139, y 1782–1848 has only text pixels ("22K"/"47K"), no vertical stroke. R129's bottom joins R127's top via the horizontal at y≈1850. Topology: R128 ∥ (R129+R127) | pixel dump (2110,1765)-(2160,1860); crop (2040,1740)-(2260,2100) ×3 |
| 8 | C42 value / polarity | 1/35, + to Q31 base | 1/35 ("1" smudged), + top | **1 µF / 35 V, + on VENV_C (Q31 base / R123 / R132), − GND.** The "1" stroke and the "+" by the upper plate are both visible | crop (1800,1900)-(1920,2040) ×5 |
| 9 | R135 value | 4.7 Ω | 4.7K | **4.7K.** A 4th glyph follows "4.7". It is a K whose vertical stroke is partly eroded (compare R161's "1K", which has the full bar) | pixel dumps (2095,3020)-(2140,3075) vs (2350,3080)-(2385,3160) |
| 10 | C47 terminals | pin 4 ↔ unnumbered lead | pin 4 ↔ pin 3 | **Pin 4 ↔ pin 1.** C47's right riser (x≈2039) ends on the triangle's lower edge between "5" and "3". The pin 3 lead is the separate horizontal to C50. "1" is the only number missing from the symbol. Page 2 shows pins 1/4 are internal compensation nodes | crop (1900,2860)-(2150,3150) ×4; p2 crop (620,880)-(1200,1500) |
| 11 | VR7 pin 3 source | IC12 pin 2 | into ENV, not traced | **IC12 (4066) pin 2** (switch 13/1/2) | crop (1200,2250)-(1700,2800) |
| 12 | D34/R137 role | part of the VEG | context | Both readers agree on the topology. D34 points up to the GATE bus, so Q36's collector (GATE_HI) also drives the GATE out via R137/D34. Kept in the netlist | crop (1250,1450)-(2600,2350) |
| 13 | Unlabelled ▲ identity | assumed +12V | 12V | **+12V.** The only labelled filled arrow is "12V" | PSU crop (2550,200)-(4600,1150) |
| 14 | J2 / R163 / R164 / C43 wiring | as drawn, odd | as drawn, two readings | **Transcribed as drawn; function unresolved** → Open item 1. Re-zoom confirms both readers: the outer loop (HP_RAIL, y≈3264) feeds node ≈4094 → R163 (tip) and R164 (ring); the inner loop (C43−, y≈3224) feeds the ▼ contact | crop (4050,3080)-(4330,3300) ×4; (3750,2650)-(4400,3320) |
| 15 | Errata | PSU only | PSU only | Confirmed: the page 9 errata do not touch this block. PSU corrections are listed in §1 | p-09.png |

---------------------------------------------------------------------------------------------------
## 4. Open items
1. **Headphone output / J2 switch (open).** The page 2 internal schematic shows LA4140 **pin 7 is Vcc**. As drawn, pin 7's rail
   (HP_RAIL) has no supply except through J2. It goes to R163 / R164 → tip / ring. C43's output goes only to the ▼ switch contact,
   which rests on the △ supply contact.
   - (a) As drawn: when the ▽ contact closes, △ powers the amp through tip → R163 → pin 7. The headphone signal could then only
     appear as ripple on a C49-decoupled rail, and C43− would be tied to △. This is not a working audio path. If it is truly as
     built, the headphone level is ≈0 apart from rail modulation.
   - (b) Drafting swap of the two loop wires (x≈1696/1735 → y≈3264/3224). C43− → R163 / R164 (10 Ω each) → tip / ring (mono to both
     ears). HP_RAIL (pin 7) → switch contacts → △ supply, so the amp is powered only through the jack switch. This gives a
     normal LA4140 output stage: 1000 µF coupling, 10 Ω series per ear.
   - (c) The headphone is returned to Vcc rather than GND ("load returned to Vcc" topology). Nothing on p5 supports this.
   - **Consequence:** this affects only the headphone output. LINE OUTPUT (J6) and the mixer are unaffected. For modelling,
     use (b) if a headphone path is needed.
2. **Which △ rail feeds J2.** There are two hollow triangles in the PSU: "+6V" (switched battery/adaptor) and an unlabelled one
   after D2 (≈ +6V − V_D, held by C1 100µ). The J2 symbol does not say which. Both give ≈5.3–6 V. Headphone amp headroom
   differs by about one diode drop.
3. **IC15 pin 4.** It is unlabelled on the BA662 pinout but drawn on the +12V stem. Either tied to +12V or NC; no effect on a model.
4. **Transistor and diode types** (Q31–Q34, D27, D32–D36) are not printed at the symbols. The parts list gives only the
   population (2SA733(P), 2SC1583(F) dual, 2SC2021(R)/2SD637(R), 2SC536(F), 2SA937(Q)/2SB642(R); diodes 1S2473,
   1SS-133, 1S-188FM Ge, …). A model needs assumed β / Is.
5. **BA662A internal transfer** (g_m vs I_ABC, pin 1 DC voltage, buffer offset) is not given on p2/p5. The pin 1 voltage sets the
   headroom of the Q31 and D27 current sources (see §5).
6. **LA4140 closed-loop gain.** It depends on the internal feedback resistor R10 (no value on p2) against R135 4.7K / C50. Not derivable.
7. Duplicate designators on the drawing: C22 (VCA .01 vs VCF 1/50) and D27 (VCA vs ENV chain). Use block-qualified names.

---------------------------------------------------------------------------------------------------
## 5. Model-relevant values (derived only from the reconciled netlist)

### VCA envelope (VEG)
- **Recharge (decay) time constant:** τ_dec = R123·C42 = 1.5 MΩ × 1 µF = **1.50 s**. C42 charges toward +12V from ≈0 V after reset.
- **Reset (attack) discharge:** C42 discharges through R132 100 + Q32 (saturated) + R130 100, so τ_att ≈ 200 Ω × 1 µF ≈ **0.2 ms**
  (0.1 ms if Q32's emitter is taken as stiff). The reset lasts as long as the Q38 collector pulse (ENV block), stretched by C41.
- **Q32 base drive:** R134 22K into C41 .1 µF: τ = **2.2 ms**. Q32's base has no discharge resistor other than its own B-E path
  (R130 100 to GND), so C41 stretches the reset pulse.
- **Envelope-to-current conversion:** Q31's emitter is fed from GATE_HI (≈ +12V less Q36 Vce(sat); estimate ≈11.8 V) through D35 and
  R_E = R133 + R131 = **222.2 kΩ**. Q31's base = V_C42(t).
  I_ABC,env(t) ≈ [V_GATE_HI − V_D35 − V_EB31 − V_C42(t)] / 222.2 kΩ, with V_C42(t) = 12·(1 − e^(−t/1.5 s)) after reset.
  With V_GATE_HI ≈ 11.8 V and drops of ≈0.65 V each: I(0) ≈ **47 µA**. The current falls to zero when V_C42 ≈ 10.5 V, i.e. at
  t ≈ 1.5·ln(12/1.5) ≈ **3.1 s**. This is a truncated exponential: the current is ∝ (12·e^(−t/τ) − ~1.5 V).
  When the gate is low, GATE_HI ≈ 0 V (R145 10K to GND) and the envelope current is 0.
- The control current is set by the Q31 collector only. Pin 1's own voltage just needs to stay below Q31's base.

### Accent current into pin 1
- Accent source = IC12 pin 2 (switched accent envelope) → VR7 50K(B) (linear) top. The wiper is a 0…1 fraction.
- Smoothing: C36 .033 µF. With D27 off, τ = R119·C36 = 47K × 33 nF = **1.55 ms**. With D27 conducting, the Thevenin value
  is (R119 ∥ R120)·C36 = 14.97K × 33 nF = **0.49 ms**. This excludes VR7's wiper output impedance (≤12.5 kΩ, adding to R119).
- Current: I_acc ≈ [V_C36 − V_D27 − V_pin1] / R120 22 kΩ. The divider R120/(R119+R120) = 22/69 = **0.319** sets the steady-state
  V_C36 vs wiper voltage when D27 conducts (ignoring V_D and V_pin1).
- The same wiper also feeds the VCF accent path (D24 / R46 47K).

### VCA signal path
- **Input bias:** both OTA inputs sit at +5.333 V through 2.2K each (R124, R125) from VB_IN. VB_IN is filtered by R126 2.2K / C37 10 µF:
  τ = **22 ms**.
- **VCF output → pin 3 (+IN):** R121 220K, C21 .01, into ≈2.15 kΩ (R124 ∥ R122 path).
  Attenuation **0.00969 (−40.3 dB)**. HPF f_c = 1/(2π·222.2K·10 nF) ≈ **71.6 Hz**.
- **Resonance wiper → pin 3:** R122 100K, C22 .01, into ≈2.18 kΩ.
  Attenuation **0.0213 (−33.4 dB)**. HPF f_c = 1/(2π·102.2K·10 nF) ≈ **156 Hz**.
- Ratio of the two contributions per volt: 2.20 (wiper path) : 1 (VCF_OUT path). VR4A is 50K(B) linear (K162T00W-50kB ×2).
- Pin 2 has no signal, so the OTA sees v_diff = v(pin 3). The VCA is non-inverting.
- **OTA load:** R_L = R128 ∥ (R129 + R127) = 220K ∥ 69K = **52.5 kΩ** to VB_VCA. VB_VCA is AC ground via C40 10 µF, and DC is
  +5.333 V via R162 2.2K (τ = R162·C40 = **22 ms**). Output voltage swing = I_out × 52.5 kΩ around 5.333 V.
  (If the stray dot were a connection: R127 ∥ R128 = 38.7 kΩ, −2.7 dB. Not adopted.)
- **Buffer:** pin 6 → pin 7 (unity buffer per the pinout) → pin 8 → C38 1 µF → VR8 (50 kΩ). HPF ≈ 1/(2π·50K·1 µF) ≈ **3.2 Hz**.
  This is lower once the wiper loading is included.

### Volume and mixer
- VR8 50K(A): audio/log taper, ganged with POWER SW.
- The Q34 base is held at +5.333 V by R157 10K / C52 10 µF (τ = **0.1 s**). SUM ≈ 5.333 − V_BE ≈ 4.7 V acts as a virtual ground
  (Q34 common-base → Q33 PNP, with R153 100K feedback). MIX_OUT DC ≈ SUM + ~0.6 V (estimate).
- **Gains (inverting):** VCA path −R153/R155 = −100K/47K = **−2.13 (+6.6 dB)**. MIX IN −R153/R156 = **−1.00 (0 dB)**.
- Input coupling HPFs: C59 1 µF with R155 47K: **3.4 Hz**. C58 1 µF with R156 100K: **1.6 Hz**.
- MIX IN is grounded by J1's switch when no plug is inserted.
- Mixer supply: +12V via R136 100 with C53 47 µF (τ = 4.7 ms).
- **Line out:** MIX_OUT → C56 1 µF → R161 1K → J6, with R165 100K to GND. Output impedance ≈ 1 kΩ. HPF into an open circuit
  ≈ 1/(2π·101K·1 µF) ≈ 1.6 Hz. MIX_OUT is loaded by R160 10K to GND, plus R153 and the R159 headphone branch.

### Headphone amp
- Input: MIX_OUT → R159 33K → C51 2.2 µF → pin 2. The HPF corner depends on the pin 2 input resistance (not given).
- NF: R135 4.7K + C50 10 µF from pin 3 to GND: corner 1/(2π·4.7K·10 µF) ≈ **3.4 Hz**. Closed-loop gain is not derivable
  (Open item 6).
- C47 6.8 nF between compensation pins 1 and 4. Output pole C45 68 nF to GND. Bootstrap C44 47 µF. Ripple C48 100 µF.
  Rail decoupling C49 100 µF.
- Output (reading (b)): C43 1000 µF, then 10 Ω per ear (R163 / R164). With a 32 Ω phone, the corner is
  1/(2π·42 Ω·1000 µF) ≈ 3.8 Hz.

### Supply references
- +5.333 V: IC16 pin 7 (TP5, trimmed by TM6).
- +12 V: IC16 pin 1 = 5.333 × (1 + 2.2/1.8) = **11.85 V**.
- Headphone △ rail ≈ +6 V (Open item 2).

---------------------------------------------------------------------------------------------------
## 6. Memory hints (unverified; not used above)
- Commonly cited: Q31 = 2SA733 and Q32 = 2SC1815-class in the 303 VCA envelope; the BA662 is an LM13600-like OTA plus buffer.
- Both readers half-remembered the LA4140 as using a load returned to Vcc. This conflicts with the page 2 pinout reading used in
  Open item 1.
