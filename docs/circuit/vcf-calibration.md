# VCF calibration against TD-3 session1

2026-10-12. This measurement fit supersedes the transcription-only calibration
and resonance conclusions in [vcf.md](vcf.md) and
[vcf-resonance.md](vcf-resonance.md). The [contract](contracts.md) remains binding.
No capture, printed passive value, rung capacitor, signal interface, VCA or
oscillator was changed. The effective Q21 gain and C25/C27 coupling corrections
are provisional, explicitly distinguishable from the printed transcription.

## Evidence and method

Source: `reference/captures/2026-10-12-session1/`, manifest
`reference/captures/manifest.toml`. Analyse all 156 valid VCF records: page-7,
11 ring positions, and 144 grid records. Compare the 84 non-accented records
(page-7 + ring + 72 v80 grid captures) through the actual release renderer.
The MIDI velocity, rather than the ACCENT knob setting, identifies accents.
The renderer's fixed velocity 0.5 is appropriate for v80; v120 records have
hardware metrics but deliberately have no model comparison. No renderer change
or new dependency was necessary.

`tools/analysis/vcf_fit.py` reads 96 kHz/24-bit PCM and the JSON onset/MIDI
schedule. Minimum-DECAY sustained windows start 350 ms after detected onset and
last 600 ms, after approximately five 68 ms MEG time constants. Other decay
settings select a later, bounded window; those settings are not present in this
fit. The matched `wave-saw-kKEY` reference uses the same envelope age and length.

Every harmonic through 20 kHz is extracted using an 8-times-or-greater padded
Hann FFT, local harmonic peak amplitudes, and a pitch estimate refined over
multiple low harmonics. This refinement matters: a 0.1 Hz fundamental error
otherwise misses high harmonics in a 600 ms window. Full JSON retains all
harmonics, their local inter-harmonic noise estimates, exact windows, reference
spectra, response masks and renderer logs. A reference harmonic must exceed
its local noise by 12 dB and remain within 60 dB of the largest reference
harmonic; the measured harmonic must also exceed its noise by 12 dB.
Clipped analysis windows are rejected. A noisy harmonic is retained in the
raw spectrum but excluded from response metrics.

Response is `20 log10(capture_harmonic / same-note_reference_harmonic)`.
**Peak height in the table is the peak of this ratio above its fundamental**,
not raw saw compensation, absolute dBFS, or peak above passband median.
Peak frequency is the strongest resolved harmonic; there is no fabricated
continuous-frequency precision between harmonics. Passband is the median of
resolved harmonics below half the measured C1 RES-max peak at the same cutoff;
when the note lies above that band, the fundamental is explicitly the fallback.
Slopes fit response against log2 frequency from 2 to 8 times the peak, with at
least three resolved harmonics. Missing slopes mean insufficient evidence.
For a prefix selection excluding the ring sweep, passband uses half the selected
capture's peak instead; use the full run for the acceptance statistics below.

These ratios include large-signal ladder/Q21/VCA behaviour and the non-flat
open-filter reference, not an isolated small-signal VCF transfer function.
Raw saw compensation (`20 log10(Ah/A1) + 20 log10(h)`) gives about 18.9 dB at
cutoff 0 and 25.6 dB at centre for the hardware. The corresponding final model
values are about 17.4 and 23.8 dB. At cutoff 1, hardware/model raw heights are
about 42.5/39.0 dB; the matched-reference height error is 1.72 dB. These different
definitions must not be interchanged.

The settled page-7 harmonic peak is **519.6 Hz**; a 50–350 ms window gives about
552 Hz. The 535 Hz damped-pole calibration produces a settled model peak at
523.3 Hz. A damped pole, magnitude maximum and attack-window maximum are different
observables. The service-note 500 Hz / 2 ms procedure is still a useful sanity
check, not an assertion that this TD-3 has exactly that peak frequency.

## Identification and fitted circuit parameters

First retain the transcribed topology and test a single loop-gain correction.
Rendered corrections on feedback injection and Q21 produce strong resonance,
but do not reproduce the entire sweep. An independent continuous nodal AC fit,
allowing eleven cutoff currents but fixing printed C25/C27 time constants,
finds Q21 gain multiplier 7.805: its low-cutoff reference-ratio peak is about
16 dB versus the measured 6 dB. Raising this scalar cannot remove that error.
The constrained fit has 2.51 dB RMS error over 2,768 weighted response samples.

One additional effective parameter suffices: shorten the crossed C25/C27
coupling time constant. The initial joint AC fit gives gain 8.619, coupling
multiplier 0.1653 and 1.61 dB RMS on the same weighted samples. Peak neighbourhoods
are weighted explicitly; a broad-band-only fit understates the sharp peak error.
The final nonlinear engine fit uses **gain 9.5 and coupling multiplier 0.18**,
with the fitted VR3 track below. This compensates the large-signal compression
that the AC identification does not model. Acceptance uses renderer output,
not the AC optimizer's score.

| Quantity | Printed/transcribed value | Applied fit | Status/evidence |
|---|---:|---:|---|
| Q21 effective collector gain | about 10.91 V/V from Q20/R108/R110 | multiplier 9.5; about 103.6 V/V | **VERIFY:** ring height and grid passband fit |
| C25/C27 | 0.1 µF each, retained in `output_capacitance` | `output_coupling_scale = 0.18`; effective 18 nF each | **VERIFY:** low-cutoff resonance and reference-ratio shape |
| R113/R114 × effective C25/C27 | 10 ms | 1.8 ms; about 88.4 Hz coupling corner | **VERIFY:** effective reduction, not verified replacement capacitors |
| TM3 track/setting | 470 kΩ track; setting unknown | setting 470 kΩ | **VERIFY:** chosen legal endpoint; not separately identifiable from expo ratio |
| Is(Q11)/Is(Q10) | unprinted | nominal fitted 3.0743234084; voice-normalised 3.1341894962 | **VERIFY:** absolute tail-current fit and centre pole normalization |
| Centre ring pole | service check 500 Hz | 535 Hz, current 7.7382713 µA | **VERIFY:** settled hardware peak 519.6 Hz / model 523.3 Hz |

The Q21 multiplier applies to collector swing **and** small-signal gm, preserving
the tanh differential-pair law, printed R108/R110, and all capacitor states.
With the original tail model it is equivalent in gain to an approximately
20.9 kΩ collector load; this is a hypothesis about missing gain, not a claim
that R110 is actually 20.9 kΩ. The coupling multiplier changes only the effective
C25/C27 companion coefficients and their contribution to the analytic pole
calibration; the physical loading through R113/R114 is retained. There are no
new filter poles, envelopes, sample delays, or extra solver state variables.
Set both correction fields to 1 to recover the transcription-only VCF.

VR3 remains a resistive 50 kΩ track, with R47/R73 and its loaded wiper intact.
Interpolate these **VERIFY: fitted** wiper fractions linearly between panel
increments; they replace the estimated 10%-at-centre A taper only for CUTOFF.
Other envelope/pot laws are unchanged. Invalid or nonmonotone tables fall back
to the nominal fitted table; endpoints must be exactly 0 and 1.

| Panel fraction | VR3 wiper fraction |
|---:|---:|
| 0.0 | 0.000000000 |
| 0.1 | 0.016988877 |
| 0.2 | 0.048210059 |
| 0.3 | 0.078453452 |
| 0.4 | 0.100557131 |
| 0.5 | 0.124966812 |
| 0.6 | 0.141500000 |
| 0.7 | 0.292831010 |
| 0.8 | 0.621164626 |
| 0.9 | 0.978428507 |
| 1.0 | 1.000000000 |

The initial AC currents were inverted through the existing loaded-pot equations.
The 0.6 knot was lowered during the nonlinear harmonic comparison, removing
the original 6.6% frequency error. TM3 and expo ratio have a strong scale
ambiguity: audio does not establish the physical trim setting or transistor
saturation currents independently. The guard still permits zero TM3.

## Acceptance results

| Check | Result | Budget |
|---|---:|---:|
| Ring sweep peak frequency, maximum absolute error | 4.84% | 5% |
| Ring sweep matched-reference peak height, maximum absolute error | 2.34 dB | 3 dB |
| RES 0 grid passband, 155 resolved/fallback harmonic comparisons | RMS 0.65 dB; max 2.53 dB | 3 dB |
| RES 0.5 grid passband, 155 resolved/fallback harmonic comparisons | RMS 0.89 dB; max 2.45 dB | 3 dB |
| Full-quality renders, 96 kHz host / 192 kHz circuit | 10.0–16.5% of one core | approximately 20% |
| Capture and reference render non-convergence | 0 | 0 |
| Maximum Newton iterations in these renders | 3 | bounded at 8 |

All workspace tests pass with `cargo test --offline --workspace`. The owned
stage suites have 9 passing VCF tests and 12 passing envelope tests, with three
manual timing tests ignored. They cover analytic Jacobians/solves, AC response
at stable currents, the fitted sharp centre peak in both quality modes,
sample-rate-independent ring timing, fitted current/track monotonicity and
sanitization, and long/extreme nonlinear runs from 44.1 to 384 kHz. The
eight-iteration/backtracking bounds and existing solver fallback are unchanged.
No new allocation, panic, or unbounded loop was added to any tick path.

`cargo clippy --offline -p dw303-vcf -p dw303-env --all-targets -- -D warnings`
and the equivalent `-p dw303-voice` check pass. The changed Rust files pass
rustfmt checks. The analysis script's
synthetic check verifies 24-bit PCM, precise pitch and a weak fundamental with
a strong 73rd harmonic, all harmonics through 20 kHz, reference-ratio peak,
passband and a known four-pole slope.

## VERIFY register and limitations

Every fitted number above is an estimate of this TD-3's measured response,
not an authenticated TB-303 component value. Outstanding fitted **VERIFY** items:

1. Q21 gain multiplier 9.5: reconcile the scan, collector load, tail bias and
   feedback injection. Measurements identify effective loop gain, not its cause.
2. Effective C25/C27 multiplier 0.18: check capacitance markings, R113/R114 and
   crossed-coupling topology. The fitted time constant does not identify the
   individual hardware component or omitted device/loading mechanism uniquely.
3. All eleven VR3 track knots: inferred from ring/grid spectra, not direct
   resistance measurements. High-end flattening and knot slopes need independent
   knob sweeps; interpolation is monotone but has slope changes at the knots.
4. TM3 setting 470 kΩ: a legal, convenient choice with expo ratio refitted, not
   a measured trim position.
5. Expo Is ratio 3.074 nominal / 3.134 voice: effective scale, not measured
   saturation-current matching or a temperature calibration.
6. Centre calibration pole 535 Hz: normalizes the selected sustained observation;
   it does not claim the attack peak or exact service-note ring target is 535 Hz.
7. Calibration rest MEG 0.5 V and sweep 3.3165 V: values from the present envelope
   and junction-drop model, not measured internal nodes. Revisit if those stages
   are recalibrated.

Inherited VCF **VERIFY** assumptions also remain: thermal voltage 25.85 mV,
Q20 Vbe 0.65 V, ideal Q18/Q19 followers (fixed bias, infinite beta, no Early
effect/parasitics), Q20/Q21 base-current omission, and the ladder's fixed arm
common-mode reduction (no common-mode redistribution, finite beta, Early effect
or junction capacitance). The cutoff converter
still inherits estimated junction drops and thermal voltage from
[env.md](env.md); none were authenticated by this fit.

The fitted loop approaches unity at the maximum captured ladder current,
approximately 37 µA. Its continuous linearization exceeds unity at the phase
crossing at 50 µA (approximately 1.018) and may self-oscillate at higher currents
with RES max. The nonlinear extreme/long-run tests still remain bounded with
zero non-convergence. A stable small-signal AC comparison at 50 µA is therefore
inappropriate and was replaced by a stable 30 µA case. This is a material
weakness outside the fitted zero-ENV-MOD range, particularly for large envelope
or accent-driven currents, and is not evidence that a stock TB-303 self-oscillates.
The transcription recheck and independent high-current captures should resolve
it before treating this model as an authenticated stock-circuit implementation.

Other limits: no accented renderer comparisons or saturation fit from v120;
no independent absolute line-level/interface/temperature calibration; capture
ratios still include VCA and reference-filter response; high-note spectra only
sample the response at coarse harmonics. The ring sweep meets its peak budgets;
the RES-max grid has worst peak-height error 4.34 dB (k60, cutoff 1) and worst
harmonic-peak frequency error 19.55% (k48, cutoff 0.6, adjacent harmonic maxima).
Those grid peak residuals are not covered by the ring-sweep acceptance claim.
The cutoff-0.8 post-peak slope differs by approximately 6.7 dB/octave;
post-peak shape was measured but is not fully fitted. Upper-cutoff slopes cannot be claimed
where the reference or filtered capture hits the noise floor. Renderer volume
is 0.35 for both capture and reference to avoid digital PCM clipping; the common
linear output attenuation cancels in the ratio. Absolute full-scale calibration
and the increased Q21 output level are outside this task.

## Reproduction

```sh
cargo build --offline -p dw303-render --release
OPENBLAS_NUM_THREADS=1 python3 tools/analysis/vcf_fit.py --self-test
OPENBLAS_NUM_THREADS=1 python3 tools/analysis/vcf_fit.py --render \
  --cache /tmp/dw303-vcf-fit --json /tmp/dw303-vcf-fit.json \
  --markdown /tmp/dw303-vcf-fit.md
OPENBLAS_NUM_THREADS=1 python3 tools/analysis/vcf_fit.py --fit-linear \
  --fixed-coupling-scale 1
OPENBLAS_NUM_THREADS=1 python3 tools/analysis/vcf_fit.py --fit-linear
cargo test --offline --workspace
cargo clippy --offline -p dw303-vcf -p dw303-env --all-targets -- -D warnings
```

Each render uses `--hold KEY --rate 96000 --set 11=1`, with panel IDs 1–7 from
the sidecar (except common output attenuation) and saw `--set 9=0`. Cache keys
include the renderer binary hash as well as settings and duration, so a rebuild
cannot reuse stale audio. Renderer logs preserve CPU and Newton telemetry.
JSON contains the complete harmonic evidence; the table below is the compact
comparison. Accented rows have em dashes in model columns.

## Full comparison

| Capture | TD peak Hz | Model Hz | Δ Hz % | TD peak dB | Model dB | Δ dB | TD/model passband dB | TD/model slope dB/oct |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| vcf-grid-k24-c00-r00-v120 | 32.5 | — | — | 0.0 | — | — | -12.4/— | -6.0/— |
| vcf-grid-k24-c00-r00-v80 | 32.5 | 32.7 | 0.7 | 0.0 | 0.0 | 0.0 | -10.9/-8.4 | -6.0/-6.9 |
| vcf-grid-k24-c00-r05-v120 | 32.5 | — | — | 0.0 | — | — | -13.5/— | 0.0/— |
| vcf-grid-k24-c00-r05-v80 | 32.5 | 32.7 | 0.7 | 0.0 | 0.0 | 0.0 | -13.9/-11.7 | -0.0/-0.4 |
| vcf-grid-k24-c00-r10-v120 | 259.8 | — | — | 6.0 | — | — | -15.3/— | -12.6/— |
| vcf-grid-k24-c00-r10-v80 | 259.8 | 261.7 | 0.7 | 6.0 | 5.8 | -0.2 | -15.6/-13.5 | -13.6/-14.2 |
| vcf-grid-k24-c04-r00-v120 | 32.5 | — | — | 0.0 | — | — | -10.2/— | -4.7/— |
| vcf-grid-k24-c04-r00-v80 | 32.5 | 32.7 | 0.7 | 0.0 | 0.0 | 0.0 | -10.4/-8.5 | -4.7/-5.5 |
| vcf-grid-k24-c04-r05-v120 | 259.8 | — | — | 0.4 | — | — | -12.3/— | -12.6/— |
| vcf-grid-k24-c04-r05-v80 | 259.8 | 32.7 | -87.4 | 0.4 | 0.0 | -0.4 | -12.5/-10.8 | -12.1/1.7 |
| vcf-grid-k24-c04-r10-v120 | 357.1 | — | — | 10.8 | — | — | -14.2/— | -11.6/— |
| vcf-grid-k24-c04-r10-v80 | 357.2 | 359.8 | 0.7 | 10.8 | 10.5 | -0.2 | -14.5/-13.1 | -11.1/-12.3 |
| vcf-grid-k24-c08-r00-v120 | 32.5 | — | — | 0.0 | — | — | -9.2/— | -3.8/— |
| vcf-grid-k24-c08-r00-v80 | 32.5 | 32.7 | 0.7 | 0.0 | 0.0 | 0.0 | -9.4/-8.1 | -3.8/-4.7 |
| vcf-grid-k24-c08-r05-v120 | 357.2 | — | — | 2.7 | — | — | -10.9/— | -10.6/— |
| vcf-grid-k24-c08-r05-v80 | 357.2 | 359.8 | 0.7 | 2.6 | 1.2 | -1.4 | -11.2/-10.3 | -10.1/-11.1 |
| vcf-grid-k24-c08-r10-v120 | 487.0 | — | — | 14.4 | — | — | -13.1/— | -8.8/— |
| vcf-grid-k24-c08-r10-v80 | 487.1 | 490.6 | 0.7 | 14.4 | 14.2 | -0.2 | -13.2/-12.6 | -8.7/-10.6 |
| vcf-grid-k24-c12-r00-v120 | 32.5 | — | — | 0.0 | — | — | -7.6/— | -3.1/— |
| vcf-grid-k24-c12-r00-v80 | 32.5 | 32.7 | 0.7 | 0.0 | 0.0 | 0.0 | -7.8/-7.4 | -3.1/-4.2 |
| vcf-grid-k24-c12-r05-v120 | 422.1 | — | — | 4.5 | — | — | -10.3/— | -8.9/— |
| vcf-grid-k24-c12-r05-v80 | 422.1 | 425.2 | 0.7 | 4.5 | 2.6 | -1.9 | -10.5/-9.9 | -9.1/-10.1 |
| vcf-grid-k24-c12-r10-v120 | 584.5 | — | — | 17.8 | — | — | -12.6/— | -7.7/— |
| vcf-grid-k24-c12-r10-v80 | 584.5 | 556.0 | -4.9 | 17.8 | 16.8 | -1.0 | -12.7/-12.2 | -8.0/-9.8 |
| vcf-grid-k24-c16-r00-v120 | 32.5 | — | — | 0.0 | — | — | -2.4/— | -0.8/— |
| vcf-grid-k24-c16-r00-v80 | 32.5 | 32.7 | 0.7 | 0.0 | 0.0 | 0.0 | -2.5/-2.8 | -0.8/-1.0 |
| vcf-grid-k24-c16-r05-v120 | 1168.9 | — | — | 12.6 | — | — | -4.5/— | -4.2/— |
| vcf-grid-k24-c16-r05-v80 | 1169.0 | 1210.1 | 3.5 | 12.6 | 11.3 | -1.3 | -4.6/-4.6 | -4.3/-5.8 |
| vcf-grid-k24-c16-r10-v120 | 1591.0 | — | — | 36.7 | — | — | -7.0/— | -15.5/— |
| vcf-grid-k24-c16-r10-v80 | 1591.1 | 1602.5 | 0.7 | 36.8 | 35.6 | -1.1 | -7.1/-7.3 | -15.3/-9.0 |
| vcf-grid-k24-c20-r00-v120 | 3409.4 | — | — | 0.5 | — | — | 0.2/— | —/— |
| vcf-grid-k24-c20-r00-v80 | 3539.3 | 32.7 | -99.1 | 0.4 | 0.0 | -0.4 | 0.0/0.0 | —/0.0 |
| vcf-grid-k24-c20-r05-v120 | 1785.8 | — | — | 17.7 | — | — | -2.2/— | —/— |
| vcf-grid-k24-c20-r05-v80 | 1785.9 | 1896.8 | 6.2 | 17.7 | 16.7 | -1.0 | -2.4/-2.4 | —/-0.4 |
| vcf-grid-k24-c20-r10-v120 | 2370.3 | — | — | 48.4 | — | — | -4.8/— | —/— |
| vcf-grid-k24-c20-r10-v80 | 2370.4 | 2485.5 | 4.9 | 48.8 | 47.1 | -1.7 | -4.9/-4.7 | —/— |
| vcf-grid-k36-c00-r00-v120 | 65.0 | — | — | 0.0 | — | — | -10.4/— | -7.8/— |
| vcf-grid-k36-c00-r00-v80 | 65.0 | 65.4 | 0.6 | 0.0 | 0.0 | 0.0 | -10.6/-8.8 | -7.8/-8.6 |
| vcf-grid-k36-c00-r05-v120 | 195.0 | — | — | 1.3 | — | — | -13.8/— | -13.5/— |
| vcf-grid-k36-c00-r05-v80 | 195.0 | 196.2 | 0.6 | 1.3 | 1.2 | -0.1 | -13.9/-12.5 | -13.5/-14.1 |
| vcf-grid-k36-c00-r10-v120 | 260.0 | — | — | 9.5 | — | — | -15.7/— | -13.5/— |
| vcf-grid-k36-c00-r10-v80 | 260.0 | 261.7 | 0.6 | 9.4 | 9.9 | 0.5 | -15.8/-14.3 | -13.3/-14.1 |
| vcf-grid-k36-c04-r00-v120 | 65.0 | — | — | 0.0 | — | — | -9.8/— | -5.9/— |
| vcf-grid-k36-c04-r00-v80 | 65.0 | 65.4 | 0.6 | 0.0 | 0.0 | 0.0 | -10.0/-8.3 | -5.9/-6.5 |
| vcf-grid-k36-c04-r05-v120 | 260.0 | — | — | 4.2 | — | — | -12.7/— | -11.9/— |
| vcf-grid-k36-c04-r05-v80 | 260.0 | 261.7 | 0.6 | 4.2 | 3.5 | -0.7 | -12.8/-11.7 | -12.2/-12.7 |
| vcf-grid-k36-c04-r10-v120 | 390.0 | — | — | 13.5 | — | — | -14.8/— | -11.4/— |
| vcf-grid-k36-c04-r10-v80 | 390.0 | 392.5 | 0.6 | 13.4 | 13.1 | -0.2 | -15.0/-13.7 | -10.8/-12.1 |
| vcf-grid-k36-c08-r00-v120 | 65.0 | — | — | 0.0 | — | — | -9.5/— | -4.6/— |
| vcf-grid-k36-c08-r00-v80 | 65.0 | 65.4 | 0.6 | 0.0 | 0.0 | 0.0 | -9.7/-8.1 | -4.7/-5.1 |
| vcf-grid-k36-c08-r05-v120 | 325.0 | — | — | 6.6 | — | — | -12.2/— | -10.4/— |
| vcf-grid-k36-c08-r05-v80 | 325.0 | 327.1 | 0.6 | 6.6 | 5.3 | -1.2 | -12.3/-11.1 | -10.3/-11.2 |
| vcf-grid-k36-c08-r10-v120 | 455.0 | — | — | 18.0 | — | — | -14.5/— | -9.3/— |
| vcf-grid-k36-c08-r10-v80 | 455.0 | 457.9 | 0.6 | 18.0 | 17.9 | -0.1 | -14.6/-13.6 | -8.9/-10.6 |
| vcf-grid-k36-c12-r00-v120 | 65.0 | — | — | 0.0 | — | — | -8.3/— | -3.7/— |
| vcf-grid-k36-c12-r00-v80 | 65.0 | 65.4 | 0.6 | 0.0 | 0.0 | 0.0 | -8.5/-7.9 | -3.7/-4.4 |
| vcf-grid-k36-c12-r05-v120 | 455.0 | — | — | 8.4 | — | — | -11.0/— | -8.5/— |
| vcf-grid-k36-c12-r05-v80 | 455.1 | 392.5 | -13.8 | 8.4 | 6.8 | -1.6 | -11.1/-10.4 | -8.3/-10.2 |
| vcf-grid-k36-c12-r10-v120 | 585.0 | — | — | 21.8 | — | — | -13.4/— | -7.7/— |
| vcf-grid-k36-c12-r10-v80 | 585.1 | 588.7 | 0.6 | 21.8 | 20.2 | -1.7 | -13.5/-12.7 | -7.9/-9.7 |
| vcf-grid-k36-c16-r00-v120 | 65.0 | — | — | 0.0 | — | — | -2.5/— | -0.8/— |
| vcf-grid-k36-c16-r00-v80 | 65.0 | 65.4 | 0.6 | 0.0 | 0.0 | 0.0 | -2.6/-2.9 | -0.8/-1.2 |
| vcf-grid-k36-c16-r05-v120 | 1170.1 | — | — | 16.9 | — | — | -4.6/— | -2.8/— |
| vcf-grid-k36-c16-r05-v80 | 1170.1 | 1177.4 | 0.6 | 16.8 | 15.9 | -0.9 | -4.6/-5.0 | -2.8/-5.2 |
| vcf-grid-k36-c16-r10-v120 | 1560.1 | — | — | 38.9 | — | — | -7.2/— | -0.4/— |
| vcf-grid-k36-c16-r10-v80 | 1560.2 | 1635.2 | 4.8 | 39.1 | 36.8 | -2.3 | -7.3/-7.4 | -0.6/-8.7 |
| vcf-grid-k36-c20-r00-v120 | 3380.2 | — | — | 0.3 | — | — | 0.2/— | —/— |
| vcf-grid-k36-c20-r00-v80 | 3380.5 | 65.4 | -98.1 | 0.1 | 0.0 | -0.1 | -0.0/0.0 | —/0.0 |
| vcf-grid-k36-c20-r05-v120 | 1820.2 | — | — | 22.1 | — | — | -2.0/— | —/— |
| vcf-grid-k36-c20-r05-v80 | 1755.2 | 1896.8 | 8.1 | 22.0 | 21.5 | -0.6 | -2.1/-2.4 | —/— |
| vcf-grid-k36-c20-r10-v120 | 2340.2 | — | — | 49.3 | — | — | -4.6/— | —/— |
| vcf-grid-k36-c20-r10-v80 | 2340.1 | 2485.5 | 6.2 | 49.9 | 50.0 | 0.1 | -4.7/-4.8 | —/— |
| vcf-grid-k48-c00-r00-v120 | 130.1 | — | — | 0.0 | — | — | -16.5/— | -11.3/— |
| vcf-grid-k48-c00-r00-v80 | 130.1 | 130.8 | 0.6 | 0.0 | 0.0 | 0.0 | -16.7/-15.3 | -11.4/-12.1 |
| vcf-grid-k48-c00-r05-v120 | 130.1 | — | — | 0.0 | — | — | -13.4/— | -12.3/— |
| vcf-grid-k48-c00-r05-v80 | 130.1 | 130.8 | 0.6 | 0.0 | 0.0 | 0.0 | -13.5/-13.0 | -12.4/-13.4 |
| vcf-grid-k48-c00-r10-v120 | 260.2 | — | — | 7.7 | — | — | -14.6/— | -13.1/— |
| vcf-grid-k48-c00-r10-v80 | 260.2 | 261.6 | 0.6 | 7.6 | 9.0 | 1.4 | -14.7/-14.3 | -13.1/-13.5 |
| vcf-grid-k48-c04-r00-v120 | 130.1 | — | — | 0.0 | — | — | -12.3/— | -9.1/— |
| vcf-grid-k48-c04-r00-v80 | 130.1 | 130.8 | 0.6 | 0.0 | 0.0 | 0.0 | -12.5/-11.2 | -9.2/-9.7 |
| vcf-grid-k48-c04-r05-v120 | 260.2 | — | — | 3.6 | — | — | -12.8/— | -11.4/— |
| vcf-grid-k48-c04-r05-v80 | 260.2 | 261.7 | 0.6 | 3.5 | 4.0 | 0.5 | -12.9/-12.7 | -11.5/-12.3 |
| vcf-grid-k48-c04-r10-v120 | 390.2 | — | — | 12.5 | — | — | -14.8/— | -10.4/— |
| vcf-grid-k48-c04-r10-v80 | 390.2 | 392.5 | 0.6 | 12.4 | 13.1 | 0.7 | -14.9/-14.5 | -10.7/-11.6 |
| vcf-grid-k48-c08-r00-v120 | 130.1 | — | — | 0.0 | — | — | -9.4/— | -7.4/— |
| vcf-grid-k48-c08-r00-v80 | 130.1 | 130.8 | 0.6 | 0.0 | 0.0 | 0.0 | -9.6/-8.5 | -7.4/-7.8 |
| vcf-grid-k48-c08-r05-v120 | 390.3 | — | — | 5.8 | — | — | -12.6/— | -9.5/— |
| vcf-grid-k48-c08-r05-v80 | 390.3 | 392.5 | 0.6 | 5.7 | 6.2 | 0.5 | -12.7/-12.4 | -9.8/-10.6 |
| vcf-grid-k48-c08-r10-v120 | 520.3 | — | — | 15.6 | — | — | -14.8/— | -9.1/— |
| vcf-grid-k48-c08-r10-v80 | 520.3 | 523.3 | 0.6 | 15.6 | 15.8 | 0.3 | -14.9/-14.5 | -8.8/-10.1 |
| vcf-grid-k48-c12-r00-v120 | 130.1 | — | — | 0.0 | — | — | -9.2/— | -6.0/— |
| vcf-grid-k48-c12-r00-v80 | 130.1 | 130.8 | 0.6 | 0.0 | 0.0 | 0.0 | -9.3/-9.1 | -6.0/-6.7 |
| vcf-grid-k48-c12-r05-v120 | 390.2 | — | — | 8.2 | — | — | -10.3/— | -8.4/— |
| vcf-grid-k48-c12-r05-v80 | 390.3 | 392.5 | 0.6 | 8.2 | 7.7 | -0.5 | -10.4/-10.4 | -8.4/-9.8 |
| vcf-grid-k48-c12-r10-v120 | 650.5 | — | — | 18.7 | — | — | -12.6/— | -7.4/— |
| vcf-grid-k48-c12-r10-v80 | 650.4 | 523.3 | -19.6 | 18.6 | 19.1 | 0.4 | -12.7/-12.4 | -7.7/-9.5 |
| vcf-grid-k48-c16-r00-v120 | 130.1 | — | — | 0.0 | — | — | -2.8/— | -1.3/— |
| vcf-grid-k48-c16-r00-v80 | 130.1 | 130.8 | 0.6 | 0.0 | 0.0 | 0.0 | -3.0/-3.1 | -1.3/-1.5 |
| vcf-grid-k48-c16-r05-v120 | 1170.9 | — | — | 17.5 | — | — | -4.3/— | -2.0/— |
| vcf-grid-k48-c16-r05-v80 | 1170.9 | 1177.4 | 0.6 | 17.5 | 17.3 | -0.1 | -4.4/-5.0 | -2.0/-4.6 |
| vcf-grid-k48-c16-r10-v120 | 1561.0 | — | — | 39.8 | — | — | -7.0/— | 1.0/— |
| vcf-grid-k48-c16-r10-v80 | 1561.0 | 1569.8 | 0.6 | 40.0 | 38.5 | -1.4 | -7.0/-7.4 | -0.6/7.4 |
| vcf-grid-k48-c20-r00-v120 | 3642.4 | — | — | 0.3 | — | — | 0.2/— | —/— |
| vcf-grid-k48-c20-r00-v80 | 3642.6 | 130.8 | -96.4 | 0.1 | 0.0 | -0.1 | 0.0/0.0 | —/0.0 |
| vcf-grid-k48-c20-r05-v120 | 1821.2 | — | — | 23.0 | — | — | -1.7/— | —/— |
| vcf-grid-k48-c20-r05-v80 | 1821.3 | 1962.2 | 7.7 | 22.9 | 23.1 | 0.2 | -1.8/-2.4 | —/— |
| vcf-grid-k48-c20-r10-v120 | 2341.8 | — | — | 50.6 | — | — | -4.4/— | —/— |
| vcf-grid-k48-c20-r10-v80 | 2341.9 | 2485.5 | 6.1 | 50.8 | 49.1 | -1.7 | -4.5/-4.9 | —/— |
| vcf-grid-k60-c00-r00-v120 | 259.9 | — | — | 0.0 | — | — | -23.5/— | -12.6/— |
| vcf-grid-k60-c00-r00-v80 | 259.9 | 261.6 | 0.7 | 0.0 | 0.0 | 0.0 | -23.8/-23.2 | -13.0/-13.3 |
| vcf-grid-k60-c00-r05-v120 | 260.0 | — | — | 0.0 | — | — | -16.0/— | -13.0/— |
| vcf-grid-k60-c00-r05-v80 | 259.9 | 261.6 | 0.7 | 0.0 | 0.0 | 0.0 | -16.1/-15.0 | -12.8/-13.1 |
| vcf-grid-k60-c00-r10-v120 | 260.0 | — | — | 0.0 | — | — | -8.4/— | -12.8/— |
| vcf-grid-k60-c00-r10-v80 | 260.0 | 261.6 | 0.7 | 0.0 | 0.0 | 0.0 | -8.6/-7.5 | -12.7/-12.5 |
| vcf-grid-k60-c04-r00-v120 | 259.9 | — | — | 0.0 | — | — | -17.9/— | -11.1/— |
| vcf-grid-k60-c04-r00-v80 | 259.9 | 261.6 | 0.7 | 0.0 | 0.0 | 0.0 | -18.1/-17.5 | -11.2/-11.4 |
| vcf-grid-k60-c04-r05-v120 | 259.9 | — | — | 0.0 | — | — | -10.0/— | -11.5/— |
| vcf-grid-k60-c04-r05-v80 | 259.9 | 261.6 | 0.7 | 0.0 | 0.0 | 0.0 | -10.1/-9.4 | -11.4/-11.8 |
| vcf-grid-k60-c04-r10-v120 | 259.9 | — | — | 0.0 | — | — | -8.4/— | -12.4/— |
| vcf-grid-k60-c04-r10-v80 | 259.9 | 261.6 | 0.7 | 0.0 | 0.0 | 0.0 | -8.4/-8.4 | -12.4/-13.2 |
| vcf-grid-k60-c08-r00-v120 | 260.0 | — | — | 0.0 | — | — | -14.1/— | -9.4/— |
| vcf-grid-k60-c08-r00-v80 | 260.0 | 261.6 | 0.6 | 0.0 | 0.0 | 0.0 | -14.3/-13.7 | -9.5/-9.7 |
| vcf-grid-k60-c08-r05-v120 | 259.9 | — | — | 0.0 | — | — | -8.5/— | -10.5/— |
| vcf-grid-k60-c08-r05-v80 | 260.0 | 261.6 | 0.7 | 0.0 | 0.0 | 0.0 | -8.6/-8.9 | -10.5/-11.2 |
| vcf-grid-k60-c08-r10-v120 | 519.8 | — | — | 10.3 | — | — | -10.4/— | -8.6/— |
| vcf-grid-k60-c08-r10-v80 | 519.8 | 523.3 | 0.7 | 10.2 | 10.9 | 0.7 | -10.5/-10.9 | -9.0/-9.2 |
| vcf-grid-k60-c12-r00-v120 | 260.0 | — | — | 0.0 | — | — | -11.3/— | -7.9/— |
| vcf-grid-k60-c12-r00-v80 | 260.0 | 261.6 | 0.6 | 0.0 | 0.0 | 0.0 | -11.4/-11.7 | -7.9/-8.6 |
| vcf-grid-k60-c12-r05-v120 | 519.9 | — | — | 3.4 | — | — | -8.7/— | -8.3/— |
| vcf-grid-k60-c12-r05-v80 | 519.9 | 523.3 | 0.6 | 3.3 | 2.9 | -0.3 | -8.8/-9.3 | -8.0/-8.8 |
| vcf-grid-k60-c12-r10-v120 | 519.9 | — | — | 13.5 | — | — | -11.0/— | -7.9/— |
| vcf-grid-k60-c12-r10-v80 | 519.9 | 523.3 | 0.6 | 13.5 | 15.1 | 1.6 | -11.1/-11.3 | -8.2/-8.9 |
| vcf-grid-k60-c16-r00-v120 | 260.0 | — | — | 0.0 | — | — | -3.3/— | -1.9/— |
| vcf-grid-k60-c16-r00-v80 | 260.0 | 261.6 | 0.6 | 0.0 | 0.0 | 0.0 | -3.4/-3.5 | -2.0/-2.1 |
| vcf-grid-k60-c16-r05-v120 | 1299.9 | — | — | 14.9 | — | — | -3.2/— | -1.7/— |
| vcf-grid-k60-c16-r05-v80 | 1299.9 | 1308.2 | 0.6 | 14.8 | 15.7 | 0.9 | -3.3/-4.2 | -1.7/-3.7 |
| vcf-grid-k60-c16-r10-v120 | 1559.7 | — | — | 35.8 | — | — | -6.0/— | -17.1/— |
| vcf-grid-k60-c16-r10-v80 | 1559.5 | 1569.7 | 0.7 | 35.8 | 35.3 | -0.5 | -6.1/-6.7 | -17.1/-25.4 |
| vcf-grid-k60-c20-r00-v120 | 3379.3 | — | — | 0.2 | — | — | 0.2/— | —/— |
| vcf-grid-k60-c20-r00-v80 | 3379.8 | 261.6 | -92.3 | 0.0 | 0.0 | -0.0 | 0.0/0.0 | —/0.0 |
| vcf-grid-k60-c20-r05-v120 | 1819.7 | — | — | 21.2 | — | — | -1.8/— | -0.1/— |
| vcf-grid-k60-c20-r05-v80 | 1819.9 | 1831.4 | 0.6 | 21.1 | 22.1 | 0.9 | -1.8/-2.7 | -0.3/-1.4 |
| vcf-grid-k60-c20-r10-v120 | 2339.6 | — | — | 45.7 | — | — | -4.6/— | —/— |
| vcf-grid-k60-c20-r10-v80 | 2339.6 | 2354.7 | 0.6 | 45.6 | 41.3 | -4.3 | -4.6/-5.1 | —/— |
| vcf-page7 | 519.6 | 523.3 | 0.7 | 16.1 | 16.0 | -0.1 | -13.3/-12.6 | -8.6/-10.1 |
| vcf-ring-c00 | 259.8 | 261.7 | 0.7 | 6.0 | 5.8 | -0.2 | -14.8/-13.5 | -13.0/-14.2 |
| vcf-ring-c02 | 292.3 | 294.4 | 0.7 | 7.6 | 7.6 | 0.1 | -14.8/-13.3 | -12.4/-13.5 |
| vcf-ring-c04 | 357.2 | 359.8 | 0.7 | 10.4 | 10.5 | 0.1 | -14.4/-13.1 | -11.8/-12.3 |
| vcf-ring-c06 | 422.1 | 425.2 | 0.7 | 13.1 | 13.0 | -0.1 | -13.8/-12.7 | -9.0/-11.3 |
| vcf-ring-c08 | 487.1 | 490.6 | 0.7 | 15.0 | 14.2 | -0.9 | -13.2/-12.6 | -9.0/-10.6 |
| vcf-ring-c10 | 519.6 | 523.3 | 0.7 | 16.1 | 16.0 | -0.1 | -12.7/-12.6 | -8.4/-10.1 |
| vcf-ring-c12 | 552.1 | 556.0 | 0.7 | 17.0 | 16.8 | -0.2 | -12.8/-12.2 | -8.1/-9.8 |
| vcf-ring-c14 | 876.8 | 883.0 | 0.7 | 24.4 | 23.9 | -0.5 | -10.5/-10.3 | -5.9/-7.7 |
| vcf-ring-c16 | 1623.8 | 1602.5 | -1.3 | 36.7 | 35.6 | -1.0 | -7.1/-7.3 | -15.7/-9.0 |
| vcf-ring-c18 | 2338.1 | 2420.1 | 3.5 | 48.2 | 45.9 | -2.3 | -4.9/-4.9 | —/— |
| vcf-ring-c20 | 2370.8 | 2485.5 | 4.8 | 48.8 | 47.1 | -1.7 | -4.9/-4.7 | —/— |
