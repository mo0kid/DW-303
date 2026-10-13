# VCF resonance: loop-gain analysis

Status: analysis only, done 2026-10-11. The TD-03 captures
are the deciding measurement.

## Loop gain from the reconciled netlist

The feedback path, traced on the scan (crop x 2200–3700, y 1150–2300 of
`p5-05.png`), runs:

> Q19 output → C14 → VR4A track → wiper → C15 → R67 → Q18 (emitter follower)
> → C23 → R97 10K / R98 2.2K → Q12 right base

The polarity check (Q22 inverts at the top rung, the C25/C27 cross-coupling into
Q21, then the Q21 right collector to Q19) gives **negative feedback at DC**.

| Stage | Gain |
|---|---|
| Ladder, Q12 differential base → C26 (top rung), at DC | 1.0 (Q12 gm = I/4V_T, Q22 pair r = 4V_T/I) |
| Q21 differential pair → R110 (single-ended), tail ≈ 0.51 mA | ≈ 10.6 |
| Q19 follower, VR4A wiper (RES max), Q18 follower | ≈ 1 |
| R98 / (R97 + R98) injection | 0.180 |
| **DC loop gain at RES max** | **≈ 1.9** |

## Linear diode-ladder phase

Rungs are C18 = 18 nF at the bottom and C19 = C24 = C26 = 33 nF, with
g = I/(4·V_T) per diode pair.

| | 180° frequency at I = 23.2 µA | \|G\| at 180° | Loop gain for oscillation |
|---|---|---|---|
| TB-303 capacitors | 1495 Hz | 0.059 | ≈ 17 |
| Equal 33 nF | 1291 Hz | 0.054 | ≈ 18 |

## Consequences

- **No self-oscillation:** with a loop gain of about 1.9, the stock circuit
  cannot self-oscillate. This agrees with the widely reported behaviour of the
  stock TB-303; the Devil Fish modification adds feedback to make it oscillate.
- **Damped ringing and the passband:** the closed-loop poles are complex. A saw
  edge therefore produces damped ringing, which is what the page-7 TM3 procedure
  calibrates ("ring period 2 ms"). The frequency-response peak is gentle. The
  negative DC feedback lowers the passband by about 9 dB at RES max, which
  matches the Rust model (−8 dB measured).
- **Agreement:** the Rust `dw303-vcf` model and the generic SPICE deck both
  agree with this transcription-based analysis.
- **What to measure:** if the TD-03 shows a much stronger resonant peak, the
  loop gain in the transcription is wrong. Candidates are the Q21 stage gain,
  the R97/R98 injection, the ladder transistor wiring, and the transistor models.
  The VCF capture items to use are: harmonic magnitudes against RESONANCE,
  ringing period and decay at the page-7 settings, and the passband drop at
  RES max.
