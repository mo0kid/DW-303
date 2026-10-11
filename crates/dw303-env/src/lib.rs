#![no_std]
//! TB-303 envelope, accent, control and output circuit reductions.
//!
//! Voltages and currents use SI units. See `docs/circuit/env.md` for the
//! unverified component estimates and the limits of these reduced models.

use core::cell::Cell;

/// Lifecycle failure; preparation accepts finite rates from 44.1 to 384 kHz.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageError {
    /// The rate is non-finite or outside the supported circuit rate range.
    InvalidSampleRate,
}

/// Component values and equivalent network parameters in SI units.
/// VERIFY: Reference designators and all estimates must be checked against a
/// legible service schematic; descriptive names deliberately avoid false IDs.
#[derive(Clone, Copy, Debug)]
pub struct Components {
    /// VERIFY: MEG storage capacitance, estimated 1 uF (likely C62).
    pub meg_capacitance: f64,
    /// VERIFY: Minimum equivalent discharge resistance, 200 kohm.
    pub meg_decay_resistance: f64,
    /// VERIFY: DECAY rheostat effective span, 2.3 Mohm, linear taper.
    pub decay_pot_resistance: f64,
    /// VERIFY: MEG charging switch/source resistance, 1 kohm.
    pub meg_attack_resistance: f64,
    /// VERIFY: Differentiated trigger charging pulse, 4 ms.
    pub meg_charge_pulse: f64,
    /// VERIFY: Charged MEG source voltage, 5 V.
    pub meg_source_voltage: f64,
    /// VERIFY: VEG storage capacitance, 1 uF.
    pub veg_capacitance: f64,
    /// VERIFY: VEG charge resistance, 330 ohm.
    pub veg_attack_resistance: f64,
    /// VERIFY: VEG fixed discharge resistance, 3.3 Mohm.
    pub veg_decay_resistance: f64,
    /// VERIFY: Gate-off discharge equivalent resistance, 10 kohm.
    pub veg_release_resistance: f64,
    /// VERIFY: VEG charging pulse, 1.5 ms.
    pub veg_charge_pulse: f64,
    /// VERIFY: VEG charging source, 5 V.
    pub veg_source_voltage: f64,
    /// VERIFY: Accent smoothing storage capacitance, 1 uF.
    pub accent_capacitance: f64,
    /// VERIFY: Accent diode charging path resistance, 100 kohm.
    pub accent_charge_resistance: f64,
    /// VERIFY: Accent discharge/load resistance, 1 Mohm.
    pub accent_discharge_resistance: f64,
    /// VERIFY: Accent diode forward voltage reduction, 0.6 V.
    pub accent_diode_voltage: f64,
    /// VERIFY: Second RESONANCE gang scales sweep from 0.25 to 1, linear.
    pub resonance_sweep_floor: f64,
    /// VERIFY: Expo reference current at centre cutoff and zero modulation, 1 uA.
    pub filter_reference_current: f64,
    /// VERIFY: Full CUTOFF rotation spans eight octaves.
    pub cutoff_octaves: f64,
    /// VERIFY: Maximum ENV MOD converter sensitivity, 1 octave/V.
    pub envelope_octaves_per_volt: f64,
    /// VERIFY: ENV MOD moves both endpoints around this 2.5 V reference.
    pub envelope_reference_voltage: f64,
    /// VERIFY: Accent sweep sensitivity, 1 octave/V.
    pub accent_octaves_per_volt: f64,
    /// VERIFY: Effective VCA control transistor junction threshold, 0.6 V.
    pub vca_threshold_voltage: f64,
    /// VERIFY: Effective junction thermal voltage, 26 mV (room temperature).
    pub thermal_voltage: f64,
    /// VERIFY: VEG voltage giving unity small-signal VCA gain, 5 V.
    pub vca_unity_voltage: f64,
    /// VERIFY: Accent control injection at full rotation, 1.5 V.
    pub vca_accent_voltage: f64,
    /// VERIFY: Effective BA662 input differential-pair scale, 1 V.
    pub vca_signal_scale: f64,
    /// VERIFY: VCA off-state signal leakage gain, 1e-5.
    pub vca_leakage: f64,
    /// VERIFY: Control-to-output voltage feedthrough ratio, 0.001.
    pub vca_feedthrough: f64,
    /// VERIFY: Output coupling capacitor, 1 uF.
    pub output_capacitance: f64,
    /// VERIFY: Effective coupling load resistance, 50 kohm.
    pub output_load_resistance: f64,
    /// VERIFY: Output buffer small-signal gain, unity.
    pub output_buffer_gain: f64,
}

impl Components {
    /// Nominal estimates; uncertainties are enumerated in the stage document.
    pub const NOMINAL: Self = Self {
        meg_capacitance: 1e-6,
        meg_decay_resistance: 200e3,
        decay_pot_resistance: 2.3e6,
        meg_attack_resistance: 1e3,
        meg_charge_pulse: 0.004,
        meg_source_voltage: 5.0,
        veg_capacitance: 1e-6,
        veg_attack_resistance: 330.0,
        veg_decay_resistance: 3.3e6,
        veg_release_resistance: 10e3,
        veg_charge_pulse: 0.0015,
        veg_source_voltage: 5.0,
        accent_capacitance: 1e-6,
        accent_charge_resistance: 100e3,
        accent_discharge_resistance: 1e6,
        accent_diode_voltage: 0.6,
        resonance_sweep_floor: 0.25,
        filter_reference_current: 1e-6,
        cutoff_octaves: 8.0,
        envelope_octaves_per_volt: 1.0,
        envelope_reference_voltage: 2.5,
        accent_octaves_per_volt: 1.0,
        vca_threshold_voltage: 0.6,
        thermal_voltage: 0.026,
        vca_unity_voltage: 5.0,
        vca_accent_voltage: 1.5,
        vca_signal_scale: 1.0,
        vca_leakage: 1e-5,
        vca_feedthrough: 0.001,
        output_capacitance: 1e-6,
        output_load_resistance: 50e3,
        output_buffer_gain: 1.0,
    };
}

/// Sanitisation telemetry. Reset clears the counter; it saturates at u64::MAX.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Diagnostics {
    /// Count of non-finite floating-point input arguments.
    pub non_finite_inputs: u64,
}

#[derive(Clone)]
struct Circuit {
    c: Components,
    dt: f64,
    bad: Cell<u64>,
    rc_coefficients: [Coefficient; 2],
    audio_taper: Cell<Option<(u64, f64)>>,
    vca_unity_drive: f64,
    meg_attack_tau: f64,
    veg_attack_tau: f64,
    veg_decay_tau: f64,
    veg_release_tau: f64,
    accent_leak_tau: f64,
    accent_charge_tau: f64,
    accent_combined_tau: f64,
    output_tau: f64,
    output_k: f64,
}
impl Circuit {
    fn new(c: Components) -> Self {
        Self {
            c,
            dt: 1.0 / 48_000.0,
            bad: Cell::new(0),
            rc_coefficients: [Coefficient::default(), Coefficient::default()],
            audio_taper: Cell::new(None),
            vca_unity_drive: unity_drive(c),
            meg_attack_tau: tau(c.meg_attack_resistance, c.meg_capacitance),
            veg_attack_tau: tau(c.veg_attack_resistance, c.veg_capacitance),
            veg_decay_tau: tau(c.veg_decay_resistance, c.veg_capacitance),
            veg_release_tau: tau(c.veg_release_resistance, c.veg_capacitance),
            accent_leak_tau: tau(c.accent_discharge_resistance, c.accent_capacitance),
            accent_charge_tau: tau(c.accent_charge_resistance, c.accent_capacitance),
            accent_combined_tau: 1.0
                / (1.0 / tau(c.accent_discharge_resistance, c.accent_capacitance)
                    + 1.0 / tau(c.accent_charge_resistance, c.accent_capacitance)),
            output_tau: tau(c.output_load_resistance, c.output_capacitance),
            output_k: -libm::expm1(
                -(1.0 / 48_000.0) / tau(c.output_load_resistance, c.output_capacitance),
            ),
        }
    }
    fn prepare(&mut self, rate: f64) -> Result<(), StageError> {
        if !rate.is_finite() || !(44_100.0..=384_000.0).contains(&rate) {
            return Err(StageError::InvalidSampleRate);
        }
        self.dt = 1.0 / rate;
        self.output_k = -libm::expm1(-self.dt / self.output_tau);
        Ok(())
    }
    fn rc(&self, slot: usize, v: f64, target: f64, time: f64, t: f64) -> f64 {
        let coefficient = self.rc_coefficients[slot].get(time, t);
        flush(target + (v - target) * coefficient)
    }
    fn audio(&self, x: f64) -> f64 {
        let key = x.to_bits();
        if let Some((old, result)) = self.audio_taper.get()
            && key == old
        {
            return result;
        }
        let result = audio(x);
        self.audio_taper.set(Some((key, result)));
        result
    }
    fn input(&self, x: f64) -> f64 {
        if x.is_finite() {
            x
        } else {
            self.bad.set(self.bad.get().saturating_add(1));
            0.0
        }
    }
    fn knob(&self, x: f64) -> f64 {
        self.input(x).clamp(0.0, 1.0)
    }
    fn diagnostics(&self) -> Diagnostics {
        Diagnostics {
            non_finite_inputs: self.bad.get(),
        }
    }
}
// Cache the exact exponential, including split pulse/crossing durations. Both
// arguments are compared bitwise; rate changes are naturally part of the key.
#[derive(Clone, Default)]
struct Coefficient {
    cached: Cell<Option<(u64, u64, f64)>>,
}
impl Coefficient {
    fn get(&self, time: f64, t: f64) -> f64 {
        let key = (time.to_bits(), t.to_bits());
        if let Some((old_time, old_tau, result)) = self.cached.get()
            && key == (old_time, old_tau)
        {
            return result;
        }
        let result = libm::exp(-time / t);
        self.cached.set(Some((key.0, key.1, result)));
        result
    }
}
fn drive(x: f64, threshold: f64, vt: f64) -> f64 {
    let z = (x - threshold) / vt;
    if z > 40.0 {
        x - threshold
    } else {
        vt * libm::log1p(libm::exp(z))
    }
}
fn unity_drive(c: Components) -> f64 {
    let threshold = value(c.vca_threshold_voltage, 0.6).min(12.0);
    let vt = value(c.thermal_voltage, 0.026).clamp(1e-6, 1.0);
    drive(
        value(c.vca_unity_voltage, 5.0).clamp(threshold + vt, 24.0),
        threshold,
        vt,
    )
    .max(1e-9)
}
// Component bounds protect against invalid user tolerance configurations as well
// as absurd finite signal inputs, without adding a second lifecycle error type.
fn value(x: f64, nominal: f64) -> f64 {
    if x.is_finite() && x >= 0.0 {
        x.min(1e9)
    } else {
        nominal
    }
}
fn tau(r: f64, c: f64) -> f64 {
    (value(r, 1e3) * value(c, 1e-6)).clamp(1e-9, 1e9)
}
fn flush(x: f64) -> f64 {
    if x.abs() < 1e-24 { 0.0 } else { x }
}
#[cfg(test)]
fn rc(v: f64, target: f64, time: f64, t: f64) -> f64 {
    flush(target + (v - target) * libm::exp(-time / t))
}
// Estimated 10%-at-midpoint audio taper, exact zero and unity endpoints.
fn audio(x: f64) -> f64 {
    (libm::exp(libm::log(81.0) * x) - 1.0) / 80.0
}

macro_rules! lifecycle {
    () => {
        /// Construct a reset circuit; usable at 48 kHz before preparation.
        pub fn new(c: Components) -> Self {
            let mut stage = Self::default_state();
            stage.circuit = Circuit::new(c);
            stage
        }
        /// Set the circuit clock, preserving capacitor voltages.
        pub fn prepare(&mut self, sample_rate: f64) -> Result<(), StageError> {
            self.circuit.prepare(sample_rate)
        }
        /// Read the non-finite-input counter without clearing it.
        pub fn diagnostics(&self) -> Diagnostics {
            self.circuit.diagnostics()
        }
    };
}

/// Main envelope capacitor and finite-duration trigger charging switch.
/// VERIFY: Trigger pulse width and charge-path reduction neglect the much
/// weaker discharge conductance during charging; verify against the schematic.
pub struct Meg {
    circuit: Circuit,
    voltage: f64,
    charge_remaining: f64,
}
impl Meg {
    fn default_state() -> Self {
        Self {
            circuit: Circuit::new(Components::NOMINAL),
            voltage: 0.0,
            charge_remaining: 0.0,
        }
    }
    lifecycle!();
    /// Discharge the storage node and clear the trigger and diagnostics.
    pub fn reset(&mut self) {
        self.voltage = 0.0;
        self.charge_remaining = 0.0;
        self.circuit.bad.set(0);
    }
    /// Trigger charges the capacitor; accent shorts the DECAY rheostat path.
    /// Returns the MEG storage-node voltage in volts.
    pub fn tick(&mut self, trigger: bool, accent: bool, decay: f64) -> f64 {
        let d = self.circuit.knob(decay);
        let c = self.circuit.c;
        if trigger {
            self.charge_remaining = value(c.meg_charge_pulse, 0.004);
        }
        let charge = self.charge_remaining.min(self.circuit.dt);
        self.charge_remaining -= charge;
        self.voltage = self.circuit.rc(
            0,
            self.voltage,
            value(c.meg_source_voltage, 5.0).min(12.0),
            charge,
            self.circuit.meg_attack_tau,
        );
        let resistance = value(c.meg_decay_resistance, 200e3)
            + if accent {
                0.0
            } else {
                d * value(c.decay_pot_resistance, 2.3e6)
            };
        self.voltage = self.circuit.rc(
            1,
            self.voltage,
            0.0,
            self.circuit.dt - charge,
            tau(resistance, c.meg_capacitance),
        );
        self.voltage
    }
}

/// VCA envelope storage node, including the gate-off discharge switch.
/// VERIFY: Finite trigger pulse and gate-off switch equivalents; simultaneous
/// discharge during charging is neglected in this reduced topology.
pub struct Veg {
    circuit: Circuit,
    voltage: f64,
    charge_remaining: f64,
}
impl Veg {
    fn default_state() -> Self {
        Self {
            circuit: Circuit::new(Components::NOMINAL),
            voltage: 0.0,
            charge_remaining: 0.0,
        }
    }
    lifecycle!();
    /// Discharge the storage node and clear diagnostics.
    pub fn reset(&mut self) {
        self.voltage = 0.0;
        self.charge_remaining = 0.0;
        self.circuit.bad.set(0);
    }
    /// Return VEG voltage; gate low overrides trigger and truncates the decay.
    pub fn tick(&mut self, gate: bool, trigger: bool) -> f64 {
        let c = self.circuit.c;
        if !gate {
            self.charge_remaining = 0.0;
        } else if trigger {
            self.charge_remaining = value(c.veg_charge_pulse, 0.0015);
        }
        let charge = self.charge_remaining.min(self.circuit.dt);
        self.charge_remaining -= charge;
        self.voltage = self.circuit.rc(
            0,
            self.voltage,
            value(c.veg_source_voltage, 5.0).min(12.0),
            charge,
            self.circuit.veg_attack_tau,
        );
        let t = if gate {
            self.circuit.veg_decay_tau
        } else {
            self.circuit.veg_release_tau
        };
        self.voltage = self
            .circuit
            .rc(1, self.voltage, 0.0, self.circuit.dt - charge, t);
        self.voltage
    }
}

/// Diode-isolated accent capacitor with a persistent discharge/load path.
pub struct AccentSweep {
    circuit: Circuit,
    voltage: f64,
}
impl AccentSweep {
    fn default_state() -> Self {
        Self {
            circuit: Circuit::new(Components::NOMINAL),
            voltage: 0.0,
        }
    }
    lifecycle!();
    /// Discharge the accent capacitor and clear diagnostics.
    pub fn reset(&mut self) {
        self.voltage = 0.0;
        self.circuit.bad.set(0);
    }
    /// Return the second RESONANCE gang's scaled capacitor voltage (V).
    /// VERIFY: Ideal fixed-drop diode reduction and unloaded linear gang divider.
    pub fn tick(&mut self, accent_gate: bool, meg: f64, accent: f64, resonance: f64) -> f64 {
        let m = self.circuit.input(meg).clamp(0.0, 12.0);
        let a = self.circuit.knob(accent);
        let r = self.circuit.knob(resonance);
        let c = self.circuit.c;
        let source = (m * a - value(c.accent_diode_voltage, 0.6)).max(0.0);
        let leak_tau = self.circuit.accent_leak_tau;
        let charge_tau = self.circuit.accent_charge_tau;
        // Charge and leakage conductances act simultaneously while the diode
        // conducts. Split exactly at diode turn-off within the sample.
        if accent_gate && source > self.voltage {
            let t = self.circuit.accent_combined_tau;
            let target = source * t / charge_tau;
            self.voltage = self.circuit.rc(0, self.voltage, target, self.circuit.dt, t);
        } else if accent_gate && source > 0.0 {
            let crossing = leak_tau * libm::log((self.voltage / source).max(1.0));
            let off = crossing.min(self.circuit.dt);
            self.voltage = self.circuit.rc(1, self.voltage, 0.0, off, leak_tau);
            let t = self.circuit.accent_combined_tau;
            self.voltage = self.circuit.rc(
                0,
                self.voltage,
                source * t / charge_tau,
                self.circuit.dt - off,
                t,
            );
        } else {
            self.voltage = self
                .circuit
                .rc(1, self.voltage, 0.0, self.circuit.dt, leak_tau);
        }
        let floor = value(c.resonance_sweep_floor, 0.25).min(1.0);
        self.voltage * (floor + (1.0 - floor) * r)
    }
}

/// Summing-network reduction and exponential converter producing amperes.
pub struct FilterControl {
    circuit: Circuit,
}
impl FilterControl {
    fn default_state() -> Self {
        Self {
            circuit: Circuit::new(Components::NOMINAL),
        }
    }
    lifecycle!();
    /// Clear converter diagnostics; this stage has no capacitor state.
    pub fn reset(&mut self) {
        self.circuit.bad.set(0);
    }
    /// Ladder bias: `Iref * 2^((cutoff-.5)*span + env*(MEG-Vref) + sweep*k)`.
    /// ENV MOD uses an estimated audio taper; result is bounded to 1 pA..1 mA.
    /// VERIFY: Summing gains, polarity, reference and limits need SPICE calibration.
    pub fn bias_current(&self, cutoff: f64, env_mod: f64, meg: f64, accent_sweep: f64) -> f64 {
        let cutoff = self.circuit.knob(cutoff);
        let env = self.circuit.audio(self.circuit.knob(env_mod));
        let meg = self.circuit.input(meg).clamp(0.0, 12.0);
        let sweep = self.circuit.input(accent_sweep).clamp(0.0, 12.0);
        let c = self.circuit.c;
        let octaves = (cutoff - 0.5) * value(c.cutoff_octaves, 8.0)
            + env
                * value(c.envelope_octaves_per_volt, 1.0)
                * (meg - value(c.envelope_reference_voltage, 2.5))
            + sweep * value(c.accent_octaves_per_volt, 1.0);
        (value(c.filter_reference_current, 1e-6) * libm::exp2(octaves.clamp(-40.0, 40.0)))
            .clamp(1e-12, 1e-3)
    }
}

/// BA662 transconductance VCA with transistor control injection and bleed.
/// VERIFY: Reduced differential pair and soft diode driver, not a full BA662 model.
pub struct Vca {
    circuit: Circuit,
}
impl Vca {
    fn default_state() -> Self {
        Self {
            circuit: Circuit::new(Components::NOMINAL),
        }
    }
    lifecycle!();
    /// Clear diagnostics; the reduced VCA has no dynamic storage.
    pub fn reset(&mut self) {
        self.circuit.bad.set(0);
    }
    /// Voltage gain for a control-node voltage, before signal saturation/bleed.
    /// The driver is exponential below its knee and linear above it. BA662
    /// transconductance is proportional to injected control current.
    pub fn gain(&self, control_voltage: f64) -> f64 {
        let v = self.circuit.input(control_voltage).clamp(0.0, 12.0);
        let c = self.circuit.c;
        let threshold = value(c.vca_threshold_voltage, 0.6).min(12.0);
        let vt = value(c.thermal_voltage, 0.026).clamp(1e-6, 1.0);
        value(c.vca_leakage, 1e-5).min(1.0) + drive(v, threshold, vt) / self.circuit.vca_unity_drive
    }
    /// Process VCF node voltage using VEG and accent control; return output V.
    pub fn tick(&mut self, input: f64, veg: f64, accent_gate: bool, accent: f64) -> f64 {
        let input = self.circuit.input(input).clamp(-24.0, 24.0);
        let veg = self.circuit.input(veg).clamp(0.0, 12.0);
        let accent = self.circuit.knob(accent);
        let c = self.circuit.c;
        let control = veg
            + if accent_gate {
                accent * value(c.vca_accent_voltage, 1.5).min(12.0)
            } else {
                0.0
            };
        let scale = value(c.vca_signal_scale, 1.0).clamp(1e-6, 24.0);
        scale * libm::tanh(input / scale) * self.gain(control)
            + value(c.vca_feedthrough, 0.001).min(1.0) * control
    }
}

/// Series coupling capacitor, audio-taper VOLUME divider and output buffer.
/// VERIFY: Effective fixed load ignores volume-dependent impedance and extra caps.
pub struct OutputStage {
    circuit: Circuit,
    capacitor_voltage: f64,
}
impl OutputStage {
    fn default_state() -> Self {
        Self {
            circuit: Circuit::new(Components::NOMINAL),
            capacitor_voltage: 0.0,
        }
    }
    lifecycle!();
    /// Remove stored capacitor charge and clear diagnostics.
    pub fn reset(&mut self) {
        self.capacitor_voltage = 0.0;
        self.circuit.bad.set(0);
    }
    /// Return AC-coupled output voltage. Piecewise-constant input is integrated
    /// exactly; output is the interval-average resistor voltage.
    /// VERIFY: VOLUME/ENV MOD audio taper is estimated at 10% at half rotation.
    pub fn tick(&mut self, input: f64, volume: f64) -> f64 {
        let input = self.circuit.input(input).clamp(-24.0, 24.0);
        let volume = self.circuit.audio(self.circuit.knob(volume));
        let c = self.circuit.c;
        let t = self.circuit.output_tau;
        let k = self.circuit.output_k;
        let delta = input - self.capacitor_voltage;
        self.capacitor_voltage = flush(self.capacitor_voltage + delta * k);
        flush(delta * k * t / self.circuit.dt * volume * value(c.output_buffer_gain, 1.0).min(24.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc_integrator_composes_exactly_and_flushes_tails() {
        let one = rc(4.0, 1.0, 0.1, 0.2);
        let two = rc(rc(4.0, 1.0, 0.05, 0.2), 1.0, 0.05, 0.2);
        assert!((one - two).abs() < 1e-14);
        assert_eq!(rc(1.0, 0.0, 100.0, 0.2), 0.0);
    }

    #[test]
    fn component_guards_and_diode_crossing_are_bounded() {
        assert_eq!(value(f64::NAN, 1.0), 1.0);
        assert_eq!(value(-1.0, 1.0), 1.0);
        assert!(tau(0.0, 0.0) > 0.0);
        let mut sweep = AccentSweep::new(Components::NOMINAL);
        sweep.voltage = 1.0;
        let dt = sweep.circuit.dt;
        let source = libm::exp(-dt / 2.0);
        let result = sweep.tick(true, source + 0.6, 1.0, 1.0);
        assert!(result < source && result > 0.0);
    }
}
