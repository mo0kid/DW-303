//! The complete DW-303 signal path, following the TB-303 block diagram:
//!
//! ```text
//! PitchCv -> Vco -> WAVEFORM -> Vcf -> Vca -> OutputStage
//!                      Meg -> AccentSweep -> FilterControl (bias) -^
//!                      Veg ----------------------------------------^ (Vca)
//! ```
//!
//! The circuit runs at `OVERSAMPLING` times the host rate and is decimated
//! with two f64 elliptic half-band stages.
use dw303_env::{AccentSweep, FilterControl, Meg, OutputStage, Vca, Veg};
use dw303_osc::{PitchCv, Vco};
use dw303_vcf::{Quality, Vcf};
use ravet_signal::{ConfiguredHalfBand2x, HalfBandIirDesign};

pub const OVERSAMPLING: usize = 4;

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
    decimators: [ConfiguredHalfBand2x; 2],
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
            decimators: [
                ConfiguredHalfBand2x::new(design.clone(), design.clone()),
                ConfiguredHalfBand2x::new(design.clone(), design),
            ],
            target: Knobs::default(),
            knobs: Knobs::default(),
            knob_coefficient: 0.0,
        }
    }

    pub fn prepare(&mut self, host_sample_rate: f64) -> Result<(), VoiceError> {
        if !(host_sample_rate.is_finite() && host_sample_rate >= 8_000.0) {
            return Err(VoiceError::InvalidSampleRate);
        }
        let rate = host_sample_rate * OVERSAMPLING as f64;
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
        let mut x = [0.0; OVERSAMPLING];
        for (i, sample) in x.iter_mut().enumerate() {
            *sample = self.tick_circuit(input, input.trigger && i == 0);
        }
        let a = self.decimators[0].downsample([x[0], x[1]]);
        let b = self.decimators[0].downsample([x[2], x[3]]);
        self.decimators[1].downsample([a, b]) / FULL_SCALE_VOLTS
    }
}

impl Default for Voice {
    fn default() -> Self {
        Self::new()
    }
}
