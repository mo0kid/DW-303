use dw303_vcf::{Components, Quality, StageError, Vcf, VcfOut};
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
    fn scale(self, k: f64) -> Self {
        Self(self.0 * k, self.1 * k)
    }
    fn abs(self) -> f64 {
        self.0.hypot(self.1)
    }
}
fn hp(w: f64, t: f64) -> Complex {
    Complex(0.0, w * t).div(Complex(1.0, w * t))
}
// Independent AC analysis: eliminate coupling branches, then ladder determinant.
fn analytic(c: Components, current: f64, f: f64, p: f64) -> f64 {
    let w = TAU * f;
    let g = current / (4.0 * c.thermal_voltage);
    let gain = (c.bias_supply / 3.0 - c.vbe) / c.amplifier_resistance[3]
        * c.amplifier_resistance[4]
        / (4.0 * c.thermal_voltage);
    let h25 = hp(w, c.output_capacitance[0] * c.output_bias_resistance[0]);
    let h27 = hp(w, c.output_capacitance[1] * c.output_bias_resistance[1]);
    let amp = h25.add(h27).scale(0.5 * gain);
    let rt = c.resonance_resistance * p * (1.0 - p);
    let rf = rt + c.feedback_resistance[0] + c.feedback_resistance[1];
    let y15 = hp(w, rf * c.coupling_capacitance[1]).scale(1.0 / rf);
    let yload = Complex(1.0 / c.resonance_resistance, 0.0).add(y15.scale(p * p));
    let h14 = Complex(0.0, w * c.coupling_capacitance[0])
        .div(Complex(0.0, w * c.coupling_capacitance[0]).add(yload));
    let feedback = y15
        .scale(
            p * c.feedback_resistance[1] * c.feedback_resistance[3]
                / (c.feedback_resistance[2] + c.feedback_resistance[3]),
        )
        .mul(hp(
            w,
            (c.feedback_resistance[2] + c.feedback_resistance[3]) * c.coupling_capacitance[2],
        ));
    let load = h25
        .scale(0.25 / c.output_bias_resistance[0])
        .add(h27.scale(0.25 / c.output_bias_resistance[1]));
    let mut prev = Complex(1.0, 0.0);
    let mut det = Complex(g, w * c.ladder_capacitance[0]);
    for i in 1..4 {
        let mut diag = Complex(2.0 * g, w * c.ladder_capacitance[i]);
        if i == 3 {
            diag = diag.add(load);
        }
        let next = diag.mul(det).sub(prev.scale(g * g));
        prev = det;
        det = next;
    }
    let core = Complex(g.powi(4), 0.0).div(det);
    let forward = core.mul(amp).mul(h14);
    let input = hp(w, (c.input_resistance[0] + c.input_resistance[1]) * c.c17)
        .scale(c.input_resistance[1] / (c.input_resistance[0] + c.input_resistance[1]));
    forward
        .mul(input)
        .div(Complex(1.0, 0.0).add(forward.mul(feedback)))
        .abs()
}
fn response(rate: f64, current: f64, f: f64, p: f64, q: Quality) -> f64 {
    let mut v = Vcf::new(Components::NOMINAL);
    v.prepare(rate).unwrap();
    v.set_quality(q);
    let settle = (rate * 2.0) as usize;
    let count = (rate * (f * 0.2).ceil().max(2.0) / f).round() as usize;
    let mut sin = 0.0;
    let mut cos = 0.0;
    for n in 0..settle + count {
        let phase = TAU * f * n as f64 / rate;
        let y = v.tick(1e-4 * phase.sin(), current, p).output;
        if n >= settle {
            sin += y * phase.sin();
            cos += y * phase.cos();
        }
    }
    assert_eq!(v.diagnostics().non_convergence, 0);
    2.0 * sin.hypot(cos) / count as f64 / 1e-4
}
#[test]
fn printed_values_gains_and_corners() {
    let c = Components::NOMINAL;
    assert_eq!(c.ladder_capacitance, [18e-9, 33e-9, 33e-9, 33e-9]);
    let fc = |r: f64, c: f64| 1.0 / (TAU * r * c);
    assert!((fc(222_200.0, c.c17) - 0.7163).abs() < 0.001);
    assert!((fc(12_200.0, c.coupling_capacitance[2]) - 13.0455).abs() < 0.001);
    assert!((fc(100_000.0, c.output_capacitance[0]) - 15.9155).abs() < 0.001);
    assert!((Vcf::new(c).output_gain() - 10.85).abs() < 0.1);
}
#[test]
fn response_matches_netlist_and_rates() {
    let c = Components::NOMINAL;
    for current in [1e-6, 10e-6, 50e-6] {
        for p in [0.0, 0.5, 1.0] {
            for f in [10.0, 100.0, 500.0, 1500.0] {
                let expected = analytic(c, current, f, p);
                let measured = response(192_000.0, current, f, p, Quality::Full);
                let db = 20.0 * (measured / expected).log10();
                assert!(db.abs() < 0.15, "I={current} p={p} f={f} error={db}");
            }
        }
    }
    for f in [100.0, 500.0, 1500.0] {
        let full = response(192_000.0, 10e-6, f, 0.5, Quality::Full);
        for (rate, q) in [
            (48_000.0, Quality::Full),
            (96_000.0, Quality::Full),
            (192_000.0, Quality::Economy),
        ] {
            let y = response(rate, 10e-6, f, 0.5, q);
            assert!((20.0 * (y / full).log10()).abs() < 0.15);
        }
    }
}
#[test]
fn coupling_blocks_dc_and_wiper_is_loaded() {
    let mut v = Vcf::new(Components::NOMINAL);
    for _ in 0..1_536_000 {
        v.tick(8.0, 10e-6, 0.5);
    }
    let y = v.tick(8.0, 10e-6, 0.5);
    assert!(
        y.output.abs() < 1e-8 && y.resonance_tap.abs() < 1e-8,
        "{y:?}"
    );
    v.reset();
    let mut loaded = false;
    for n in 0..10000 {
        let y = v.tick(3.0 * (n as f64 * 0.02).sin(), 10e-6, 0.5);
        loaded |= (y.resonance_tap - 0.5 * y.output).abs() > 1e-6;
    }
    assert!(loaded);
    v.reset();
    for n in 0..10000 {
        let y = v.tick(3.0 * (n as f64 * 0.02).sin(), 10e-6, 1.0);
        assert_eq!(y.output, y.resonance_tap);
    }
    assert_eq!(v.diagnostics().non_convergence, 0);
}
#[test]
fn extreme_grid_and_long_run() {
    for q in [Quality::Full, Quality::Economy] {
        for rate in [44_100.0, 48_000.0, 96_000.0, 192_000.0, 384_000.0] {
            let mut v = Vcf::new(Components::NOMINAL);
            v.prepare(rate).unwrap();
            v.set_quality(q);
            for current in [0.0, 1e-9, 1e-7, 1e-6, 10e-6, 100e-6, 1e-3] {
                for p in [0.0, 0.5, 1.0] {
                    for amp in [0.0, 0.5, 3.0, 12.0] {
                        for n in 0..256 {
                            let y = v.tick(amp * (TAU * n as f64 / 37.0).sin(), current, p);
                            assert!(y.output.is_finite() && y.resonance_tap.is_finite());
                            assert!(y.output.abs() < 2.0);
                        }
                    }
                }
            }
            assert_eq!(
                v.diagnostics().non_convergence,
                0,
                "{q:?} {rate} {:?}",
                v.diagnostics()
            );
            assert!(v.diagnostics().max_iterations <= 8);
        }
        let mut v = Vcf::new(Components::NOMINAL);
        v.set_quality(q);
        for n in 0..1_000_000 {
            let y = v.tick(
                6.0 * (TAU * n as f64 / 131.0).sin(),
                if n % 8192 < 4096 { 50e-6 } else { 0.1e-6 },
                1.0,
            );
            assert!(y.output.is_finite() && y.resonance_tap.is_finite());
        }
        assert_eq!(v.diagnostics().non_convergence, 0);
    }
}
#[test]
fn lifecycle_and_sanitization() {
    let mut v = Vcf::new(Components::NOMINAL);
    for rate in [0.0, f64::NAN, f64::INFINITY, 44099.0, 384001.0] {
        assert_eq!(v.prepare(rate), Err(StageError::InvalidSampleRate));
    }
    assert_eq!(
        v.tick(f64::NAN, f64::INFINITY, f64::NEG_INFINITY),
        VcfOut::default()
    );
    assert_eq!(v.diagnostics().sanitized_inputs, 3);
    v.reset();
    for _ in 0..1000 {
        assert_eq!(v.tick(0.0, 10e-6, 1.0), VcfOut::default());
    }
    assert_eq!(v.diagnostics().max_iterations, 1);
}
#[test]
#[ignore]
fn stage_timing() {
    use std::{hint::black_box, time::Instant};
    for q in [Quality::Full, Quality::Economy] {
        let mut v = Vcf::new(Components::NOMINAL);
        v.set_quality(q);
        let t = Instant::now();
        for n in 0..384_000 {
            black_box(v.tick(black_box(libm::sin(n as f64 * 0.02) * 3.0), 20e-6, 0.7));
        }
        eprintln!("{q:?} {:?}, {:?}", t.elapsed(), v.diagnostics());
    }
}

#[test]
fn page7_ring_calibration() {
    let current = Vcf::new(Components::NOMINAL)
        .calibration_current(500.0)
        .expect("calibration pole");
    eprintln!("page7 current {current:e} A");
    for rate in [48_000.0, 96_000.0, 192_000.0] {
        let mut v = Vcf::new(Components::NOMINAL);
        v.prepare(rate).unwrap();
        let mut older = 0.0;
        let mut old = 0.0;
        let mut peaks = Vec::new();
        for n in 0..(rate * 0.02) as usize {
            let y = v.tick(if n == 0 { 0.01 } else { 0.0 }, current, 1.0).output;
            if n > 2 && old > older && old > y {
                peaks.push((n - 1, old));
            }
            older = old;
            old = y;
        }
        eprintln!("rate {rate} peaks {peaks:?}");
        assert!(peaks.len() >= 2);
        let period = (peaks[1].0 - peaks[0].0) as f64 / rate;
        assert!((0.0015..=0.0025).contains(&period), "period={period}");
        assert_eq!(v.diagnostics().non_convergence, 0);
    }
    assert!(
        Vcf::new(Components::NOMINAL)
            .calibration_current(f64::NAN)
            .is_none()
    );
}
