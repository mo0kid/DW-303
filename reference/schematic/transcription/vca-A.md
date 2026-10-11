# TB-303 transcription — VCA / VCA-envelope / Mixer / Headphone / Output / supply rails (reader vcaA)

Source: p5-05.png (300 dpi main board), p-08.png (parts list), p-09.png (errata), p-07.png (nothing VCA-specific).
Coordinates below (x,y) are p5-05.png pixels, for cross-checking. Wires were followed visually and with a
straight-line pixel tracer; wire crossings without a dot are treated as NOT connected.

Symbol conventions observed on the page
- Filled arrow (▲) = positive supply. The only filled arrow with a printed label is "12V" at IC16 pin 1 (≈2660,700).
  All unlabelled ▲ in this block are written `+V▲` (most likely +12V — see uncertainties).
- Open triangle (△) = the "+6V" symbol in the POWER SUPPLY block (labelled "+6V" at ≈3350,370). Written `+6V△`.
- Circle-with-cross (⊕) = 5.333V reference; boxed note on the page: "5.333V ⊕ connects to pin 7 of IC16 (POWER SUPPLY)".

---------------------------------------------------------------------------------------------------
## 1. Component table

### VCA (IC15) and VCA envelope generator
| Des | Value as printed | Type | Conf |
|---|---|---|---|
| IC15 | — | BA662A (printed "IC15 BA662A"; parts list "BA662(A) Vari-conductance amp") | ✓ |
| Q31 | — | PNP (emitter arrow points in). Type not printed near symbol | ✓ symbol / ✗ type |
| Q32 | — | NPN. Type not printed | ✓ / ✗ type |
| R119 | 47K | resistor | ✓ |
| R120 | 22K | resistor | ✓ |
| R121 | 220K | resistor | ✓ |
| R122 | 100K | resistor | ✓ |
| R123 | 1.5M | resistor | ✓ |
| R124 | 2.2K | resistor | ✓ |
| R125 | 2.2K | resistor | ✓ |
| R126 | "2.?K" (second digit damaged; looks like 2.2K) | resistor | ? |
| R127 | 47K | resistor | ✓ |
| R128 | 220K | resistor | ✓ |
| R129 | 22K | resistor | ✓ |
| R130 | 100 | resistor | ✓ |
| R131 | 220K | resistor | ✓ |
| R132 | 100 | resistor | ✓ |
| R133 | 2.2K | resistor | ✓ |
| R134 | 22K | resistor | ✓ |
| R137 | 1K | resistor | ✓ |
| R162 | 2.2K | resistor | ✓ |
| C21 | .01 | non-polar | ✓ |
| C22 | .01 (VCA one; NOTE a second "C22 1/50" is printed in the VCF next to R66/R99 — duplicate designator on the drawing) | non-polar | ✓ |
| C36 | .033 | non-polar | ✓ |
| C37 | 10/16 | electrolytic, + to R124/R125 node | ✓ |
| C38 | 1/50 | electrolytic, + toward IC15 pin 8 | ✓ |
| C40 | 10/16 | electrolytic, + to bias node | ✓ |
| C41 | .1 | non-polar | ✓ |
| C42 | 1/35 | electrolytic, + to Q31 base node | ✓ (polarity ?) |
| D27 | — (VCA one; NOTE a second "D27" is printed in the ENV block chain D28/D27/D26 — duplicate designator) | diode | ✓ |
| D34 | — | diode | ✓ |
| D35 | — | diode | ✓ |
| D36 | — | diode | ✓ |

Out-of-block parts whose terminals feed this block (named only): Q36 (PNP, ENV block, R142 10K, R143 10K, C72 10/16, R145 10K, R147 22), Q37, Q38 (PNP), IC12 (4066) pins 1/2/13, VR7.

### Mixer
| Des | Value | Type | Conf |
|---|---|---|---|
| Q33 | — | PNP (arrow into emitter). Type not printed | ✓ / ✗ type |
| Q34 | — | NPN. Type not printed | ✓ / ✗ type |
| R136 | 100 | resistor | ✓ |
| R153 | 100K | resistor | ✓ |
| R154 | 100K | resistor | ✓ |
| R155 | 47K | resistor | ✓ |
| R156 | 100K | resistor | ✓ |
| R157 | 10K | resistor | ✓ |
| R158 | 10K | resistor | ✓ |
| R160 | 10K | resistor | ✓ |
| R161 | 1K | resistor | ✓ |
| R165 | 100K | resistor | ✓ |
| C52 | 10/16 | electrolytic, + to Q34 base | ✓ |
| C53 | 47/16 | electrolytic, + to Q33-emitter rail | ✓ |
| C56 | 1/50 | electrolytic, + to mixer output node | ✓ |
| C58 | 1/50 | electrolytic, + to Q34 emitter side, "−" printed on R156 side | ✓ |
| C59 | 1/50 | electrolytic, + to Q34 emitter side, "−" printed on R155 side | ✓ |

### Headphone amp
| Des | Value | Type | Conf |
|---|---|---|---|
| IC14 | — | LA4140 (printed "IC14 LA4140"; parts list "AF Power amp") | ✓ |
| C43 | 1000/6.3 | electrolytic, + to IC14 pin 6 node | ✓ |
| C44 | 47/16 | electrolytic, + to pin-8 node | ✓ |
| C45 | .068 | non-polar | ✓ |
| C46 | .01 | non-polar | ✓ |
| C47 | .0068 | non-polar | ✓ |
| C48 | 100/6.3 | electrolytic, + to pin 9 | ✓ |
| C49 | 100/6.3 | electrolytic, + to top rail | ✓ |
| C50 | 10/16 | electrolytic, + to pin 3 | ✓ |
| C51 | 2.2/50 | electrolytic, + on R159 side | ✓ (polarity ?) |
| R135 | 4.7 (printed "4.7" with no K; 4.7 Ω) | resistor | ✓ |
| R159 | 33K | resistor | ✓ |
| R163 | 10 ("10 x2" printed under R164 applies to R163/R164) | resistor | ✓ |
| R164 | 10 | resistor | ✓ |
| D32 | — | diode | ✓ |
| D33 | — | diode | ✓ |

### Front-panel / jacks
| Des | Value | Notes | Conf |
|---|---|---|---|
| VR7 | 50K(B) "ACCENT" | parts list: K161B0-50KB "TUNING, ACCENT" | ✓ |
| VR8 | 50K(A) "VOLUME" | parts list: VM11R851A-5M1411-50KA "master vol. & power switch"; dashed mechanical link from VR8 to POWER SW | ✓ |
| J1 | "MIX IN" | switched jack (tip NC contact to GND) | ✓ |
| J6 | "OUTPUT" | plain jack | ✓ |
| J2 | "HEAD PHONE" | two tips (stereo) + switch contact pair | ✓ |

Label placement: every jack designator is printed above-right of its jack (J1 over MIX IN, J6 over OUTPUT, J2 over
HEAD PHONE, also J3/CV, J4/GATE). So **J1 = MIX IN, J6 = OUTPUT** (the task brief said "MIX IN jack J6" — the drawing
does not support that; see uncertainties). Parts list jacks: HLJ0305-01-030 MONO, HLJ0305-01-010 STEREO, SG8026 MINI.

### Power supply items used by these stages
| Des | Value | Type | Conf |
|---|---|---|---|
| IC16 | — | AN6562 dual op amp | ✓ |
| R172 | 100 | | ✓ |
| R173 | 1K | | ✓ |
| R174 | 5.6K | | ✓ |
| R176 | 6.8K | | ✓ |
| R177 | 6.8K | | ✓ |
| R178 | 2.2K | | ✓ |
| R179 | 1.8K | | ✓ |
| R180 | 39K | | ✓ |
| TM6 | 4.7K trimmer, flagged "CV" adjust | | ✓ |
| D43 | 1SZ46A (zener) | | ✓ |
| C60 | 10/16 | | ✓ |
| C61 | 10/16 | | ✓ |
| C68 | 47/16 | | ✓ |
| C69 | 47/16 | | ✓ |
| C70 | 10/16 | | ✓ |
| D41 | — | rectifier from T2 pin 2 | ✓ |
| T2 | MC102 (parts list MC102C "DC-DC converter") | | ✓ |
| Q43 | 2SD667C | | ✓ |
| Q42 | unlabelled on p5 → errata: 2SC536 | | ✓ (errata) |
| Q44 | "2SB64‾(C)" (last digit obscured; parts list has 2SB647(C)) | PNP | ? |
| Q45 | 2SB596(O) | | ✓ |
| R166 | 100K | | ✓ |
| R167 | 1.5K | | ✓ |
| R168 | 2.7 FRNB¼W (fuse resistor) | | ✓ |
| R169 | 1.5K | | ✓ |
| R170 | 22 (pin numbering corrected by errata) | | ✓ |
| R171 | 3.3K | | ✓ |
| D40 | RD6.8EB3 (printed "D4C", arrow from "RD6.8EB3") | zener | ? designator |
| D42 | RD15EB3 | zener | ✓ |
| C63 | 100/16; C64 10/16; C65 10/16 | | ✓ |
| C66 | .001 → errata 0.0022 | | ✓ |
| C67 | 2.2/50 → errata 10/16 | | ✓ |
| C1 / D2 | 100/6.3 / — (backup supply off +6V) | | ✓ |
| POWER SW, J7 (adaptor jack, "9VDC"), battery "1.5V X4" (terminals 29/30), D38, D39 | | | ✓ |

---------------------------------------------------------------------------------------------------
## 2. Netlist

### Supply / reference nets
- `+V▲` : filled-arrow supply (assumed +12V). Used at: R123 top, IC15 pin 9/4 arrow, R136 top, R147 (Q36 emitter, ENV).
- `+12V` : IC16 pin 1 output node: IC16-1, R178 (one end), C61 + (other end GND), labelled "12V ▲".
- `+5.333V` (⊕) : IC16 pin 7 output = IC16 pin 3 = C60 + (C60 − GND) = R177 one end = TP5. Used in this block by R162, R126, R157.
- `+6V△` : open triangle; at J2 switch contact (see headphone).
- `GND`.

### IC16 (AN6562) regulator section (as drawn)
- IC16A: pin 3 (+) = +5.333V; pin 2 (−) = R178 2.2K to pin 1 and R179 1.8K to GND; pin 1 = +12V rail (C61 10/16 to GND).
  (Gain 1+2.2/1.8 = 2.22 → 5.333 × 2.22 ≈ 11.85 V, consistent with "12V" label.)
- IC16B: pin 5 (+) = R180 39K top / TM6 bottom; pin 6 (−) = R177 6.8K from pin 7 (follower); pin 7 = +5.333V.
- Divider: node RAW_Z (R176 6.8K bottom, D43 1SZ46A cathode (anode GND), R174 5.6K top) → R174 → TM6 4.7K (wiper strapped to its top end) → pin 5 node → R180 39K → GND.
- R176 top = C70 + (10/16 to GND) = R173 1K → RAW (D41 cathode / C68 47/16 node, from T2 pin 2 via D41).
- R172 100 from RAW → C69 47/16 node → vertical line down (presumably IC16 V+ supply; pin not numbered).

### VCA-envelope
| Part | Terminal → net |
|---|---|
| D34 | cathode → `GATE` bus (line runs from D34 up to y≈1356, right to x≈2027, up to the GATE bus y≈1157 that is labelled "GATE"); anode → `N_GSRC` |
| R137 1K | `N_GSRC` (top, D34 anode side) ↔ `N_GSUM` |
| `N_GSUM` | R137 bottom, D35 anode, Q36 collector (ENV block; line ≈x1294→1040, down to Q36) |
| D35 | anode `N_GSUM`, cathode → `N_E31` |
| R133 2.2K | `N_E31` ↔ `N_R131` |
| R131 220K | `N_R131` ↔ Q31 emitter |
| Q31 (PNP) | E → R131; B → `N_C42`; C → `VCA_CTRL` (IC15 pin 1) |
| R123 1.5M | `+V▲` ↔ `N_C42` |
| C42 1/35 | + `N_C42`, − GND |
| R132 100 | `N_C42` ↔ Q32 collector |
| Q32 (NPN) | C → R132; B → `N_B32`; E → R130 100 → GND |
| R134 22K | `N_B32` ↔ D36 cathode |
| C41 .1 | `N_B32` ↔ GND |
| D36 | cathode R134; anode → `TRIG` = Q38 collector (ENV block; Q38 collector also feeds D37/R152 to C62 in ENV) |

Upstream of TRIG/N_GSUM (ENV block, recorded for context only): connector pin "9" → R146 22K → Q37 base (R144 10K to GND), Q37 emitter GND, Q37 collector → R143 10K → Q36 base node (R142 10K and C72 10/16 to Q36-emitter rail, Q36 emitter → R147 22 → +V▲) and → C54 .047 → Q38 base (R148 10K, R149 10K / R150 22, C55 47/16 on Q38 emitter rail).

### Accent path into VCA
| Part | Terminal → net |
|---|---|
| VR7 50K(B) ACCENT | pin 3 ← IC12 (4066) pin 2 (line y≈2430→2708→ right to 4058 → up to VR7 pin 3); pin 2 (wiper) → `ACC_W`; pin 1 → GND |
| `ACC_W` | VR7 wiper, R119 (line y≈2546 → x≈2540 → up to R119), plus a dotted junction at x≈3736 going up into the VCF area |
| R119 47K | `ACC_W` ↔ `N_C36` |
| C36 .033 | `N_C36` ↔ GND |
| R120 22K | `N_C36` ↔ D27 anode |
| D27 (VCA) | anode R120, cathode → `VCA_CTRL` |

### VCA (IC15 BA662A)
| Pin | Net |
|---|---|
| 1 | `VCA_CTRL` (Q31 collector + D27 cathode) |
| 2 (inverting? upper input) | R125 2.2K → `N_BIASIN` |
| 3 (lower input) | `VCA_IN`: R124 2.2K → `N_BIASIN`; R121 220K–C21 .01 series → `VCF_OUT_A`; R122 100K–C22 .01 series → `VCF_OUT_B` |
| 5 | GND |
| 6 (amp output) | tied to pin 7 → `N_67` |
| 7 (buffer input) | `N_67` |
| 8 (buffer output) | C38 1/50 (+) → `VCA_OUT` |
| 9 / 4 | supply arrow `+V▲`; arrow stem drawn between labels "9" and "4" on the buffer triangle's top edge (see uncertainties) |

- `N_BIASIN`: R124 bottom, R125 bottom, C37 + (C37 − GND), R126 "2.?K" → ⊕ +5.333V.
- Output bias/feedback: `N_67` → R128 220K → `N_BIASOUT`; and `N_67` → R127 47K → R129 22K → `N_BIASOUT` (R127+R129 series, in parallel with R128).
- `N_BIASOUT`: C40 + (− GND), R162 2.2K → ⊕ +5.333V.
- `VCF_OUT_A` = right plate of C14 (1/50, + on Q19-emitter side) in the VCF; same net continues right (junction ≈3542,1841) to VR4 RESONANCE area (≈4182,1811).
- `VCF_OUT_B` = right plate of C15 (1/50, + on R67/Q18 side) in the VCF; junction ≈3585,1896 continues right to VR4 area (≈4303,1891).
- Wire path C21: y≈2152 → x≈2460 down → y≈2468 right → x≈3540 up → C14. Wire path C22: y≈2215 → x≈2420 down → y≈2508 right → x≈3582 up → C15 line.
- `VCA_OUT`: C38 (−) → line right to x≈2500, down to y≈2747, right → VR8 pin 3.

### Volume / mixer
| Part | Terminal → net |
|---|---|
| VR8 50K(A) VOLUME | pin 3 ← `VCA_OUT`; pin 2 (wiper) → `VOL_W` (line y≈2809 → x≈3950 down → y≈3104 left → x≈3055 up → y≈3025 → R155); pin 1 → GND; mechanically ganged (dashed line) with POWER SW |
| J1 MIX IN | tip → `MIX_IN` (line y≈2907 → x≈3992 down → y≈3143 left → x≈3016 up → y≈3106 → R156); NC switch contact on tip → sleeve → GND (input grounded when no plug) |
| R155 47K | `VOL_W` ↔ C59 (−) ; C59 (+) → `SUMJ` |
| R156 100K | `MIX_IN` ↔ C58 (−) ; C58 (+) → `SUMJ` |
| `SUMJ` | Q34 emitter, R153 100K, C58 +, C59 + |
| Q34 (NPN) | E `SUMJ`; B → `N_B34`; C → `N_C34` |
| `N_B34` | R157 10K → ⊕ +5.333V; C52 10/16 (+ at N_B34) → GND |
| `N_C34` | R154 100K → `MIXV`; R158 10K → Q33 base |
| R136 100 | `+V▲` ↔ `MIXV` |
| `MIXV` | R136, R154 top, Q33 emitter, C53 47/16 + (− GND) |
| Q33 (PNP) | E `MIXV`; B via R158 → `N_C34`; C → `MIX_OUT` |
| `MIX_OUT` | Q33 collector, R160 10K → GND, R153 100K → `SUMJ`, R159 33K (→ headphone), C56 + |
| C56 1/50 | + `MIX_OUT`, − → R161 1K → `LINE_OUT` |
| `LINE_OUT` | R161, R165 100K → GND, line y≈3182 right → x≈4031 up → y≈3028 → J6 OUTPUT tip |
| J6 OUTPUT | tip `LINE_OUT`, sleeve GND |

### Headphone amp (IC14 LA4140) — drawn with output apex at left
| Pin | Net |
|---|---|
| 2 | C51 2.2/50 → R159 33K → `MIX_OUT` (input) |
| 3 | C50 10/16 (+ at pin) → R135 4.7 → GND |
| 4 | C47 .0068 → `IC14_X` |
| unnumbered lead (`IC14_X`) | short lead at lower edge of triangle right of pin 5, only goes to C47 (pin 1 is the only number not printed) |
| 5 | GND |
| 6 (apex) | `HP_OUT6`: C43 1000/6.3 (+), C44 47/16 (−), C45 .068 → GND, D32 cathode (D32 anode GND), D33 anode |
| 7 | `HP_RAIL` |
| 8 | `HP_BOOT`: C44 (+), C46 .01 → `HP_RAIL` |
| 9 | C48 100/6.3 (+) → GND |

- `HP_RAIL`: IC14 pin 7, D33 cathode, C49 100/6.3 (+, − GND), C46 left plate; continues left (x≈1693) down and along y≈3264 to J2 area (x≈4096) → R163 10 and R164 10 → the two J2 tips.
- C43 (−) → `HP_C43N` (x≈1736 down, y≈3222 right, x≈4072 up, y≈3152) → filled switch contact (▼) in J2.
- J2 switch: ▼ contact (on `HP_C43N`) mates with ▽ contact on a line from `+6V△`. The `+6V△` vertical crosses the `HP_C43N` horizontal with NO junction dot.
- J2 sleeve → GND.
- As drawn this puts R163/R164 on the D33-cathode/C49 rail and C43 on the +6V switch contact — see uncertainties.

### Power switching (context)
POWER SW (on VR8) between Q44 2SB64?(C) area and J7; J7 = adaptor jack (9VDC input, switching contacts), battery 1.5V×4 at terminals 29/30, D38/D39 steering; +6V → T2 MC102 DC-DC (Q43 2SD667C, Q42) → D41 → RAW → R173/R176/D43 → IC16.

---------------------------------------------------------------------------------------------------
## 3. Printed annotations and page 7–9 items
- Box near VCA: "5.333V ⊕ connects to pin 7 of IC16 (POWER SUPPLY)".
- "12V" (filled arrow) at IC16 pin 1; "5.333V ⊕" at IC16 pin 3/7 with "(TP5)" arrow; "CV KEY C=1.000V, C̄=4.000V" next to TP5; "CV" box arrow to TM6.
- "+6V" open triangle in POWER SUPPLY; "9VDC"; "1.5 V X4"; "POWER SW"; "FRNB¼W" on R168.
- "DECAY VR MAX T=2.5sec, MIN T=200ms" with decay-curve sketch (100%/10%/T), left of HEADPHONE — belongs to ENV/VR6 (VCF env), not VCA.
- Parts list p.8: IC BA662(A) "Vari-conductance amp"; LA4140 "AF Power amp"; AN6562 "Dual op amp"; pots: VM11R851A-5M1411-50KA "master vol. & power switch"; K161B0-50KB "TUNING, ACCENT"; K161B0-50kA "CUT OFF FREQ, ENV MOD"; K161B0-1MA "DECAY"; K162T00W-50kB×2 "RESONANCE"; FRNB 2.7Ω "Fuse resistor"; MC102C "DC-DC converter"; diodes 1S2473, S-5500G, 1S-188FM (Ge), RD6.8EB-3/RD6.8EB2-T, RD15EB-3, 1SS-133, 158-46A (presumably 1SZ46A as printed on p5).
- Errata p.9, "2. CIRCUIT DIAGRAM (page 5)", power-supply inset: **C66 .001 → 0.0022**; **Q42 (unlabelled transistor next to Q43) → 2SC536**; **C67 2.2/50 → 10/16**; **R170 pin numbering corrected** (magnified inset re-labels the R170/T2 terminals "3" and "2"; exact original-vs-corrected assignment hard to read). Left inset corrects IC10 (4050) pin numbering (3→2 / 14→15) — VCO/DAC block, not this one.
- Errata p.9, "1. (page 1)": RESONANCE pot K162T00W-50kB×2 part no. 13219774 → 13219775; front-panel pot callouts K161B0-50kB / -50kA / -1MA.
- Errata p.9, "3. PARTS LIST (page 8)": Sub panel N-277 2221027700 → 2221027701; potentiometer 13219774 → 13219775 (K162T00W-50KB×2); transistor list adds 2SD637(R) as alternative to 2SC2021(R), adds 2SC536(F) and 2SA937(Q) (or 2SB642(R)).
- p.7: VCF adjust note (TP6, TM3) only; nothing for VCA/mixer/headphone. Transistor outline legend: 2SC536F, 2SA733P, 2SC2021R, 2SA937Q.

---------------------------------------------------------------------------------------------------
## 4. Functional description (from this transcription only)
- VCA envelope: Q32 (NPN) is fired through D36/R134 (C41 .1 on base) by the TRIG pulse from Q38's collector; when on it discharges timing cap C42 (1µ) through R132 100 Ω. C42 then recharges from +V via R123 1.5 MΩ (τ ≈ 1.5 s). Q31 (PNP) has its base on C42 and its emitter fed from `N_GSUM` (Q36 collector; also tied through R137 1K/D34 to the GATE bus) via D35, R133 2.2K, R131 220K. Its collector current drives IC15 pin 1 (gain control). As C42 charges, Q31 emitter–base voltage falls and the control current decays → exponential VCA decay restarted by each trigger.
- Accent: the 4066-switched accent envelope (IC12 pin 2) passes through ACCENT pot VR7 and R119 47K / C36 .033 / R120 22K / D27 into the same IC15 pin 1 node, adding current.
- VCA: BA662A, both inputs biased from 5.333V (R126, C37, R124/R125). Signal into pin 3 is the sum of two VCF outputs (C14 side via 220K, C15 side via 100K), both AC-coupled by .01. Pin 6 output → internal buffer pins 7→8; DC bias of the output set by R127/R128/R129 to C40/R162/5.333V. Output via C38 1µ to VOLUME VR8 top.
- Mixer: Q34 common-base/summing stage (base at 5.333V via R157, C52), emitter is the summing node for VOLUME wiper (47K) and MIX IN (100K); Q33 PNP second stage; R153 100K global feedback; gains ≈ −100K/47K (VCA) and −1 (MIX IN). Output → C56/R161 1K → OUTPUT J6, and → R159 33K/C51 → headphone amp.
- Headphone: LA4140 with bootstrap C44, NFB C50/R135, ripple C48, output C43 1000µ, clamps D32/D33, feeding J2 through R163/R164 10 Ω each; J2 contains a switch to the +6V line.
- Supplies: +6V (batteries 4×1.5V or 9VDC adaptor via J7, POWER SW on VR8, Q44/Q45 regulator) → DC-DC converter T2 → RAW → zener D43 reference → IC16B follower = 5.333V (trimmed by TM6) → IC16A ×2.22 = 12V.

---------------------------------------------------------------------------------------------------
## 5. Uncertainties
1. Jack designators: drawing shows J1 = MIX IN, J6 = OUTPUT, J2 = HEAD PHONE (labels above-right of each jack). Brief said MIX IN = J6 — not supported.
2. Unlabelled filled-arrow supply (R123, IC15 9/4, R136, Q36/R147) assumed +12V; only one filled arrow on the page carries a text label ("12V"). No "9V" internal rail found; "9VDC" is only the adaptor input.
3. IC15 supply pins: the arrow stem sits between printed "9" and "4" on the buffer triangle; cannot tell if V+ is pin 9, pin 4, or both. Which input (2 or 3) is inverting is not marked.
4. R126 value: "2.?K", 2nd digit damaged (probably 2.2K).
5. IC14 lead from C47 has no printed pin number (only "1" is missing from 1–9). Pin 7 and pin 4 sit on the same x at top/bottom of the triangle; read as two separate pins.
6. J2 wiring as drawn is odd: R163/R164 hang on the D33-cathode/C49/pin-7 rail, while C43's (−) side goes to the J2 switch contact that mates with +6V. Rechecked at 4× zoom; traces do not cross or swap. Either the drawing is as built or there is a drafting error. Second reader please check.
7. Duplicate designators on the drawing: D27 (VCA and ENV chain D28/D27/D26); C22 (.01 in VCA and 1/50 in VCF).
8. C42, C51 polarity marks are small; read as stated.
9. Q31–Q34 transistor types are not printed next to the symbols.
10. Q44 type "2SB64‾" last digit obscured; D40 printed as "D4C".
11. Errata R170: the inset shows R170 terminals re-numbered (3/2), but the before/after mapping is not clear enough to state precisely.
12. TM6 wiper appears strapped to its upper end (variable resistor); IC16 pin 6 connection read as R177 only.

## Memory hints (unverified)
- The common TB-303 description has the VCA built around a BA662 OTA with a 2SA733 (Q31) / 2SC1815-type (Q32) envelope; the drawing here does not give those types.
- From memory, the LA4140 output pin is often pin 1 and pin 6 is the bootstrap pin, which could explain the odd J2/C43 topology in item 6 (i.e. the triangle labels may not match how the IC works). Not verified from this schematic.
