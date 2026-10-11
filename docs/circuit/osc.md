# Pitch CV, VCO and square shaper — revision 2

Authority: [reconciled netlist](../../reference/schematic/netlist.md), the entire
[VCO transcription](../../reference/schematic/transcription/vco.md), and the
binding [revision-2 contract](contracts.md). Changes are confined to
`crates/dw303-osc` and this document. No dependencies were added.

## Printed components and calibration

`Components::NOMINAL` cites the relevant designators. The DAC uses the selected
200k R74–R90/R96 network, with R78 terminating at the reference rail. Its output
is `(code+1)/12 V`. The API accepts a semitone index, maps it to
`code=min(note+11,63)`, and therefore gives C=1 V (note 0), A=2.75 V (21),
upper C=4 V (36), and a maximum of 64/12 V (52 and above).

C35 is **0.22 uF**. With SLIDE high, IC12 A connects it to the ladder's
**100k Thevenin resistance**, giving **22 ms**, 48.34 ms 10–90%, and 101.31 ms
to 99%. IC11b follows this ladder/C35 node to TP4/CV OUT. With SLIDE low,
IC12 B connects C35 to the buffered output and precharges it immediately.
`PitchCv` stores the capacitor voltage and uses the exact held-input exponential.
The contract accepts already-latched notes; it does not reproduce IC9's clock.

Q26 is the **2SC1583 matched NPN pair**. IC11a holds Q26a's reference collector
at the 5.333 V reference. R103 220k supplies about 30.3 uA, supplemented by
TM4 47k/R102. R104 2.2k and C34 1 nF belong to the reference regulator; this
model treats it as ideal. Rather than claiming to resolve R102's ambiguous unit,
TM4's net current is calibrated to the printed page-5/page-7 A adjustment:
**2.75 V = 110 Hz at centre tuning**. Thus 1 V = **32.703195663 Hz**, i.e.
the printed 32.70 Hz lowest C, not the old model's octave-higher value.

The expo base divider uses **R106 27k + TM5 4.7k rheostat**, **R118 100k**,
and **R100 560 ohm posistor**. At the chosen 300 K operating point,
`gain_cv = (1/Rcv)/(1/Rcv + 1/R118 + 1/R100)`.
TM5's nominal position is solved for `gain_cv = (kT/q) ln(2)`;
it is within the printed trimmer range (about 3.6k). That position is held
when components vary. The current ratio is `exp(deltaVbe/Vt)`.
R100's estimated ideal PTC coefficient compensates Vt with temperature;
tests bound the remaining two-octave tracking error by the page-7 **±0.5%** target.

VR2 is **50k(B), linear**, coupled through R118. Its source resistance is
`50k*x*(1-x)`, so the model includes loading in the rotation curve and the
expo divider's R118 gain. The nominal endpoint span is calibrated to the
page-1 **±700 cents** specification. This is an explicit resolution policy,
not a claim that VR2 pin 3 is connected to a particular rail: O3 remains open.
A 5.333 V endpoint predicts roughly ±0.8 octave; an open pin predicts only
about 0.53 octave total, neither matching the specified ±0.5833 octave.
The effective voltage span is about 3.8 V with this divider. The centre-current
adjustment absorbs the unknown DC offset. Rail identification requires further
evidence. The loaded curve differs slightly from a straight cents-vs-angle line.

## Saw topology and analytic cycle

**Q26b sinks current from C33 10 nF**, producing a falling ramp. As its voltage
reaches the threshold established by diode-connected Q24 and R101 10k to the
reference rail, Q25 conducts and pulls Q27's PNP base down. Q27 supplies the
reset through D25/RST_X, giving an upward regenerative reset. This replaces
the old model's rising charge and downward discharge topology.

The printed **Q28 2SK30(O) source / R105 10k** waveform falls **12 V to 5.5 V**.
The analytic model uses these source-node endpoints, treating the follower as
an ideal calibrated transfer and Q26b as a constant-current sink:
`Qcycle = C33*(12-5.5) = 65 nC`, so `I_A = 110*65 nC = 7.15 uA`.
`charging_current_a` exposes the sink current. C33 variation changes the period
without changing the calibrated converter current. Cycle phase is an analytic
coordinate for the capacitor voltage; there is no sampled reset discontinuity.

The printed reset is effectively instantaneous. Unlike the old arbitrary
current-tracking reset fraction, the new cycle is a falling linear ramp with
instantaneous upward reset. Its DC is 8.75 V and sine coefficient is
`6.5/(pi*h)`. The harmonic projection is calculated once, with a maximum of
256 harmonics, a raised-cosine taper over harmonics 224–256, and a rate-dependent
pass/stop band at 0.40/0.48 times the stage rate. Optional bandwidth has a 20%
transition. Reconstruction can overshoot the endpoints near reset (Gibbs ringing).
Frequency and bandwidth coefficients retain bitwise caches; slides are never
quantised. Device reset time, Early-effect curvature and follower loading
remain limitations, rather than invented component values.

## Q8 square topology

The base has **R35 100k directly from the saw**, in parallel with
**C10 10 nF in series with R34 10k**. The emitter is supplied from **12 V through
R45 22k** and stored on **C11 1 uF**. The collector returns through
**R36 10k to 5.333 V**. There is **no output coupling capacitor** in this block.
The old arbitrary base bias/divider and two coupling high-passes were removed.

`SquareShaper` stores C10 voltage and C11 emitter voltage. Backward-Euler
companions solve the actual two-capacitor network with a piecewise-linear PNP
in cutoff, forward-active and saturation regions. All branches are analytic,
with no iterative solve. In active operation:
`ib = g_base*(Ve-Veb) - drive`, `ic = beta*ib`, and `ie = ib+ic`.
In saturation, `Vc=Ve-Vec_sat`; collector load current and base current both
load C11. This generates a non-50% pulse and drooping high plateau directly
from the saw. R45*C11 is **22 ms** unloaded. R34*C10 is **0.1 ms** when the
base is stiff; with Q8 off, C10 discharges through R34+R35, **1.1 ms**.

The schematic's low annotation rounds the actual return rail to **5 V**;
the model correctly settles at **5.333333 V**, including when Q8 remains off.
At A=110 Hz, the estimated Q8 model gives duty about **44.5–44.8%**, a high
plateau averaging **8.49–8.50 V**, and peak **8.89–8.90 V**. This exceeds the
printed approximately 8 V high annotation. The tests expose this limitation:
plateau tolerance is 0.5 V, peak tolerance 1 V. These are coarse device-model
checks, not a precision waveform agreement claim. Q8 needs a measured/SPICE
part model before claiming the printed high level is reproduced accurately.

## Complete VERIFY register and open-item disposition

| Code VERIFY item | Assumption / remaining evidence |
|---|---|
| `tuning_octaves` / O3 | ±700 cents from page-1 specification; pin-3 rail unidentified; loaded linear pot with calibrated endpoint span |
| `posistor_temperature_coefficient` | R100 assumed ideal 1/300 per kelvin PTC; actual curve unprinted |
| `transistor_vbe` / O6 | Q8 constant 0.65 V forward junction; unprinted device type |
| `transistor_beta` | Q8 beta 200 estimated, not measured |
| `transistor_vce_sat` | Q8 0.2 V saturation estimate |
| `cv_for_note` | IC10 supply unprinted; ideal high is the DAC reference rail, 64/12 V |
| `PitchCv` / O2 | Already-latched DAC input; IC9 clock/SLIDE shared-net ambiguity delegated to sequencer under the binding API |
| `PitchCv` / O7 | Ideal IC12 switches and IC11b follower; supply unprinted; omit Ron, settling, clamps' junction physics and Q29/Q30 switching transients |
| `SquareShaper` | Piecewise-linear Q8 omits smooth junction curvature, reverse-active operation and parasitics |
| `Vco` reset | Instantaneous regenerative reset follows printed waveform; no reset recovery model |
| `Vco` Q25 / O5 | NPN selected from reconciled topology; Q24/Q25 Vbe and D25 drop absorbed into printed endpoint |
| `Vco` Q26 | Ideal sink; Early effect, offsets and pair mismatch omitted |
| `Vco` reference / O1 | Ideal IC11a/R104/C34 regulation; TM4 current calibrated rather than choosing 100 ohm vs 100k for R102 |
| `Vco` follower | Q28/R105 transfer folded into annotated source endpoints; no JFET device model or base-network back-loading |

O4 (printed R96 in the R86 position) has no electrical consequence; use the
transcribed 200k arm. TM4=47k follows reconciliation L3, without claiming its
missing printed glyph was recovered. O6 also covers unspecified reset, clamp
and slide semiconductors; their reductions are listed above. No unresolved
printed topology was silently promoted to verified device physics.

## Validation and performance

The frozen provisional regression/reference files were deleted. Tests now
check schematic-derived DAC annotations and six-bit saturation, bidirectional
22 ms slide, measured pitch within one cent at 44.1/48/96/192/384 kHz,
110 Hz adjustment, 1 V/oct and four-to-one tracking, ±700 cents tuning,
resistor/C33 sensitivity, falling saw/DC/endpoints, square DC/plateau/duty,
C11 22 ms recharge and C10 1.1 ms cutoff discharge, extreme inputs, lifecycle,
diagnostics, deterministic reset and a two-million-sample finite-state run.
The steady saw alias test retains a coherent FFT and ideal fourfold decimation;
it measures about **−243 dB** non-harmonic energy below host Nyquist, a numerical
floor for this analytic source, not whole-instrument or square alias performance.

Validation commands (only this package):

```
cargo test --offline -p dw303-osc -- --nocapture
cargo clippy --offline -p dw303-osc --all-targets -- -D warnings
cargo --offline fmt -p dw303-osc -- --check
cargo test --offline --release -p dw303-osc --test performance -- --ignored --nocapture
```

The manual timing test uses 384,000 samples at 192 kHz, CV switching through
0–3 V every 24,000 samples, and a 20 kHz passband. Before edits, the optimized
provisional crate measured **149.08 ms** on this machine. Revision 2 measured
**153.83 ms** in the normal workspace check (**154.19 ms** in isolation),
about **1.03x** the previous cost;
construction is excluded. CPU evidence is wall-clock timing, not an instrument
profile. A temporary isolated manifest was needed while another agent's env
crate temporarily had no source target; it was removed before delivery. The
normal workspace checks subsequently passed: **11 tests**, the separately run
manual release benchmark, clippy across all targets with warnings denied, and
package-scoped rustfmt verification. The timing test is ignored in normal runs.

The library remains `no_std`, with fixed arrays, bounded harmonic loops, no
unsafe code or heap allocation, finite-input sanitisation and denormal flushing.
No allocation-counter instrumentation is claimed. The harmonic cap reduces
very-low-note bandwidth; fast pitch modulation can create sidebands, and the
nonlinear square still requires the voice's oversampling/decimation. These
tests establish circuit-derived invariants, not SPICE or hardware agreement.
