# DW-303 circuit-model contracts

Every circuit crate builds against this contract. Each stage crate is written
independently. `dw303-voice` composes them later, so the interfaces below are
the integration boundary. Change them only by editing this file first.

## Shared conventions (all circuit crates)

- **Builds:** `#![no_std]`, using `libm` for math. Tests may use `std` via
  `extern crate std` under `cfg(test)`. No `unsafe`.
- **Numbers and units:** all state and math is in `f64` SI units: volts,
  amperes, seconds, hertz, ohms and farads. Signals are the **voltage at the
  named circuit node**, not normalised ±1.
- **Rates:** the stage runs at the sample rate passed to `prepare`. The voice
  runs stages at an oversampled internal rate, nominally 4× the host rate;
  stages must not assume any particular rate in 44.1 kHz–384 kHz.
- **Lifecycle:** the hot path is `prepare(sample_rate) -> Result<(), StageError>`,
  `reset()` and `tick(...)`. It must not allocate, panic or loop without a
  bound. Non-finite input is sanitised to 0 and counted in diagnostics.
- **Knob inputs:** given as the **panel rotation fraction 0.0..=1.0**, fully
  counter-clockwise to fully clockwise. Each crate applies the pot taper and
  wiring from the schematic itself. Calibration against the TD-03 may later
  replace the taper tables.
- **Component values:** each crate has a `Components` struct with a
  `Components::NOMINAL` constant. Values come from the TB-303 service-notes
  schematic and are named by reference designator where known (`R95`, `C33`,
  …).
  - Mark any value not yet checked against the schematic with a doc comment
    containing `VERIFY:`.
  - All `VERIFY:` items are listed in the crate's `docs/circuit/<stage>.md`.
  - Never present an unverified value as authoritative.
- **Determinism:** no randomness. Tolerance offsets are introduced later through
  `Components` values only.
- **Tests:** unit tests sit beside the code, and physics/behaviour tests go in
  `tests/`. The tests check:
  - stability over long runs at extreme settings
  - finite output
  - no denormal-scale stalls
  - sample-rate independence of time constants and frequencies (within
    tolerance at 48k/96k/192k)
  - the documented qualitative behaviour of the original circuit

## `dw303-osc`: pitch CV, VCO and square shaper

```rust
pub struct PitchCv { .. }          // sequencer DAC + slide lag
impl PitchCv {
    pub fn new(c: Components) -> Self;
    pub fn prepare(&mut self, sample_rate: f64) -> Result<(), StageError>;
    pub fn reset(&mut self);
    /// `note`: semitone index from the sequencer (0 = lowest note, C).
    /// The octave/transpose are already folded into the index.
    /// `slide`: slide switch state for the current step.
    /// Returns the control voltage presented to the VCO expo converter (V).
    pub fn tick(&mut self, note: u8, slide: bool) -> f64;
}
pub struct Vco { .. }
impl Vco {
    pub fn new(c: Components) -> Self;
    pub fn prepare(&mut self, sample_rate: f64) -> Result<(), StageError>;
    pub fn reset(&mut self);
    /// `cv`: from PitchCv. `tuning`: TUNING knob 0..=1 (0.5 = centre).
    pub fn tick(&mut self, cv: f64, tuning: f64) -> VcoOut;
    pub fn frequency_hz(&self, cv: f64, tuning: f64) -> f64; // static expo law
}
pub struct VcoOut { pub saw: f64, pub square: f64 } // node voltages at the WAVEFORM switch
```

Saw reset discontinuities need band-limited correction (BLEP/BLAMP or an
equivalent); naive resets are not acceptable. The square comes from the
transistor waveshaper driven by the saw, not from a comparator.

## `dw303-vcf`: diode-ladder VCF

```rust
pub struct Vcf { .. }
impl Vcf {
    pub fn new(c: Components) -> Self;
    pub fn prepare(&mut self, sample_rate: f64) -> Result<(), StageError>;
    pub fn reset(&mut self);
    /// `input`: voltage at the VCF input node (after WAVEFORM switch/mixing).
    /// `bias_current`: ladder bias current from the cutoff expo converter (A).
    /// `resonance`: RESONANCE knob 0..=1 (feedback gang only).
    /// Returns the VCF output node voltage.
    pub fn tick(&mut self, input: f64, bias_current: f64, resonance: f64) -> f64;
    pub fn set_quality(&mut self, q: Quality);   // Full (Newton) | Economy
    pub fn diagnostics(&self) -> VcfDiagnostics; // iterations, non-convergence count
}
```

The model has these properties:
- It is solved by Newton–Raphson per sample with trapezoidal (ZDF)
  discretisation, an analytic Jacobian, a warm start and bounded iterations.
- The ladder stage capacitors are unequal, as on the original.
- There is a feedback path through the RESONANCE pot, and a high-pass coupling
  in the loop that reduces resonance at low cutoff.
- Economy mode is a cheaper approximation, tested against Full.

## `dw303-env`: MEG, VEG, accent sweep, cutoff control, VCA, output

```rust
pub struct Meg { .. }   // main (filter) envelope generator
impl Meg {
    /// `trigger`: true on the sample a new note's gate begins (not on slid steps).
    /// `accent`: accent active for the current step (forces minimum decay).
    pub fn tick(&mut self, trigger: bool, accent: bool, decay: f64) -> f64; // V
}
pub struct Veg { .. }   // VCA envelope generator
impl Veg { pub fn tick(&mut self, gate: bool, trigger: bool) -> f64; } // V
pub struct AccentSweep { .. }  // accent RC network, dual-gang with RESONANCE
impl AccentSweep {
    /// `accent_gate`: accent && gate for the current step. `meg`: MEG voltage.
    pub fn tick(&mut self, accent_gate: bool, meg: f64, accent: f64, resonance: f64) -> f64; // V
}
pub struct FilterControl { .. } // cutoff pot + env mod + accent sweep -> expo converter
impl FilterControl {
    /// Returns the VCF ladder bias current (A), the input to `Vcf::tick`.
    pub fn bias_current(&self, cutoff: f64, env_mod: f64, meg: f64, accent_sweep: f64) -> f64;
}
pub struct Vca { .. }
impl Vca { pub fn tick(&mut self, input: f64, veg: f64, accent_gate: bool, accent: f64) -> f64; }
pub struct OutputStage { .. } // coupling HPF + VOLUME pot + buffer
impl OutputStage { pub fn tick(&mut self, input: f64, volume: f64) -> f64; }
```

All of these also implement `new(Components)`, `prepare` and `reset` as above.

## Shared error type

Each crate defines its own:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageError { InvalidSampleRate }
```
