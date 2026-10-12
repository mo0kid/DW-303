# VCF loop gain: re-check against the scan

Status: re-check done 2026-10-12. Follows `vcf-resonance.md` (DC loop gain ≈ 1.9).
TD-03 measurements show a resonant peak of +19 to +35 dB, so this re-check looks for the
missing ~8–10× of loop gain.

Crops are from `p5-05.png` (300 dpi), enlarged 2–7×, saved as `/tmp/vcfcheck-*.png`.
Coordinates below are source pixels.

## 1. Item-by-item check against the scan

| # | Item | What the scan shows | Matches transcription? |
|---|---|---|---|
| 1 | R97 / R98 | R98 is printed "2.2K" and runs from the BIAS_IN node (the R68 / R70 / C16+ junction, drawn as a solid T-blob at ≈(2700,2125)) to a **dotted T** at ≈(2988,2127). Q12R's base drops to that dot. R97 is printed "10K" and runs from the dot to C23 +. R69 crosses the R98 line with no dot. | **Yes.** Injection = 2.2/(10+2.2) = 0.180. R98 does not go anywhere else, and R97 does not go directly to the base. |
| 2a | R110 / R111 | Both labels read "2.2K" with a clear decimal point (zoomed 7×). Both go from the VCF_RAIL line to Q21R C and Q21L C respectively. | **Yes, as printed.** |
| 2b | R108, R112, R109, R116, Q20 | R108 "2.2K" (Q20 E → GND). R112 "10K" (BIAS_DIFF → Q20 B). R109 "10K" (Q20 B → GND). R116 "10K" from ⊕. ⊕ is confirmed as 5.333 V at IC16. | **Yes.** Q20 B = 1.78 V, tail ≈ 0.51 mA. |
| 3 | Q21 output / Q19 base | A single dot on the R110 / Q21R-collector line, with a horizontal line to the Q19 base. Q21L collector goes only to R111. The output is single-ended. | **Yes.** |
| 4 | R67, R99, R66, R94, Q18, C15, C23 | Q18: collector to a filled arrow (12 V); emitter dot on the C23 − / R94 line. R94 "10K" → ground. R67 "2.2K" in series from the C15 + line to the Q18 base. R99 "100K" from the Q18 base to the R66 "100K" / C22 + node. C15 + is on the R67 side. C23 + is on the R97 side. Q18's emitter arrow has the same style as the NPN ladder devices. | **Yes.** Q18 is an emitter follower (gain ≈ 0.99), not a gain stage. |
| 5 | VR4A | Terminal 3 is on the VCF_OUT line (dot at x≈3545 from C14). Terminal 2 (the wiper, with an arrowhead) is on the line to C15 −. Terminal 1 goes to a ground symbol. | **Yes.** It is a divider, not a rheostat. At RES max the gain is 1. |
| 6 | R62 / R70 input | R62 "220K" → C17 "1/50" (+ toward R62) → Q12L base line. R70 "2.2K" from that line to the BIAS_IN T-junction. | **Yes.** This path is not in the loop. |
| 7 | Ladder wiring, Q22 bias | Zoomed at 6×, each strap runs from the base bar out, up, and back to its **own** column (dots at both ends). There is a clear gap between the left and right straps, so there is no cross-coupling. Emitter arrows point down and out (NPN, current flows top to bottom). Q22's bases are joined inside the envelope and taken to the R115 / C30 node (AC ground). C25 comes from L1: the y≈1553 line has a dot on the L column and crosses the R column with no dot. C27 comes from R1, on the C26 line, which is dotted at R1. | **Yes.** The ladder is diode-connected. The feedback is negative at DC (I re-derived the sign). |
| 8 | Rest of the loop | C14 is + at Q19 E. R96 is 10K. R95 is 100. C28 is 47/16. Rung caps are .033 / .033 / .033 / .018. No extra path, gain stage or second injection point is drawn. | **Yes.** |

**I found no misreads.** Every value and connection in the loop matches
`transcription/vcf.md`.

## 2. Is the analysis itself right?

I re-ran the linear model independently, using g = I/(4V_T) per diode pair, Q22 as an AC-ground
conductance, and Q12 as a current source g·v_d into L4/R4.

- **Ladder:** DC gain = 1.000. At I = 23.2 µA the 180° point is 1496 Hz, where |G| = 0.0588. Oscillation therefore needs a loop gain of 17.0.
  - The DC gain of 1 is exact even at large signal: Q12 and the diodes follow the same exponential law.
  - So no ladder or transistor-model effect can supply the missing gain.
- **Q21 stage gain:** A = gm·R110/2 = **V_R110 / (2·V_T)**, where V_R110 is the DC drop across R110.
  - As printed, V_R110 = 0.26 mA × 2.2K = 0.56 V, so A ≈ 10.8.
- **Loading** (β = 200): the Q21 input (2r_π ∥ 200K) loads the top rung, so the DC loop gain is lower than 1.9.
  - It is **1.4 at 8 µA, 1.7 at 23 µA and 1.8 at 60 µA**.
  - At k = 1.9 the closed-loop poles have Q = 0.66 and the ringing decays ×0.001 per cycle. There is no peak at all.

The analysis arithmetic is correct. Given the printed values, the loop gain really is about 1.4–1.9.

## 3. The service notes contradict their own values

- **Ringing sketches:** the page-5 sketch shows about 5 ring cycles per burst, decaying about ×0.8 per cycle. The page-7 sketch shows 4 or more cycles.
  - In this ladder model, a decay of 0.8 per cycle needs k ≈ 13–14, which gives Q ≈ 10. A decay of 0.4 per cycle needs k ≈ 10.
  - The TM3 "adjust for 2 ms ring period" procedure cannot be carried out at k ≈ 2, because nothing rings.
- **Measured peak:** the TD-03's +19 to +35 dB needs k ≈ 14–16.5. In this model, peak height above the passband against k is:

  | k | Peak above passband |
  |---|---|
  | 10 | +10 dB |
  | 13 | +16 dB |
  | 14 | +19 dB |
  | 15 | +23 dB |
  | 16 | +29 dB |

- **TP6 DC levels** (open item 1 in `vcf.md`): the sketch shows a 5.2 V baseline and 5.8 V peaks, against a "0" line, so the measurement is DC-coupled.
  - VCF_OUT is 0 V DC after C14, so it cannot be that node. The natural monitor node is Q19 E, which the printed values put at ≈ 10.8 V.
  - For Q19 E to sit at ≈ 5.5 V, V_R110 must be ≈ 5.6 V. That means A = 5.6 / (2 × 0.0259) ≈ **108**, which is 10× the printed stage gain.
  - This one DC observation independently implies the missing ~10×.

## 4. Hypotheses, ranked

1. **The real Q21 collector load is ~10× the printed value (R110 = 22K, not 2.2K).** This is the most likely explanation. It is a printing or drafting error in the service notes, not a misread: the scan clearly says "2.2K".
   - Q19 E ≈ 5.7 V, which matches the 5.2–5.8 V TP6 sketch.
   - A ≈ 102 after loading by R110 ∥ Q21 r_o ∥ Q19 input. The DC loop gain becomes:

     | Ladder current I | Ladder 180° frequency | k | Peak above passband |
     |---|---|---|---|
     | 8 µA | ≈ 556 Hz (the page-7 point) | 13.1 | +19.6 dB |
     | 23 µA | ≈ 1.5 kHz | 16.1 | +37 dB |
     | 60 µA | ≈ 3.9 kHz | 17.3 | marginal oscillation |

   - Because Q21's input loading falls as the ladder current rises, k rises with CUTOFF. That predicts a peak that grows from about +19 dB at low cutoff toward about +35 dB at high cutoff. This is the shape to check against the TD-03 data.
   - The predicted ringing is about ×0.6–0.8 per cycle, which matches the sketches.
   - The stock unit sits just under self-oscillation, depending on β and r_π. That agrees with "doesn't (quite) self-oscillate".
   - R111 would presumably also be 22K, for symmetry. It does not affect the gain.
2. **R108 ≈ 220 Ω (tail ≈ 5 mA).** This also gives V_R110 ≈ 5.6 V and A ≈ 103, but it is less consistent with the measurements:
   - Q21's r_π drops to about 2K, which loads the ladder heavily. k is only 4.3 / 8.7 / 12.8 at 8 / 23 / 60 µA.
   - C25 and C27 would form a ~400 Hz high-pass into the 4K input.
3. **A larger injection fraction** (for example, R97 and R98 labels swapped, giving 0.82) only reaches k ≈ 8.7. On its own it cannot reach the measured peaks, and it does not explain the TP6 levels.
4. The ladder wiring, device models and coupling-capacitor high-passes (13–16 Hz) were all ruled out as the source of the missing ~8×. None of them can add gain at the ladder's 180° frequency.

## 5. Corrected numbers and next checks

- **As printed:** the DC loop gain is 1.4–1.9. This is confirmed and is not a transcription error.
- **With R110 = 22K (proposed model correction):** the DC loop gain is ≈ 13 at the page-7 point and ≈ 16–17 at high cutoff.
- **Discriminating TD-03 checks:**
  - The **passband drop at RES max** should be about −23 to −25 dB (1/(1+k)) under hypothesis 1, against −9 dB as printed.
  - **Peak height should rise with cutoff** under hypothesis 1, and should be about +19 dB near 550 Hz.
  - If a real TB-303 or TD-03 board is available, measure the **DC at Q19 E** (≈ 5.7 V under hypothesis 1, against ≈ 10.8 V as printed). Also measure the resistance of R110 directly.
- **Model change:** in `dw303-vcf` and the SPICE deck, make the Q21 collector load a parameter (default 22K, flagged as "service-notes misprint suspected"). Then refit against the captures.

## TD-3 measurement against the R110 = 22K hypothesis (2026-10-12)

Source: the `vcf-grid-k24-c*-r{00,05,10}-v80` captures. The fundamental (32.6 Hz)
level was measured at RES 0, 0.5 and 1.

| CUTOFF | RES 0 → 1 change at the fundamental |
|---|---|
| 0.2 | −8.2 dB |
| 0.4 | −8.8 dB |
| 0.6 | −9.4 dB |
| 0.8 | −10.2 dB |
| 1.0 | −10.0 dB |

The measured passband drop at RES max is **−8 to −10 dB**. That matches the
**as-printed DC loop gain (≈1.9, which predicts −9 dB)**. It contradicts the
R110 = 22K hypothesis, which predicts about −24 dB.

The TD-3 nevertheless shows a strong resonant peak, +19 to +35 dB above the
saw-compensated fundamental. So the strong peak does not come from a large DC
loop gain. It points to ladder dynamics with a much lower oscillation threshold
than the idealised linear diode-ladder model (k ≈ 17), or to loop gain that
rises with frequency.

Next step: re-derive the ladder's small-signal model from the actual transistor
wiring. That includes the Q22 top pair, the Q12 bottom pair driven differentially
as current sources, and base currents and loading. Then check whether a loop gain
of 1.9 can produce a peak of about 16 dB above the fed-back passband.
