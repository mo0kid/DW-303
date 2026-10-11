# TB-303 page 5: ENV / MEG / ACCENT block (reconciled)

Sources: env-A.md and env-B.md, cross-checked against p5-05.png (300 dpi) and p-07/p-08/p-09.
Coordinates are (x,y) pixel positions in p5-05.png. Crops used for verification are listed in the Resolution log.

Scope: gate and accent logic, MEG (C62 envelope, decay network, DECAY pot, accent short-decay), ENV MOD feed, the ACCENT pot, and accent routing to the VCA and VCF. Parts of the VCA drive (Q31/Q32, IC15 pin 1) are included because the gate, trigger and accent all end there.

## Rails (resolved)

| Symbol | Value | Evidence |
|---|---|---|
| Filled arrow ▲ | **+12V** | Labelled "12V" at IC16 in the POWER SUPPLY block. Crop (2300,250)-(3400,1100); the arrow at ≈(2668,680) carries the "12V" label. |
| Open triangle △ | **+6V** | Labelled "+6V" in POWER SUPPLY (≈3700,350). Used at IC13 pin 14. |
| Circled cross ⊕ | **+5.333V** | IC16 pin 7 output, labelled "5.333V". Boxed note: "5.333V ⊕ connects to pin 7 of IC16". |
| Hatched symbol | GND | |

Neither reader had labelled ▲. Both A and B left it as unresolved; it is now resolved as +12V.

## 1. Component table (reconciled)

Both readers agree on every value. Transistor types are not printed per designator on page 5, except Q39.

| Ref | Value | Type / role | Conf. |
|---|---|---|---|
| IC13 | HD14013 | 4013 dual D flip-flop, section on pins 8–13 (accent latch) | ✓ |
| IC12 | 4066 | quad analog switch. Sections 3/4 (ctl 5) and 1/2 (ctl 13) are used here; 8/9 and 10/11 belong to the VCO slide | ✓ |
| Q35 | – | NPN, accent inverter | ✓ |
| Q36 | – | PNP, gate high-side switch | ✓ |
| Q37 | – | NPN, gate input inverter | ✓ |
| Q38 | – | PNP, trigger pulse switch | ✓ |
| Q39 | 2SK30(Y) | N-JFET, G tied to S tied to GND, constant-current sink (Idss) | ✓ |
| Q40, Q41 | – | NPN Darlington emitter follower (both emitter arrows outward, verified) | ✓ |
| Q31 | – | PNP, VCA control current source | ✓ |
| Q32 | – | NPN, VCA envelope discharge | ✓ |
| R117 | 22K | | ✓ |
| R119 | 47K | | ✓ |
| R120 | 22K | | ✓ |
| R123 | 1.5M | | ✓ |
| R130 | 100 | | ✓ |
| R131 | 220K | | ✓ |
| R132 | 100 | | ✓ |
| R133 | 2.2K | | ✓ |
| R134 | 22K | | ✓ |
| R137 | 1K | | ✓ |
| R138 | 68K | | ✓ |
| R139, R140, R141 | 100K | | ✓ |
| R142, R143, R144, R145 | 10K | | ✓ |
| R146 | 22K | | ✓ |
| R147 | 22 | drawn with "( )" brackets around the zigzag | ✓ value, ? meaning |
| R148, R149 | 10K | | ✓ |
| R150 | 22 | | ✓ |
| R151 | 22K | | ✓ |
| R152 | 100 | | ✓ |
| R46 | 47K | | ✓ |
| R61 | 10K | | ✓ |
| R63 | 220K | | ✓ |
| R72 | 100K | | ✓ |
| C13 | 1/50 | 1µF/50V electrolytic, + toward VR4b pin 3 | ✓ |
| C36 | .033 | µF | ✓ |
| C41 | .1 | µF | ✓ |
| C42 | 1/35 | 1µF/35V electrolytic, + toward the Q31 base node (see log) | ✓ value, polarity resolved |
| C54 | .047 | µF | ✓ |
| C55 | 47/16 | electrolytic, + at Q38 emitter | ✓ |
| C62 | 1/35 | electrolytic, + at MEG_C | ✓ |
| C72 | 10/16 | electrolytic, + at the +12V side | ✓ |
| D24, D26, D27, D28, D34, D35, D36, D37 | – | Si diodes. The parts list gives 1S2473 / 1SS-133 / 1S-188FM, with no per-designator mapping | ✓ |
| D27 (2nd) | – | **duplicate designator**, in the VCA accent path | ✓ printed |
| VR4b | 50K(B)×2, second gang | RESONANCE, K162T00W-50KB×2 | ✓ |
| VR5 | 50K(A) ENV.MOD | K161B0-50kA | ✓ |
| VR6 | 1M(A) ENV.DECAY | K161B0-1MA | ✓ |
| VR7 | 50K(B) ACCENT | K161B0-50kB | ✓ |

## 2. Netlist (reconciled)

### CPU connector, pins 7/8/9
- **Pin 7** → vertical x≈739 → IC13 pin 9 (D). Net `ACC_DATA`.
  - It crosses the pin-8 vertical at y≈1510 with no dot.
- **Pin 8** → vertical x≈777–780. Net `STROBE`. It joins three things:
  - IC9 pin 9 (4174 CLK) at the top;
  - IC13 pin 11 (T), with a junction dot at (777,1573);
  - the "SLIDE ON/OFF" horizontal, with a junction dot at (777,1279).
  - The SLIDE ON/OFF line runs right along y≈1278 to x≈1462, then up to R59 100K → Q29 base (VCO slide driver: Q29/Q30 → IC12 pin 6 → slide switches 8/9 and 10/11).
  - **One drawn net covers the pitch-latch clock, the accent-latch clock and the slide control.**
- **Pin 9** → R146 22K → Q37 base. Net `GATE_IN`.
- Pins 1–6 go to the IC9 data inputs (DAC block, out of scope).

### Accent latch
- **IC13**:
  - pin 14 → +6V; pin 7 → GND; pins 8 (S) and 10 (R) → GND;
  - pin 9 D ← `ACC_DATA`; pin 11 T ← `STROBE`;
  - pin 12 Q̄ → R141 100K → `Q35B`; pin 13 Q is not drawn.
- R140 100K: `Q35B`–GND.
- Q35: B = `Q35B`, E = GND, C = `ACCENT`.
- R139 100K: +12V–`ACCENT`.
- `ACCENT` (labelled vertical x≈1217) → IC12 pin 5. A dot at (1217,2230) branches to IC12 pin 13.
- Logic: ACCENT = latched pin-7 level. 0V when not accented, about +12V (via 100K) when accented.

### Gate
- Q37 (NPN): B ← R146 from pin 9, R144 10K B–GND; E = GND; C = `N_GATE` (x≈835).
- R143 10K: `N_GATE`–`Q36B`.
- R142 10K ∥ C72 10/16 (+ at the rail): +12V–`Q36B`.
- Q36 (PNP): E → R147 22 → +12V; B = `Q36B`; C = `GATE_HI`.
- R145 10K: `GATE_HI`–GND.
- `GATE_HI` → node G1 (R137 lower end, D35 anode).
  - R137 1K → D34 anode; D34 cathode → `GATE` bus (y≈1150) → J4 GATE jack.
  - D35 cathode → R133 2.2K → R131 220K → Q31 emitter.

### Trigger pulse generator (Q38)
- `N_GATE` (dot ≈835,2860) → C54 .047 → R148 10K → `Q38B`.
- R149 10K: +12V–`Q38B`.
- R150 22: +12V–`Q38E`. C55 47/16: + `Q38E`, − GND. This is a local reservoir for the pulse current.
- Q38 (PNP): C = `TRIG`.
- R151 22K: `TRIG`–GND.
- `TRIG` → D37 anode. `TRIG` also → x≈1093 up → D36 anode (VCA envelope).

### MEG
- D37 cathode → R152 100 → `MEG_C`. C62 1/35: + `MEG_C`, − GND.
- R138 68K: `MEG_C`–`DECAY`.
  - `DECAY` runs up x≈1145 to y≈2312, then along the labelled "DECAY" line.
  - It reaches IC12 pin 3 (dot ≈1281,2312), then runs right to VR6 pin 1 (via y≈2587).
- IC12 pin 4 = `DEC_LO`.
  - Route: dot ≈1281,2452 → x≈1377 → y≈2352 → right → y≈2625.
  - It ends at VR6 pin 3. **VR6 wiper (pin 2) is tied to pin 3**, by a short loop with a dot on the pin-3 line.
- `DEC_LO` → D28 (A→K) → D27 (A→K) → `DREF` → D26 (A→K) → GND. All three point down.
- R117 22K: +5.333V–`DREF`. The junction dot is between the D27 cathode and the D26 anode (verified at 2× zoom).
- Q41: B = `MEG_C`; C = +12V (shared with the Q40 collector); E → Q40 B.
- Q40: C = +12V; E = `ENV_OUT`.
- Q39: D = `ENV_OUT`; G = S = GND.
- `ENV_OUT` (vertical x≈1413):
  - → IC12 pin 1 (dot ≈1413,2552);
  - → y≈2390 → x≈2257 → y≈2668 → VR5 pin 3.
- IC12 switch 3/4, control pin 5 = ACCENT. When closed it shorts VR6.

### ENV MOD (VR5)
- VR5 pin 3 ← `ENV_OUT`.
- Wiper → line y≈2165 → x≈3658 → y≈2268 → R63 220K → VCF Q10 base node (shared with R71 2.2K and R72 100K).
- Pin 1 → R61 10K → node (≈4100,2220). That node goes to:
  - R47 10K / VR3 CUTOFF pin 1;
  - the Q9/Q10/Q11 bias line, y≈2388 (VCF block).
- This agrees with p-08 Fig. 2.

### Accent switch, ACCENT pot and routing
- IC12 switch 1/2: pin 1 = `ENV_OUT`, pin 2 = `ACC_ENV`, control pin 13 = ACCENT.
- `ACC_ENV` → y≈2430 → x≈2217 → y≈2705 → VR7 pin 3.
- VR7: pin 1 = GND, wiper = `ACC_W` (line y≈2547).
- **To the VCA**:
  - `ACC_W` → x≈2537 up → R119 47K → `ACC_RC` (dot ≈2207,1673).
  - C36 .033: `ACC_RC`–GND.
  - R120 22K → D27(2nd) anode; its cathode → x≈2013 down → IC15 pin 1 (dot ≈2013,1953). The Q31 collector is on the same node.
- **To the VCF**:
  - `ACC_W` (dot ≈3737,2547) → D24 anode; cathode → R46 47K → y≈2008 → VR4b pin 1.
  - VR4b pin 3 → y≈1968 → C13 + (− GND).
  - VR4b wiper → y≈2045 → x≈3815 → down → R72 100K → Q10 base node.

### VCA envelope and gate current (into IC15 pin 1)
- D36 (from `TRIG`) → R134 22K → `Q32B`. C41 .1: `Q32B`–GND.
- Q32 (NPN): E → R130 100 → GND; C → R132 100 → `Q31B`.
- R123 1.5M: +12V–`Q31B`. C42 1/35: + `Q31B`, − GND.
- Q31 (PNP): E ← R131 ← R133 ← D35 ← `GATE_HI`; B = `Q31B`; C = IC15 pin 1.
- IC15 bias (VCA block, noted because of R129):
  - R162 2.2K: ⊕–`VBUS` (y≈1777). C40 10/16 (+) runs from `VBUS` to GND.
  - **R129 22K and R127 47K are in series** from `VBUS` to the IC15 pin 6/7 node. **R128 220K runs from `VBUS` straight to the same pin 6/7 node**, so it is in parallel with the R129+R127 string.
  - Net effect: 69K ∥ 220K = 52.5K from `VBUS` to pins 6/7.

## 3. Annotations
- Under Q39/Q40: decay sketch (100% → 10%, interval T) with an arrow to `ENV_OUT`. Text: "DECAY VR MAX T=2.5 sec. MIN T=200ms."
- Printed labels: "ENV", "ACCENT" (Q35 C vertical), "DECAY" (R138/IC12-3/VR6-1 line), "SLIDE ON/OFF" (pin-8 branch), "GATE" (D34 bus), "VCA".
- p-07 VCF adjustment, TM3 at TP6: CUTOFF center, SAWTOOTH, RES full CW, **ENV MOD, DECAY, ACC full counterclockwise**. Reader A missed this; it was verified on p-07.
- p-08: ENV MOD text and Fig. 2, as in the ENV MOD section above. Parts list:
  - 50kB = TUNING, ACCENT; 50kA = CUTOFF, ENV MOD; 1MA = DECAY;
  - 2SK30TM(Y)/(O); HD14013BP/MN4013B; HD14066BP; 1S2473 (and others).
- p-09 errata:
  - The RESONANCE pot part number changes from 13219774 to 13219775 (K162T00W-50KB×2). The value is unchanged.
  - Transistor list additions: 2SC536(F), 2SA937(Q) or 2SB642(R), 2SD637(R) as an alternative for 2SC2021.
  - **No corrections affect this block.** The D27 duplicate is not addressed.

## 4. Resolution log

| # | Item | A | B | Decision / evidence |
|---|---|---|---|---|
| 1 | ▲ rail | unlabelled ("VH") | unlabelled | **+12V**. Label "12V" on the same symbol at IC16, crop (2300,250)-(3400,1100). |
| 2 | Pin 8 net / SLIDE ON/OFF | dots joining IC9 CLK, IC13 T and the label line | same, dot (778,1278) | **Confirmed.** Dots at (777,1279) and (777,1573). Crop (700,1200)-(900,1620) at 2×. A column scan of x=778 shows a continuous line. The line continues to R59 → Q29 (VCO slide driver), crop (350,200)-(1700,1350). The pin-7 line crosses at y≈1510 without a dot. |
| 3 | R117 junction / clamp | D27 K / D26 A | same | **Confirmed**, crop (1100,2250)-(1500,2650) at 2×. |
| 4 | VR6 wiring | DECAY → pin 1; wiper tied to pin 3 = DEC_LO | same | **Confirmed**, crop (3550,1900)-(4600,2800): pin 1 → y≈2585 line, pin 3 + wiper loop → y≈2625 line. VR6 is a rheostat between DECAY and DEC_LO equal to R(1–2). |
| 5 | IC12 switches driven by ACCENT | 3/4 (ctl 5) shorts VR6; 1/2 (ctl 13) ENV_OUT → VR7 | same | **Confirmed**, crop (1100,2150)-(2700,3000). Control lines 5 and 13 both come from the ACCENT vertical. Switches 8/9 and 10/11 (ctl 6) are slide, not accent. |
| 6 | Q41/Q40/Q39 | NPN Darlington + JFET sink | same | **Confirmed**, crop (1100,2650)-(1340,2920) at 4×. Both NPN; collectors on a +12V arrow; Q39 D at the emitter node, G–S boxed to GND. |
| 7 | D27 duplicate | printed twice | printed twice | **Confirmed printed twice**: diode string ≈(1180,2540) and VCA path ≈(1990,1673). Errata silent. Kept as D27 (string) and D27′ (VCA). See Open items. |
| 8 | C42 polarity | smudged, probably + at Q31B | obscured | **+ at the Q31 base node.** Crop (1780,1870)-(1980,2020) at 4×: a blob above-right of the body, merged with "1" of 1/35, on the upper (Q31B) side. Also electrically required: the node idles near +12V via R123 and the other side is GND. |
| 9 | C72 polarity | + toward rail, "?" | + at supply | **+ at the +12V side**, crop (800,2000)-(1200,2350) at 2×. The "+" sits beside the upper plate. |
| 10 | R129 wiring | not detailed | R129 → R127 top; "? topology" | **R129 + R127 in series, with R128 in parallel**, crop (2050,1740)-(2300,2120) at 3×. Stray dot at (≈2137,1777) on the top line has no third branch (dot only). |
| 11 | VR5 wiper route | y≈2269 | y≈2165 → x≈3658 | **Both partial, same net.** Wiper leaves at y≈2165, jogs at x≈3660 down to y≈2268, then to R63. |
| 12 | Q38 collector route to D36 | x≈1093 up | x≈1097 | **Same**, crop (900,2450)-(1300,2900): the Q38 C node at y≈2717 → right to x≈1092 → up. |
| 13 | p-07 ENV/ACC settings | "none" | ENV MOD/DECAY/ACC full CCW for TM3 | **B correct** (p-07, VCF column). |
| 14 | Pin "," = 4 | assumed | assumed | Out of scope; accepted. |

## 5. Open items

1. **Duplicate D27.** Electrically both are plain series Si diodes, so there is no model impact. The BOM/reference ambiguity remains; check the PCB silk on p-07 if needed.
2. **R147 22Ω in brackets "( )".** It may mean optional/selected, or not fitted on some units.
   - Fitted: Q36 emitter degeneration of 22Ω, negligible.
   - Not fitted: the emitter is either tied directly to +12V or open. If open, Q36 cannot work, so it is presumably a jumper.
   - Either way the model impact is nil, provided the emitter goes to +12V.
3. **Pin 8 = latch clock = SLIDE ON/OFF.** The drawing is unambiguous, but the functional meaning is unexplained on the page. Either the CPU holds pin 8 at a level that both acts as the strobe edge and sets slide, or the drawing simplifies. Model consequence: the slide state and the accent/pitch latch timing share one CPU line. Confirm against CPU firmware or other sources before modelling sequencer timing.
4. **4066 VDD/VSS (IC12) are not drawn.** ACCENT swings to +12V via R139, so VDD = +12V is likely but unverified. This affects switch Ron (≈100–500Ω, negligible against 68K or 50K).
5. **Diode and transistor types per designator** are not printed (β, Vf assumed generic Si / 1S2473-class).

## 6. Model-relevant values (derived from the reconciled netlist)

Assumptions: Vd ≈ 0.6V at about 0.1–1mA, falling to about 0.4V at µA levels; Vbe ≈ 0.6V; Vce(sat) ≈ 0.1–0.2V; rails +12V / +5.333V. Values marked "est." are first-order hand estimates; verify in SPICE.

**Accent logic**
- ACCENT high is about +12V through R139 100K. Q35 is driven by Q̄ via the R141/R140 100K/100K divider, so its base gets ~3V at 6V logic and saturates easily.
- ACCENT high closes IC12 3/4 (decay → minimum) and IC12 1/2 (envelope → ACCENT pot).

**Gate (Q37/Q36)**
- Q36 base drive: R143 10K from `N_GATE` with R142 10K ∥ C72 10µF across rail–base, which is effectively across Q36 B–E (+R147).
- C72 is clamped to about Vbe, so it slows the edges.
  - Turn-on delay (est.): C72·Vbe/I_R143 ≈ 10µF·0.6V / 1.1mA ≈ **5 ms**.
  - Turn-off: C72 discharges through Q36 B–E and R142 (τ_R142 = 100 ms max). The result is a soft tail of a few ms.
- GATE_HI is about 11.8V when on. R145 10K is the load.

**Trigger pulse (Q38)**
- On the gate-on falling edge of `N_GATE`, C54 couples a negative step through R148 into the Q38 base, and B–E clamps.
- Initial base current ≈ (12 − 0.6)/R148 ≈ 1.1 mA.
- It decays with τ = R148·C54 = 10K·0.047µF = **0.47 ms**.
- Q38 conducts until I_R148 < Vbe/R149 ≈ 0.06 mA, i.e. for t ≈ τ·ln(1.1/0.06) ≈ **1.4 ms** (est.). The hard-saturated part is roughly 0.5–1 ms.
- Gate-off produces a positive edge, so Q38 stays off (reverse-biased base, limited by R148/R149).
- Supply decoupling: R150·C55 = 22Ω·47µF = **1.03 ms**. C62 charge (≈11µC) sags C55 by about 0.23V.
- R151 22K pulls `TRIG` to 0 between pulses; D37 and D36 then block.

**MEG charge (attack)**
- Path: Q38 (sat) → D37 → R152 100Ω → C62 1µF, τ_charge = R152·C62 = **100 µs** (+ R150 22Ω and Q38 Rce).
- The pulse lasts about 5–10 τ, so C62 charges to nearly the full peak: V_peak ≈ 12 − Vce(sat) − Vd(D37) ≈ **11.2 V**.
- Attack is effectively instantaneous (~0.1–0.3 ms rise).

**MEG discharge (decay)**
- Discharge resistance: R_dec = R138 + R_VR6(1→wiper) = 68K + 0…1M (1M audio taper, wiper shorted to pin 3).
- Accent: IC12 3/4 shorts VR6, so R_dec = 68K + Ron.
- Clamp:
  - DREF current = (5.333 − 0.6)/22K ≈ **0.215 mA** through D26, so V_DREF ≈ **0.55–0.6 V**.
  - DEC_LO floor = V_DREF + Vd(D28) + Vd(D27). That is ≈ 1.7 V at the start of the decay (tens of µA through D28/D27) and creeps to ≈ 1.3–1.5 V as the current falls toward 0 (diode log law). Nominal **V_floor ≈ 3·Vd ≈ 1.5–1.8 V**.
- Decay law: V_C62(t) ≈ V_floor + (V_peak − V_floor)·e^(−t/τ). The tail is slower than exponential near the floor, because the diode Vf drops with current.
- τ_min = 68K·1µF = **68 ms**, so t(100%→10%) = τ·ln10 = **157 ms**.
- τ_max = 1.068M·1µF = **1.07 s**, so t(10%) = **2.46 s**.
- Printed values are MIN 200 ms and MAX 2.5 s. The max matches; the min is 27% high, consistent with electrolytic tolerance of C62 and pot end resistance.
- **Accent decay = τ_min = 68 ms**, independent of the DECAY knob.
- Darlington load on C62: I_B ≈ I_Q39 / β² ≈ 1–3 mA / ~10⁴ ≈ 0.1–0.3 µA. Against 10 µA through 1M at 10V, this is about 2%, and it matters only at the max-decay tail.

**ENV_OUT**
- ENV_OUT = V_C62 − 2·Vbe (Q41 + Q40), with the Q39 sink at Idss.
- 2SK30 Y-rank Idss is 1.2–3.0 mA (datasheet range, not printed on page 5).
- Peak ≈ 11.2 − 1.3 ≈ **9.9 V**.
- Rest ≈ V_floor − 2·Vbe ≈ **0.2–0.5 V** (est.). The 3-diode clamp roughly cancels the Darlington 2·Vbe and leaves about one diode drop of offset.
- Source impedance is low when sourcing current; sink capability is limited to Idss.

**ENV MOD (VR5 50K(A), log)**
- ENV_OUT → VR5 track (50K) → R61 10K → VCF bias node.
- Wiper → R63 220K → Q10 base.
- The fraction of ENV_OUT at the wiper is about (R_1w + 10K)/(60K) relative to the bias node, with an audio taper.
- R63 220K sets the injection current into the Q10 base node (shared with R72 100K and R71 2.2K, VCF block).

**Accent amount (VR7 50K(B), linear)**
- Input = ENV_OUT only while ACCENT is high (IC12 1/2 Ron ≈ 0.1–0.5K).
- Wiper fraction k = linear rotation. Wiper source impedance ≤ 12.5K.

**Accent → VCA**
- Smoothing: (R119 + R_w)·C36 = 47K…59.5K × 0.033µF = **1.55–1.96 ms**.
- Then R120 22K + D27′ inject current into IC15 pin 1: I ≈ (V_C36 − Vd − V_pin1)/22K, plus the R119 path. The source Thevenin is ≈ R120 + R119∥… so it is effectively ~22K–69K.
- At V_ACC ≈ 9.9V·k, the peak current is roughly 0.1–0.4 mA·k (est.; V_pin1 is set by the VCA block).

**Accent → VCF (accent sweep)**
- D24 (one-way) → R46 47K → VR4b pin 1. The VR4b track (50K, linear) runs to pin 3 and C13 1µF.
- Wiper → R72 100K → Q10 base.
- C13 charge τ ≈ (R46 + 50K)·C13 = **97 ms**, ignoring the R72 load. With the R72 load the Thevenin is lower, depending on wiper position.
- Discharge cannot go back through D24. It goes only through the track, wiper and R72 into the Q10 base node, which is the slow path. As a result **C13 accumulates across consecutive accents**.
- The RESONANCE position selects the mix:
  - wiper at pin 1: mostly direct (fast);
  - wiper at pin 3: mostly the C13 voltage (smoothed/accumulated).

**VCA envelope (Q32/Q31)**
- C41 charge from `TRIG` via D36 + R134: τ = 22K·0.1µF = **2.2 ms**. In a ~1 ms pulse it reaches about 35–40% of 11V, roughly 4V, which turns Q32 on hard.
- C41 then discharges through the Q32 base (R130 100Ω emitter). Q32 conducts for a few ms (est.), pulling C42 toward 0 through R132 100Ω (τ = 100 µs).
- C42 recovers through R123: τ = 1.5M·1µF = **1.5 s**. The VCA decay is fixed and not tied to the DECAY knob.
- Q31 current: I ≈ (V_GATE_HI − Vd(D35) − Vbe − V_C42)/(R133 + R131 = 222.2K).
  - Max ≈ (11.8 − 0.6 − 0.6 − 0.2)/222.2K ≈ **47 µA** (est.).
  - It falls to 0 as V_C42 approaches ~10.6V; from 0.2V that takes ≈ 1.5 s·ln(11.8/1.4) ≈ **3.2 s** (est.).
  - The current is cut immediately when the gate ends, because the emitter supply is removed.
- Accent current via D27′ adds to IC15 pin 1.

**IC15 bias (VCA block, for reference)**
- `VBUS` = 5.333V via R162 2.2K with C40 10µF (τ = 22 ms).
- R129 + R127 = 69K in parallel with R128 220K gives **52.5K** to IC15 pins 6/7.

## Memory hints (unverified)
- Common descriptions: "accent forces the shortest decay", and "C13 / RESONANCE gang = accent sweep circuit". Both are consistent with the netlist above; neither was used to fill gaps.
- Typical 303 docs describe the 3-diode string as setting the MEG floor. That is consistent with item 3 of the Resolution log.
