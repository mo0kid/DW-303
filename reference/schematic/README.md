# TB-303 service notes

`TB-303.pdf` is the Roland TB-303 Service Notes, first edition (Feb. 19 1982), 3rd
printing (Sep. 1986), with the 1992 errata pages. It is a scanned copy supplied
by the project owner. Roland owns its copyright. It is kept here for private
reference only and must not be redistributed.

| Page | Content |
|---|---|
| 1 | Specifications, front panel, parts locations |
| 2 | IC pinouts (µPD650C CPU, 4000-series logic, LA4140, AN6562, BA662) |
| 3 | Block diagram |
| 4 | Main board: CPU, memory and switch board |
| **5** | **Main board, analog section: VCO, VCF, VCA, ENV, mixer, headphone and power supply** |
| 6 | Adjustments (TEMPO, INT CLOCK, CV), PCB component side |
| 7 | Adjustments (VCO TUNE/WIDTH, VCF TM3), PCB foil side, preliminary diagnosis |
| 8 | "VCF envelope modulation" explanation, parts list |
| 9–10 | Errata and supplement |

Regenerate the page renders (git-ignored) with:

```sh
pdftoppm -r 300 -png -f 5 -l 5 TB-303.pdf pages/p5
```

Independent per-block transcriptions are in `transcription/`. Their reconciled
result, `netlist.md`, is the input to the SPICE netlists and the
`Components::NOMINAL` values.

## Facts read directly from page 5 (verified by eye)

- **CV / pitch:** C = 1.000 V and C (upper) = 4.000 V, so the scale is
  1 V/octave. "TUNE: 110 Hz (SET TUNING centre, CV OUT: 2.75 V (A key))". A at
  2.75 V is 110 Hz, so the lowest C (1.000 V) is **32.70 Hz (C1)**.
- **VCO saw** at the Q28 (2SK30) source: ramp between about 5.5 V and 12 V.
- **Square:** about 5 V to 8 V; period 9 ms at the A key (110 Hz).
- **DAC:** resistor ladder R74–R90 with 200K values, "matched pair .1% selected".
- **Expo converter:** Q26 is a 2SC1583 matched pair. R100 is a posistor (PTC)
  for temperature compensation.
- **Trimmers:** TM4 is TUNE and TM5 is WIDTH (1 oct/V).
