# TB-303 page 5 — ENV / MEG / ACCENT block — reader envA

Source: p5-05.png (300 dpi), crops at full resolution. Coordinates (x,y) are p5-05.png pixels, given so the cross-checker can find things.
Block area roughly x 400–2600, y 1200–3200, plus the pot strip VR4b/VR5/VR6/VR7 at x 3600–4500.

## Rail / symbol legend (as observed)
- **Open triangle** = +6V. It is labelled "+6V" in the POWER SUPPLY block, and IC13 pin 14 uses this symbol.
- **Circled cross ⊕** = 5.333V. A boxed note says "5.333V ⊕ connects to pin 7 of IC16 (POWER SUPPLY)".
- **Filled arrow ▲** = a higher supply rail. I found no label for it in my crops, so I call it **VH** below. See Uncertainties.
- **Hatched ground** = GND.

## 1. Component table

| Ref | Value as printed | Type | Conf. |
|---|---|---|---|
| IC13 | "14013" (one D-FF section used) | 4013 dual D flip-flop (parts list: HD14013BP / MN4013B) | ✓ |
| IC12 | "4066" (two switches used: 3/4 ctl 5, 1/2 ctl 13) | 4066 quad analog switch (HD14066BP) | ✓ |
| Q35 | — (no part number printed) | NPN | ✓ |
| Q36 | — | PNP (emitter arrow points into base) | ✓ |
| Q37 | — | NPN | ✓ |
| Q38 | — | PNP | ✓ |
| Q39 | 2SK30(Y) | N-JFET, gate tied to source (current source) | ✓ |
| Q40 | — | NPN (Darlington with Q41) | ✓ |
| Q41 | — | NPN | ✓ |
| Q31 | — | PNP (VCA drive) | ✓ |
| Q32 | — | NPN (VCA drive) | ✓ |
| R137 | 1K | resistor | ✓ |
| R139 | 100K | | ✓ |
| R140 | 100K | | ✓ |
| R141 | 100K | | ✓ |
| R142 | 10K | | ✓ |
| R143 | 10K | | ✓ |
| R144 | 10K | | ✓ |
| R145 | 10K | | ✓ |
| R146 | 22K | | ✓ |
| R147 | 22 | (drawn with bracket marks either side, see Uncertainties) | ✓/? |
| R148 | 10K | | ✓ |
| R149 | 10K | | ✓ |
| R150 | 22 | | ✓ |
| R151 | 22K | | ✓ |
| R152 | 100 | | ✓ |
| R138 | 68K | | ✓ |
| R117 | 22K | | ✓ |
| R133 | 2.2K | | ✓ |
| R131 | 220K | | ✓ |
| R132 | 100 | | ✓ |
| R123 | 1.5M | | ✓ |
| R134 | 22K | | ✓ |
| R130 | 100 | | ✓ |
| R119 | 47K | | ✓ |
| R120 | 22K | | ✓ |
| R46 | 47K | | ✓ |
| R63 | 220K | (VCF side, ENV MOD destination) | ✓ |
| R72 | 100K | (VCF side, accent destination) | ✓ |
| R61 | 10K | | ✓ |
| C62 | 1/35 (1µF/35V elec.) | + toward Q41 base node | ✓ |
| C54 | .047 | film | ✓ |
| C55 | 47/16 | elec., + at Q38 emitter | ✓ |
| C72 | 10/16 | elec., + toward VH rail | ✓ (polarity ?) |
| C41 | .1 | | ✓ |
| C42 | 1/35 | elec., + mark smudged; probably + at Q31 base node | ? |
| C36 | .033 | to GND | ✓ |
| C13 | 1/50 | elec., + at VR4b pin 3, − to GND | ✓ |
| D34 | — | diode (1S2473 per parts list, not printed here) | ✓ |
| D35 | — | diode | ✓ |
| D36 | — | diode | ✓ |
| D37 | — | diode | ✓ |
| D26, D27, D28 | — | diode string under IC12 | ✓ |
| D27 (2nd use!) | — | diode in VCA area, between R120 and Q31 collector | ✓ (duplicate designator) |
| D24 | — | diode (accent → VCF) | ✓ |
| VR5 | 50K(A) ENV.MOD | pot | ✓ |
| VR6 | 1M(A) ENV.DECAY | pot wired as a rheostat (wiper 2 tied to pin 3) | ✓ |
| VR7 | 50K(B) ACCENT | pot | ✓ |
| VR4b | 2nd gang of VR4 RESONANCE 50K(B)×2 (dashed ganging line) | pot | ✓ |

Also present and only partly in scope (VCA block): IC15 BA662A, R124 2.2K, R125 2.2K, R126 "2.?K" (smudged, probably 2.2K), C37 10/16, R127 47K, R128 220K, R129 22K, C40 10/16, R162 2.2K, C38 1/50, R121 220K + C21 .01, R122 100K + C22 .01.

## 2. Netlist

### CPU / connector inputs
Pins 1–9 are on the left-edge connector. The fourth connector is printed as "," and is probably "4".
- **Pins 1–6**: go up the vertical bus to the IC9 (4174) data inputs. These are DAC/pitch lines and not in my block.
- **Pin 7** → bus line at x≈735 → **IC13 pin 9 (D)**. Net **ACC_DATA**.
- **Pin 8** → bus line at x≈775. Connects (dots) to **IC9 pin 9 (CLK)**, to the horizontal line labelled **"SLIDE ON/OFF"**, and to **IC13 pin 11 (T/clock)**. Net **STROBE/SLIDE** (the label is as printed; see Uncertainties).
- **Pin 9** → R146 22K → **Q37 base**. Net **GATE_IN**.

I found no printed "ACC ON/OFF", "TRIGGER" or "GATE" text at the connector pins themselves. Their roles come from the tracing only.

### Accent latch
- **IC13 (4013)**:
  - pin 14 → +6V; pin 7 → GND.
  - pin 8 (S) and pin 10 (R) → GND.
  - pin 9 D ← ACC_DATA; pin 11 T ← STROBE/SLIDE.
  - pin 12 (printed Q with overbar) → R141 100K → node **Q35B**.
- **R140 100K**: Q35B ↔ GND.
- **Q35 (NPN)**: B = Q35B, E = GND, C = net **ACCENT**.
- **R139 100K**: VH ↔ ACCENT.
- **ACCENT** (vertical line labelled "ACCENT", x≈1218, y 1626–2393):
  - → IC12 pin 5 (control, switch A).
  - Via a dot at (1216,2231), horizontal to x≈1472, then down → IC12 pin 13 (control, switch B).
  - It crosses the DECAY line with no junction dot.

### Gate input / VCA gate / GATE output
- **Q37 (NPN)**: B ← R146 22K ← pin 9, with R144 10K B↔GND. E = GND. C = net **GATE_N** (vertical x≈835).
- **GATE_N** → R143 10K → node **Q36B**, and → C54 .047 → R148 10K → node **Q38B**.
- **Q36 (PNP)**:
  - E → R147 22 → VH.
  - B = Q36B; R142 10K and C72 10/16 are in parallel from VH to Q36B.
  - C = net **GATE_HI**; R145 10K GATE_HI ↔ GND.
- **GATE_HI** runs up x≈1040 to y≈1913, then right to x≈1298, then up. It reaches the node **D35 anode / R137 lower end**.
- **R137 1K**: that node ↔ D34 anode.
- **D34**: cathode (up) runs along x 1300 up to y 1357, right to x 2027, up to y 1158, then joins the **GATE** bus line (labelled "GATE", y≈1150). That bus goes right to x≈4148, which is the **J4 GATE** output jack.
- **D35**: cathode → R133 2.2K → node at the top of R131.
- **R131 220K** → **Q31 emitter**.

### Envelope trigger and the MEG (main envelope generator)
- **Q38 (PNP)**:
  - E → node **Q38E**; R150 22 connects VH ↔ Q38E; C55 47/16 (+) connects Q38E ↔ GND.
  - B = Q38B; R149 10K connects VH ↔ Q38B. R149's top joins R150's top at the VH arrow.
  - C = net **TRIG**; R151 22K connects TRIG ↔ GND.
- **TRIG** → D37 anode; D37 cathode → R152 100 → node **MEG_C**.
- **TRIG** also runs up x≈1093–1100 to y≈1969, then right to **D36** anode (the VCA envelope).
- **C62 1/35**: + at MEG_C, − at GND.
- **R138 68K**: MEG_C ↔ the line labelled **DECAY** (y≈2311). DECAY:
  - → IC12 pin 3.
  - → right to x≈2338, down to y≈2587, right to **VR6 pin 1**.
- **VR6 1M(A)**:
  - pin 2 (wiper) and pin 3 are tied together. That node runs left on y≈2628 to x≈2299, up to y≈2350, then left to x≈1376 and down to **IC12 pin 4**. Net **DEC_LOW**.
- **IC12 switch A**: pins 3 (DECAY) ↔ 4 (DEC_LOW), control pin 5 = ACCENT. When closed it shorts VR6.
- **Diode string from DEC_LOW to GND**: D28 (anode at DEC_LOW) → D27 → D26 (cathode at GND).
- **R117 22K**: ⊕5.333V ↔ the D27-cathode/D26-anode junction.
- **Q41 (NPN)**: B = MEG_C, C = VH, E → Q40 B.
- **Q40 (NPN)**: C = VH, E = net **ENV_OUT**.
- **Q39 2SK30(Y)**: D = ENV_OUT; G and S tied together to GND. This forms a constant-current pull-down.
- **ENV_OUT** runs right to x≈1413 and up the vertical (y 2870 → 2390). Along the way:
  - At (1414,2552), a dot → **IC12 pin 1**.
  - At the top, y≈2390, right to x≈2257, down to y≈2668, right → **VR5 pin 3**.

### ENV MOD (VR5)
- **VR5 50K(A)**:
  - pin 3 ← ENV_OUT.
  - pin 2 (wiper) → line y≈2269 → **R63 220K** → VCF Q10 base node (shared with R72 and R71 2.2K; VCF block).
  - pin 1 → **R61 10K** → node at (4100,2221). That node goes up x 4100 to the R47 10K / VR3 CUTOFF pin-1 line, and left along y≈2388 to x≈2858, the Q9/Q10/Q11 common line (VCF block).
- This matches p-08 Fig. 2: VR5 ③ gets the envelope, ② goes to the Q10 base, ① goes to the VQ9 bias.

### Accent level (VR7), accent → VCA and accent → VCF
- **IC12 switch B**: pin 1 = ENV_OUT ↔ pin 2, control pin 13 = ACCENT.
- Pin 2 → line y≈2430 → x≈2217 → down to y≈2708 → **VR7 pin 3**.
- **VR7 50K(B)**: pin 1 = GND; pin 2 (wiper) = net **ACC_LVL** (line y≈2547, x 2534–4301).
- **Accent → VCA**:
  - ACC_LVL → up x≈2537 → **R119 47K** → node **ACC_RC**.
  - **C36 .033** connects ACC_RC to GND.
  - ACC_RC → **R120 22K** → D27(VCA) anode. D27(VCA) cathode → down x≈2015 → node at (2012,1950) = **IC15 pin 1** = Q31 collector.
- **Accent → VCF**:
  - ACC_LVL → at (3737,2546) → **D24** anode. D24 cathode → **R46 47K** → line y≈2007 → x≈4383 → **VR4b pin 1** (RESONANCE second gang).
  - **VR4b pin 3** → line y≈1968 → **C13 1/50** (+). C13 − → GND.
  - **VR4b pin 2** (wiper) → line y≈2045 → x≈3816 → down to y≈2333 → **R72 100K** → VCF Q10 base node (same node as R63).

### VCA drive (gate/envelope → IC15 pin 1)
- D36 cathode → R134 22K → node **Q32B**; C41 .1 connects Q32B to GND.
- **Q32 (NPN)**: B = Q32B; E → R130 100 → GND; C → R132 100 → node **Q31B**.
- **R123 1.5M**: VH ↔ Q31B. **C42 1/35**: Q31B ↔ GND (polarity ?).
- **Q31 (PNP)**: E ← R131 220K ← R133 2.2K ← D35 ← GATE_HI. B = Q31B. C → IC15 pin 1 (also where D27(VCA) arrives).
- IC15 (BA662A) pins 2/3 bias network, pin 6/7/8, C38, C21/C22: VCA block, not transcribed in detail here.
  - R121 220K + C21 .01 → line → x≈2457 → y≈2468 → x≈3540 → up into VCF.
  - R122 100K + C22 .01 → x≈2416 → y≈2507 → x≈3581 → up to the C15/R67 node, VCF.
  - C38 output → x≈2496 → down to y≈2748 → VR8 VOLUME pin 3.

## 3. Annotations
- Next to Q39/Q40 there is an arrow pointing at the ENV_OUT node, with a sketch of an exponential decay from "100%" down through "10%", the interval marked "T", and an origin "0". Text: **"DECAY VR MAX T=2.5 sec.  MIN T=200ms."**
- The label **"DECAY"** is on the R138/IC12-pin-3/VR6-pin-1 line. **"ACCENT"** is on the Q35 collector line. **"SLIDE ON/OFF"** is on the line that branches from connector pin 8. **"GATE"** is on the bus line that D34 feeds (with "CV" above it and "TUNE" below).
- Pot legends on page 5: VR5 "50K(A) ENV.MOD", VR6 "1M(A) ENV.DECAY", VR7 "50K(B) ACCENT", VR4 "50K(B) RESONANCE" (dual, dashed gang).
- p-08 parts list: K161B0-50KB = TUNING, ACCENT; K161B0-50kA = CUT OFF FREQ, ENV MOD; K161B0-1MA = DECAY; K162T00W-50kB×2 = RESONANCE. Also HD14013BP or MN4013B, HD14066BP, BA662(A) "Vari-conductance amp", 2SK30TM(Y)/(O), 1S2473 Si diode.
- p-08 text: "Rotating wiper ② of VR5 closer to terminal ③ increases ENV MOD voltage being fed to base of Q10 while changing the bias…" Fig. 2 shows Q10/Q11 with VR5 ③ = env, ② → Q10 base, ① → VQ9.
- p-09 errata: the RESONANCE pot part number changed (13219774 → 13219775, K162T00W-50KBx2). The pot values match page 5. There are no errata in the ENV/accent area.
- p-07: no ENV/accent adjustment found (it covers VCO/VCF trims).

## 4. Functional description (from this transcription only)

**Accent latch.** The CPU puts the accent bit on pin 7, and IC13 latches it on the pin-8 strobe. That strobe also clocks IC9, and its line carries the "SLIDE ON/OFF" label. Q̄ is inverted by Q35, so ACCENT is high (pulled up to VH via 100K) when the latched bit is high. ACCENT drives the 4066 controls 5 and 13.

**Gate.** The pin-9 gate turns Q37 on, which turns Q36 on. Q36's collector (GATE_HI) does three things:
- drives the external GATE jack through R137 1K / D34;
- feeds the VCA control (Q31 emitter) through D35 / R133 2.2K / R131 220K;
- C72 10µ on Q36's base softens its switching.

**Trigger and envelope.** The falling edge on Q37's collector is AC-coupled through C54 .047 + R148 10K. It gives a short pulse that turns on Q38. Q38 dumps charge into C62 (1µF) through D37 + R152 100Ω; this is the instant attack. C62 then discharges through R138 68K + VR6 (0–1M rheostat) into a diode-string node: D28-D27-D26 to GND, with D26 pre-biased from 5.333V via R117 22K.
- Time constants: 68K·1µF ≈ 68 ms → time to 10% ≈ 157 ms. (68K+1M)·1µF ≈ 1.07 s → time to 10% ≈ 2.46 s. This agrees with the printed MIN 200 ms / MAX 2.5 s.
- **On accent, IC12 switch A shorts VR6**, so the decay is forced to the minimum value.
- The Q41/Q40 Darlington buffers C62, loaded by the Q39 JFET current sink, giving ENV_OUT.

**Where ENV_OUT goes.**
- Always to the top of VR5 (ENV MOD). VR5's wiper goes to the VCF Q10 base via R63 220K; its pin 1 goes to the Q9 bias / cutoff line via R61 10K.
- **Only during accent**, via IC12 switch B, to the top of VR7 (ACCENT). The VR7 wiper (accented envelope × accent level) then goes two ways:
  - **To the VCA**: R119 47K, C36 .033 to GND, R120 22K, D27(VCA), into IC15 pin 1. This is an accent boost of the VCA control current, added on top of the gate/env drive from Q31.
  - **To the VCF**: D24, R46 47K, the second gang of the RESONANCE pot (C13 1µF on its pin 3), then from the wiper through R72 100K to the Q10 base. This is the accent sweep, and its amount and smoothing depend on the resonance setting.

**VCA envelope.** TRIG (Q38 collector) also goes through D36 / R134 22K / C41 .1µ to Q32 → Q31 base (R123 1.5M, C42 1µ). This forms a VCA-side envelope at IC15 pin 1.

## 5. Uncertainties
1. **VH (filled-arrow rail)**: no voltage label found in my crops. It feeds R139, R142/R147, R149/R150, Q40/Q41 collectors, R123 and IC15 pin 9.
2. **Duplicate designator D27**: one is in the IC12 diode string (D28/D27/D26), the other in the VCA accent path (D27–R120). It is printed twice on page 5, and p-09 does not mention it.
3. **Pin 8 = IC9 CLK = IC13 T = "SLIDE ON/OFF"**: all three are clearly joined by dots. The label sits under a branch that goes right toward the VCO. The meaning ("slide" on the strobe line) is as printed; worth checking with the other reader.
4. **Connector pin 4** is printed as ","; I assume it is 4.
5. **C42 polarity** is smudged. **R126** is "2.?K".
6. **R147 22** is drawn with small bracket marks around it (maybe "optional/selected"?).
7. There is a stray junction dot on the R129/R162 top line at (2135,1776) with no visible branch. It is in the VCA block.
8. **IC12/IC13 supply pins**: 4066 VDD/VSS are not drawn near IC12. IC13 pin 14 is on +6V.
9. **Transistor part numbers**: none are printed for Q31, Q32 and Q35–Q38, Q40, Q41. Polarity comes from the arrow direction only.
10. **IC13 pin 12**: the overbar is on the printed "Q". In a 4013, pin 12 is Q̄2, which is consistent.

## Memory hints (unverified)
- Commonly cited TB-303 descriptions call the 4066 accent switching "accent shortens decay" and describe the VR4b/C13 network as the "accent sweep circuit". That is consistent with what is traced here, but it was not used to fill any gaps.
