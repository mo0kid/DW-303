use dw303_vcf::{Components, Quality, StageError, Vcf};
use std::f64::consts::TAU;

#[derive(Clone, Copy)]
struct Complex(f64, f64);
impl Complex {
    fn add(self, b: Self) -> Self {
        Self(self.0 + b.0, self.1 + b.1)
    }
    fn sub(self, b: Self) -> Self {
        Self(self.0 - b.0, self.1 - b.1)
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
    fn abs(self) -> f64 {
        self.0.hypot(self.1)
    }
}
// Independent frequency-domain determinant of the documented loaded netlist.
fn analytic(c: Components, current: f64, frequency: f64, resonance: f64) -> f64 {
    let g = current / (4.0 * c.thermal_voltage);
    let mut previous = Complex(1.0, 0.0);
    let mut determinant = Complex(g, TAU * frequency * c.ladder_capacitance[0]);
    for i in 1..4 {
        let next = Complex(2.0 * g, TAU * frequency * c.ladder_capacitance[i])
            .mul(determinant)
            .sub(Complex(g * g, 0.0).mul(previous));
        previous = determinant;
        determinant = next;
    }
    let h = Complex(g.powi(4), 0.0).div(determinant);
    let mut hp = Complex(1.0, 0.0);
    for i in 0..2 {
        let w = TAU * frequency * c.feedback_resistance[i] * c.feedback_capacitance[i];
        hp = hp.mul(Complex(0.0, w).div(Complex(1.0, w)));
    }
    h.div(Complex(1.0, 0.0).add(h.mul(hp).mul(Complex(resonance * c.resonance_gain, 0.0))))
        .abs()
        * c.output_gain
        * c.input_attenuation
}
fn response(
    c: Components,
    rate: f64,
    current: f64,
    frequency: f64,
    resonance: f64,
    quality: Quality,
) -> f64 {
    let mut v = Vcf::new(c);
    v.prepare(rate).unwrap();
    v.set_quality(quality);
    let g = current / (4.0 * c.thermal_voltage * c.ladder_capacitance[1]);
    let settle = (rate * (10.0 / (0.12 * g)).max(1.5)).ceil() as usize;
    let periods = (frequency * 0.2).ceil().max(2.0);
    let measure = (rate * periods / frequency).ceil() as usize;
    let amplitude = 1e-4;
    let mut sin = 0.0;
    let mut cos = 0.0;
    for n in 0..settle + measure {
        let phase = TAU * frequency * n as f64 / rate;
        let y = v.tick(amplitude * phase.sin(), current, resonance);
        if n >= settle {
            sin += y * phase.sin();
            cos += y * phase.cos();
        }
    }
    assert_eq!(v.diagnostics().non_convergence, 0);
    2.0 * sin.hypot(cos) / measure as f64 / amplitude
}
#[test]
fn small_signal_matches_analytic_loaded_ladder_below_peak() {
    let c = Components::NOMINAL;
    for current in [1e-6, 10e-6, 50e-6] {
        for resonance in [0.0, 0.3, 0.6] {
            for ratio in [0.01, 0.03, 0.06, 0.1, 0.2, 0.4, 0.6] {
                let f = ratio * current / (4.0 * c.thermal_voltage * c.ladder_capacitance[1]) / TAU;
                let measured = response(c, 192_000.0, current, f, resonance, Quality::Full);
                let expected = analytic(c, current, f, resonance);
                let db = 20.0 * (measured / expected).log10();
                assert!(
                    db.abs() < 0.5,
                    "I={current} r={resonance} f={f}: error {db} dB"
                );
            }
        }
    }
}
#[test]
fn unequal_capacitors_and_four_pole_asymptote() {
    let c = Components::NOMINAL;
    let current = 1e-6;
    let g = current / (4.0 * c.thermal_voltage * c.ladder_capacitance[1]);
    let a = analytic(c, current, 12.0 * g / TAU, 0.0);
    let b = analytic(c, current, 24.0 * g / TAU, 0.0);
    let slope = 20.0 * (a / b).log10();
    assert!((slope - 24.08).abs() < 0.6, "{slope}");
    let mut equal = c;
    equal.ladder_capacitance[0] = equal.ladder_capacitance[1];
    let f = g / TAU;
    assert!(
        (20.0 * (analytic(c, current, f, 0.0) / analytic(equal, current, f, 0.0)).log10()).abs()
            > 1.0
    );
    // Measure the asymptote too, at a low bias so frequencies remain far below Nyquist.
    let a = response(c, 192_000.0, current, 12.0 * g / TAU, 0.0, Quality::Full);
    let b = response(c, 192_000.0, current, 24.0 * g / TAU, 0.0, Quality::Full);
    assert!((20.0 * (a / b).log10() - 24.08).abs() < 1.0);
}
#[test]
fn economy_close_at_low_drive_and_rate_independence() {
    let c = Components::NOMINAL;
    for f in [100.0, 500.0, 1500.0] {
        let full = response(c, 192_000.0, 10e-6, f, 0.5, Quality::Full);
        let economy = response(c, 192_000.0, 10e-6, f, 0.5, Quality::Economy);
        assert!((20.0 * (full / economy).log10()).abs() < 0.05);
        for rate in [48_000.0, 96_000.0] {
            let measured = response(c, rate, 10e-6, f, 0.5, Quality::Full);
            assert!((20.0 * (measured / full).log10()).abs() < 0.15);
        }
    }
}
fn oscillation(current: f64, rate: f64) -> (f64, f64) {
    let mut v = Vcf::new(Components::NOMINAL);
    v.prepare(rate).unwrap();
    let mut energy = 0.0;
    let mut crossings = 0;
    let mut last = 0.0;
    let mut first_cross = None;
    let mut final_cross = 0;
    for n in 0..(rate * 0.6) as usize {
        let y = v.tick(if n < 32 { 0.1 } else { 0.0 }, current, 1.0);
        if n > (rate * 0.3) as usize {
            energy += y * y;
            if y > 0.0 && last <= 0.0 {
                crossings += 1;
                first_cross.get_or_insert(n);
                final_cross = n;
            }
        }
        last = y;
    }
    assert_eq!(v.diagnostics().non_convergence, 0);
    let frequency = if crossings > 1 {
        (crossings - 1) as f64 * rate / (final_cross - first_cross.unwrap()) as f64
    } else {
        0.0
    };
    ((energy / (rate * 0.3)).sqrt(), frequency)
}
#[test]
fn self_oscillation_tracks_current_and_rate() {
    let (a, f) = oscillation(10e-6, 192_000.0);
    let (b, f2) = oscillation(20e-6, 192_000.0);
    eprintln!("oscillation at 10/20 uA: {a:.6}/{b:.6} Vrms, {f:.3}/{f2:.3} Hz");
    assert!(a > 0.01 && b > 0.01, "{a} {b}");
    assert!((f2 / f - 2.0).abs() < 0.15, "{f} {f2}");
    for rate in [48_000.0, 96_000.0] {
        let (_, fr) = oscillation(10e-6, rate);
        assert!((fr / f - 1.0).abs() < 0.02, "{fr} {f}");
    }
    let (low, _) = oscillation(1e-9, 192_000.0);
    assert!(low < a * 0.1, "low {low} high {a}");
}
#[test]
fn low_cutoff_feedback_is_attenuated() {
    let c = Components::NOMINAL;
    let peak_ratio = |current| {
        let mut maximum: f64 = 0.0;
        for ratio in [0.1, 0.3, 0.5, 0.8, 1.0, 1.2, 1.5, 2.0] {
            let f = ratio * current / (4.0 * c.thermal_voltage * c.ladder_capacitance[1]) / TAU;
            maximum = maximum.max(analytic(c, current, f, 0.65) / analytic(c, current, f, 0.0));
        }
        maximum
    };
    assert!(peak_ratio(1e-8) < peak_ratio(10e-6) * 0.5);
    let c_low = response(c, 192_000.0, 1e-7, 10.0, 0.65, Quality::Full);
    let c_zero = response(c, 192_000.0, 1e-7, 10.0, 0.0, Quality::Full);
    assert!((20.0 * (c_low / c_zero).log10()).abs() < 1.0);
}
#[test]
fn sweep_grid_bounded_iterations_and_long_run_stability() {
    for quality in [Quality::Full, Quality::Economy] {
        for rate in [44_100.0, 48_000.0, 96_000.0, 192_000.0, 384_000.0] {
            let mut v = Vcf::new(Components::NOMINAL);
            v.prepare(rate).unwrap();
            v.set_quality(quality);
            for current in [0.0, 1e-9, 1e-7, 1e-6, 10e-6, 100e-6, 1e-3] {
                for resonance in [0.0, 0.5, 0.8, 1.0] {
                    for amplitude in [0.0, 0.5, 2.0, 6.0, 12.0] {
                        for n in 0..256 {
                            let y = v.tick(
                                amplitude * (TAU * n as f64 / 37.0).sin(),
                                current,
                                resonance,
                            );
                            assert!(y.is_finite() && y.abs() <= 4.0);
                        }
                    }
                }
            }
            let d = v.diagnostics();
            assert_eq!(d.non_convergence, 0, "{quality:?} {rate} {d:?}");
            eprintln!(
                "{quality:?} {rate} Hz: max {} iterations, mean {:.3}",
                d.max_iterations,
                d.iterations as f64 / d.samples as f64
            );
            assert!(d.max_iterations <= 8);
            assert!(d.iterations <= d.samples * 8);
        }
        let mut v = Vcf::new(Components::NOMINAL);
        v.set_quality(quality);
        for n in 0..1_000_000 {
            let current = if n % 8192 < 4096 { 50e-6 } else { 0.1e-6 };
            let y = v.tick(6.0 * (TAU * n as f64 / 131.0).sin(), current, 1.0);
            assert!(y.is_finite() && y.abs() <= 4.0);
        }
        assert_eq!(v.diagnostics().non_convergence, 0);
    }
}
#[test]
fn lifecycle_nonfinite_and_silent_state() {
    let mut v = Vcf::new(Components::NOMINAL);
    for rate in [0.0, f64::NAN, f64::INFINITY, 44099.0, 384001.0] {
        assert_eq!(v.prepare(rate), Err(StageError::InvalidSampleRate));
    }
    assert_eq!(v.tick(f64::NAN, f64::INFINITY, f64::NEG_INFINITY), 0.0);
    assert_eq!(v.diagnostics().sanitized_inputs, 3);
    v.reset();
    assert_eq!(v.diagnostics().samples, 0);
    for _ in 0..1000 {
        assert_eq!(v.tick(0.0, 1e-6, 1.0), 0.0);
    }
    assert_eq!(v.diagnostics().max_iterations, 1);
    let mut invalid = Components::NOMINAL;
    invalid.thermal_voltage = f64::NAN;
    assert_eq!(Vcf::new(invalid).tick(0.0, 0.0, 0.0), 0.0);
}

#[test]
fn half_bottom_cap_matches_published_core_polynomial() {
    let mut c = Components::NOMINAL;
    c.ladder_capacitance[0] = c.ladder_capacitance[1] * 0.5;
    let current = 10e-6;
    let rate = current / (4.0 * c.thermal_voltage * c.ladder_capacitance[1]);
    for w in [0.01_f64, 0.1, 0.5, 1.0, 2.0, 10.0] {
        let denominator = Complex(
            0.5 * w.powi(4) - 10.0 * w * w + 1.0,
            8.0 * w - 4.0 * w.powi(3),
        );
        let expected = c.output_gain * c.input_attenuation / denominator.abs();
        let actual = analytic(c, current, w * rate / TAU, 0.0);
        assert!((actual / expected - 1.0).abs() < 1e-12);
    }
}
