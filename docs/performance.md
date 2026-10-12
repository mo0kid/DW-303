# Circuit performance pass — 2026-10-12

The circuit equations, topology, component defaults and public integration APIs
are unchanged. This pass changes numerical evaluation and voice overhead in
`dw303-osc`, `dw303-vcf`, `dw303-env` and `dw303-voice` only. Authority remains
[the contract](circuit/contracts.md) and [the netlist](../reference/schematic/netlist.md).
No new dependency, manifest change, commit or branch operation was made.

## Renderer CPU

Release build, Rust 1.99.0, this arm64 Apple-silicon Mac. The sandbox denies
chip-name `sysctl`; no exact chip model is asserted. Three exclusive runs per
pitch, **16-second held MIDI notes, 48 kHz host / 192 kHz circuit, Full quality,
default knobs**, with file writing outside the renderer's timed processing.
MIDI keys 24/48 are C1/C3. Before runs use the saved original executable;
after runs use the final executable. Percentages below are computed from the
renderer's elapsed seconds printed to three decimal places.

| Pitch | Before median (range), % core | After median (range), % core | Reduction |
|---|---:|---:|---:|
| C1 | 18.04 (18.04–18.23) | 9.94 (9.93–9.96) | 44.89% |
| C3 | 15.77 (15.74–15.93) | 9.57 (9.56–9.57) | 39.32% |

All six final holds report zero non-convergence and zero sanitized inputs,
maximum three Newton evaluations. C1 uses 4,983,047 evaluations for 3,072,000
circuit ticks; C3 uses 5,646,155. CPU target is met for these renderer holds;
C1 has little margin. Early envelope activity, retriggers, slides, automation
and other settings/rates can cost more. This is measured performance, not an
all-workload upper bound.

## Stage profile

Profiled before editing the model using `std::time::Instant`, not `sample`.
The reproducible ignored benchmark is
`crates/dw303-voice/tests/performance.rs`. It batches each stage over actual
outputs from the preceding stages, with allocations confined to benchmark
setup and preallocated test buffers. Each timing uses 1,536,000 circuit ticks
(eight seconds at 192 kHz); clocks are outside the tick loops. Buffer writes
and `black_box` overhead remain included. The original linked benchmark
executable is retained in `/tmp/perf-profile-before`.

Exclusive before/after timings, **nanoseconds per circuit tick**:

| Group | C1 before | C1 after | C3 before | C3 after |
|---|---:|---:|---:|---:|
| Pitch CV + VCO, including square-state updates | 513.34 | 243.62 | 383.10 | 177.69 |
| Trigger + MEG + accent + sweep + cutoff + VEG | 85.53 | 75.70 | 84.69 | 74.11 |
| VCF | 319.77 | 202.41 | 335.03 | 241.79 |
| VCA + output | 22.27 | 21.24 | 21.01 | 20.75 |

The separate eight-second whole-voice timing is 18.17% → 10.12% at C1 and
15.90% → 9.72% at C3. Its shorter age distribution includes proportionally
more early envelope work than the 16-second renderer holds. Isolated group
costs do not exactly sum to the composed cost: buffer traffic and compiler
context differ. Preserve this distinction when comparing future changes.

## Retained changes

- VCO: four independent harmonic rotations and sums enable packed arithmetic;
  cached weights are folded into sine/cosine coefficients. The instantaneous
  frequency and harmonic-band decisions remain exact; no frequency epsilon
  or quantization was introduced. Held-frequency phase rotation is resynced
  with `libm::sincos` every 64 circuit ticks; changing frequency evaluates the
  actual phase each tick, with lazy step construction.
- Q8: cache fixed reciprocal conductances/scales at preparation. Select the
  saturation branch with its monotone load-line boundary before solving,
  and stop Halley updates only at a bitwise fixed root. Both C10 and C11
  still update on every tick, including while saw is selected.
- VCF: inline flow/residual evaluation, cache fixed coupling reciprocals,
  reuse accepted Q21 output and omit Jacobian construction only when its
  residual is already accepted. The ten capacitor states, analytic Jacobian,
  sparse solve, cubic predictor, eight-iteration/eight-backtrack limits and
  held-solution fallback remain.
- VCF junctions: degree-thirteen tanh series below |x|=0.1 (remainder <1.5e-18)
  and the denominator-19 continued fraction folded into polynomials through
  |x|=1 (truncation <2e-19); retain the bounded exponential elsewhere.
  Junction and derivative grids retain their original tight error checks.
- ENV: exact zero-state trigger and inactive accent shortcuts retain input
  diagnostics. Q31 uses `W(z)=z-z²+1.5z³` for −80 ≤ x < −16, where z=exp(x)
  and the omitted term is <4.3e-28; the previous bounded solve remains
  elsewhere. Charge feedback and the low-current tail remain. Q10/Q11
  current is reused only for four bitwise-identical sanitized controls.
- Voice: settled knob smoothing skips repeated assignments of identical bits,
  retaining the original path for invalid targets. Direct 2×/4× decimation
  avoids the 64-sample temporary and variable reduction loop. Tests compare
  it bit-for-bit with the generic path during notes, slides, accents, knob
  changes and waveform switches; smoothing tests include NaN, infinity and
  signed zero.

## Numerical budget and component register

**Printed nominal → fitted component changes: none.** Existing fitted
oscillator, filter, envelope, pot and output values are unchanged.

The numerical Newton residual budget changes from a fixed **1e-13 V** to
**1e-13 V + 4e-8 × max(|four differential rung voltages|)**. Coupling-capacitor
DC biases are excluded from this scale. The original absolute limit remains
near silence; this is a solver precision choice, not a new device law or
component fit. Its acceptance requires the independent small-signal and decay
tests plus the strict **1e-6 full-scale** replay gate. The 1e-5 rounding exception
is not used.

Rejected experiments: eight harmonic chains, polynomial Q31 logarithms,
reused previous Jacobians and sixth-degree prediction gave insufficient CPU
benefit or extra iterations. Fixed 1e-11 V stopping failed small-signal AC and
quiet-tail tests; fixed 1e-9 V also exceeded replay error. Relative budgets
1e-7 and 5e-8 exceeded the strict replay gate (1.41934e-6 and 1.00955e-6);
they were reverted. No physics test tolerance was weakened.

**New circuit VERIFY items: none.** All existing device, rail, tolerance and
reduction assumptions remain in the [OSC register](circuit/osc.md),
[VCF register](circuit/vcf.md), [VCF round-5 register](circuit/vcf-fitting-round5-2026-10-12.md)
and [ENV register](circuit/env.md). In particular, fitted transistor gains,
Q21 excess gain, square-shaper device estimates, ideal followers/switches,
pot tracks and recording-chain calibration are still provisional.

## Equivalence and validation

Before edits, saved all 346 unmodified replay WAVs to `/tmp/perf-ref` and the
original executable to `/tmp/perf-ref-render`. Every retained optimization
was replayed and compared against that baseline. Final comparison covers
**346/346 renders, 192,480,000 float32 samples**, with identical lengths and
finite output. Maximum absolute difference is **5.736947059631348e-7** full
scale, in `vcf-grid-k24-c20-r10-v120`; every capture passes 1e-6.
Detailed records are `/tmp/perf-final5-comparison.json`, and accepted WAVs are
`/tmp/perf-final5`. Outputs are equivalent within the requested tolerance;
they are not asserted byte-identical.

All ten workspace packages, explicitly scoped with offline `-p` commands:
**128 tests passed, zero failed, five ignored manual/platform tests**.
Modified-package all-target clippy with `-D warnings`, rustfmt checks and
`git diff --check` pass. New tests retain independent serial-harmonic checks
at five circuit rates during held/variable pitches and bandwidth changes,
and check derivative omission, exact voice reductions and settled smoothing.
No allocation, lock, unsafe code, panic or unbounded loop was added to ticks.

Caches assume immutable component sets and the current static device
reductions; future mutable temperature/component models must invalidate them.
The replay gate covers this session and the retained test grids, rather than
all possible automation or component variations. Relative numerical precision
is deliberately looser for driven states; future physics changes require a
fresh equivalence/AC/decay audit. Remaining CPU is dominated by the harmonic
VCO and ladder evaluations; the hardware-fit errors are retained.

## Reproduction and artifacts

```sh
cargo build --offline --release -p dw303-render
./target/release/dw303-render /tmp/c1.wav --hold 24 --seconds 16 --rate 48000
./target/release/dw303-render /tmp/c3.wav --hold 48 --seconds 16 --rate 48000
cargo test --offline --release -p dw303-voice --test performance -- --ignored --nocapture
python3 tools/analysis/harness/score.py /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session1 --cache /tmp/perf-harness-cache --json /tmp/perf-score.json --jobs 4
python3 tools/analysis/harness/solver_audit.py /Volumes/PROTOOLS/Dropbox/DEVELOPER/dw-303/reference/captures/2026-10-12-session1 --json /tmp/perf-solver-audit.json --jobs 4
```

CPU records: `/tmp/perf-cpu-candidate12.json` (original rows only),
`/tmp/perf-cpu-final5.json`; stage profiles:
`/tmp/perf-profile-before-exclusive.log`, `/tmp/perf-profile-final5-exclusive.log`.
Baseline full score: `/tmp/perf-before-score.json`.

## Full-session score and solver audit

The final default **Harness v4** run analyses **346/346** captures with zero
analysis failures. Overall error is **0.763448944799 → 0.763449786372**
(change **+0.000000841573**), within ±0.0005.
Category aggregates are `0.75 × median + 0.25 × p90`:

| Category | Before | After | Change |
|---|---:|---:|---:|
| pitch | 0.139133648 | 0.139133647 | -0.000001% |
| tuning | 0.214151217 | 0.214151357 | +0.000065% |
| waveform | 0.739873072 | 0.739877716 | +0.000628% |
| vcf | 0.704221256 | 0.704223437 | +0.000310% |
| envelopes | 0.347450999 | 0.347450600 | -0.000115% |
| amplitude | 1.351360429 | 1.351360575 | +0.000011% |
| accent | 0.441316139 | 0.441316263 | +0.000028% |
| slide | 0.039124674 | 0.039124674 | +0.000000% |
| gate | 0.664427825 | 0.664427825 | +0.000000% |
| behavior | 2.579377129 | 2.579377119 | -0.000000% |

Every category change is far below 2%; no hardware-fit adjustment was made.
The full replay solver audit confirms **346 captures, zero non-convergence,
zero sanitized inputs, maximum four Newton evaluations**. Exact counters and
source/library hashes are `/tmp/perf-solver-audit.json`.

Original renderer SHA-256: `fb96bdaeca3f68cb9d36602008f132549bbe759b40747818f66f9fa027ce2825`.
Final renderer SHA-256: `bec944ad7a5fa195aa9a1e1f5b97903416823eab0fa887a97aa30f045a139f19`.
Scoring analysis SHA-256 (both runs): `21e97c904de319dad15c5276ceaf4cc3fdef79b5f41e7d1aae4d42484afc4583`.

Final scores: `/tmp/perf-score.json` and `/tmp/perf-score.md`; the accepted
renderer, replay audit and score identities agree. These artifacts are local
validation records; the integrator should preserve them before `/tmp` cleanup.
