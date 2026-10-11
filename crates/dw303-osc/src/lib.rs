#![no_std]
//! TB-303 pitch voltage, exponential VCO, capacitor reset and PNP square shaper.
//!
//! Voltages are SI node voltages. See `docs/circuit/osc.md` for calibration
//! assumptions, unverified values and the bandwidth/CPU tradeoff.

use core::f64::consts::{LN_2, PI, TAU};

const HARMONICS: usize = 256;
const BOLTZMANN_OVER_Q: f64 = 8.617_333_262_145e-5;

/// Stage preparation error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageError {
    /// Rate is non-finite or outside the supported 44.1–384 kHz interval.
    InvalidSampleRate,
}

/// Component/calibration values; all quantities are SI.
///
/// Reference designators are deliberately omitted until the schematic scan can
/// be read reliably. Invalid/out-of-range values fall back individually to nominal.
#[derive(Clone, Copy, Debug)]
pub struct Components {
    /// VERIFY: slide switch's effective series resistance, ohms.
    pub slide_resistance: f64,
    /// VERIFY: slide storage capacitor, farads (nominal RC = 60 ms).
    pub slide_capacitance: f64,
    /// VERIFY: lowest sequencer C calibration in Hz (C2 in MIDI convention).
    pub reference_hz: f64,
    /// Matched-pair junction temperature, kelvin; nominal laboratory temperature.
    pub temperature_kelvin: f64,
    /// VERIFY: full TUNING pot excursion in octaves; linear pot/summing assumption.
    pub tuning_octaves: f64,
    /// VERIFY: timing capacitor in farads.
    pub timing_capacitance: f64,
    /// VERIFY: low reset endpoint of timing/source-follower node, volts.
    pub saw_low: f64,
    /// VERIFY: high threshold of timing/source-follower node, volts.
    pub saw_high: f64,
    /// VERIFY: charging transistor effective Early voltage, volts.
    pub early_voltage: f64,
    /// VERIFY: reset fraction of cycle; reset conductance tracks charging current.
    pub reset_fraction: f64,
    /// VERIFY: reset spans this many exponential RC time constants.
    pub reset_time_constants: f64,
    /// VERIFY: square PNP emitter supply, volts.
    pub square_supply: f64,
    /// VERIFY: square base DC bias below emitter, volts.
    pub square_bias: f64,
    /// VERIFY: saw-to-base coupling divider gain.
    pub square_drive: f64,
    /// VERIFY: input coupling resistance, ohms.
    pub square_input_resistance: f64,
    /// VERIFY: input coupling capacitor, farads.
    pub square_input_capacitance: f64,
    /// VERIFY: PNP collector load resistor, ohms.
    pub square_collector_resistance: f64,
    /// VERIFY: collector parasitic/loading capacitor, farads.
    pub square_collector_capacitance: f64,
    /// VERIFY: output coupling load resistance, ohms.
    pub square_output_resistance: f64,
    /// VERIFY: output coupling capacitor, farads.
    pub square_output_capacitance: f64,
    /// VERIFY: 2SA733 substitute saturation current, amperes.
    pub transistor_is: f64,
    /// VERIFY: 2SA733 substitute forward common-base current gain.
    pub transistor_alpha: f64,
}

impl Components {
    /// Best-estimate nominal values, pending schematic/SPICE validation.
    pub const NOMINAL: Self = Self {
        slide_resistance: 100_000.0,
        slide_capacitance: 0.6e-6,
        reference_hz: 65.406_391_325_149_66,
        temperature_kelvin: 298.15,
        tuning_octaves: 1.0,
        timing_capacitance: 10e-9,
        saw_low: 0.5,
        saw_high: 5.0,
        early_voltage: 60.0,
        reset_fraction: 0.0025,
        reset_time_constants: 4.6,
        square_supply: 5.333,
        square_bias: 0.60,
        square_drive: 0.40,
        square_input_resistance: 100_000.0,
        square_input_capacitance: 100e-9,
        square_collector_resistance: 22_000.0,
        square_collector_capacitance: 150e-12,
        square_output_resistance: 100_000.0,
        square_output_capacitance: 220e-9,
        transistor_is: 1e-14,
        transistor_alpha: 0.995,
    };

    fn valid(mut self) -> Self {
        let n = Self::NOMINAL;
        macro_rules! bound {
            ($field:ident, $lo:expr, $hi:expr) => {
                if !self.$field.is_finite() || !($lo..=$hi).contains(&self.$field) {
                    self.$field = n.$field;
                }
            };
        }
        bound!(slide_resistance, 1.0, 1e9);
        bound!(slide_capacitance, 1e-12, 1.0);
        bound!(reference_hz, 1.0, 10_000.0);
        bound!(temperature_kelvin, 200.0, 400.0);
        bound!(tuning_octaves, 0.0, 4.0);
        bound!(timing_capacitance, 1e-12, 1e-3);
        bound!(saw_low, 0.0, 10.0);
        bound!(saw_high, 0.1, 20.0);
        if self.saw_high - self.saw_low < 1e-3 {
            self.saw_low = n.saw_low;
            self.saw_high = n.saw_high;
        }
        bound!(early_voltage, 1.0, 1000.0);
        bound!(reset_fraction, 1e-5, 0.1);
        bound!(reset_time_constants, 0.1, 20.0);
        bound!(square_supply, 1.0, 20.0);
        bound!(square_bias, 0.0, 1.0);
        bound!(square_drive, 0.01, 2.0);
        bound!(square_input_resistance, 1.0, 1e9);
        bound!(square_input_capacitance, 1e-12, 1.0);
        bound!(square_collector_resistance, 1.0, 1e9);
        bound!(square_collector_capacitance, 1e-12, 1.0);
        bound!(square_output_resistance, 1.0, 1e9);
        bound!(square_output_capacitance, 1e-12, 1.0);
        bound!(transistor_is, 1e-20, 1e-6);
        bound!(transistor_alpha, 0.5, 0.99999);
        self
    }
}

/// Diagnostic counters since reset; saturating, deterministic and allocation-free.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OscDiagnostics {
    /// Number of non-finite signal/knob arguments replaced by zero.
    pub non_finite_inputs: u64,
    /// Number of VCO samples whose requested frequency exceeds 0.45 × rate.
    pub frequency_limited_samples: u64,
}

fn sanitise(x: f64, count: &mut u64) -> f64 {
    if x.is_finite() {
        x
    } else {
        *count = count.saturating_add(1);
        0.0
    }
}

fn rate_valid(rate: f64) -> Result<(), StageError> {
    if rate.is_finite() && (44_100.0..=384_000.0).contains(&rate) {
        Ok(())
    } else {
        Err(StageError::InvalidSampleRate)
    }
}

fn clean_state(x: f64) -> f64 {
    if x.abs() < 1e-24 { 0.0 } else { x }
}

/// Ideal calibrated sequencer DAC mapping, volts: one semitone is 1/12 V.
/// VERIFY: physical DAC resistor ladder errors and absolute offset are not captured.
pub fn cv_for_note(note: u8) -> f64 {
    f64::from(note) / 12.0
}

/// Sequencer DAC and switched slide capacitor voltage.
pub struct PitchCv {
    c: Components,
    voltage: f64,
    retention: f64,
}

impl PitchCv {
    /// Construct at 48 kHz with a discharged slide capacitor.
    pub fn new(c: Components) -> Self {
        let c = c.valid();
        Self {
            c,
            voltage: 0.0,
            retention: libm::exp(-1.0 / (48_000.0 * c.slide_resistance * c.slide_capacitance)),
        }
    }
    /// Set rate without altering the capacitor voltage; invalid rates preserve state.
    pub fn prepare(&mut self, sample_rate: f64) -> Result<(), StageError> {
        rate_valid(sample_rate)?;
        self.retention =
            libm::exp(-1.0 / (sample_rate * self.c.slide_resistance * self.c.slide_capacitance));
        Ok(())
    }
    /// Discharge the slide storage capacitor.
    pub fn reset(&mut self) {
        self.voltage = 0.0;
    }
    /// Ideal DAC voltage for a sequencer semitone index (lowest C = zero).
    pub fn cv_for_note(note: u8) -> f64 {
        cv_for_note(note)
    }
    /// Return expo-converter CV. Bypassing slide clamps the capacitor to the DAC.
    /// Slide uses the exact RC solution for a held DAC value over one sample.
    pub fn tick(&mut self, note: u8, slide: bool) -> f64 {
        let target = cv_for_note(note);
        self.voltage = if slide {
            clean_state(target + (self.voltage - target) * self.retention)
        } else {
            target
        };
        self.voltage
    }
}

/// Static PNP Ebers–Moll collector solution, with input/output coupling capacitors.
/// VERIFY: stiff emitter/base drive approximation omits base-current loading,
/// emitter impedance and buffer loading; RC networks are reduced equivalents.
///
/// The collector satisfies Vc/Rc = alpha*Is*(exp(Veb/Vt)-1)
/// minus Is*(exp(Vcb/Vt)-1). Bounded bisection covers cutoff and saturation,
/// including the reverse base-collector junction; no comparator is used.
/// Iteration bound and tolerance for the shaper's collector solve.
const MAX_SHAPER_ITERATIONS: usize = 40;
const SHAPER_TOLERANCE_VOLTS: f64 = 1.0e-9;

pub struct SquareShaper {
    c: Components,
    input_cap: f64,
    collector: f64,
    /// Previous static collector solution (Newton warm start).
    solution: f64,
    output_cap: f64,
    input_retention: f64,
    collector_retention: f64,
    output_retention: f64,
    diagnostics: OscDiagnostics,
}

impl SquareShaper {
    /// Construct at 48 kHz with zero capacitor voltages.
    pub fn new(c: Components) -> Self {
        let c = c.valid();
        let mut s = Self {
            c,
            input_cap: 0.0,
            collector: 0.0,
            solution: 0.0,
            output_cap: 0.0,
            input_retention: 0.0,
            collector_retention: 0.0,
            output_retention: 0.0,
            diagnostics: OscDiagnostics::default(),
        };
        s.set_rate(48_000.0);
        s
    }
    fn set_rate(&mut self, rate: f64) {
        self.input_retention = libm::exp(
            -1.0 / (rate * self.c.square_input_resistance * self.c.square_input_capacitance),
        );
        self.collector_retention = libm::exp(
            -1.0 / (rate
                * self.c.square_collector_resistance
                * self.c.square_collector_capacitance),
        );
        self.output_retention = libm::exp(
            -1.0 / (rate * self.c.square_output_resistance * self.c.square_output_capacitance),
        );
    }
    /// Prepare RC coefficients while preserving capacitor voltages.
    pub fn prepare(&mut self, sample_rate: f64) -> Result<(), StageError> {
        rate_valid(sample_rate)?;
        self.set_rate(sample_rate);
        Ok(())
    }
    /// Discharge all capacitor states and clear diagnostics.
    pub fn reset(&mut self) {
        self.input_cap = 0.0;
        self.collector = 0.0;
        self.solution = 0.0;
        self.output_cap = 0.0;
        self.diagnostics = OscDiagnostics::default();
    }
    /// Diagnostics since the last reset.
    pub fn diagnostics(&self) -> OscDiagnostics {
        self.diagnostics
    }

    fn static_collector(&self, base: f64, guess: f64) -> f64 {
        let vt = BOLTZMANN_OVER_Q * self.c.temperature_kelvin;
        let forward = self.c.transistor_alpha
            * self.c.transistor_is
            * libm::expm1(((self.c.square_supply - base) / vt).clamp(-80.0, 40.0));
        // KCL residual f(vc) is strictly increasing in vc, so Newton steps are
        // safeguarded by a shrinking [low, high] bracket (bisection fallback).
        // Warm-started from the previous solution this typically takes 2-3 steps.
        let mut low = 0.0;
        let mut high = self.c.square_supply;
        let mut vc = guess.clamp(low, high);
        for _ in 0..MAX_SHAPER_ITERATIONS {
            let x = ((vc - base) / vt).clamp(-80.0, 40.0);
            let e = libm::exp(x);
            let f = vc / self.c.square_collector_resistance - forward
                + self.c.transistor_is * (e - 1.0);
            if f > 0.0 {
                high = vc;
            } else {
                low = vc;
            }
            let slope = 1.0 / self.c.square_collector_resistance + self.c.transistor_is * e / vt;
            let mut next = vc - f / slope;
            if !(next > low && next < high) {
                next = 0.5 * (low + high);
            }
            let converged =
                (next - vc).abs() < SHAPER_TOLERANCE_VOLTS || high - low < SHAPER_TOLERANCE_VOLTS;
            vc = next;
            if converged {
                break;
            }
        }
        vc
    }
    /// Process the saw-node voltage into the AC-coupled WAVEFORM square terminal.
    pub fn tick(&mut self, saw: f64) -> f64 {
        let saw = sanitise(saw, &mut self.diagnostics.non_finite_inputs).clamp(-100.0, 100.0);
        // Capacitor voltage at the beginning of the sample drives the base.
        let ac = saw - self.input_cap;
        self.input_cap = clean_state(saw + (self.input_cap - saw) * self.input_retention);
        let base = self.c.square_supply - self.c.square_bias + self.c.square_drive * ac;
        let target = self.static_collector(base, self.solution);
        self.solution = target;
        self.collector = clean_state(target + (self.collector - target) * self.collector_retention);
        let output = self.collector - self.output_cap;
        self.output_cap = clean_state(
            self.collector + (self.output_cap - self.collector) * self.output_retention,
        );
        output
    }
}

/// Node voltages at the WAVEFORM switch, in volts.
#[derive(Clone, Copy, Debug, Default)]
pub struct VcoOut {
    /// Band-limited saw/source-follower voltage, including DC bias.
    pub saw: f64,
    /// AC-coupled PNP collector voltage.
    pub square: f64,
}

/// Exponential matched-pair converter and finite-reset capacitor oscillator.
/// VERIFY: reset conductance proportional to converter current is a tracking
/// approximation, not a verified reduction of the transistor reset topology.
/// VERIFY: source-follower gain/offset are folded into the saw endpoint voltages.
/// VERIFY: Early-effect charging curvature needs schematic/SPICE confirmation.
///
/// Harmonic projection is equivalent to convolving the analytic capacitor
/// waveform with a band-limited reconstruction kernel. It removes reset/slope
/// aliases without a sampled discontinuity. Work is bounded by 256 harmonics.
pub struct Vco {
    c: Components,
    phase: f64,
    sample_rate: f64,
    dc: f64,
    reference_frequency: f64,
    band_limit: Option<f64>,
    cosine: [f64; HARMONICS],
    sine: [f64; HARMONICS],
    square: SquareShaper,
    diagnostics: OscDiagnostics,
}

// Charge/current time product for the Early-effect charging interval plus reset.
fn cycle_charge(c: Components) -> f64 {
    c.timing_capacitance
        * c.early_voltage
        * libm::log((c.early_voltage + c.saw_high) / (c.early_voltage + c.saw_low))
        / (1.0 - c.reset_fraction)
}

// Fourier integral of a + b*exp(k*(phase-start)) on one segment.
fn segment(a: f64, b: f64, k: f64, start: f64, length: f64, w: f64) -> (f64, f64) {
    let angle = w * length;
    let co = libm::cos(angle);
    let si = libm::sin(angle);
    let e = libm::exp(k * length);
    let nr = e * co - 1.0;
    let ni = -e * si;
    let denominator = k * k + w * w;
    let re = a * si / w + b * (nr * k - ni * w) / denominator;
    let im = a * (co - 1.0) / w + b * (nr * w + ni * k) / denominator;
    let cs = libm::cos(w * start);
    let ss = libm::sin(w * start);
    (re * cs + im * ss, im * cs - re * ss)
}

impl Vco {
    /// Construct a calibrated oscillator at 48 kHz, with precomputed reset spectrum.
    pub fn new(c: Components) -> Self {
        let c = c.valid();
        let mut s = Self {
            c,
            phase: 0.0,
            sample_rate: 48_000.0,
            dc: 0.0,
            reference_frequency: c.reference_hz * cycle_charge(Components::NOMINAL)
                / cycle_charge(c),
            band_limit: None,
            cosine: [0.0; HARMONICS],
            sine: [0.0; HARMONICS],
            square: SquareShaper::new(c),
            diagnostics: OscDiagnostics::default(),
        };
        let d = 1.0 - c.reset_fraction;
        // dV/dt = I/C * (1 + V/Vearly), i.e. finite transistor output resistance.
        let k = libm::log((c.early_voltage + c.saw_high) / (c.early_voltage + c.saw_low)) / d;
        let a = -c.early_voltage;
        let b = c.early_voltage + c.saw_low;
        let kr = -c.reset_time_constants / c.reset_fraction;
        let br = (c.saw_high - c.saw_low) / -libm::expm1(-c.reset_time_constants);
        let ar = c.saw_high - br;
        s.dc = a * d
            + b * libm::expm1(k * d) / k
            + ar * c.reset_fraction
            + br * libm::expm1(kr * c.reset_fraction) / kr;
        for h in 1..=HARMONICS {
            let w = TAU * h as f64;
            let charge = segment(a, b, k, 0.0, d, w);
            let reset = segment(ar, br, kr, d, c.reset_fraction, w);
            s.cosine[h - 1] = 2.0 * (charge.0 + reset.0);
            s.sine[h - 1] = -2.0 * (charge.1 + reset.1);
        }
        s
    }
    /// Set the stage rate; preserve phase and capacitor states on success or error.
    pub fn prepare(&mut self, sample_rate: f64) -> Result<(), StageError> {
        rate_valid(sample_rate)?;
        self.square.prepare(sample_rate)?;
        self.sample_rate = sample_rate;
        Ok(())
    }
    /// Reset phase, capacitor states and counters.
    pub fn reset(&mut self) {
        self.phase = 0.0;
        self.square.reset();
        self.diagnostics = OscDiagnostics::default();
    }
    /// Counters since reset, including the square-stage input diagnostics.
    pub fn diagnostics(&self) -> OscDiagnostics {
        let mut d = self.diagnostics;
        d.non_finite_inputs = d
            .non_finite_inputs
            .saturating_add(self.square.diagnostics().non_finite_inputs);
        d
    }
    /// Static exponential law, before the sampling-frequency safety limit.
    /// The reference current is calibrated using NOMINAL capacitor/endpoints;
    /// changing timing components therefore changes frequency physically.
    ///
    /// Matched pair: Iout/Iref = exp(delta_Vbe/Vt). The calibrated divider
    /// produces Vt_nominal*ln(2) per input volt, retaining temperature dependence.
    /// CV is limited to ±16 V for bounded numerical behaviour; non-finite CV and
    /// knob values map to zero. This read-only query does not update diagnostics.
    pub fn frequency_hz(&self, cv: f64, tuning: f64) -> f64 {
        let cv = if cv.is_finite() {
            cv.clamp(-16.0, 16.0)
        } else {
            0.0
        };
        let tuning = if tuning.is_finite() {
            tuning.clamp(0.0, 1.0)
        } else {
            0.0
        };
        let nominal_vt = BOLTZMANN_OVER_Q * Components::NOMINAL.temperature_kelvin;
        let vt = BOLTZMANN_OVER_Q * self.c.temperature_kelvin;
        let delta_vbe = nominal_vt * LN_2 * (cv + (tuning - 0.5) * self.c.tuning_octaves);
        self.reference_frequency * libm::exp(delta_vbe / vt)
    }
    /// Charging-current solution of the matched-pair law, amperes.
    /// Finite Early voltage and the calibrated reset fraction set the reference current.
    pub fn charging_current_a(&self, cv: f64, tuning: f64) -> f64 {
        self.frequency_hz(cv, tuning) * cycle_charge(self.c)
    }
    /// Limit synthesised harmonics to `hz` (passband edge; fade over a further
    /// 20%). When the voice runs oversampled and decimates, harmonics above the
    /// host band are discarded anyway, so this saves work without audible
    /// change. `None` restores the full oversampled band (0.40/0.48 x rate).
    pub fn set_band_limit(&mut self, hz: Option<f64>) {
        self.band_limit = hz.filter(|hz| hz.is_finite() && *hz > 0.0);
    }
    fn band_ratios(&self) -> (f64, f64) {
        match self.band_limit {
            Some(hz) => {
                let pass = (hz / self.sample_rate).min(0.40);
                (pass, (pass * 1.2).min(0.48))
            }
            None => (0.40, 0.48),
        }
    }
    /// Advance the capacitor cycle and return saw and PNP square node voltages.
    /// Frequencies above 0.45 × sample rate are limited and counted.
    pub fn tick(&mut self, cv: f64, tuning: f64) -> VcoOut {
        let cv = sanitise(cv, &mut self.diagnostics.non_finite_inputs);
        let tuning = sanitise(tuning, &mut self.diagnostics.non_finite_inputs);
        let requested = self.frequency_hz(cv, tuning);
        let frequency = requested.min(0.45 * self.sample_rate);
        if requested > frequency {
            self.diagnostics.frequency_limited_samples =
                self.diagnostics.frequency_limited_samples.saturating_add(1);
        }
        let angle = TAU * self.phase;
        let co = libm::cos(angle);
        let si = libm::sin(angle);
        let mut hc = co;
        let mut hs = si;
        let mut saw = self.dc;
        // Flat passband to `pass` (default 0.4*rate); smooth fade to zero at
        // `stop` (default 0.48*rate). Also taper the fixed work limit to avoid
        // a hard 256th-harmonic edge.
        let (pass, stop) = self.band_ratios();
        for h in 1..=HARMONICS {
            let ratio = h as f64 * frequency / self.sample_rate;
            if ratio >= stop {
                break;
            }
            let rate_gain = if ratio <= pass {
                1.0
            } else {
                0.5 + 0.5 * libm::cos(PI * (ratio - pass) / (stop - pass))
            };
            let cap_gain = if h <= 224 {
                1.0
            } else {
                0.5 + 0.5 * libm::cos(PI * (h - 224) as f64 / 32.0)
            };
            saw += rate_gain * cap_gain * (self.cosine[h - 1] * hc + self.sine[h - 1] * hs);
            let next = hc * co - hs * si;
            hs = hs * co + hc * si;
            hc = next;
        }
        self.phase += frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        VcoOut {
            saw,
            square: self.square.tick(saw),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ebers_moll_root_satisfies_collector_kcl() {
        let s = SquareShaper::new(Components::NOMINAL);
        let vt = BOLTZMANN_OVER_Q * s.c.temperature_kelvin;
        for veb in [0.1, 0.4, 0.55, 0.6, 0.65, 0.75] {
            let base = s.c.square_supply - veb;
            let vc = s.static_collector(base, 0.0);
            let ic = s.c.transistor_alpha * s.c.transistor_is * libm::expm1(veb / vt)
                - s.c.transistor_is * libm::expm1((vc - base) / vt);
            let error = (vc / s.c.square_collector_resistance - ic).abs();
            assert!(error < 1e-8, "collector KCL residual {error}");
        }
        // Finite endpoints remain bounded even at strongly overdriven junctions.
        for base in [-100.0, 100.0] {
            let vc = s.static_collector(base, 0.0);
            assert!(vc >= 0.0 && vc <= s.c.square_supply);
        }
    }

    #[test]
    fn timing_capacitor_changes_frequency_not_converter_current() {
        let a = Vco::new(Components::NOMINAL);
        let b = Vco::new(Components {
            timing_capacitance: 20e-9,
            ..Components::NOMINAL
        });
        assert!((a.frequency_hz(2.0, 0.5) / b.frequency_hz(2.0, 0.5) - 2.0).abs() < 1e-12);
        assert!((a.charging_current_a(2.0, 0.5) - b.charging_current_a(2.0, 0.5)).abs() < 1e-18);
    }

    #[test]
    fn tiny_endpoint_span_is_sanitised_and_states_flush() {
        let c = Components {
            saw_low: 1.0,
            saw_high: 1.0 + f64::EPSILON,
            ..Components::NOMINAL
        };
        assert!(Vco::new(c).tick(0.0, 0.5).saw.is_finite());
        assert_eq!(clean_state(1e-30), 0.0);
        assert_eq!(clean_state(-1e-30), 0.0);
    }

    #[test]
    fn square_input_capacitor_has_rate_independent_rc_solution() {
        for rate in [44_100.0, 48_000.0, 96_000.0, 192_000.0, 384_000.0] {
            let mut s = SquareShaper::new(Components::NOMINAL);
            s.prepare(rate).unwrap();
            let tau = s.c.square_input_resistance * s.c.square_input_capacitance;
            let n = (rate * tau) as usize;
            for _ in 0..n {
                s.tick(1.0);
            }
            let expected = 1.0 - libm::exp(-(n as f64) / (rate * tau));
            assert!((s.input_cap - expected).abs() < 1e-11);
        }
    }
}
