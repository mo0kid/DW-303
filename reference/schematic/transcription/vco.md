# TB-303 page 5 — Pitch CV / DAC / slide / VCO / square shaper / WAVEFORM switch (RECONCILED)

Sources: vco-A.md, vco-B.md (independent readers); scan p5-05.png (300 dpi, 4961x3505), p-07/p-08/p-09.
Reconciler crops: /tmp/vcoR/r*.png. Coordinates below are full-res p5-05 pixels (x0,y0,x1,y1).

## Conventions
- **+5.333V** = ⊕ glyph (circle + bar + dot, ~50 px wide). Page note: "5.333V ⊕ connects to pin 7 of IC16 (POWER SUPPLY)"; IC16 pin 7 = TP5.
- **+12V** = filled up-arrow. Unlabelled inside the VCO. It is taken as 12 V because the identical arrow at IC16 is labelled "12V" and the saw waveform annotation tops out at "12V".
- GND = hatched ground. ■ next to a resistor = MFR (metal film). "POSISTOR: R100".

## 1. Component table (reconciled)

| Ref | Value as printed | Type | Conf | Notes |
|---|---|---|---|---|
| IC9 | — | 4174 hex D-FF (HD14174BP) | ✓ | pins 1, 16 → +5.333V; pin 8 → GND |
| IC10 | — | 4050 hex buffer (HD14050BP) | ✓ | p-09 erratum applied (see §2) |
| R74–R85, R87–R90 | 200K | MFR, ".1% SELECTED", "MATCHED PAIR" | ✓ | group note "(R74—R90) ↓ 200K" |
| R96 (sic) | 200K (group) | MFR | ✓ label | It sits in the R86 slot. Printed "R96" on p5 and on the p-09 copy (see O4) |
| IC11a, IC11b | — | AN6562 dual op-amp | ✓ | |
| R107 | 2.2K | resistor | ✓ | IC11b feedback (follower) |
| D30, D31 | — | diode, type not printed | ✓ | |
| IC12 | — | 4066 (HD14066BP) | ✓ | sections 8/9 ctl 6, 10/11 ctl 12 |
| C35 | .22 | 0.22 µF | ✓ | slide capacitor |
| Q29, Q30 | — | NPN, type not printed | ✓ | |
| R91 | 1M | | ✓ | |
| R92, R93 | 100K | | ✓ | |
| R59 | 100K | | ✓ | |
| TM4 "TUNE" | "4 K (B)" (one blank glyph gap) | trimmer | ? | resolved to 47K(B), see L3 |
| R102 | 100 | resistor | ? unit | see O1 |
| R103 | 220K | | ✓ | |
| Q26 | 2SC1583 | dual NPN, 2SC1583(F) "Dual Tr" (p-09) | ✓ | |
| R104 | 2.2K | | ✓ | |
| C34 | .001 | 1 nF | ✓ | |
| TM5 "WIDTH" | 4.7K | trimmer as rheostat (wiper to lower end) | ✓ | |
| R106 | 27K | MFR ■ | ✓ | |
| R118 | 100K | | ✓ | the "Q" beside it belongs to the Q25 label |
| R100 | 560 | posistor (PTC) | ✓ | |
| Q25 | — | NPN (decided from topology) | ? symbol | see L5 |
| Q24 | — | NPN, diode-connected | ✓ | |
| Q27 | — | PNP | ✓ | |
| D25 | — | diode | ✓ | |
| R101 | 10K | | ✓ | |
| C33 | .01 | 10 nF timing capacitor | ✓ | |
| Q28 | 2SK30(O) | N-JFET (2SK30TM(O)) | ✓ | |
| R105 | 10K | | ✓ | |
| C31, C32 | 10/16 | 10 µF/16 V electrolytic | ✓ | decoupling |
| C10 | .01 | 10 nF | ✓ | |
| R34 | 10K | | ✓ | |
| R35 | 100K | | ✓ | see L8 |
| Q8 | — | PNP | ✓ | |
| R45 | 22K | | ✓ | |
| C11 | 1/50 | 1 µF/50 V electrolytic | ✓ | |
| R36 | 10K | | ✓ | |
| S1 "WAVEFORM" | — | 3-terminal slide switch | ✓ | |
| VR2 "TUNING" | 50K(B) | panel pot K161B0-50kB (p-09 page-1 list) | ✓ | |
| J3 "CV" | — | jack | ✓ | |

## 2. Netlist (reconciled; p-09 erratum applied)

### DAC
- IC9 (4174):
  - 1, 16 → +5.333V; 8 → GND.
  - D inputs 4, 13, 3, 14, 6, 11 come from the CPU bus (off-block).
  - **9 (CLK) = net SLIDE_CLK**: it runs down, has a junction dot to the "SLIDE ON/OFF" line at (778,1280), and continues to IC13 (HD14013) pin 11 "T". See L1/O2.
- IC9 10 → IC10 in 5, 7, 9, 11 (4 in parallel) → out 4, 6, 10, 12 → R90 → **D5 = DAC_OUT**.
- IC9 7 → IC10 **in 3, 14 → out 2, 15** (p-09 erratum; p5 printed in 2, 15 / out 3, 14) → R87 → D4.
- IC9 15 → R79 → D3. IC9 2 → R80 → D2. IC9 12 → R81 → D1. IC9 5 → R82 → D0.
- Series arms (each two 200K in parallel = 100K):
  - D5–D4: R88 ∥ R89
  - D4–D3: R85 ∥ R96
  - D3–D2: R83 ∥ R84
  - D2–D1: R76 ∥ R77
  - D1–D0: R74 ∥ R75
- Termination: D0 → R78 200K → **+5.333V** (⊕). See L2.
- DAC_OUT → IC11b pin 5 (+) and IC12 pin 8.

### CV buffer, clamp, CV OUT
- IC11b:
  - pin 5 = DAC_OUT.
  - pin 6 → R107 2.2K → pin 7.
  - pin 7 = **CV** ("(TP4)"; black TP4 badge at the D31 node).
- D31: anode CV, cathode +5.333V. D30: anode GND, cathode CV.
- CV → top rail → TM5 top end, and → x≈2545 down to the "CV" bus (y≈1115) → J3 tip. No series parts. J3 sleeve → GND.

### Slide
- IC12 switch A: pin 8 = DAC_OUT, pin 9 = SLEW, control pin 6 = SW_A.
- IC12 switch B: pin 10 = SLEW, pin 11 = CV (joined to IC11b pin 7 with a dot), control pin 12 = SW_B.
- C35 0.22 µF: SLEW → GND.
- Q29 (NPN):
  - B → R59 100K → SLIDE ON/OFF (= SLIDE_CLK).
  - E → GND.
  - C = SW_B, which also has R92 100K → +12V and R91 1M → Q30 B.
- Q30 (NPN):
  - B ← R91.
  - E → GND.
  - C = SW_A, which also has R93 100K → +12V.

### Exponential converter
- IC11a:
  - 3 (+) → +5.333V.
  - 2 (−) = EXP_REF.
  - 1 = EXP_DRV.
- C34 .001: pin 2 ↔ pin 1.
- EXP_REF connects to:
  - R103 220K → +12V.
  - R102 100 → TM4 wiper.
  - Q26a C.
- TM4: one end → +5.333V, other end → +12V, wiper → R102.
- R104 2.2K: EXP_DRV → common emitters (Q26a E, Q26b E).
- Q26a: B → +5.333V, C = EXP_REF.
- Q26b: B = EXP_IN, C = SAW_CAP.
- EXP_IN connects to:
  - R106 27K → TM5 (4.7K rheostat) → CV.
  - R118 100K → TUNE line.
  - R100 560 PTC → +5.333V.
- TUNE line → "TUNE" bus (y≈1315) → VR2 pin 2 (wiper).
- VR2: pin 1 → GND; pin 3 → T-bar terminal glyph (see O3).

### Saw core and reset
- C33 .01: SAW_CAP → GND.
- Q25 (NPN): E = SAW_CAP, B = RST_X, C → Q27 B.
- Q27 (PNP): E → +12V, C → D25 anode.
- D25: cathode → RST_X.
- Q24 (NPN): B = C = RST_X; E → R101 10K → +5.333V.
- Q28 2SK30(O): G = SAW_CAP, D → +12V, S = **VCO_SAW**.
- R105 10K: VCO_SAW → GND.
- C32 10/16: + → +12V, − → GND.
- C31 10/16: + → +5.333V, − → GND.

### Square shaper
- R35 100K: VCO_SAW → Q8 B.
- C10 .01: VCO_SAW → N_C10. R34 10K: N_C10 → Q8 B.
- Q8 (PNP): E = Q8E, which also has R45 22K → +12V and C11 1/50 (+ on Q8E, − on GND). C = **VCO_SQR**.
- R36 10K: VCO_SQR → +5.333V.

### WAVEFORM switch
- S1: top = VCO_SAW, middle (common) = **VCF_IN** ("saw/square" glyph), bottom = VCO_SQR. The slider is drawn on top + middle (saw).
- VCF_IN → bus → down at x≈2664 → VCF block, R62 220K top. That endpoint belongs to the VCF transcription.

### Block I/O
- In:
  - IC9 D0–D5 (CPU)
  - SLIDE_CLK (IC9 9 / SLIDE ON/OFF / IC13 11)
  - +12V, +5.333V, GND
- Out:
  - CV → J3
  - VCF_IN → VCF
- Panel: TUNE ↔ VR2

## 3. Annotations, adjustments, errata
- p5:
  - "TUNE: 110Hz (SET TUNING [knob at 12 o'clock] CV OUT: 2.75V (A KEY))"
  - "WIDTH: 1 OCT./VOLT"
  - "CV KEY C=1.000V, C̄=4.000V"
  - Saw at Q28 S: falling ramp with instant reset, 12V → 5.5V.
  - Square at Q8 C: low 5V, high 8V (with droop), period "(9ms KEY A)".
- p-07 (VCO):
  - Check point Q28 source or S1; TUNE in its middle position.
  - PITCH mode: tap CL/CH alternately and set TM5 for 2:1.
  - Press A and set TM4 for 110 Hz.
  - Verify 4:1 ±0.5% with CL/CH + TRANSPOSE UP.
- p-08: trimmers 47kB / 220kB / 470kB / 4.7kB; 2SK30TM(Y)/(O); HD14174BP, HD14050BP, HD14066BP, AN6562.
- p-09 errata relevant to this block:
  - (a) IC10 second buffer → in 3/14, out 2/15. **Applied.**
  - (b) 2SC1583(F) = Dual Tr (confirms Q26).
  - (c) Pot list: TUNING = K161B0-50kB (B taper), consistent with VR2 "50K(B)".
  - No other erratum touches this block. The p-09 copy of the DAC region still shows "R96".

## 4. Resolution log
The readers agreed on almost every value and connection. The entries below cover every flagged item and the points the caller asked about.

| # | Item | Evidence (crop) | Decision |
|---|---|---|---|
| L1 | IC9 pin 9 / SLIDE ON/OFF junction | (700,900,1500,1500) and (600,1150,1300,1800) | **Confirmed as drawn.** IC9 pin 9 runs down; there is a filled junction dot at (778,1280) where "SLIDE ON/OFF" branches right to R59; the line continues down to IC13 pin 11 (T). One net. Its oddity is kept as O2. |
| L2 | Ladder termination | (700,900,1500,1500) | **R78 → ⊕ +5.333V** (both readers agree). Same ⊕ glyph as IC9 pin 1/16 "5.333V". |
| L3 | TM4 value | (1770,520,1860,560) at 10x NEAREST | "4", then a blank gap exactly one glyph wide with no ink, then "K", then "(B)" below. Taken as **47K(B)**: 47kB is in the p-08 trimmer list, and 4.7K would print as "4.7K" (as TM5 does) and leave no gap. Medium confidence. The "↙" under "(B)" has no connection; probably a stray mark or rotation indicator. |
| L4 | IC12 / C35 slide wiring and switch polarity | (1100,250,1900,1000) | Switch A (8–9) puts C35 on DAC_OUT (IC11b + input, ladder node). Switch B (10–11) ties C35 to IC11b output pin 7 (dot on the output node). Logic, from the drawn Q29/Q30 inverter pair: <br>• **SLIDE high:** Q29 on, so SW_B is low and B is open. Q30's base is pulled down via R91 from the saturated Q29 collector, so Q30 is off, SW_A is pulled high through R93, and **A is closed: slide active.** <br>• **SLIDE low:** Q29 off, so SW_B is high through R92 and **B is closed**. Q30 is on via R92+R91, so SW_A is low and A is open. C35 is then pre-charged to CV by the op-amp output, and there is no glide. <br>Reader B's "odd topology" concern is resolved: C35 loads the ladder node (high impedance), which is the op-amp input, so glide happens at the input while B only pre-charges. |
| L5 | Q25 polarity | (2020,480,2220,700); (2020,615,2090,650) 12x | The arrowhead is fused with the "5" of the label, so its direction is not decidable at pixel level. **Decided NPN** from topology. With emitter = SAW_CAP, base = RST_X and collector driving the PNP Q27 base, only an NPN gives a working reset: it conducts when the saw falls to ≈RST_X−Vbe and its collector pulls Q27 on. A PNP would conduct permanently and would source, not sink, Q27 base current. Both readers said NPN. |
| L6 | VR2 pin 3 | (4260,1360,4360,1420) vs a known ⊕ at (1120,1210,1180,1260) | The glyph is a bar + dot, ~30 px, with **no circle**. The known ⊕ is ~50 px with a circle. VR3 pin 3 uses a filled arrow. So it is **not** an identifiable 5.333V/12V symbol. Left open as O3. |
| L7 | "R96" vs R86 | (700,250,1500,1000); p-09 copy | Clearly printed "R96". It occupies the only gap in R74–R90 and sits in the 200K matched group. **Designator kept as printed "R96"; value 200K** (electrically it is the R86 position). |
| L8 | R35 value | (1850,250,2700,1000) | "100K" sits directly above R35. It is the only unassigned value text there, and both readers read it so. **100K.** |
| L9 | R102 value | (1680,440,1960,680) 4x | Rotated text "100", with no unit adjacent. The horizontal "K" belongs to TM4's "4 K". Transcribed **"100"**. Unit left open (O1). |
| L10 | D31 cathode glyph (B unc. 7) | (1100,250,1900,1000) | Same ⊕ glyph and size as the 5.333V symbol. The black "TP4" badge is a separate label to its left. **D31 cathode → +5.333V.** |
| L11 | Supply arrows | IC16 "12V" label; saw annotation "12V" | **+12V** (assumption shared by both readers, supported by the 12 V saw peak). |
| L12 | CV → J3 path | (2400,950,4000,1350); (3900,900,4600,1700) | The CV bus at y≈1115 runs to the J3 tip; no series part is seen. **Confirmed.** |

## 5. Open items
- **O1 — R102 unit (100 Ω vs 100 kΩ).** Roland's page convention is that bare numbers are ohms (cf. R100 "560"), which reads as 100 Ω. Electrically:
  - The summing node EXP_REF is held at 5.333V. TM4 (47K) spans 5.333V to 12V, so the wiper source is V = 5.333 + 6.667α with R_th = 47K·α(1−α).
  - **100 Ω:** I_TM4 = 6.667α / (100 + 47K·α(1−α)). This gives ≈281 µA at mid-travel and up to ~67 mA at the 12V end, with an extremely non-linear, coarse trim. Iref (with R103's 30.3 µA) ≈ 30 µA to mA.
  - **100 kΩ:** I_TM4 = 0–66.7 µA (≈29.8 µA at mid), so Iref ≈ 30–97 µA, a 1.7-octave smooth trim range. This is consistent with the ~7 µA saw current needed for 110 Hz (§6).
  - 100K is the electrically plausible reading. The printed text supports only "100".
- **O2 — SLIDE ON/OFF = IC9 CLK = IC13 T.** It is drawn as one net (L1). Consequence: the DAC latches on the rising edge of the same line that closes switch A. A note latched with the line left high glides (τ below). A note strobed with a short high pulse gets only a brief (pulse-width) partial glide. Model either as drawn, or with separate latch-clock and slide signals if other evidence (CPU port map) shows separate lines.
- **O3 — VR2 pin 3 destination.** Unidentified T-bar glyph.
  - (a) If it is +5.333V: TUNE = 0…5.333V, R_th up to 12.5K. The TUNE contribution at the expo base is 5.46 mV/V, so the span is ≈29 mV ≈ **±0.8 oct**.
  - (b) If it is unconnected: VR2 is a rheostat from TUNE to GND. The R118 current varies from 53.3 to 35.6 µA, a span of ≈9.5 mV ≈ **0.53 oct**.
- **O4 — "R96" designator.** It may be a printing error for R86. It has no electrical consequence (200K in the R85 ∥ R96 arm).
- **O5 — Q25 symbol** is illegible at pixel level. NPN is chosen from function (L5).
- **O6 — unprinted semiconductor types** for Q8, Q24, Q25, Q27, Q29, Q30, D25, D30, D31. The p-09 list (2SA733(P), 2SC2021(R)/2SD637(R), …) is not mapped to designators on this page.
- **O7 — IC12 supply** is not shown in the block. The control-high level comes from 12V through 100K, which implies a 12V-supplied 4066.

## 6. Model-relevant values (derived from the reconciled netlist)
- **DAC.** 6-bit R-2R with R = 100K (series arms) and 2R = 200K (bit and termination resistors), driven at logic levels 0 / 5.333V. The termination goes to Vref, not GND, so:
  - V_DAC = 5.333·(code + 1)/64 = (code + 1)/12 V.
  - 1 LSB = 83.33 mV = 1 semitone at 1 V/oct.
  - Range is 0.0833–5.333 V.
  - Check against the annotations: C = 1.000 V is code 11; C̄ = 4.000 V is code 47 (36 steps = 3 oct); A key 2.75 V is code 32.
  - The assumption that 4174/4050 high = 5.333V follows from IC9 pins 1/16 at 5.333V. The IC10 supply is not shown.
- **DAC Thevenin output resistance** = R90 (200K) ∥ (R88∥R89 + rest of ladder = 100K + 100K) = **100 kΩ**. The 4050 source resistance is negligible.
- **Slide** (switch A closed, SLIDE high): **τ = 100 kΩ × 0.22 µF = 22 ms**. 4066 Ron (~0.1–1 kΩ) adds <1%. 10–90% ≈ 48 ms; 99% ≈ 101 ms. With SLIDE low, C35 tracks CV through B (pre-charge, effectively instant).
- **CV buffer:** IC11b unity follower (R107 2.2K feedback). Clamp: D30 ≥ −0.6 V, D31 ≤ 5.333 + 0.6 V. CV OUT (J3) = buffer output, unbuffered.
- **Expo input network** (to Q26b base, relative to 5.333V). G_total = 1/560 + 1/R_cv + 1/100K, with R_cv = 27K + TM5 (0–4.7K).
  - CV gain = (1/R_cv)/G_total = **20.2 mV/V** (TM5 = 0) to **17.3 mV/V** (TM5 = 4.7K).
  - Required for 1 oct/V at 300 K: VT·ln2 = 17.9 mV, which gives R_cv ≈ 30.6K (TM5 ≈ 3.6K).
  - TUNE gain = 1e-5/G_total ≈ **5.46 mV/V** (≈0.30 oct/V).
  - Source impedance at the base ≈ 1/G_total ≈ 545 Ω.
  - R100 is a PTC, compensating VT ∝ T.
- **Expo reference current:** Iref = (12 − 5.333)/220K = **30.3 µA** (R103) + TM4/R102 contribution (see O1). IC11a holds Q26a's collector at 5.333V (Q26a base also at 5.333V, so Vcb = 0).
  - I_saw = Iref·exp(ΔV_B/VT), where ΔV_B = V(Q26b B) − 5.333.
  - R104 2.2K is emitter drive; C34 1 nF is the IC11a integrator compensation.
- **Saw:**
  - C33 = 10 nF. Ramp falls from ≈12 V to ≈5.5 V (ΔV ≈ 6.5 V, annotation).
  - f ≈ I_saw / (C33·ΔV). For 110 Hz, I_saw ≈ 7.15 µA (ignoring reset time).
  - Reset threshold ≈ V(RST_X) − Vbe(Q25) ≈ 5.333 + Vbe(Q24) − Vbe(Q25) ≈ 5.3–5.5 V.
  - Reset is regenerative via Q27 → D25 → RST_X; R101 10K sets the Q24 bias to 5.333V.
- **Saw output:** Q28 2SK30(O) source follower, R105 10K to GND. VCO_SAW = saw − Vgs(Q28).
- **Square:**
  - Q8 PNP. The emitter is fed by R45 22K from 12V and held by C11 1 µF, giving τ_E = 22 ms.
  - The base is driven by R35 100K (DC) ∥ (C10 10 nF + R34 10K; τ = 0.1 ms) from VCO_SAW.
  - Collector load R36 10K to 5.333V, so the output is low = 5.333V (off) and high ≈ 8V (annotated 5 V / 8 V).
- **S1** selects VCO_SAW or VCO_SQR → VCF_IN (VCF R62 220K).
- **Pots/trimmers:**
  - VR2 TUNING 50K B-taper (linear).
  - TM4 47K(B) (L3).
  - TM5 4.7K(B?) rheostat. Its taper is not printed on p5; p-08 lists a 4.7kB trimmer.

## Memory hints (unverified)
- The TB-303 slide is often quoted at ~60 ms. That does not match the 22 ms derived here from the printed values; not used.
- On many 303 redraws the slide line and the latch strobe are distinct CPU port pins. That is not supported by this page (O2).
