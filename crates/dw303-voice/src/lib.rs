//! The complete DW-303 signal path, following the TB-303 block diagram:
//!
//! ```text
//! PitchCv -> Vco -> WAVEFORM -> Vcf -> Vca -> OutputStage
//!                      Meg -> AccentSweep -> FilterControl (bias) -^
//!                      Veg ----------------------------------------^ (Vca)
//! ```
//!
//! The circuit runs at an oversampled rate (1x/2x/4x/8x, chosen from the
//! host rate by [`oversampling_for`]) and is decimated with f64 elliptic
//! half-band stages.
use dw303_env::{AccentSweep, FilterControl, Meg, OutputStage, Vca, Veg};
use dw303_osc::{PitchCv, Vco};
use dw303_vcf::{Quality, Vcf};
use ravet_signal::{ConfiguredHalfBand2x, HalfBandIirDesign};

/// Preferred minimum internal circuit rate (4x at 44.1 kHz).
pub const MIN_CIRCUIT_RATE: f64 = 176_400.0;
/// Highest internal rate the circuit stages accept.
pub const MAX_CIRCUIT_RATE: f64 = 384_000.0;
/// Lowest internal rate the circuit stages accept.
const STAGE_MIN_RATE: f64 = 44_100.0;
/// Up to 64x oversampling (hosts down to 1 kHz).
const MAX_DECIMATION_STAGES: usize = 6;

/// How the circuit rate relates to the host rate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateRatio {
    /// Circuit runs n times faster (n = 1..=64, power of two) and is decimated.
    Over(usize),
    /// Circuit runs n times slower (hosts above 384 kHz); its output is
    /// linearly interpolated, so content near the host Nyquist is imaged far
    /// above the audio band.
    Under(usize),
}

/// Circuit/host rate ratio: the smallest oversampling reaching
/// `MIN_CIRCUIT_RATE` within the stages' accepted range, else the nearest
/// accepted rate. Returns `None` only for non-finite or non-positive rates.
pub fn oversampling_for(host_rate: f64) -> Option<RateRatio> {
    if !(host_rate.is_finite() && host_rate > 0.0) {
        return None;
    }
    let valid = |rate: f64| (STAGE_MIN_RATE..=MAX_CIRCUIT_RATE).contains(&rate);
    let factors = [1, 2, 4, 8, 16, 32, 64];
    if let Some(&f) = factors
        .iter()
        .find(|&&f| valid(host_rate * f as f64) && host_rate * f as f64 >= MIN_CIRCUIT_RATE)
        .or_else(|| factors.iter().rev().find(|&&f| valid(host_rate * f as f64)))
    {
        return Some(RateRatio::Over(f));
    }
    [2, 4, 8, 16]
        .into_iter()
        .find(|&n| valid(host_rate / n as f64))
        .map(RateRatio::Under)
}

/// Highest VCO harmonic frequency synthesised (above audibility, below the
/// decimator's stopband at 44.1 kHz host rates).
pub const VCO_BAND_LIMIT_HZ: f64 = 20_000.0;

/// Output node voltage that maps to digital full scale (1.0).
/// VERIFY: calibrate against the TD-03 line level captures.
pub const FULL_SCALE_VOLTS: f64 = 2.0;

/// Knob positions as panel rotation fractions (0 = fully counter-clockwise).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Knobs {
    pub tuning: f64,
    pub cutoff: f64,
    pub resonance: f64,
    pub env_mod: f64,
    pub decay: f64,
    pub accent: f64,
    pub volume: f64,
    pub square: bool,
}

impl Default for Knobs {
    fn default() -> Self {
        Self {
            tuning: 0.5,
            cutoff: 0.5,
            resonance: 0.5,
            env_mod: 0.5,
            decay: 0.5,
            accent: 0.5,
            volume: 0.7,
            square: false,
        }
    }
}

/// The sequencer's control outputs for the current host sample.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VoiceInput {
    /// Semitone index from the sequencer DAC (0 = lowest C).
    pub note: u8,
    pub gate: bool,
    /// True only on the host sample where a new (non-slid) note begins.
    pub trigger: bool,
    pub accent: bool,
    pub slide: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VoiceError {
    InvalidSampleRate,
}

/// Knob smoothing time; the original pots have no zipper, so host steps are
/// slewed before they reach the circuit.
const KNOB_SMOOTHING_SECONDS: f64 = 0.005;

pub struct Voice {
    pitch: PitchCv,
    vco: Vco,
    vcf: Vcf,
    meg: Meg,
    veg: Veg,
    accent: AccentSweep,
    filter_control: FilterControl,
    vca: Vca,
    output: OutputStage,
    decimators: [ConfiguredHalfBand2x; MAX_DECIMATION_STAGES],
    ratio: RateRatio,
    /// Undersampled mode: host samples since the last circuit sample, the
    /// last two circuit outputs, and a trigger latched until the next tick.
    under_phase: usize,
    under_previous: f64,
    under_current: f64,
    under_trigger: bool,
    target: Knobs,
    knobs: Knobs,
    knob_coefficient: f64,
}

impl Voice {
    pub fn new() -> Self {
        // 4x: each stage passes the audio band with >100 dB stopband.
        let design =
            HalfBandIirDesign::from_attenuation(0.1, 110.0).expect("valid half-band design");
        Self {
            pitch: PitchCv::new(dw303_osc::Components::NOMINAL),
            vco: Vco::new(dw303_osc::Components::NOMINAL),
            vcf: Vcf::new(dw303_vcf::Components::NOMINAL),
            meg: Meg::new(dw303_env::Components::NOMINAL),
            veg: Veg::new(dw303_env::Components::NOMINAL),
            accent: AccentSweep::new(dw303_env::Components::NOMINAL),
            filter_control: FilterControl::new(dw303_env::Components::NOMINAL),
            vca: Vca::new(dw303_env::Components::NOMINAL),
            output: OutputStage::new(dw303_env::Components::NOMINAL),
            decimators: core::array::from_fn(|_| ConfiguredHalfBand2x::new(design, design)),
            ratio: RateRatio::Over(4),
            under_phase: 0,
            under_previous: 0.0,
            under_current: 0.0,
            under_trigger: false,
            target: Knobs::default(),
            knobs: Knobs::default(),
            knob_coefficient: 0.0,
        }
    }

    pub fn prepare(&mut self, host_sample_rate: f64) -> Result<(), VoiceError> {
        if !host_sample_rate.is_finite() {
            return Err(VoiceError::InvalidSampleRate);
        }
        let ratio = oversampling_for(host_sample_rate).ok_or(VoiceError::InvalidSampleRate)?;
        let rate = match ratio {
            RateRatio::Over(n) => host_sample_rate * n as f64,
            RateRatio::Under(n) => host_sample_rate / n as f64,
        };
        let ok = self.pitch.prepare(rate).is_ok()
            && self.vco.prepare(rate).is_ok()
            && self.vcf.prepare(rate).is_ok()
            && self.meg.prepare(rate).is_ok()
            && self.veg.prepare(rate).is_ok()
            && self.accent.prepare(rate).is_ok()
            && self.vca.prepare(rate).is_ok()
            && self.output.prepare(rate).is_ok();
        if !ok {
            return Err(VoiceError::InvalidSampleRate);
        }
        self.ratio = ratio;
        // Harmonics above the host band are removed by the decimator anyway.
        self.vco
            .set_band_limit(Some((0.45 * host_sample_rate).min(VCO_BAND_LIMIT_HZ)));
        self.knob_coefficient = 1.0 - (-1.0 / (KNOB_SMOOTHING_SECONDS * rate)).exp();
        self.reset();
        Ok(())
    }

    pub fn reset(&mut self) {
        self.pitch.reset();
        self.vco.reset();
        self.vcf.reset();
        self.meg.reset();
        self.veg.reset();
        self.accent.reset();
        self.vca.reset();
        self.output.reset();
        for d in &mut self.decimators {
            d.reset();
        }
        self.under_phase = 0;
        self.under_previous = 0.0;
        self.under_current = 0.0;
        self.under_trigger = false;
        self.knobs = self.target;
    }

    pub fn set_knobs(&mut self, knobs: Knobs) {
        self.target = knobs;
    }

    pub fn set_quality(&mut self, quality: Quality) {
        self.vcf.set_quality(quality);
    }

    pub fn vcf_diagnostics(&self) -> dw303_vcf::VcfDiagnostics {
        self.vcf.diagnostics()
    }

    fn smooth_knobs(&mut self) {
        let k = self.knob_coefficient;
        // Snap when settled so downstream coefficient caches see identical values.
        let s = |current: &mut f64, target: f64| {
            let delta = target - *current;
            if delta.abs() < 1.0e-7 {
                *current = target;
            } else {
                *current += delta * k;
            }
        };
        s(&mut self.knobs.tuning, self.target.tuning);
        s(&mut self.knobs.cutoff, self.target.cutoff);
        s(&mut self.knobs.resonance, self.target.resonance);
        s(&mut self.knobs.env_mod, self.target.env_mod);
        s(&mut self.knobs.decay, self.target.decay);
        s(&mut self.knobs.accent, self.target.accent);
        s(&mut self.knobs.volume, self.target.volume);
        self.knobs.square = self.target.square;
    }

    /// One oversampled circuit tick, returning the output node voltage.
    fn tick_circuit(&mut self, input: VoiceInput, trigger: bool) -> f64 {
        self.smooth_knobs();
        let k = self.knobs;
        let cv = self.pitch.tick(input.note, input.slide);
        let osc = self.vco.tick(cv, k.tuning);
        let source = if k.square { osc.square } else { osc.saw };
        let meg = self.meg.tick(trigger, input.accent, k.decay);
        let accent_gate = input.accent && input.gate;
        let sweep = self.accent.tick(accent_gate, meg, k.accent, k.resonance);
        let bias = self
            .filter_control
            .bias_current(k.cutoff, k.env_mod, meg, sweep);
        let filtered = self.vcf.tick(source, bias, k.resonance);
        let veg = self.veg.tick(input.gate, trigger);
        let amplified = self.vca.tick(filtered, veg, accent_gate, k.accent);
        self.output.tick(amplified, k.volume)
    }

    /// Render one host-rate sample (digital full scale = `FULL_SCALE_VOLTS`).
    pub fn tick(&mut self, input: VoiceInput) -> f64 {
        match self.ratio {
            RateRatio::Over(n) => self.tick_over(input, n),
            RateRatio::Under(n) => self.tick_under(input, n),
        }
    }

    fn tick_over(&mut self, input: VoiceInput, factor: usize) -> f64 {
        let mut x = [0.0; 1 << MAX_DECIMATION_STAGES];
        let mut count = factor;
        for (i, sample) in x[..count].iter_mut().enumerate() {
            *sample = self.tick_circuit(input, input.trigger && i == 0);
        }
        let mut stage = 0;
        while count > 1 {
            for i in 0..count / 2 {
                x[i] = self.decimators[stage].downsample([x[2 * i], x[2 * i + 1]]);
            }
            count /= 2;
            stage += 1;
        }
        x[0] / FULL_SCALE_VOLTS
    }

    fn tick_under(&mut self, input: VoiceInput, divisor: usize) -> f64 {
        self.under_trigger |= input.trigger;
        if self.under_phase == 0 {
            let trigger = core::mem::take(&mut self.under_trigger);
            self.under_previous = self.under_current;
            self.under_current = self.tick_circuit(input, trigger);
        }
        let t = self.under_phase as f64 / divisor as f64;
        self.under_phase = (self.under_phase + 1) % divisor;
        (self.under_previous + (self.under_current - self.under_previous) * t) / FULL_SCALE_VOLTS
    }

    /// Current circuit/host rate ratio.
    pub fn rate_ratio(&self) -> RateRatio {
        self.ratio
    }
}

impl Default for Voice {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oversampling_covers_common_and_extreme_host_rates() {
        use RateRatio::{Over, Under};
        for (rate, ratio) in [
            (1_000.0, Over(64)),
            (8_000.0, Over(32)),
            (22_050.0, Over(8)),
            (44_100.0, Over(4)),
            (48_000.0, Over(4)),
            (88_200.0, Over(2)),
            (96_000.0, Over(2)),
            (192_000.0, Over(1)),
            (384_000.0, Over(1)),
            (768_000.0, Under(2)),
        ] {
            assert_eq!(oversampling_for(rate), Some(ratio), "{rate}");
        }
        assert_eq!(oversampling_for(0.0), None);
        assert_eq!(oversampling_for(f64::NAN), None);
    }

    #[test]
    fn prepares_and_renders_finite_audio_across_host_rates() {
        for rate in [
            1_000.0, 8_000.0, 44_100.0, 96_000.0, 192_000.0, 384_000.0, 768_000.0,
        ] {
            let mut voice = Voice::new();
            voice.prepare(rate).expect("supported rate");
            let mut input = VoiceInput {
                note: 12,
                gate: true,
                trigger: true,
                accent: true,
                slide: false,
            };
            for _ in 0..(rate as usize / 10) {
                assert!(voice.tick(input).is_finite());
                input.trigger = false;
            }
        }
    }
}
