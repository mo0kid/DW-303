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
    inverse_capacitance: [f64; 4],
    feedback_rate: [f64; 2],
    inverse_scale: f64,
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
            inverse_scale: 1.0 / (2.0 * c.thermal_voltage),
            inverse_capacitance: c.ladder_capacitance.map(|c| 1.0 / c),
            feedback_rate: core::array::from_fn(|i| {
                1.0 / (c.feedback_resistance[i] * c.feedback_capacitance[i])
            }),
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
        // Euler predictor from the previous trapezoidal endpoint derivative.
        // Only the initial guess changes; the residual and tolerance do not.
        let prediction = core::array::from_fn(|i| 2.0 * self.history[i] - self.x[i]);
        let step = core::array::from_fn(|i| prediction[i] - self.x[i]);
        let mut x = if max_abs(step) <= 0.001 {
            prediction
        } else {
            self.x
        };
        let mut converged = false;
        let mut used = 0;
        // Accepted trial evaluations are the next Newton evaluation, and the
        // final flow supplies the trapezoidal history without another call.
        let mut evaluation = self.residual(x, input, current, feedback);
        for iteration in 1..=MAX_ITERATIONS {
            used = iteration;
            let (r, jacobian, _, _) = evaluation;
            let norm = max_abs(r);
            if norm <= TOLERANCE {
                converged = true;
                break;
            }
            let Some(delta) = solve_ladder(jacobian, r) else {
                break;
            };
            // Bounded backtracking guarantees accepted steps reduce residual.
            let mut damping = 1.0;
            for _ in 0..8 {
                let candidate = core::array::from_fn(|i| x[i] - damping * delta[i]);
                let trial = self.residual(candidate, input, current, feedback);
                if max_abs(trial.0) < norm {
                    evaluation = trial;
                    x = candidate;
                    break;
                }
                damping *= 0.5;
            }
        }
        if !converged {
            converged = max_abs(evaluation.0) <= TOLERANCE;
        }
        if converged {
            self.x = x.map(flush);
            let f = if self.x == x {
                evaluation.2
            } else {
                self.flow(self.x, input, current, feedback).0
            };
            self.history = core::array::from_fn(|i| flush(self.x[i] + self.half_step * f[i]));
        } else {
            // Hold the previous physical solution rather than accept an unstable step.
            self.diagnostics.non_convergence = self.diagnostics.non_convergence.saturating_add(1);
        }
        self.diagnostics.samples = self.diagnostics.samples.saturating_add(1);
        self.diagnostics.iterations = self.diagnostics.iterations.saturating_add(u64::from(used));
        self.diagnostics.last_iterations = used;
        self.diagnostics.max_iterations = self.diagnostics.max_iterations.max(used);
        let out = if converged && self.x == x {
            evaluation.3
        } else {
            self.amplifier(self.x[3]).0
        };
        self.c.output_gain * out
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
    ) -> ([f64; N], [[f64; N]; N], [f64; N], f64) {
        let (f, mut j, out) = self.flow(x, input, current, feedback);
        let r = core::array::from_fn(|i| x[i] - self.history[i] - self.half_step * f[i]);
        for (i, row) in j.iter_mut().enumerate() {
            for value in row.iter_mut() {
                *value *= -self.half_step;
            }
            row[i] += 1.0;
        }
        (r, j, f, out)
    }

    /// VERIFY: fixed arm common-mode reduction omits common-mode redistribution,
    /// transistor beta, Early effect and junction capacitance.
    fn flow(
        &self,
        x: [f64; N],
        input: f64,
        current: f64,
        feedback: f64,
    ) -> ([f64; N], [[f64; N]; N], f64) {
        let inverse_scale = self.inverse_scale;
        let half_current = 0.5 * current;
        let current_over_scale = half_current * inverse_scale;
        let (out, out_d) = self.amplifier(x[3]);
        let hp = out - x[4] - x[5];
        let t = libm::tanh((input - feedback * hp) * inverse_scale);
        let injection = half_current * t;
        let injection_d = current_over_scale * (1.0 - t * t);
        let mut edge = [0.0; 4];
        let mut edge_d = [0.0; 4];
        for i in 0..4 {
            let voltage = x[i] - if i == 3 { 0.0 } else { x[i + 1] };
            let (value, derivative) = if self.quality == Quality::Full {
                junction(voltage * inverse_scale)
            } else {
                (voltage * inverse_scale, 1.0)
            };
            edge[i] = half_current * value;
            edge_d[i] = current_over_scale * derivative;
        }
        let mut f = [0.0; N];
        let mut j = [[0.0; N]; N];
        for i in 0..4 {
            let inv_c = self.inverse_capacitance[i];
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
        let drive_d = injection_d * feedback * self.inverse_capacitance[0];
        j[0][3] -= drive_d * out_d;
        j[0][4] += drive_d;
        j[0][5] += drive_d;
        for i in 0..2 {
            let rate = self.feedback_rate[i];
            let row = i + 4;
            f[row] = (out - if i == 0 { 0.0 } else { x[4] } - x[row]) * rate;
            j[row][3] = out_d * rate;
            j[row][row] = -rate;
            if i == 1 {
                j[row][4] = -rate;
            }
        }
        (f, j, out)
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
    // One exponential supplies both sinh and cosh (same junction law).
    let e = libm::exp(a);
    let inverse = 1.0 / e;
    let s = 0.5 * (e - inverse);
    let d = 0.5 * (e + inverse);
    ((s + d * (x.abs() - a)).copysign(x), d)
}
// Eliminate the two RC rows, then the tridiagonal ladder with its single
// feedback corner. This is the same six-equation linear system as `solve`;
// exploit its known zeros rather than eliminate a dense 6x6 matrix.
fn solve_ladder(a: [[f64; N]; N], b: [f64; N]) -> Option<[f64; N]> {
    let inv4 = 1.0 / a[4][4];
    let inv5 = 1.0 / a[5][5];
    let rc4 = b[4] * inv4;
    let gain4 = a[4][3] * inv4;
    let rc5 = (b[5] - a[5][4] * rc4) * inv5;
    let gain5 = (a[5][3] - a[5][4] * gain4) * inv5;
    let corner0 = a[0][3] - a[0][4] * gain4 - a[0][5] * gain5;
    let rhs0 = b[0] - a[0][4] * rc4 - a[0][5] * rc5;
    let inv0 = 1.0 / a[0][0];
    let ratio1 = a[1][0] * inv0;
    let pivot1 = a[1][1] - ratio1 * a[0][1];
    let corner1 = a[1][3] - ratio1 * corner0;
    let rhs1 = b[1] - ratio1 * rhs0;
    let inv1 = 1.0 / pivot1;
    let ratio2 = a[2][1] * inv1;
    let pivot2 = a[2][2] - ratio2 * a[1][2];
    let corner2 = a[2][3] - ratio2 * corner1;
    let rhs2 = b[2] - ratio2 * rhs1;
    let inv2 = 1.0 / pivot2;
    let ratio3 = a[3][2] * inv2;
    let pivot3 = a[3][3] - ratio3 * corner2;
    let rhs3 = b[3] - ratio3 * rhs2;
    // Rare ill-conditioned/custom-component cases retain the pivoted solver.
    if [a[0][0], pivot1, pivot2, pivot3, a[4][4], a[5][5]]
        .iter()
        .any(|p| !p.is_finite() || p.abs() < 1e-20)
    {
        return solve(a, b);
    }
    let x3 = rhs3 / pivot3;
    let x2 = (rhs2 - corner2 * x3) * inv2;
    let x1 = (rhs1 - a[1][2] * x2 - corner1 * x3) * inv1;
    let x0 = (rhs0 - a[0][1] * x1 - corner0 * x3) * inv0;
    let x = [x0, x1, x2, x3, rc4 - gain4 * x3, rc5 - gain5 * x3];
    if x.iter().all(|v| v.is_finite()) {
        Some(x)
    } else {
        solve(a, b)
    }
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
    fn sparse_solve_matches_pivoted_system() {
        for quality in [Quality::Full, Quality::Economy] {
            for rate in [48_000.0, 96_000.0, 192_000.0] {
                let mut v = Vcf::new(Components::NOMINAL);
                v.prepare(rate).unwrap();
                v.set_quality(quality);
                for current in [0.0, 1e-9, 10e-6, 1e-3] {
                    for resonance in [0.0, 12.0, 24.0] {
                        let x = [0.02, -0.008, 0.004, 0.003, 0.001, -0.001];
                        let (r, j, _, _) = v.residual(x, 0.04, current, resonance);
                        let sparse = solve_ladder(j, r).unwrap();
                        let dense = solve(j, r).unwrap();
                        for i in 0..N {
                            assert!(
                                (sparse[i] - dense[i]).abs() <= 1e-11 * dense[i].abs().max(1.0)
                            );
                            let error =
                                j[i].iter().zip(sparse).map(|(a, b)| a * b).sum::<f64>() - r[i];
                            assert!(error.abs() <= 1e-11 * r[i].abs().max(1.0));
                        }
                    }
                }
            }
        }
        // Deliberately zero the first pivot: the original pivoted fallback
        // must recover this solvable structured system.
        let mut a =
            core::array::from_fn(|i| core::array::from_fn(|j| if i == j { 1.0 } else { 0.0 }));
        a[0][0] = 0.0;
        a[0][1] = 1.0;
        a[1][0] = 1.0;
        assert_eq!(solve_ladder(a, [1.0; N]), solve(a, [1.0; N]));
    }

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
