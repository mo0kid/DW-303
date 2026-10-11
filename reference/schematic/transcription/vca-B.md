# TB-303 main board (p5) — VCA, VCA envelope, mixer, output, headphone amp — reader vcaB

Source: p5-05.png (300 dpi, 4961x3505). Approx. full-res pixel regions: VCA x1300–2560 y1500–2300;
MIXER x2400–3100 y2780–3200; HEADPHONE x1650–2350 y2780–3300; jacks/pots x3800–4350 y2700–3300;
POWER SUPPLY x2600–4500 y250–1100. Long bus wires were followed with a pixel run tracer (horizontal/vertical
run detection), not just by eye.

## Symbol legend (as used on this page)
- Filled up-arrow = **12V**. Labelled "12V" at IC16 (AN6562) pin 1 output in the power-supply block.
- Circle-with-cross (⊕) = **5.333V**. Boxed note: "5.333V ⊕ connects to pin 7 of IC16 (POWER SUPPLY)".
- Hollow up-arrow (△): one is labelled **"+6V"** in the power supply (battery/switched rail at D2/D4?),
  and a second unlabelled △ sits at D2's other end (D2 with C1 100/6.3 to GND). A △ also appears at J2 (headphone).
- Ground = hatched ground symbol.
- No rail labelled "9V" was found on p5. The only 9 V item is the **9VDC** adapter input at J7. Battery is drawn
  as "1.5" cells "x4" between terminals 29/30. POWER SW (with BU) feeds Q44 2SB647(C) and R168 2.7 FRNB ¼W.

## 1. Component table

### VCA (IC15) and VCA envelope
| Ref | Value as printed | Type | Conf |
|---|---|---|---|
| IC15 | BA662A | OTA ("Vari-conductance amp", parts list BA662(A)) + output buffer on the same IC | ✓ |
| Q31 | — (no type printed) | PNP (emitter arrow points in) | ✓ symbol / type ✗ |
| Q32 | — (no type printed) | NPN (emitter arrow points out) | ✓ symbol / type ✗ |
| D27 | — | diode (1S2473/1SS-133 family not stated at symbol) | ✓ |
| D35 | — | diode | ✓ |
| D36 | — | diode | ✓ |
| R119 | 47K | | ✓ |
| R120 | 22K | | ✓ |
| R121 | 220K | | ✓ |
| R122 | 100K | | ✓ |
| R123 | 1.5M | | ✓ |
| R124 | 2.2K | | ✓ |
| R125 | 2.2K | | ✓ |
| R126 | 2.?K (2nd digit hidden by a blot; reads like 2.2K) | | ? |
| R127 | 47K | | ✓ |
| R128 | 220K | | ✓ |
| R129 | 22K | | ✓ |
| R130 | 100 | | ✓ |
| R131 | 220K | | ✓ |
| R132 | 100 | | ✓ |
| R133 | 2.2K | | ✓ |
| R134 | 22K | | ✓ |
| R162 | 2.2K | | ✓ |
| C21 | .01 | film/ceramic (non-polar) | ✓ |
| C22 | .01 | non-polar | ✓ |
| C36 | .033 | non-polar | ✓ |
| C37 | 10/16 | electrolytic, + at top (R124/R125/R126 node) | ✓ |
| C38 | 1/50 | electrolytic, + toward IC15 pin 8 | ✓ |
| C40 | 10/16 | electrolytic, + at top (bias node) | ✓ |
| C41 | .1 | non-polar | ✓ |
| C42 | 1/35 (the "1" is partly smudged) | electrolytic, + toward Q31 base node (top) | ? |

### Mixer
| Ref | Value | Type | Conf |
|---|---|---|---|
| Q33 | — (no type printed) | PNP (emitter arrow in) | ✓ symbol |
| Q34 | — (no type printed) | NPN (emitter arrow out) | ✓ symbol |
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
| C52 | 10/16 | electrolytic, + top | ✓ |
| C53 | 47/16 | electrolytic, + top | ✓ |
| C56 | 1/50 | electrolytic, + top (mixer output side) | ✓ |
| C58 | 1/50 | electrolytic, "−" marked at R156 side | ✓ |
| C59 | 1/50 | electrolytic, "+" at Q34-emitter side, "−" at R155 side | ✓ |

### Headphone amp
| Ref | Value | Type | Conf |
|---|---|---|---|
| IC14 | LA4140 | AF power amp (parts list) | ✓ |
| R135 | 4.7K | | ✓ |
| R159 | 33K | | ✓ |
| R163 | 10 ("10 X2" printed under R163/R164) | | ✓ |
| R164 | 10 | | ✓ |
| C43 | 1000/6.3 | electrolytic, + at IC14 pin 6 side | ✓ |
| C44 | 47/16 | electrolytic, + at top (pin 8 side) | ✓ |
| C45 | .068 | non-polar | ✓ |
| C46 | .01 | non-polar | ✓ |
| C47 | .0068 | non-polar | ✓ |
| C48 | 100/6.3 | electrolytic, + at pin 9 side | ✓ |
| C49 | 100/6.3 | electrolytic, + at top rail | ✓ |
| C50 | 10/16 | electrolytic, + at pin 3 side | ✓ |
| C51 | 2.2/50 | electrolytic, + at R159 side | ✓ |
| D32 | — | diode | ✓ |
| D33 | — | diode | ✓ |

### Front-panel parts / jacks
| Ref | Value | Notes | Conf |
|---|---|---|---|
| VR7 | 50K(B) ACCENT | p8: K161B0-50KB "TUNING, ACCENT" | ✓ |
| VR8 | 50K(A) VOLUME | dashed mechanical link to POWER SW; p8: VM11R851A-5M1411-50KA "master vol. & power switch" | ✓ |
| J1 | MIX IN | mono jack with switch contact to sleeve | ✓ |
| J6 | OUTPUT | mono jack | ✓ |
| J2 | HEAD PHONE | stereo jack (tip + ring contacts), with a switch contact pair (▼ / ▽) | ✓ drawing / ? function |

## 2. Netlist

### VCA envelope generator (drawn inside the "VCA" box; not the IC12 "ENV" block)
- **GATE_HI** (Q36 collector, ENV block): Q36 collector → wire up x≈1041 → right along y≈1915 → up x≈1298 to node N_D35A.
- **N_D35A**: R137 (1K) bottom, D35 anode, (Q36 collector). R137 top → D34 anode. D34 cathode → wire up to y≈1356 →
  right to x≈2024 → up to y≈1158 → right … → **J4 GATE** jack. *(R137/D34 belong to the GATE-out path; listed here only for context.)*
- D35: A = N_D35A; K = R133 left.
- R133 2.2K: R133 left = D35 K; R133 right = R131 top (via a wire only).
- R131 220K: top = R133 right; bottom = Q31 E.
- **Q31 (PNP)**: E = R131 bottom; B = **VENV_C** node; C = **IABC** (IC15 pin 1).
- **VENV_C** node: Q31 B, R132 right, R123 bottom, C42 +.
- R123 1.5M: bottom = VENV_C; top = **12V** (filled arrow).
- C42 1/35: + = VENV_C; − = GND.
- R132 100: left = Q32 C; right = VENV_C.
- **Q32 (NPN)**: C = R132 left; B = **Q32B**; E = R130 top.
- R130 100: R130 top = Q32 E; R130 bottom = GND.
- C41 .1: Q32B to GND.
- R134 22K: left = D36 K; right = Q32B.
- D36: A = **ENV_TRIG** (Q38 emitter in ENV block: D36 anode wire left along y≈1969 → down x≈1097 to y≈2718 → left to the
  Q38 E / R151 22K / D37 anode node); K = R134 left.

### Accent input to the VCA
- **ACCENT_WIPER**: VR7 pin 2 (wiper) → long bus (y≈2547) → x≈2540 up to y≈1672 → R119 right end. The same bus also connects
  (junction at x≈3738) to the D24 / R46 47K vertical on the VCF side.
- VR7: pin 3 ← bus y≈2707 → x≈2215 → y≈2429 → x≈1552 into the ENV block (accent-gated level from the ENV/IC12 area; source not
  traced by this reader); pin 1 = GND.
- R119 47K: right = ACCENT_WIPER; left = **N_C36**.
- C36 .033: N_C36 to GND.
- R120 22K: right = N_C36; left = D27 A.
- D27: A = R120 left; K = **IABC** (the diode points left, toward pin 1).

### IC15 BA662A
- pin 1 = **IABC**: Q31 C, D27 K.
- pin 2 (OTA input): R125 top.
- pin 3 (OTA input): R124 top, R121 left, R122 left (**VCA_IN** node).
- pin 5 = GND.
- pin 6 (OTA output) = pin 7 (buffer input) = **VCA_OTA_OUT** node: also R127 bottom and R128 bottom (joined by a wire).
- pin 8 (buffer output): C38 +.
- pins "9" and "4": one up-arrow (**12V**) line rises from the buffer triangle. The label "4" sits beside the arrow line and the
  label "9" sits at the triangle's top-left corner. Most likely pin 4 = 12V and pin 9 has no visible wire. Ambiguous.
- Bias network:
  - **VB_VCA** node: R162 left, C40 +, R129 top, R128 top. There is also a junction dot on this line directly above R127,
    but no wire is drawn down from it.
  - R162 2.2K: right = **5.333V** (⊕).
  - C40 10/16: − to GND.
  - R129 22K: top = VB_VCA; bottom = R127 top.
  - R127 47K: top = R129 bottom; bottom = VCA_OTA_OUT.
  - R128 220K: top = VB_VCA; bottom = VCA_OTA_OUT.
  - So R128 is in parallel with the series pair R129+R127, between VB_VCA and the OTA output.
- Input bias: **VB_IN** node: R125 bottom, R124 bottom, R126 right, C37 +.
  - R126 2.?K: left = 5.333V (⊕).
  - C37 10/16: − to GND.
  - R125 2.2K: VB_IN to pin 2. R124 2.2K: VB_IN to pin 3.

### VCF → VCA coupling
- C21 .01: right = wire x≈2460 → bus y≈2468 → x≈3540 → up to the **VCF_OUT** node. VCF_OUT is C14 (1/50)'s right/− end, and
  C14 + is at Q19 emitter (with R96 10K). The same x≈3540 vertical also feeds VR4 RESONANCE (section A) pin 3.
  C21 left → R121 220K → VCA_IN (pin 3).
- C22 .01: right = wire x≈2418 → bus y≈2507 → x≈3582 → up to the **RES_WIPER** node. RES_WIPER is the C15 (1/50) line, joined
  to VR4 section A pin 2 (wiper); VR4A pin 1 = GND. C22 left → R122 100K → VCA_IN.

### VCA output → volume
- C38 1/50: + = IC15 pin 8; − → wire x≈2500 down → bus y≈2747 → **VR8 pin 3**.
- VR8 50K(A) VOLUME: pin 1 = GND; pin 2 (wiper) = **VOL_WIPER** → left along y≈2808 → x≈3949 down → bus y≈3104 → R155 right
  end (via the x≈3056 step).
- VR8 is ganged with **POWER SW** (dashed line runs to the power switch at J7).

### Mixer (Q33/Q34)
- **V_MIX** (local supply): R136 right, C53 +, Q33 E, R154 top.
  - R136 100: left = **12V** (filled arrow).
  - C53 47/16: − to GND.
- **Q34 (NPN)**: B = **Q34B** node; C = **Q34C** node; E = **SUM** node.
- Q34B: R157 left, C52 + (via wire). R157 10K right = 5.333V (⊕). C52 10/16: − to GND.
- Q34C: R154 bottom (R154 100K to V_MIX) and R158 right.
- R158 10K: left = Q33 B.
- **Q33 (PNP)**: E = V_MIX; B = R158 left; C = **MIX_OUT**.
- MIX_OUT: Q33 C, R160 top (R160 10K to GND), R153 left, C56 +, R159 right (via a vertical wire at x≈2337).
- R153 100K: MIX_OUT to SUM (feedback).
- SUM: Q34 E, R153 right, C59 +, C58 +.
- C59 1/50 − → R155 47K → VOL_WIPER (VR8 pin 2).
- C58 1/50 − → R156 100K → bus y≈3144 → x≈3990 up → **J1 MIX IN** tip.
- J1: the tip contact has a switch contact (arrow) to the sleeve/GND line, so the input reads as grounded when no plug is
  inserted. Sleeve = GND.

### Line output
- C56 1/50: + = MIX_OUT; − = R161 top.
- R161 1K: bottom = **OUT_NODE**.
- OUT_NODE: R165 100K to GND, and bus y≈3184 → x≈4031 up → **J6 OUTPUT** tip. J6 sleeve = GND.

### Headphone amp IC14 LA4140 (pin numbers as printed: 2,3,4,5,6,7,8,9; no pin 1 shown)
The triangle points LEFT: pins 2/3 are on the right, pin 6 is at the tip.
- pin 2: C51 − (C51 + → R159 33K → MIX_OUT).
- pin 3: C50 + ; C50 − → R135 4.7K → GND. pin 3 also → C47 right end (vertical wire at the triangle's right).
- pin 4: C47 .0068 left end (C47 sits between pin 4 and pin 3).
- pin 5: GND.
- pin 6 = **HP_OUT** node: D33 anode, D32 cathode (D32 anode = GND), C43 +, C44 −, C45 top (C45 .068 to GND).
- pin 7: **HP_RAIL** (top rail).
- pin 8: **BOOT** node: C44 + and C46 right plate.
- pin 9: C48 + ; C48 − to GND.
- HP_RAIL: pin 7, D33 cathode, C49 + (C49 − to GND), C46 left plate. It is also routed by a wire (outer loop x≈1696, bus y≈3264)
  to the node x≈4094 that feeds **R163 and R164**.
- R163 10 → J2 **tip** contact. R164 10 → J2 **ring** contact. J2 sleeve = GND.
- C43 1000/6.3: − end → wire (inner loop x≈1735, bus y≈3224) → x≈4071 up → J2 switch contact **▼** (filled).
- J2 switch: ▼ (C43−) rests on a middle contact that is wired to a **hollow up-arrow (△)**. Below it, the ▽ (open) contact
  hangs from that middle contact onto the **tip** line (R163 side). The geometry was confirmed in a nearest-neighbour zoom.
  The electrical function is not stated on the drawing (see Uncertainties).
- No other supply connection to IC14 was found on the drawing.

## 3. Annotations and pages 7–9
- Near VCA: block label "VCA". The "DECAY" label next to the wire under R126/C37 belongs to the ENV/VR6 bus, not the VCA.
- ENV block note (VCF env, for context only): "DECAY VR MAX T=2.5 sec. MIN T=200ms".
- Power: "12V" (IC16 AN6562 pin 1), "5.333V" (IC16 pin 7, TP5 note C=1.000V / C̄=4.000V on CV), "+6V" (hollow arrow), 9VDC
  adapter J7, battery "1.5 x4", Q44 2SB647(C), R168 2.7 FRNB¼W, D38/D39, BU, POWER SW ganged to VR8.
- p8 parts list:
  - IC: BA662(A) "Vari-conductance amp"; LA4140 "AF Power amp"; AN6562 "Dual op amp".
  - Pots: master vol. & power switch = VM11R851A-5M1411-50KA. TUNING/ACCENT = K161B0-50KB. CUTOFF/ENV MOD = K161B0-50kA.
    DECAY = K161B0-1MA. RESONANCE = K162T00W-50kB x2.
  - Jacks: HLJ0305-01-030 MONO; HLJ0305-01-010 STEREO; SG8026 MINI; HEC0470-01-230 AC adaptor.
  - Diodes listed: 1S2473, 1SS-133, 1S-188FM (Ge), RD6.8EB-3, RD15EB-3, S-5500G, 158-46A (per-designator types are not given).
- p9 errata, "CIRCUIT DIAGRAM (page 5)":
  - **None of the corrections touch the VCA, mixer or headphone parts.**
  - Main-board top-left DAC area: IC10 4050 buffer pin numbering corrected; the inset shows a buffer with pins 3 / 14 / 2 / 15.
  - Power supply:
    - C66 .001 → **0.0022**.
    - The unlabelled transistor next to Q43 → **Q42 2SC536**.
    - The C67 value next to it, printed 2.2/50 → **C67 10/16**.
    - R170 22 re-routed: the inset circle shows R17x between the T2 pin 3 node and pin 2 side ("3", "2" labels).
  - Parts-list corrections: RESONANCE pot part no. 13219774 → 13219775 (K162T00W-50KBx2); sub-panel 2221027700 → 2221027701.
    Transistor list adds 2SC536(F), "2SC2021(R) or 2SD637(R)", "2SA937(Q) or 2SB642(R)".
  - p9 page-1 pot callouts: K162T00W-50kB x2 (resonance), K161B0-50kA ×2, K161B0-1MA, K161B0-50kB ×2.
- p7: no VCA/mixer adjustments noted for this block.

## 4. Functional description (from this transcription only)
**VCA (IC15 BA662A):**
- The VCF output reaches the OTA's pin-3 input two ways: directly through R121 220K/C21, and from the resonance-pot wiper
  through R122 100K/C22.
- Both OTA inputs are biased at 5.333V through 2.2K resistors from a C37-decoupled node.
- The OTA output (pin 6) is loaded by a resistor network to a C40-decoupled 5.333V point (R162 2.2K), then buffered
  (pin 7 → pin 8) and AC-coupled out through C38.
- Gain is set by the current into pin 1 (IABC), which comes from two sources:
  - **Envelope.** A gate-high level (Q36 collector) flows through D35, R133 and R131 220K into PNP Q31's emitter. Q31's base
    sits on C42 (1 µF), which R123 1.5M pulls toward 12V. On each trigger pulse (Q38 emitter → D36 → R134 → Q32), Q32 dumps
    C42 through 100 Ω. Q31 then conducts fully and the current decays as C42 recharges via 1.5M (RC ≈ 1.5 s nominal).
  - **Accent.** The ACCENT pot wiper adds current through R119 47K / C36 .033 / R120 22K / D27.
- VCA output → VR8 VOLUME, which is ganged with the power switch.

**Mixer:**
- Q34 is common-base (base held at 5.333V by R157, with C52) and its emitter is a summing node.
- Inputs: volume wiper via R155 47K + C59, and MIX IN via R156 100K + C58. J1 grounds the input when nothing is plugged in.
- Q34's collector (R154 100K load) drives PNP Q33, whose collector output is fed back to the summing node through R153 100K.
- Mixer supply: 12V via R136 100 / C53.
- MIX_OUT → C56 → R161 1K → J6 OUTPUT (R165 100K to GND).
- MIX_OUT also → R159 33K → C51 → IC14 LA4140 pin 2.

**Headphones:** IC14 LA4140 power amp.
- Feedback network at pin 3: C50 + R135 4.7K to GND. C47 .0068 between pins 3 and 4. Ripple capacitor C48 on pin 9.
- Bootstrap: C44 between the output and pin 8, and C46 between pin 8 and the rail. C49 decouples the rail.
- Clamp diodes D32/D33 on the output.
- The C43 1000 µF output capacitor and the rail both go to J2 (see the open question below).

## 5. Uncertainties
1. C42 value "1/35": the "1" is partly smudged.
2. R126 value "2.?K": the second digit is obscured.
3. IC15 pins 9/4: one 12V arrow line, two labels. It is unclear which pin(s) it connects to.
4. R127/R128/R129 topology: there is a junction dot on the top line above R127 with no visible wire down. If a wire were
   intended there, R129 would be shorted. As drawn, R128 ∥ (R129+R127).
5. Transistor types for Q31–Q34 and diode types for D27/D32–D36 are not printed at the symbols.
6. Headphone wiring as drawn:
   - The IC14 pin-7 rail (with C49/D33) goes to R163/R164 → J2 tip/ring.
   - C43(−) goes to the J2 ▼ switch contact; the middle contact is a hollow-arrow supply (△, unlabelled at J2).
   - IC14 has no other visible supply connection.
   - Two readings are possible: (a) the load sits between C43 and the rail, with the switch applying △ power through the jack;
     or (b) the two long loop wires (x≈1696/1735 → y≈3264/3224) were drafted swapped.
   - The tracer confirms the wires as drawn (no crossings). A real board or a second source is needed to resolve this.
7. Hollow-arrow rail identity at J2: there are two hollow arrows in the PSU ("+6V", and one at D2's other end); which one is
   meant is not determined. No "9V" rail label exists on p5.
8. VR7 pin-3 source (accent-gated voltage from the ENV/IC12 area) and the exact ENV_TRIG / GATE_HI generation were not
   transcribed. They belong to the ENV block reader.

## Memory hints (unverified)
- I recall that LA4140-class small amplifiers often use a "load returned to Vcc" output connection. That would fit reading (a)
  in item 6 above, but nothing on p5 confirms it.
