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

## Revision 2 (schematic-based)

The schematic is now transcribed. The authority is
`reference/schematic/netlist.md` and its per-block files. `Components::NOMINAL`
values must cite reference designators from those files. Anything not printed
on the schematic (transistor models, saturation currents, betas) stays marked
`VERIFY:`.

## `dw303-osc`: pitch CV, VCO and square shaper

```rust
pub struct PitchCv { .. }   // 6-bit DAC + IC11b buffer + slide (IC12, C35)
impl PitchCv {
    /// `note`: sequencer semitone index, 0 = lowest C = 1.000 V (DAC code 11).
    /// Voltage = (code+1)/12 V with code = note + 11, clamped to the 6-bit range.
    /// `slide`: SLIDE line. When high, C35 (0.22 uF) charges through the ladder
    /// Thevenin resistance (100 k, tau 22 ms). When low, C35 tracks the buffered CV.
    /// Returns the CV node voltage (TP4 / CV OUT jack).
    pub fn tick(&mut self, note: u8, slide: bool) -> f64;
}
pub struct Vco { .. }
impl Vco {
    /// `cv`: from PitchCv. `tuning`: VR2 rotation 0..=1 (0.5 = centre, about +/-700 cents).
    pub fn tick(&mut self, cv: f64, tuning: f64) -> VcoOut;
    pub fn frequency_hz(&self, cv: f64, tuning: f64) -> f64; // 1.000 V -> 32.70 Hz at centre
    pub fn set_band_limit(&mut self, hz: Option<f64>);
}
pub struct VcoOut { pub saw: f64, pub square: f64 } // Q28 source (12 V -> 5.5 V ramp), Q8 collector (5-8 V)
```

## `dw303-vcf`: diode-ladder VCF

```rust
pub struct Vcf { .. }
impl Vcf {
    /// `input`: WAVEFORM switch common voltage (before R62 220K / C17).
    /// `ladder_current`: Q11 collector current (A) from FilterControl.
    /// `resonance`: VR4 rotation (gang A, feedback).
    pub fn tick(&mut self, input: f64, ladder_current: f64, resonance: f64) -> VcfOut;
    pub fn set_quality(&mut self, q: Quality);
    pub fn diagnostics(&self) -> VcfDiagnostics;
}
pub struct VcfOut {
    pub output: f64,        // C14 output node (VR4A terminal 3), AC volts
    pub resonance_tap: f64, // VR4A wiper, which also feeds the VCA through C22/R122
}
```

The model must cover:
- rung capacitors C26/C24/C19 = 33 nF and C18 = 18 nF;
- the Q12 input pair with R62/R70 input attenuation and C17 coupling;
- the Q21 differential output with C25/C27 coupling and the Q19 follower;
- the C14 output;
- resonance feedback VR4A → C15 → Q18 → C23 → R97/R98 into the Q12 right base.

## `dw303-env`: trigger, MEG, accent, cutoff control, VEG, VCA, mixer

```rust
pub struct TriggerPulse { .. } // Q38 one-shot (R148/C54, about 1.4 ms)
impl TriggerPulse { pub fn tick(&mut self, trigger: bool) -> f64; } // pulse node voltage
pub struct Meg { .. }  // C62 charge (D37/R152), discharge R138 + VR6 (DECAY) into D26-D28 clamp; Q41/Q40 buffer
impl Meg { pub fn tick(&mut self, pulse: f64, accent: bool, decay: f64) -> f64; } // buffered MEG voltage
pub struct Accent { .. } // IC12 1/2 gate + VR7 ACCENT pot
impl Accent { pub fn tick(&mut self, accent: bool, meg: f64, accent_knob: f64) -> f64; } // VR7 wiper voltage
pub struct AccentSweep { .. } // D24, R46, VR4B gang, C13, R72
impl AccentSweep { pub fn tick(&mut self, accent_wiper: f64, resonance: f64) -> f64; } // VR4B wiper voltage
pub struct FilterControl { .. } // VR3 CUTOFF, TM3, R73, VR5 ENV MOD, R63, R72/R71, Q9 VQ9, Q10/Q11 expo
impl FilterControl {
    pub fn ladder_current(&mut self, cutoff: f64, env_mod: f64, meg: f64, sweep_wiper: f64) -> f64; // A
}
pub struct Veg { .. } // C42 / R123 / Q32 discharge / Q31 control-current source
impl Veg { pub fn tick(&mut self, pulse: f64) -> f64; } // VCA control current (A) before accent
pub struct Vca { .. } // BA662A OTA; input sums VCF output (R121/C21) and resonance tap (R122/C22)
impl Vca {
    pub fn tick(&mut self, vcf: VcfSignals, veg_current: f64, accent_wiper: f64) -> f64; // C38 node
}
pub struct VcfSignals { pub output: f64, pub resonance_tap: f64 } // mirror of dw303_vcf::VcfOut (no crate dependency)
pub struct OutputStage { .. } // VR8 VOLUME (50K A), mixer Q34/Q33 (VCA -2.13, MIX IN -1), C56/R161 to J6
impl OutputStage { pub fn tick(&mut self, vca: f64, volume: f64, mix_in: f64) -> f64; } // OUTPUT jack volts
```

All stages also provide `new(Components)`, `prepare(rate)` and `reset()`.

## Shared error type

Each crate defines its own:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageError { InvalidSampleRate }
```
