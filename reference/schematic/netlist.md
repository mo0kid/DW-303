# TB-303 analog netlist (reconciled)

This is the authoritative transcription of service-notes page 5, with the page 9
errata applied. Each block was transcribed independently by two readers
(`transcription/*-A.md`, `*-B.md`), then reconciled against the scan. Every
disagreement was resolved by zooming into the scan, and the resolution logs give
crop coordinates.

| Block | File | Open items |
|---|---|---|
| Pitch CV, DAC, slide, VCO, square shaper, WAVEFORM | [transcription/vco.md](transcription/vco.md) | 7 |
| VCF ladder, output stage, resonance, cutoff expo converter, accent-sweep input | [transcription/vcf.md](transcription/vcf.md) | 4 |
| Gate/accent logic, MEG, DECAY, ENV MOD, ACCENT routing | [transcription/env.md](transcription/env.md) | 5 |
| VCA (BA662A OTA), VCA envelope, mixer, outputs, headphone, rails | [transcription/vca.md](transcription/vca.md) | 7 |

Each file has a section 6 (VCA: section 5), "Model-relevant values". These are
the inputs for `Components::NOMINAL` and the SPICE netlists in
`reference/spice/`.

## Headline corrections to the provisional model

- **Pitch:**
  - The DAC output is (code+1)/12 V into a 1 V/oct VCO.
  - The lowest C is 1.000 V = **32.70 Hz (C1)**, not 65.4 Hz. A at 2.75 V is
    110 Hz.
  - Tuning is ±700 cents (spec, page 1).
- **Slide:** C35 0.22 µF is charged through the ladder's 100 kΩ Thevenin
  resistance, so **τ = 22 ms**. With slide off, C35 tracks the buffered CV.
- **VCO:** the saw at the Q28 source falls from 12 V to 5.5 V. The square at Q8
  is 5–8 V.
- **VCF:**
  - The rung capacitors are 33, 33 and 33 nF, with **18 nF** at the bottom
    (Q12 collectors).
  - Input attenuation is −40 dB (R62/R70).
  - Feedback is injected at 0.18 through a 13 Hz high-pass (C23).
  - The output is coupled through C25/C27 (15.9 Hz).
  - The exponential converter Q10/Q11 is referenced to VQ9 ≈ 3.3 V. ENV MOD
    contributes 1.79 V/oct at the VR5 wiper, and the accent sweep 0.81 V/oct at
    the VR4B wiper.
- **MEG:**
  - C62 1 µF is charged by a Q38 pulse of about 1.4 ms.
  - Decay τ is 68 ms (min, and the fixed value under accent) to 1.07 s, through
    R138 + VR6 (1 MΩ A).
  - The decay settles onto a three-diode clamp.
- **Accent:**
  - IC12 shorts VR6 and gates the MEG onto VR7 (ACCENT).
  - The VR7 wiper feeds the VCA (R119/C36/R120/D27) and the VCF (D24 → R46 →
    VR4B gang → C13 1 µF → R72).
- **VCA:**
  - The VCA is a BA662A OTA.
  - C42 1 µF recharges through 1.5 MΩ (τ 1.5 s). Q32 discharges it on the
    trigger pulse.
  - The control current reaches the OTA through Q31.
  - The OTA input sums the VCF output (−40 dB, 72 Hz high-pass) with the
    **resonance pot wiper** (−33 dB, 159 Hz high-pass).
- **Output:** the mixer (Q34/Q33) applies −2.13 to the VCA path and −1 to MIX IN.
