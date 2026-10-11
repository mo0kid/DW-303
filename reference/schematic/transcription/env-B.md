# TB-303 ENV / accent block: transcription by reader envB

Source: p5-05.png (300 dpi). Coordinates in parentheses are approximate full-page pixel positions (x,y), so a cross-checker can find them.
Symbols: `▲` = filled supply arrow (rail value is not printed next to these symbols in this block), `△` = open supply triangle (IC13 pin 14), `⊕` = circled-cross reference. A boxed note on the page (≈1300–2000, 1380–1500) says "5.333V ⊕ connects to pin 7 of IC16 (POWER SUPPLY)", so ⊕ = +5.333V.

## 1. Component table

| Ref | Value as printed | Type / notes | Conf |
|---|---|---|---|
| IC13 | 14013 ("HD 14013") | dual D flip-flop. Only the section on pins 8–13 is drawn. Parts list: HD14013BP or MN4013B | ✓ |
| IC12 | 4066 | quad analog switch. Two sections used here: 3/4 (ctl 5) and 1/2 (ctl 13). Sections 8/9 (ctl 6) and 10/11 belong to the VCO slide | ✓ |
| Q35 | – (type not printed) | NPN by symbol | ✓ symbol |
| Q36 | – | PNP by symbol | ✓ symbol |
| Q37 | – | NPN | ✓ symbol |
| Q38 | – | PNP | ✓ symbol |
| Q39 | 2SK30(Y) | N-JFET, gate tied to source | ✓ |
| Q40 | – | NPN (Darlington with Q41) | ✓ symbol |
| Q41 | – | NPN | ✓ symbol |
| Q31 | – | PNP (VCA control) | ✓ symbol |
| Q32 | – | NPN (VCA envelope) | ✓ symbol |
| R137 | 1K | | ✓ |
| R139 | 100K | | ✓ |
| R140 | 100K | | ✓ |
| R141 | 100K | | ✓ |
| R142 | 10K | | ✓ |
| R143 | 10K | | ✓ |
| R144 | 10K | | ✓ |
| R145 | 10K | | ✓ |
| R146 | 22K | | ✓ |
| R147 | 22 | drawn with parentheses "( )" around the zigzag (optional/selected part?) | ✓ value, ? meaning |
| R148 | 10K | | ✓ |
| R149 | 10K | | ✓ |
| R150 | 22 | | ✓ |
| R151 | 22K | | ✓ |
| R152 | 100 | | ✓ |
| R138 | 68K | MEG discharge (fixed part) | ✓ |
| R117 | 22K | | ✓ |
| C54 | .047 | µF | ✓ |
| C55 | 47/16 | 47µF/16V electrolytic | ✓ |
| C62 | 1/35 | 1µF/35V electrolytic, + to the MEG node | ✓ |
| C72 | 10/16 | 10µF/16V electrolytic, + at the supply side | ✓ |
| D26, D27, D28 | – | diode string on IC12 pin 4 (type not printed) | ✓ |
| D34, D35, D36, D37 | – | diodes (type not printed) | ✓ |
| D24 | – | accent-to-VCF diode | ✓ |
| D27 (2nd) | – | **duplicate designator**: a second "D27" in the VCA accent path, next to R120 (≈2020,1673) | ✓ printed; designator conflict |
| R119 | 47K | | ✓ |
| R120 | 22K | | ✓ |
| C36 | .033 | µF | ✓ |
| R46 | 47K | | ✓ |
| C13 | 1/50 | 1µF/50V electrolytic | ✓ |
| R133 | 2.2K | | ✓ |
| R131 | 220K | | ✓ |
| R132 | 100 | | ✓ |
| R123 | 1.5M | | ✓ |
| C42 | 1/35 | 1µF/35V electrolytic; the polarity mark is obscured | ✓ value, ? polarity |
| R134 | 22K | | ✓ |
| C41 | .1 | µF | ✓ |
| R130 | 100 | | ✓ |
| R129 | 22K | VCA bias network | ? topology |
| R127 | 47K | | ✓ |
| R128 | 220K | | ✓ |
| R162 | 2.2K | | ✓ |
| C40 | 10/16 | | ✓ |
| R61 | 10K | ENV MOD pot pin 1 to the VCF bias node | ✓ |
| R63 | 220K | ENV MOD wiper to VCF (Q10 base node) | ✓ |
| VR5 | 50K(A) "ENV.MOD" | parts list: K161B0-50kA | ✓ |
| VR6 | 1M(A) "ENV.DECAY" | parts list: K161B0-1MA "DECAY" | ✓ |
| VR7 | 50K(B) "ACCENT" | parts list: K161B0-50KB "TUNING, ACCENT" | ✓ |

## 2. Netlist

### CPU interface (left-edge connector arrows 1..9)
These were traced by scanning pixel columns x≈499..778.
- Pins 1–6 go to IC9 (4174) data inputs: pin 2 → IC9-13, pin 3 → IC9-3, pin "," (4) → IC9-14, pin 5 → IC9-6, pin 6 → IC9-11. Pin 1 → IC9-4 (? taken from the routing order). These belong to the pitch DAC, not this block.
- **Pin 7 → IC13 pin 9 (D)** (vertical x≈737). Net `ACC_DATA`.
- **Pin 8 → IC9 pin 9 (CLK) = IC13 pin 11 (T) = the "SLIDE ON/OFF" horizontal** (vertical x≈778; junction dot at (778,1278)). Net `STROBE`. The drawing joins all four; see Uncertainties.
- **Pin 9 → R146 → Q37 base.** Net `GATE_CPU`.

### Accent latch
- IC13: pin 14 → △ supply; pin 7 → GND; pins 8 (S) and 10 (R) → GND; 9 D ← `ACC_DATA`; 11 T ← `STROBE`; 12 Q̄ → R141; Q (13) not drawn/unused.
- R141 100K: IC13-12 ↔ Q35 base. R140 100K: Q35 base ↔ GND.
- Q35: B as above; E → GND; C → `ACCENT` (node at (1220,1627)).
- R139 100K: ▲ ↔ `ACCENT`.
- `ACCENT` (labelled vertical x≈1217) → IC12 pin 5 (ctl, switch 3/4), plus a junction dot at (1217,2230) → IC12 pin 13 (ctl, switch 1/2).

### Gate driver
- R146 22K: `GATE_CPU` ↔ Q37 B. R144 10K: Q37 B ↔ GND. Q37: E → GND, C → `N_GATE` (vertical x≈835).
- R143 10K: `N_GATE` ↔ Q36 base.
- R142 10K: Q36 base ↔ `SUP36` (top node). C72 10/16: + `SUP36`, − Q36 base.
- `SUP36` → ▲. R147 22 ("( )"): `SUP36`/▲ ↔ Q36 emitter.
- Q36 (PNP): C → `GATE_INT`. R145 10K: `GATE_INT` ↔ GND.
- `GATE_INT` → line (1041→y1915→1298) → node `G1` (R137 bottom, D35 anode).
- R137 1K: `G1` ↔ D34 anode. D34 cathode → line up and right → the **GATE** bus (y≈1158, labelled "GATE"), which runs to the right-edge GATE jack J4.
- D35: anode `G1`, cathode → R133 2.2K → node `VCA_G` (≈1950,1770) → R131 220K → Q31 emitter.

### Trigger one-shot (Q38)
- C54 .047 and R148 10K are in series: `N_GATE` (junction ≈835,2860) ↔ C54 ↔ R148 ↔ Q38 base.
- R149 10K: ▲ ↔ Q38 base.
- R150 22: ▲ ↔ Q38 emitter. C55 47/16: + Q38 emitter, − GND.
- Q38 (PNP): C → `TRIG`.
- R151 22K: `TRIG` ↔ GND.
- `TRIG` → D37 anode, plus a line (x≈1097, y≈1970) → D36 anode.

### MEG (main envelope generator)
- D37 cathode → R152 100 → `MEG_C`.
- C62 1/35: + `MEG_C`, − GND.
- R138 68K: `MEG_C` ↔ `DECAY` (vertical x≈1143 up to y≈2310).
- `DECAY` horizontal (labelled "DECAY", y≈2310): → IC12 pin 3 (dot ≈1280,2310). It then runs right, jogs to y≈2588, and ends at **VR6 pin 1**.
- IC12 pin 4 → `DEC_LO`. `DEC_LO` runs y≈2350, then y≈2627, and ends at **VR6 pin 3**. VR6 pin 2 (wiper) is tied to pin 3. So VR6 is a variable resistor between `DECAY` and `DEC_LO`, and IC12 3/4 (closed when ACCENT is active) shorts it.
- `DEC_LO` → D28 anode; D28 cathode → D27 anode; D27 cathode → `DREF`; D26 anode `DREF`, cathode → GND. All three are drawn pointing down.
- R117 22K: ⊕ (+5.333V) ↔ `DREF`.
- Q41: B ← `MEG_C`; C → ▲ (shared top node with the Q40 collector); E → Q40 B.
- Q40: C → ▲; E → `MEG_OUT`.
- Q39 2SK30(Y): D → `MEG_OUT`; G and S tied → GND.
- `MEG_OUT` (vertical x≈1413) → IC12 pin 1 (dot ≈1413,2550). It also runs y≈2390, then y≈2668, and ends at **VR5 pin 3**.

### Accent gating and ACCENT pot
- IC12 pin 2 → `ACC_ENV`. `ACC_ENV` runs y≈2430, then y≈2708, and ends at **VR7 pin 3**. VR7 pin 1 → GND. VR7 pin 2 (wiper) → `ACC_W` (y≈2547).
- `ACC_W` → vertical x≈2536 → R119 47K → `ACC_RC` (dot ≈2207,1673).
- C36 .033: `ACC_RC` ↔ GND.
- R120 22K: `ACC_RC` ↔ D27(2nd) anode. D27(2nd) cathode → line x≈2013 → IC15 pin 1 node (dot ≈2013,1953). That node also connects to the Q31 collector.
- `ACC_W` also feeds D24 (junction dot ≈3737,2547): D24 anode `ACC_W`, cathode → R46 47K → line y≈2008 → VR4b (RESONANCE gang) pin 1. VR4b pin 3 ← C13 + (1/50, − to GND). VR4b wiper → line y≈2050 → x≈3815 down → R72 100K (VCF). This is VCF territory and only partly checked.

### ENV MOD pot
- VR5 50K(A): pin 3 = `MEG_OUT`. Pin 2 (wiper) → y≈2165 → x≈3658 → R63 220K → Q10 base node (VCF; shared with R71 2.2K and R72 100K).
- VR5 pin 1 → R61 10K → node (≈4100,2220). That node also connects to R47 10K → VR3 (CUTOFF) pin 1, and via line x≈3700/y≈2387 to the Q9/Q11/Q10/R71 lower bus (≈3250,2389) in the VCF. This matches the p-08 Fig. 2, where VR5 pin 1 returns to the Q9 (VQ9) bias node.

### VCA envelope path (accent and gate reach the VCA)
- D36: anode `TRIG`, cathode → R134 22K → Q32 base node. C41 .1: that node ↔ GND.
- Q32 (NPN): E → R130 100 → GND; C → R132 100 → `VCA_ENV`.
- `VCA_ENV`: R123 1.5M to ▲; C42 1/35 to GND; Q31 base.
- Q31 (PNP): E ← R131 220K ← `VCA_G` (gate via D35/R133); C → IC15 pin 1 (also receives the accent via D27(2nd)).
- IC15 bias: R162 2.2K connects ⊕ to `VBUS` (≈1778). On `VBUS`: C40 10/16 (+) to GND, R128 220K, and R129 22K. R129 → R127 47K top. R127 bottom and R128 bottom join → IC15 pin 6 → pin 7 node.
- Not this block, but noted: IC15 output C38 1/50 → line y≈2748 → VR8 (VOLUME) pin 3. R121 220K/C21 .01 and R122 100K/C22 .01 run to y≈2470/2508, then up via x≈3540/3580 (VCF/VCA reader).

## 3. Annotations
- Under Q39: "DECAY VR MAX T=2.5 sec. MIN T=200ms."
- Small graph with an arrow to the `MEG_OUT` node: an exponential decay from 100%, with T measured from 0 to the 10% crossing.
- Labels: "ENV" box, "ACCENT" on the vertical, "DECAY" on the horizontal, "VCA", "SLIDE ON/OFF", "GATE".
- p-08 (VCF ENVELOPE MODULATION): VR5 wiper ② feeds the base of Q10. Turning ② toward ③ increases ENV MOD and changes the bias, which lowers cutoff. Q9 sets the initial cutoff bias (VQ9), and VR5 pin ① returns to the Q9 bias (Fig. 2).
- p-08 parts list:
  - pots: K161B0-50KB (TUNING, ACCENT), K161B0-50kA (CUTOFF FREQ, ENV MOD), K161B0-1MA (DECAY)
  - FET: 2SK30TM(Y)/(O)
  - diodes: 1S2473, 1SS-133, 1S-188FM etc. The per-designator mapping is not given.
- p-07 VCF adjustment: ENV MOD, DECAY, ACC full counterclockwise for the TM3 adjustment.
- p-09 errata: nothing relevant to this block (IC10 pin numbers, power-supply Q42/C66/C67, resonance pot part number, added transistor types).

## 4. Functional description (from this transcription only)
1. CPU pin 9 (GATE) turns on Q37, which pulls Q36's base low. Q36 (PNP) then drives `GATE_INT` high for the length of the gate. `GATE_INT` drives:
   - the GATE output jack via R137/D34;
   - the VCA gate current via D35 → R133 → R131 → Q31 emitter.
2. The falling edge on `N_GATE` (gate start) is AC-coupled through C54/R148 into Q38. Q38 gives a short positive pulse `TRIG`, roughly R148·C54 ≈ 0.5 ms scale. The pulse:
   - quickly charges C62 (1µF) via D37/R152 100Ω, which is the MEG attack;
   - charges C41 via D36/R134 for the VCA envelope (Q32 → C42/R123 → Q31).
3. C62 discharges through R138 68K + VR6 (0–1M) into a 3-diode clamp. The clamp is D28 + D27 to node `DREF`, which R117 from +5.333V and D26 hold at about one diode drop. So the envelope decays toward roughly 3 diode drops. Printed decay times are 200 ms (min) to 2.5 s (max). When ACCENT is active, IC12 3/4 shorts VR6, so decay is set by R138 alone (shortest).
4. Q41/Q40 buffer the C62 voltage as a Darlington follower, with Q39 as a JFET current-sink load, giving `MEG_OUT`. `MEG_OUT` drives:
   - the ENV MOD pot (VR5) → R63 → VCF Q10 base;
   - IC12 1/2. This switch is closed only while ACCENT is active, and passes the envelope to the ACCENT pot VR7.
5. The VR7 wiper (`ACC_W`) goes to two places:
   - R119/C36 (47K·.033µF) smoothing → R120 → D27(2nd) → IC15 pin 1, which adds accent to the VCA control current;
   - D24 → R46 47K → resonance pot VR4b / C13 (1µF), the accent-sweep path into the VCF.
6. The accent latch: CPU pin 7 is clocked into IC13 by pin 8 (the same strobe that clocks the IC9 pitch latch). Q̄ drives Q35, an inverter, so ACCENT = latched pin 7, active high with R139 pull-up.

## 5. Uncertainties
- **D27 duplicate**: D27 appears both in the D28/D27/D26 decay clamp and in the VCA accent path (D27–R120). One of them is probably a misprint, and p-09 does not correct it.
- Supply rails for the ▲ arrows (R139, R142/R147, R149/R150, Q40/Q41 collectors, R123) and the △ at IC13 pin 14 are not labelled in this block.
- The pin 8 net: the drawing shows IC9 CLK, IC13 T, the "SLIDE ON/OFF" line and CPU pin 8 all joined at a dot (778,1278). The pixel scan confirms one continuous line.
- Pin 1 → IC9-4 was inferred from the routing order, not fully traced.
- R129 22K topology: drawn between `VBUS` and the R127 top, with an extra junction dot on `VBUS` (≈2136,1777) that has no visible third branch.
- C42 polarity mark is obscured.
- R147 is drawn with parentheses; the meaning (optional, or not fitted on some units?) is unknown.
- Diode and transistor part types are not printed per designator in this block (only Q39 2SK30(Y)).
- C13/VR4b/R72 accent-to-resonance wiring was traced only coarsely (VCF reader to confirm).
- IC12/IC13 VDD/VSS pins: only IC13 pins 14/7 are drawn.

## Memory hints (unverified)
- In common TB-303 lore, C13 is the "accent sweep" capacitor and the resonance pot gang sets how much accent pushes the cutoff. That is consistent with the above.
- The usual Roland pattern is filled ▲ = +12V and △ = +5V or the logic supply. This is not verified on this page.
- The diode string is often described as clamping the MEG floor. Typical diodes are 1S2473.
