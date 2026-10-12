#![no_std]
//! Differential TB-303 diode ladder with implicit trapezoidal integration.
//! See `docs/circuit/vcf.md` for the reduced netlist and
//! `docs/circuit/vcf-fitting-round5-2026-10-12.md` for the current measurement fit.
//! Q12 differential base loading retains ten physical capacitor states;
//! unmeasured transistor betas and the effective Q21 gain remain VERIFY items.

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

/// Schematic component values in SI units (reconciled service-notes page 5).
#[derive(Clone, Copy, Debug)]
pub struct Components {
    /// C18, C19, C24, C26, bottom to top.
    pub ladder_capacitance: [f64; 4],
    /// C17 input coupling.
    pub c17: f64,
    /// C25 (L1 to Q21R), C27 (R1 to Q21L), crossed output coupling.
    pub output_capacitance: [f64; 2],
    /// C14 output, C15 Q18 input, C23 Q12 feedback coupling.
    pub coupling_capacitance: [f64; 3],
    /// R62 series input, R70 input bias.
    pub input_resistance: [f64; 2],
    /// R114 Q21R bias, R113 Q21L bias.
    pub output_bias_resistance: [f64; 2],
    /// R67 series, R99 Q18 bias, R97 injection, R98 Q12 bias.
    pub feedback_resistance: [f64; 4],
    /// VR4A 50K linear track; terminal 1 ground, terminal 3 C14.
    pub resonance_resistance: f64,
    /// R116, R112, R109 Q20 bias divider, R108 emitter, R110 collector load.
    pub amplifier_resistance: [f64; 5],
    /// Printed 5.333 V reference feeding the bias dividers.
    pub bias_supply: f64,
    /// VERIFY: junction temperature / ideality; VT estimated at 300 K.
    pub thermal_voltage: f64,
    /// VERIFY: Q20 Vbe used to derive Q21 tail current; not printed.
    pub vbe: f64,
    /// VERIFY: Q21 common-emitter current gain at its approximately 0.26 mA
    /// collector bias. Fitted base loading; no internal current measurement.
    pub output_beta: f64,
    /// VERIFY: Q12 current gain; differential base currents load R62/R70 and
    /// R97/R98. Fitted to 300 for the unmeasured low-current 2SC1583 pair;
    /// see the round-5 fit register for evidence and identifiability limits.
    pub input_beta: f64,
    /// VERIFY: effective Q21 collector gain, fitted to TD-3 ring captures.
    /// Preserves the printed resistors; multiplies Q21 collector swing and gm.
    /// The excess gain is provisional pending Q21/feedback reconciliation.
    pub output_gain_correction: f64,
    /// Optional C25/C27 multiplier for reproducing the earlier empirical fit.
    /// Default 1.0 retains the printed capacitors; Q21 base loading sets their corner.
    pub output_coupling_scale: f64,
}
impl Components {
    /// Printed passives/supplies with explicitly marked device and fit estimates.
    pub const NOMINAL: Self = Self {
        ladder_capacitance: [18e-9, 33e-9, 33e-9, 33e-9],
        c17: 1e-6,
        output_capacitance: [0.1e-6; 2],
        coupling_capacitance: [1e-6; 3],
        input_resistance: [220_000.0, 2_200.0],
        output_bias_resistance: [100_000.0; 2],
        feedback_resistance: [2_200.0, 100_000.0, 10_000.0, 2_200.0],
        resonance_resistance: 50_000.0,
        amplifier_resistance: [10_000.0, 10_000.0, 10_000.0, 2_200.0, 2_200.0],
        bias_supply: 5.333,
        thermal_voltage: 0.02585,
        vbe: 0.65,
        output_beta: 302.5,
        input_beta: 300.0,
        output_gain_correction: 9.125,
        output_coupling_scale: 1.0,
    };
}

/// AC node voltages returned to the VCA.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VcfOut {
    /// C14 negative terminal / VR4A terminal 3, volts.
    pub output: f64,
    /// Loaded VR4A wiper / RES_FB, volts.
    pub resonance_tap: f64,
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

const N: usize = 10;
const MAX_ITERATIONS: u8 = 8;
const TOLERANCE: f64 = 1e-13;

/// Four ladder voltages plus C17, C25, C27, C14, C15 and C23 charges / capacitances.
/// Constructed instances run at 192 kHz until prepared.
pub struct Vcf {
    c: Components,
    quality: Quality,
    half_step: f64,
    x: [f64; N],
    history: [f64; N],
    previous: [[f64; N]; 3],
    previous_controls: [f64; 2],
    diagnostics: VcfDiagnostics,
    inverse_capacitance: [f64; 4],
    input_rate: f64,
    input_gain: f64,
    output_rate: [f64; 2],
    output_conductance: [f64; 2],
    output_bias_conductance: [f64; 2],
    inverse_output_capacitance: [f64; 2],
    base_output_ratio: f64,
    feedback_rate: f64,
    feedback_gain: f64,
    amplifier_swing: f64,
    inverse_scale: f64,
    input_alpha: f64,
    input_base_drop: f64,
    input_base_rate: f64,
    feedback_base_rate: f64,
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
        macro_rules! sanitize_array {
            ($field:ident) => {
                for (value, nominal) in c.$field.iter_mut().zip(Components::NOMINAL.$field) {
                    *value = positive(*value, nominal);
                }
            };
        }
        sanitize_array!(output_capacitance);
        sanitize_array!(coupling_capacitance);
        sanitize_array!(input_resistance);
        sanitize_array!(output_bias_resistance);
        sanitize_array!(feedback_resistance);
        sanitize_array!(amplifier_resistance);
        c.c17 = positive(c.c17, Components::NOMINAL.c17);
        c.resonance_resistance = positive(
            c.resonance_resistance,
            Components::NOMINAL.resonance_resistance,
        );
        c.bias_supply = positive(c.bias_supply, Components::NOMINAL.bias_supply);
        c.thermal_voltage = positive(c.thermal_voltage, Components::NOMINAL.thermal_voltage);
        c.vbe = positive(c.vbe, Components::NOMINAL.vbe);
        c.output_beta = positive(c.output_beta, Components::NOMINAL.output_beta).clamp(1.0, 1e9);
        c.input_beta = positive(c.input_beta, Components::NOMINAL.input_beta).clamp(50.0, 1e9);
        c.output_gain_correction = positive(
            c.output_gain_correction,
            Components::NOMINAL.output_gain_correction,
        )
        .clamp(0.1, 100.0);
        c.output_coupling_scale = positive(
            c.output_coupling_scale,
            Components::NOMINAL.output_coupling_scale,
        )
        .clamp(0.001, 100.0);
        for capacitance in &mut c.output_capacitance {
            *capacitance *= c.output_coupling_scale;
        }
        let ar = c.amplifier_resistance;
        let tail = ((c.bias_supply * ar[2] / (ar[0] + ar[1] + ar[2]) - c.vbe) / ar[3]).max(0.0);
        // Differential excitation leaves the matched pair's common emitter
        // stationary. The zero-signal base conductance is 1/r_pi = Ic/(beta VT).
        // Tick uses the complementary large-signal base currents instead.
        let alpha = c.output_beta / (c.output_beta + 1.0);
        let collector_current = 0.5 * tail * alpha;
        let base_conductance = collector_current / (c.output_beta * c.thermal_voltage);
        let output_conductance = c.output_bias_resistance.map(|r| 1.0 / r + base_conductance);
        Self {
            c,
            quality: Quality::Full,
            half_step: 0.5 / 192_000.0,
            x: [0.0; N],
            history: [0.0; N],
            previous: [[0.0; N]; 3],
            previous_controls: [0.0; 2],
            diagnostics: VcfDiagnostics::default(),
            inverse_scale: 1.0 / (2.0 * c.thermal_voltage),
            input_alpha: c.input_beta / (c.input_beta + 1.0),
            input_base_drop: (c.input_resistance[0] * c.input_resistance[1]
                / (c.input_resistance[0] + c.input_resistance[1])
                + c.feedback_resistance[2] * c.feedback_resistance[3]
                    / (c.feedback_resistance[2] + c.feedback_resistance[3]))
                / c.input_beta,
            input_base_rate: c.input_resistance[1]
                / ((c.input_resistance[0] + c.input_resistance[1]) * c.input_beta * c.c17),
            feedback_base_rate: c.feedback_resistance[3]
                / ((c.feedback_resistance[2] + c.feedback_resistance[3])
                    * c.input_beta
                    * c.coupling_capacitance[2]),
            inverse_capacitance: c.ladder_capacitance.map(|c| 1.0 / c),
            input_rate: 1.0 / ((c.input_resistance[0] + c.input_resistance[1]) * c.c17),
            input_gain: c.input_resistance[1] / (c.input_resistance[0] + c.input_resistance[1]),
            output_rate: core::array::from_fn(|i| output_conductance[i] / c.output_capacitance[i]),
            output_conductance,
            output_bias_conductance: c.output_bias_resistance.map(|r| 1.0 / r),
            inverse_output_capacitance: c.output_capacitance.map(|c| 1.0 / c),
            base_output_ratio: 1.0 / (c.output_beta * c.output_gain_correction * ar[4]),
            feedback_rate: 1.0
                / ((c.feedback_resistance[2] + c.feedback_resistance[3])
                    * c.coupling_capacitance[2]),
            feedback_gain: c.feedback_resistance[3]
                / (c.feedback_resistance[2] + c.feedback_resistance[3]),
            amplifier_swing: c.output_gain_correction * collector_current * ar[4],
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
        self.previous = [[0.0; N]; 3];
        self.previous_controls = [0.0; 2];
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
    pub fn tick(&mut self, input: f64, ladder_current: f64, resonance: f64) -> VcfOut {
        let input = self.sanitize(input).clamp(-12.0, 12.0);
        let current = self.sanitize(ladder_current).clamp(0.0, 1e-3);
        let feedback = self.sanitize(resonance).clamp(0.0, 1.0);
        // Cubic extrapolation for smooth controls; endpoint Euler predictor otherwise.
        // Only the initial guess changes; the residual and tolerance do not.
        let prediction = if self.diagnostics.samples >= 4
            && (current - self.previous_controls[0]).abs()
                <= 0.05 * current.max(self.previous_controls[0])
            && (feedback - self.previous_controls[1]).abs() <= 0.02
        {
            core::array::from_fn(|i| {
                4.0 * self.x[i] - 6.0 * self.previous[0][i] + 4.0 * self.previous[1][i]
                    - self.previous[2][i]
            })
        } else {
            core::array::from_fn(|i| 2.0 * self.history[i] - self.x[i])
        };
        let step = core::array::from_fn(|i| prediction[i] - self.x[i]);
        let mut x = if max_abs(step) <= 0.01 {
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
            self.previous[2] = self.previous[1];
            self.previous[1] = self.previous[0];
            self.previous[0] = self.x;
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
        self.previous_controls = [current, feedback];
        self.diagnostics.samples = self.diagnostics.samples.saturating_add(1);
        self.diagnostics.iterations = self.diagnostics.iterations.saturating_add(u64::from(used));
        self.diagnostics.last_iterations = used;
        self.diagnostics.max_iterations = self.diagnostics.max_iterations.max(used);
        let (output, resonance_tap, _) = self.nodes(self.x, feedback);
        VcfOut {
            output,
            resonance_tap,
        }
    }

    /// Q21 effective single-ended small-signal gain: Q20/R108/R110 derivation
    /// including finite-beta collector current, then the fitted `output_gain_correction`.
    pub fn output_gain(&self) -> f64 {
        self.amplifier_swing * self.inverse_scale
    }

    /// Find Q11 collector current for the page-7 damped ring frequency at RES max.
    /// Call on the control thread to calibrate TM3, then pass the returned amperes
    /// to `tick`. Uses continuous small-signal poles, independent of sample rate.
    /// Returns `None` for invalid targets or failure of the bounded pole search.
    /// This is a model calibration, not a claim of measured transistor accuracy.
    pub fn calibration_current(&self, ring_hz: f64) -> Option<f64> {
        if !ring_hz.is_finite() || !(20.0..=20_000.0).contains(&ring_hz) {
            return None;
        }
        let w = core::f64::consts::TAU * ring_hz;
        let scale = 4.0 * self.c.thermal_voltage * self.c.ladder_capacitance[3] * w;
        for seed in [0.3, 0.7, 1.5, 3.0] {
            let mut damping = -0.5 * w;
            let mut current = scale * seed;
            for _ in 0..32 {
                let r = self.characteristic(Complex(damping, w), current);
                if r.norm() < 1e-8 && damping < 0.0 && (0.0..=1e-3).contains(&current) {
                    return Some(current);
                }
                let ds = w * 1e-5;
                let di = current * 1e-5;
                let a = self
                    .characteristic(Complex(damping + ds, w), current)
                    .sub(self.characteristic(Complex(damping - ds, w), current))
                    .scale(0.5 / ds);
                let b = self
                    .characteristic(Complex(damping, w), current + di)
                    .sub(self.characteristic(Complex(damping, w), current - di))
                    .scale(0.5 / di);
                let det = a.0 * b.1 - a.1 * b.0;
                if !det.is_finite() || det.abs() < 1e-20 {
                    break;
                }
                let dd = (r.0 * b.1 - r.1 * b.0) / det;
                let dc = (a.0 * r.1 - a.1 * r.0) / det;
                let mut fraction = 1.0;
                for _ in 0..8 {
                    let next_i = current - fraction * dc;
                    let next_d = damping - fraction * dd;
                    if next_i > 0.0
                        && next_i <= 1e-3
                        && next_d < 0.0
                        && self.characteristic(Complex(next_d, w), next_i).norm() < r.norm()
                    {
                        damping = next_d;
                        current = next_i;
                        break;
                    }
                    fraction *= 0.5;
                }
            }
        }
        None
    }

    // Determinant of the continuous matched-bias linearisation, normalised by g^4.
    fn characteristic(&self, s: Complex, current: f64) -> Complex {
        let c = self.c;
        let g = current * self.input_alpha / (4.0 * c.thermal_voltage);
        let h25 = s.highpass(1.0 / self.output_rate[0]);
        let h27 = s.highpass(1.0 / self.output_rate[1]);
        let load = h25
            .scale(0.25 * self.output_conductance[0])
            .add(h27.scale(0.25 * self.output_conductance[1]));
        let rf = c.feedback_resistance[0] + c.feedback_resistance[1];
        let y15 = s.highpass(rf * c.coupling_capacitance[1]).scale(1.0 / rf);
        let y14 = Complex(1.0 / c.resonance_resistance, 0.0).add(y15);
        let sc14 = s.scale(c.coupling_capacitance[0]);
        let h14 = sc14.div(sc14.add(y14));
        let feedback = y15
            .scale(c.feedback_resistance[1] * self.feedback_gain)
            .mul(s.highpass(1.0 / self.feedback_rate));
        let forward = h25.add(h27).scale(0.5 * self.output_gain()).mul(h14);
        let mut previous = Complex(1.0, 0.0);
        let mut determinant = Complex(1.0, 0.0).add(s.scale(c.ladder_capacitance[0] / g));
        for i in 1..4 {
            let mut diagonal = Complex(2.0, 0.0).add(s.scale(c.ladder_capacitance[i] / g));
            if i == 3 {
                diagonal = diagonal.add(load.scale(1.0 / g));
            }
            let next = diagonal.mul(determinant).sub(previous);
            previous = determinant;
            determinant = next;
        }
        let input_impedance = Complex(1.0, 0.0)
            .sub(s.highpass(1.0 / self.input_rate).scale(self.input_gain))
            .scale(c.input_resistance[1]);
        let feedback_impedance = Complex(1.0, 0.0)
            .sub(
                s.highpass(1.0 / self.feedback_rate)
                    .scale(self.feedback_gain),
            )
            .scale(c.feedback_resistance[3]);
        let degeneration = Complex(1.0, 0.0).div(
            Complex(1.0, 0.0).add(
                input_impedance
                    .add(feedback_impedance)
                    .scale(g / c.input_beta),
            ),
        );
        determinant.add(forward.mul(feedback).mul(degeneration))
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
        let (t, derivative) = junction(voltage * self.inverse_scale);
        (
            self.amplifier_swing * t,
            self.amplifier_swing * self.inverse_scale * derivative,
        )
    }

    // Resistively loaded VR4A and C15 network, with ideal Q19/Q18 followers.
    /// VERIFY: Q18/Q19 are ideal unity followers, with fixed bias, infinite beta
    /// and no Early effect or parasitics; Q20 base loading is omitted.
    /// Q21 base currents share its fixed-tail nonlinear collector law.
    fn nodes(&self, x: [f64; N], p: f64) -> (f64, f64, f64) {
        let out = self.amplifier(x[3] + x[5] - x[6]).0 - x[7];
        let rt = self.c.resonance_resistance * p * (1.0 - p);
        let i =
            (p * out - x[8]) / (rt + self.c.feedback_resistance[0] + self.c.feedback_resistance[1]);
        (out, p * out - rt * i, i)
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

    /// Q12 fixed-tail input; Q13/Q15, Q14/Q17, Q16/Q23 diode-connected
    /// rungs; Q22 common-base top termination at AC-grounded BIAS_Q22.
    /// Arm sums equal the tail current: rung capacitors carry opposite currents.
    /// Eliminating the common-mode voltage therefore gives tanh, not sinh.
    /// VERIFY: neglects base currents, Early effect, common-mode capacitance
    /// and Q21's common-mode loading; differential Q21 loading is retained.
    fn flow(
        &self,
        x: [f64; N],
        input: f64,
        current: f64,
        feedback: f64,
    ) -> ([f64; N], [[f64; N]; N], f64) {
        let inverse_scale = self.inverse_scale;
        let half_current = 0.5 * current * self.input_alpha;
        let current_over_scale = half_current * inverse_scale;
        let (amp, amp_d) = self.amplifier(x[3] + x[5] - x[6]);
        let out = amp - x[7];
        let rt = self.c.resonance_resistance * feedback * (1.0 - feedback);
        let conductance =
            1.0 / (rt + self.c.feedback_resistance[0] + self.c.feedback_resistance[1]);
        let fb_i = (feedback * out - x[8]) * conductance;
        let follower = self.c.feedback_resistance[1] * fb_i;
        let u = self.input_gain * (input - x[4]) - self.feedback_gain * (follower - x[9]);
        let (t, derivative) =
            degenerated_junction(u * inverse_scale, current_over_scale * self.input_base_drop);
        let injection = half_current * t;
        let injection_d = current_over_scale * derivative;
        let mut out_d = [0.0; N];
        out_d[3] = amp_d;
        out_d[5] = amp_d;
        out_d[6] = -amp_d;
        out_d[7] = -1.0;
        let fi_d: [f64; N] = core::array::from_fn(|i| {
            conductance * (feedback * out_d[i] - if i == 8 { 1.0 } else { 0.0 })
        });
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
        let mut injection_derivative = [0.0; N];
        for col in 0..N {
            let du = -self.feedback_gain * self.c.feedback_resistance[1] * fi_d[col]
                - if col == 4 { self.input_gain } else { 0.0 }
                + if col == 9 { self.feedback_gain } else { 0.0 };
            injection_derivative[col] = injection_d * du;
            j[0][col] += injection_derivative[col] * self.inverse_capacitance[0];
        }
        // Q21's fixed tail gives complementary base currents, with the same
        // tanh law as its collectors. Their DC sum is absorbed in the bias;
        // the differential current and its derivative load C25/C27 and C26.
        // VERIFY: constant beta, fixed tail and settled common-mode base bias.
        let br = -0.5 * x[3] - x[5];
        let bl = 0.5 * x[3] - x[6];
        let gr = self.output_bias_conductance[0];
        let gl = self.output_bias_conductance[1];
        let base = amp * self.base_output_ratio;
        let base_d = amp_d * self.base_output_ratio;
        let ir = gr * br - base;
        let il = gl * bl + base;
        let ic = self.inverse_capacitance[3];
        f[3] -= 0.5 * (il - ir) * ic;
        j[3][3] -= (0.25 * (gl + gr) + base_d) * ic;
        j[3][5] -= (0.5 * gr + base_d) * ic;
        j[3][6] += (0.5 * gl + base_d) * ic;
        // Complementary Q12 base currents also charge C17 and C23. Their
        // settled DC sum is absorbed into the bias, as for Q21.
        // VERIFY: matched, constant beta and stiff common-mode bias.
        f[4] = (input - x[4]) * self.input_rate + self.input_base_rate * injection;
        j[4][4] = -self.input_rate;
        let cr = self.inverse_output_capacitance[0];
        let cl = self.inverse_output_capacitance[1];
        f[5] = ir * cr;
        j[5][3] = (-0.5 * gr - base_d) * cr;
        j[5][5] = (-gr - base_d) * cr;
        j[5][6] = base_d * cr;
        f[6] = il * cl;
        j[6][3] = (0.5 * gl + base_d) * cl;
        j[6][5] = base_d * cl;
        j[6][6] = (-gl - base_d) * cl;
        f[7] =
            (out / self.c.resonance_resistance + feedback * fb_i) / self.c.coupling_capacitance[0];
        f[8] = fb_i / self.c.coupling_capacitance[1];
        f[9] = (follower - x[9]) * self.feedback_rate - self.feedback_base_rate * injection;
        for col in 0..N {
            j[4][col] += self.input_base_rate * injection_derivative[col];
            j[7][col] = (out_d[col] / self.c.resonance_resistance + feedback * fi_d[col])
                / self.c.coupling_capacitance[0];
            j[8][col] = fi_d[col] / self.c.coupling_capacitance[1];
            j[9][col] = (self.c.feedback_resistance[1] * fi_d[col]
                - if col == 9 { 1.0 } else { 0.0 })
                * self.feedback_rate
                - self.feedback_base_rate * injection_derivative[col];
        }
        (f, j, out)
    }
}

#[derive(Clone, Copy)]
struct Complex(f64, f64);
impl Complex {
    fn add(self, b: Self) -> Self {
        Self(self.0 + b.0, self.1 + b.1)
    }
    fn sub(self, b: Self) -> Self {
        Self(self.0 - b.0, self.1 - b.1)
    }
    fn scale(self, k: f64) -> Self {
        Self(self.0 * k, self.1 * k)
    }
    fn mul(self, b: Self) -> Self {
        Self(self.0 * b.0 - self.1 * b.1, self.0 * b.1 + self.1 * b.0)
    }
    fn div(self, b: Self) -> Self {
        let d = b.0 * b.0 + b.1 * b.1;
        Self(
            (self.0 * b.0 + self.1 * b.1) / d,
            (self.1 * b.0 - self.0 * b.1) / d,
        )
    }
    fn norm(self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }
    fn highpass(self, tau: f64) -> Self {
        let z = self.scale(tau);
        z.div(Complex(1.0, 0.0).add(z))
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
// For a matched pair of exponential junctions with fixed total current,
// (I_right - I_left) / (I_right + I_left) = tanh(delta_v / (2 VT)).
fn junction(x: f64) -> (f64, f64) {
    let a = x.abs();
    if a < 1e-4 {
        // Avoid cancellation at zero. The omitted x^5 term is below 1.4e-21;
        // the derivative's omitted x^4 term is below 6.7e-17.
        let square = x * x;
        (x * (1.0 - square / 3.0), 1.0 - square)
    } else {
        // One bounded exponential supplies tanh and sech^2 together.
        let e = libm::exp(-2.0 * a);
        let inverse = 1.0 / (1.0 + e);
        (
            ((1.0 - e) * inverse).copysign(x),
            4.0 * e * inverse * inverse,
        )
    }
}
// t = tanh(x - a t), from the differential base-current voltage drop.
// With beta >= 50 and I <= 1 mA, a <= 0.77 for printed source resistors.
// Monotone scalar law, with a bounded Newton solve and analytic derivative.
fn degenerated_junction(x: f64, a: f64) -> (f64, f64) {
    let mut intrinsic = x / (1.0 + a);
    let (mut value, mut derivative) = junction(intrinsic);
    for _ in 0..5 {
        let correction = (intrinsic + a * value - x) / (1.0 + a * derivative);
        if correction.abs() < 2e-16 * (1.0 + intrinsic.abs()) {
            break;
        }
        intrinsic -= correction;
        // Tanh addition evaluates the same junction at the corrected voltage.
        // For |step| < 0.03, the degree-nine Taylor remainder is < 1e-18;
        // avoid another expensive exponential at the captured microamp bias.
        let step = -correction;
        let delta = if step.abs() < 0.03 {
            let square = step * step;
            step * (1.0
                + square
                    * (-1.0 / 3.0
                        + square
                            * (2.0 / 15.0 + square * (-17.0 / 315.0 + square * (62.0 / 2835.0)))))
        } else {
            junction(step).0
        };
        value = ((value + delta) / (1.0 + value * delta)).clamp(-1.0, 1.0);
        derivative = (1.0 - value) * (1.0 + value);
    }
    (value, derivative / (1.0 + a * derivative))
}
// Schur reduction: all coupling states are affine in the top rung correction.
// C14/C15 form a 2x2 block because VR4A is loaded by the feedback branch.
fn solve_ladder(a: [[f64; N]; N], b: [f64; N]) -> Option<[f64; N]> {
    let mut rhs = [0.0; N];
    let mut gain = [0.0; N];
    // Q21's complementary base currents couple C25 and C27.
    let det_base = a[5][5] * a[6][6] - a[5][6] * a[6][5];
    let inverse_base = 1.0 / det_base;
    rhs[5] = (b[5] * a[6][6] - a[5][6] * b[6]) * inverse_base;
    rhs[6] = (a[5][5] * b[6] - a[6][5] * b[5]) * inverse_base;
    gain[5] = (a[5][3] * a[6][6] - a[5][6] * a[6][3]) * inverse_base;
    gain[6] = (a[5][5] * a[6][3] - a[6][5] * a[5][3]) * inverse_base;
    let r7 = b[7] - a[7][5] * rhs[5] - a[7][6] * rhs[6];
    let r8 = b[8] - a[8][5] * rhs[5] - a[8][6] * rhs[6];
    let g7 = a[7][3] - a[7][5] * gain[5] - a[7][6] * gain[6];
    let g8 = a[8][3] - a[8][5] * gain[5] - a[8][6] * gain[6];
    let det = a[7][7] * a[8][8] - a[7][8] * a[8][7];
    let inverse_coupling = 1.0 / det;
    rhs[7] = (r7 * a[8][8] - a[7][8] * r8) * inverse_coupling;
    rhs[8] = (a[7][7] * r8 - r7 * a[8][7]) * inverse_coupling;
    gain[7] = (g7 * a[8][8] - a[7][8] * g8) * inverse_coupling;
    gain[8] = (a[7][7] * g8 - g7 * a[8][7]) * inverse_coupling;
    // Q12 base currents couple C17 and C23. Both rows still depend only on
    // the top rung and already eliminated output/coupling states.
    let mut r4 = b[4];
    let mut r9 = b[9];
    let mut g4 = a[4][3];
    let mut g9 = a[9][3];
    for i in 5..9 {
        r4 -= a[4][i] * rhs[i];
        r9 -= a[9][i] * rhs[i];
        g4 -= a[4][i] * gain[i];
        g9 -= a[9][i] * gain[i];
    }
    let det_input = a[4][4] * a[9][9] - a[4][9] * a[9][4];
    let inverse_input = 1.0 / det_input;
    rhs[4] = (r4 * a[9][9] - a[4][9] * r9) * inverse_input;
    rhs[9] = (a[4][4] * r9 - a[9][4] * r4) * inverse_input;
    gain[4] = (g4 * a[9][9] - a[4][9] * g9) * inverse_input;
    gain[9] = (a[4][4] * g9 - a[9][4] * g4) * inverse_input;
    let mut r0 = b[0];
    let mut corner = a[0][3];
    let mut r3 = b[3];
    let mut d3 = a[3][3];
    for i in 4..N {
        r0 -= a[0][i] * rhs[i];
        corner -= a[0][i] * gain[i];
        r3 -= a[3][i] * rhs[i];
        d3 -= a[3][i] * gain[i];
    }
    let inv0 = 1.0 / a[0][0];
    let t1 = a[1][0] * inv0;
    let d1 = a[1][1] - t1 * a[0][1];
    let c1 = a[1][3] - t1 * corner;
    let r1 = b[1] - t1 * r0;
    let inv1 = 1.0 / d1;
    let t2 = a[2][1] * inv1;
    let d2 = a[2][2] - t2 * a[1][2];
    let c2 = a[2][3] - t2 * c1;
    let r2 = b[2] - t2 * r1;
    let inv2 = 1.0 / d2;
    let t3 = a[3][2] * inv2;
    let d3 = d3 - t3 * c2;
    if [a[0][0], d1, d2, d3, det, det_base, det_input]
        .iter()
        .any(|v| !v.is_finite() || v.abs() < 1e-20)
    {
        return solve(a, b);
    }
    let mut x = [0.0; N];
    x[3] = (r3 - t3 * r2) / d3;
    x[2] = (r2 - c2 * x[3]) * inv2;
    x[1] = (r1 - a[1][2] * x[2] - c1 * x[3]) * inv1;
    x[0] = (r0 - a[0][1] * x[1] - corner * x[3]) * inv0;
    for i in 4..N {
        x[i] = rhs[i] - gain[i] * x[3];
    }
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
    fn junction_current_is_tail_limited() {
        // A fixed-tail rung cannot create arm current when its differential
        // voltage grows: the opposing arm cuts off instead. Check a voltage
        // sweep well outside the small-signal region, including overflow-sized
        // rejected Newton trial points.
        for voltage in [
            -1000.0, -1.0, -0.2, -0.01, -5.17e-6, -1e-12, 0.0, 1e-12, 5.17e-6, 0.01, 0.2, 1.0,
            1000.0,
        ] {
            let (fraction, derivative) = junction(voltage / (2.0 * 0.02585));
            assert!(fraction.is_finite() && fraction.abs() <= 1.0);
            assert!((0.0..=1.0).contains(&derivative));
            let expected = libm::tanh(voltage / (2.0 * 0.02585));
            assert!((fraction - expected).abs() < 2e-15);
            assert!((derivative - (1.0 - expected * expected)).abs() < 2e-15);
            assert_eq!(junction(-voltage / (2.0 * 0.02585)).0, -fraction);
        }
        assert!(junction(0.2 / (2.0 * 0.02585)).0 > 0.999);
        assert!(junction(1.0 / (2.0 * 0.02585)).1 < 1e-12);
    }

    #[test]
    fn degenerated_pair_matches_implicit_law_and_derivative() {
        for a in [0.0, 0.001, 0.006, 0.15, 0.8] {
            for i in -200..=200 {
                let x = f64::from(i) * 0.1;
                let (t, derivative) = degenerated_junction(x, a);
                let exact = junction(x - a * t);
                assert!((t - exact.0).abs() < 2e-14, "a={a} x={x}");
                assert!((derivative - exact.1 / (1.0 + a * exact.1)).abs() < 2e-14);
                let numerical = (degenerated_junction(x + 1e-6, a).0
                    - degenerated_junction(x - 1e-6, a).0)
                    / 2e-6;
                assert!((derivative - numerical).abs() < 2e-9);
            }
        }
    }

    #[test]
    fn sparse_solve_matches_pivoted_system() {
        for quality in [Quality::Full, Quality::Economy] {
            for rate in [48_000.0, 96_000.0, 192_000.0] {
                let mut v = Vcf::new(Components::NOMINAL);
                v.prepare(rate).unwrap();
                v.set_quality(quality);
                for current in [0.0, 1e-9, 10e-6, 1e-3] {
                    for resonance in [0.0, 0.5, 1.0] {
                        let x = [
                            0.02, -0.008, 0.004, 0.003, 0.001, -0.001, 0.002, 0.003, 0.004, 0.005,
                        ];
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
            let x = [
                0.02, -0.008, 0.004, 0.003, 0.001, -0.001, 0.002, 0.003, 0.004, 0.005,
            ];
            let j = v.flow(x, 0.04, 10e-6, 0.75).1;
            for col in 0..N {
                let mut hi = x;
                let mut lo = x;
                hi[col] += 1e-7;
                lo[col] -= 1e-7;
                let f_hi = v.flow(hi, 0.04, 10e-6, 0.75).0;
                let f_lo = v.flow(lo, 0.04, 10e-6, 0.75).0;
                for row in 0..N {
                    let numeric = (f_hi[row] - f_lo[row]) / 2e-7;
                    assert!((numeric - j[row][col]).abs() < 1e-5 * (1.0 + numeric.abs()));
                }
            }
        }
    }
}
