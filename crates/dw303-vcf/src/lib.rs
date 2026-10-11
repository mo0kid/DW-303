#![no_std]
//! Differential TB-303 diode ladder with implicit trapezoidal integration.
//! See `docs/circuit/vcf.md` for the reduced netlist and calibration limits.

/// Solver quality; switching preserves capacitor voltages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quality {
    /// Nonlinear junctions, analytic Newton solve.
    Full,
    /// Linear diode arms with nonlinear input differential pair.
    Economy,
}

/// Preparation failure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StageError {
    /// Rate must be finite and in 44.1 kHz–384 kHz.
    InvalidSampleRate,
}

/// Circuit values, in SI units. Unverified estimates are explicitly identified.
#[derive(Clone, Copy, Debug)]
pub struct Components {
    /// VERIFY: bottom-to-top service-note designators and values; estimated
    /// 18 nF, 33 nF, 33 nF, 33 nF (not the x0xb0x's tenfold larger values).
    pub ladder_capacitance: [f64; 4],
    /// VERIFY: junction temperature assumed 300 K, V_T = kT/q.
    pub thermal_voltage: f64,
    /// VERIFY: effective two coupling capacitors, not a complete schematic reduction.
    pub feedback_capacitance: [f64; 2],
    /// VERIFY: effective coupling resistances; each RC corner is about 10 Hz.
    pub feedback_resistance: [f64; 2],
    /// VERIFY: output differential amplifier small-signal voltage gain.
    pub output_gain: f64,
    /// VERIFY: amplifier input-referred saturation voltage (output limit gain times this).
    pub amplifier_headroom: f64,
    /// VERIFY: maximum input-referred feedback gain and linear pot taper.
    pub resonance_gain: f64,
    /// VERIFY: waveform-node to input-pair differential voltage attenuation.
    pub input_attenuation: f64,
}

impl Components {
    /// Provisional service-note values and explicitly unverified reductions.
    pub const NOMINAL: Self = Self {
        ladder_capacitance: [18e-9, 33e-9, 33e-9, 33e-9],
        thermal_voltage: 0.02585,
        feedback_capacitance: [1e-6, 1e-6],
        feedback_resistance: [15_000.0, 15_000.0],
        output_gain: 20.0,
        amplifier_headroom: 0.2,
        resonance_gain: 24.0,
        input_attenuation: 0.02,
    };
}

/// Cumulative bounded-solver telemetry; reset clears all counters.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VcfDiagnostics {
    /// Samples processed.
    pub samples: u64,
    /// Total Newton iterations (both modes).
    pub iterations: u64,
    /// Most recent sample's iterations, at most eight.
    pub last_iterations: u8,
    /// Highest iteration count since reset.
    pub max_iterations: u8,
    /// Samples failing the residual tolerance; previous voltage is retained.
    pub non_convergence: u64,
    /// Non-finite scalar inputs replaced by zero.
    pub sanitized_inputs: u64,
}

const N: usize = 6;
const MAX_ITERATIONS: u8 = 8;
const TOLERANCE: f64 = 1e-13;

/// Four differential ladder capacitor voltages and two feedback capacitor voltages.
/// Constructed instances run at 192 kHz until prepared.
pub struct Vcf {
    c: Components,
    quality: Quality,
    half_step: f64,
    x: [f64; N],
    history: [f64; N],
    diagnostics: VcfDiagnostics,
}

impl Vcf {
    /// Construct without allocation. Invalid component values use nominal values.
    pub fn new(mut c: Components) -> Self {
        for i in 0..4 {
            c.ladder_capacitance[i] = positive(
                c.ladder_capacitance[i],
                Components::NOMINAL.ladder_capacitance[i],
            );
        }
        for i in 0..2 {
            c.feedback_capacitance[i] = positive(
                c.feedback_capacitance[i],
                Components::NOMINAL.feedback_capacitance[i],
            );
            c.feedback_resistance[i] = positive(
                c.feedback_resistance[i],
                Components::NOMINAL.feedback_resistance[i],
            );
        }
        c.thermal_voltage = positive(c.thermal_voltage, Components::NOMINAL.thermal_voltage);
        c.output_gain = positive(c.output_gain, Components::NOMINAL.output_gain);
        c.amplifier_headroom =
            positive(c.amplifier_headroom, Components::NOMINAL.amplifier_headroom);
        c.resonance_gain = positive(c.resonance_gain, Components::NOMINAL.resonance_gain);
        c.input_attenuation = positive(c.input_attenuation, Components::NOMINAL.input_attenuation);
        Self {
            c,
            quality: Quality::Full,
            half_step: 0.5 / 192_000.0,
            x: [0.0; N],
            history: [0.0; N],
            diagnostics: VcfDiagnostics::default(),
        }
    }

    /// Set integration rate and clear state. Invalid rates leave the instance unchanged.
    pub fn prepare(&mut self, sample_rate: f64) -> Result<(), StageError> {
        if !sample_rate.is_finite() || !(44_100.0..=384_000.0).contains(&sample_rate) {
            return Err(StageError::InvalidSampleRate);
        }
        self.half_step = 0.5 / sample_rate;
        self.reset();
        Ok(())
    }

    /// Clear capacitor voltages and diagnostics, preserving rate and quality.
    pub fn reset(&mut self) {
        self.x = [0.0; N];
        self.history = [0.0; N];
        self.diagnostics = VcfDiagnostics::default();
    }

    /// Switch solver model, preserving capacitor voltages and trapezoidal history.
    pub fn set_quality(&mut self, q: Quality) {
        self.quality = q;
    }

    /// Return cumulative solver counters.
    pub fn diagnostics(&self) -> VcfDiagnostics {
        self.diagnostics
    }

    /// Process waveform-node volts, total ladder tail current in amperes, and
    /// RESONANCE panel rotation. Finite current is limited to 0..1 mA and input
    /// to ±12 V for numerical protection outside the instrument's operating range.
    pub fn tick(&mut self, input: f64, bias_current: f64, resonance: f64) -> f64 {
        let input = self.sanitize(input).clamp(-12.0, 12.0) * self.c.input_attenuation;
        let current = self.sanitize(bias_current).clamp(0.0, 1e-3);
        let feedback = self.sanitize(resonance).clamp(0.0, 1.0) * self.c.resonance_gain;
        let mut x = self.x; // warm start from preceding solution
        let mut converged = false;
        let mut used = 0;
        for iteration in 1..=MAX_ITERATIONS {
            used = iteration;
            let (r, jacobian) = self.residual(x, input, current, feedback);
            let norm = max_abs(r);
            if norm <= TOLERANCE {
                converged = true;
                break;
            }
            let Some(delta) = solve(jacobian, r) else {
                break;
            };
            // Bounded backtracking guarantees accepted steps reduce residual.
            let mut damping = 1.0;
            for _ in 0..8 {
                let candidate = core::array::from_fn(|i| x[i] - damping * delta[i]);
                if max_abs(self.residual(candidate, input, current, feedback).0) < norm {
                    x = candidate;
                    break;
                }
                damping *= 0.5;
            }
        }
        if !converged {
            converged = max_abs(self.residual(x, input, current, feedback).0) <= TOLERANCE;
        }
        if converged {
            self.x = x.map(flush);
            let f = self.flow(self.x, input, current, feedback).0;
            self.history = core::array::from_fn(|i| flush(self.x[i] + self.half_step * f[i]));
        } else {
            // Hold the previous physical solution rather than accept an unstable step.
            self.diagnostics.non_convergence = self.diagnostics.non_convergence.saturating_add(1);
        }
        self.diagnostics.samples = self.diagnostics.samples.saturating_add(1);
        self.diagnostics.iterations = self.diagnostics.iterations.saturating_add(u64::from(used));
        self.diagnostics.last_iterations = used;
        self.diagnostics.max_iterations = self.diagnostics.max_iterations.max(used);
        self.c.output_gain * self.amplifier(self.x[3]).0
    }

    fn sanitize(&mut self, value: f64) -> f64 {
        if value.is_finite() {
            value
        } else {
            self.diagnostics.sanitized_inputs = self.diagnostics.sanitized_inputs.saturating_add(1);
            0.0
        }
    }

    fn amplifier(&self, voltage: f64) -> (f64, f64) {
        let t = libm::tanh(voltage / self.c.amplifier_headroom);
        (self.c.amplifier_headroom * t, 1.0 - t * t)
    }

    fn residual(
        &self,
        x: [f64; N],
        input: f64,
        current: f64,
        feedback: f64,
    ) -> ([f64; N], [[f64; N]; N]) {
        let (f, mut j) = self.flow(x, input, current, feedback);
        let r = core::array::from_fn(|i| x[i] - self.history[i] - self.half_step * f[i]);
        for (i, row) in j.iter_mut().enumerate() {
            for value in row.iter_mut() {
                *value *= -self.half_step;
            }
            row[i] += 1.0;
        }
        (r, j)
    }

    /// VERIFY: fixed arm common-mode reduction omits common-mode redistribution,
    /// transistor beta, Early effect and junction capacitance.
    fn flow(
        &self,
        x: [f64; N],
        input: f64,
        current: f64,
        feedback: f64,
    ) -> ([f64; N], [[f64; N]; N]) {
        let scale = 2.0 * self.c.thermal_voltage;
        let (out, out_d) = self.amplifier(x[3]);
        let hp = out - x[4] - x[5];
        let t = libm::tanh((input - feedback * hp) / scale);
        let injection = 0.5 * current * t;
        let injection_d = 0.5 * current * (1.0 - t * t) / scale;
        let mut edge = [0.0; 4];
        let mut edge_d = [0.0; 4];
        for i in 0..4 {
            let voltage = x[i] - if i == 3 { 0.0 } else { x[i + 1] };
            let (value, derivative) = if self.quality == Quality::Full {
                junction(voltage / scale)
            } else {
                (voltage / scale, 1.0)
            };
            edge[i] = 0.5 * current * value;
            edge_d[i] = 0.5 * current * derivative / scale;
        }
        let mut f = [0.0; N];
        let mut j = [[0.0; N]; N];
        for i in 0..4 {
            let inv_c = 1.0 / self.c.ladder_capacitance[i];
            f[i] = (if i == 0 { injection } else { edge[i - 1] } - edge[i]) * inv_c;
            j[i][i] = -edge_d[i] * inv_c;
            if i > 0 {
                j[i][i - 1] = edge_d[i - 1] * inv_c;
                j[i][i] -= edge_d[i - 1] * inv_c;
            }
            if i < 3 {
                j[i][i + 1] = edge_d[i] * inv_c;
            }
        }
        let drive_d = injection_d * feedback / self.c.ladder_capacitance[0];
        j[0][3] -= drive_d * out_d;
        j[0][4] += drive_d;
        j[0][5] += drive_d;
        for i in 0..2 {
            let rate = 1.0 / (self.c.feedback_resistance[i] * self.c.feedback_capacitance[i]);
            let row = i + 4;
            f[row] = (out - if i == 0 { 0.0 } else { x[4] } - x[row]) * rate;
            j[row][3] = out_d * rate;
            j[row][row] = -rate;
            if i == 1 {
                j[row][4] = -rate;
            }
        }
        (f, j)
    }
}

fn positive(value: f64, nominal: f64) -> f64 {
    if value.is_finite() && (1e-15..=1e9).contains(&value) {
        value
    } else {
        nominal
    }
}
fn flush(value: f64) -> f64 {
    if value.abs() < 1e-30 { 0.0 } else { value }
}
fn max_abs(x: [f64; N]) -> f64 {
    x.iter().fold(0.0, |a, b| {
        if b.is_finite() {
            a.max(b.abs())
        } else {
            f64::INFINITY
        }
    })
}
// Exponential continuation avoids overflow for rejected Newton trial points.
fn junction(x: f64) -> (f64, f64) {
    let a = x.abs().min(20.0);
    let s = libm::sinh(a);
    let d = libm::cosh(a);
    ((s + d * (x.abs() - a)).copysign(x), d)
}
// Fixed-size partial-pivot Gaussian elimination, all bounds compile-time known.
fn solve(mut a: [[f64; N]; N], mut b: [f64; N]) -> Option<[f64; N]> {
    for k in 0..N {
        let mut pivot = k;
        for i in k + 1..N {
            if a[i][k].abs() > a[pivot][k].abs() {
                pivot = i;
            }
        }
        if !a[pivot][k].is_finite() || a[pivot][k].abs() < 1e-20 {
            return None;
        }
        a.swap(k, pivot);
        b.swap(k, pivot);
        for i in k + 1..N {
            let ratio = a[i][k] / a[k][k];
            let pivot_row = a[k];
            for (col, value) in a[i].iter_mut().enumerate().skip(k + 1) {
                *value -= ratio * pivot_row[col];
            }
            b[i] -= ratio * b[k];
        }
    }
    let mut x = [0.0; N];
    for i in (0..N).rev() {
        let mut value = b[i];
        for (col, entry) in x.iter().enumerate().skip(i + 1) {
            value -= a[i][col] * entry;
        }
        x[i] = value / a[i][i];
        if !x[i].is_finite() {
            return None;
        }
    }
    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn analytic_jacobian_matches_differences() {
        for quality in [Quality::Full, Quality::Economy] {
            let mut v = Vcf::new(Components::NOMINAL);
            v.set_quality(quality);
            let x = [0.02, -0.008, 0.004, 0.003, 0.001, -0.001];
            let j = v.flow(x, 0.04, 10e-6, 18.0).1;
            for col in 0..N {
                let mut hi = x;
                let mut lo = x;
                hi[col] += 1e-7;
                lo[col] -= 1e-7;
                let f_hi = v.flow(hi, 0.04, 10e-6, 18.0).0;
                let f_lo = v.flow(lo, 0.04, 10e-6, 18.0).0;
                for row in 0..N {
                    let numeric = (f_hi[row] - f_lo[row]) / 2e-7;
                    assert!((numeric - j[row][col]).abs() < 1e-5 * (1.0 + numeric.abs()));
                }
            }
        }
    }
}
