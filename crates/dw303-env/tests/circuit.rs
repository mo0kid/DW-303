//! Physical checks against the reconciled netlist rather than provisional traces.
use dw303_env::*;
use std::f64::consts::TAU;

fn near(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual {actual:e}, expected {expected:e}, tolerance {tolerance:e}"
    );
}
fn rates() -> [f64; 3] {
    [48_000.0, 96_000.0, 192_000.0]
}

#[test]
fn q38_pulse_is_edge_triggered_and_derived_from_c54() {
    let c = Components::NOMINAL;
    let duration = c.r148 * c.c54 * ((c.supply - c.vbe) * c.r149 / (c.vbe * c.r148)).ln();
    near(duration, 0.0014, 0.0001);
    for rate in rates() {
        let mut p = TriggerPulse::new(c);
        p.prepare(rate).unwrap();
        let mut samples = 0;
        for _ in 0..(rate * 0.01) as usize {
            if p.tick(true) > 0.0 {
                samples += 1;
            }
        }
        near(f64::from(samples) / rate, duration, 1.0 / rate);
        assert_eq!(p.tick(true), 0.0);
        p.tick(false);
        near(p.tick(true), 11.8, 1e-12);
        p.reset();
        assert_eq!(p.tick(false), 0.0);
    }
}

#[test]
fn meg_charge_buffer_clamp_and_decay_follow_r152_r138_vr6_c62() {
    let c = Components::NOMINAL;
    let floor = 3.0 * c.diode_drop;
    for rate in rates() {
        for (decay, accent, r) in [
            (0.0, false, c.r138),
            (1.0, false, c.r138 + c.vr6),
            (1.0, true, c.r138),
            (0.5, false, c.r138 + 0.1 * c.vr6),
        ] {
            let mut m = Meg::new(c);
            m.prepare(rate).unwrap();
            near(m.tick(0.0, false, decay), 0.5, 1e-12);
            for _ in 0..(rate * 0.002) as usize {
                m.tick(11.8, accent, decay);
            }
            let peak = m.capacitor_voltage();
            near(peak, 11.2, 0.02);
            let n = (r * c.c62 * rate).round() as usize;
            for _ in 0..n {
                m.tick(0.0, accent, decay);
            }
            near(
                m.capacitor_voltage(),
                floor + (peak - floor) * (-(n as f64) / rate / (r * c.c62)).exp(),
                1e-9,
            );
        }
        let mut m = Meg::new(c);
        m.prepare(rate).unwrap();
        let value = m.tick(11.8, false, 0.0);
        let r = 1.0 / (1.0 / c.r152 + 1.0 / c.r138);
        let target = r * ((11.8 - c.diode_drop) / c.r152 + floor / c.r138);
        near(
            value + 2.0 * c.vbe,
            target + (floor - target) * (-1.0 / rate / (r * c.c62)).exp(),
            1e-12,
        );
    }
    // Printed 100%→10% maximum is 2.5 s; the minimum sketch is approximate.
    near((c.r138 + c.vr6) * c.c62 * 10.0_f64.ln(), 2.5, 0.05);
    near(c.r138 * c.c62 * 10.0_f64.ln(), 0.2, 0.05);
}

#[test]
fn accent_gate_is_linear_and_switches_off() {
    let mut a = Accent::new(Components::NOMINAL);
    near(a.tick(true, 9.9, 0.5), 4.95, 1e-12);
    near(a.tick(true, 9.9, 1.0), 9.9, 1e-12);
    assert_eq!(a.tick(false, 9.9, 1.0), 0.0);
}

#[test]
fn accent_sweep_uses_loaded_track_and_retains_charge() {
    let c = Components::NOMINAL;
    let reference = c.reference / 2.0 + c.vbe;
    let charge_r = c.r46 + c.vr4b;
    let load = c.r72 + c.r71;
    let rth = 1.0 / (1.0 / charge_r + 1.0 / load);
    let target = rth * ((10.0 - c.diode_drop) / charge_r + reference / load);
    for rate in rates() {
        let mut sweep = AccentSweep::new(c);
        sweep.prepare(rate).unwrap();
        near(sweep.tick(0.0, 1.0), reference, 1e-12);
        let n = (rth * c.c13 * rate).round() as usize;
        for _ in 0..n {
            sweep.tick(10.0, 1.0);
        }
        near(
            sweep.capacitor_voltage(),
            target + (reference - target) * (-(n as f64) / rate / (rth * c.c13)).exp(),
            1e-10,
        );
        let old = sweep.capacitor_voltage();
        let n = (load * c.c13 * rate).round() as usize;
        for _ in 0..n {
            sweep.tick(0.0, 1.0);
        }
        near(
            sweep.capacitor_voltage(),
            reference + (old - reference) * (-(n as f64) / rate / (load * c.c13)).exp(),
            1e-10,
        );
        let mut direct = AccentSweep::new(c);
        direct.prepare(rate).unwrap();
        let mut smooth = AccentSweep::new(c);
        smooth.prepare(rate).unwrap();
        assert!(direct.tick(10.0, 0.0) > smooth.tick(10.0, 1.0) + 1.0);
        let mut peaks = [0.0; 8];
        for peak in &mut peaks {
            for _ in 0..(rate * 0.02) as usize {
                *peak = smooth.tick(10.0, 1.0);
            }
            for _ in 0..(rate * 0.02) as usize {
                smooth.tick(0.0, 1.0);
            }
        }
        for pair in peaks.windows(2) {
            assert!(pair[1] > pair[0]);
        }
    }
}

#[test]
fn converter_has_vq9_reference_loaded_pots_and_netlist_sensitivities() {
    let c = Components::NOMINAL;
    let mut f = FilterControl::new(c);
    let r = f.reference_voltage();
    near(r, 3.3165, 1e-12);
    let neutral = f.ladder_current(0.5, 0.0, r, r);
    let vw = r + (c.supply - r) * 0.25;
    let expected = (vw - r - c.vbe) / (c.r73 + c.tm3 + 11_250.0);
    near(neutral, expected, 1e-15);
    let base = f.ladder_current(0.5, 1.0, r, r);
    let env_octave = f.ladder_current(0.5, 1.0, r + 1.79, r);
    near(env_octave / base, 2.0, 0.05);
    let sweep_octave = f.ladder_current(0.5, 1.0, r, r + 0.81);
    near(sweep_octave / base, 2.0, 0.07);
    assert!(f.ladder_current(0.5, 1.0, 0.5, r) < f.ladder_current(0.5, 0.0, 0.5, r));
    assert!(f.ladder_current(1.0, 0.0, r, r) > f.ladder_current(0.0, 0.0, r, r));
    let compliance = (c.reference - c.vbe - r - c.saturation) / c.r69;
    near(f.ladder_current(1.0, 1.0, 12.0, 12.0), compliance, 1e-15);
    let mut zero_trim = c;
    zero_trim.tm3 = 0.0;
    assert!(FilterControl::new(zero_trim).ladder_current(0.5, 0.0, r, r) > neutral);
}

// Minimal complex arithmetic keeps the analytical AC reference independent of
// the library's time-domain solvers and adds no dependencies.
#[derive(Clone, Copy)]
struct Z(f64, f64);
impl Z {
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
    fn norm(self) -> f64 {
        self.0.hypot(self.1)
    }
}
fn hp(w: f64, tau: f64) -> Z {
    Z(0.0, w * tau).div(Z(1.0, w * tau))
}

#[test]
fn page_seven_trim_sets_500_hz_ladder_loop_phase() {
    let c = Components::NOMINAL;
    let mut f = FilterControl::new(c);
    let reference = f.reference_voltage();
    let current = f.ladder_current(0.5, 0.0, 3.0 * c.diode_drop - 2.0 * c.vbe, reference);
    near(current, 7.372454385731738e-6, 1e-15);
    // Cross-ladder small-signal conductance is I/(4VT). The unequal rung
    // capacitors load neighbouring nodes: determinant of the tridiagonal Y.
    let g = current / (4.0 * c.thermal_voltage);
    let w = TAU * 500.0;
    let mut older = Z(1.0, 0.0);
    let mut last = Z(g, w * 18e-9);
    for cap in [33e-9, 33e-9, 33e-9] {
        let next = Z(2.0 * g, w * cap).mul(last).sub(Z(g * g, 0.0).mul(older));
        older = last;
        last = next;
    }
    let mut loop_transfer = Z(g.powi(4), 0.0).div(last);
    // C15/R67/R99, C23/R97/R98, C25/C27 and C14 load reductions.
    for tau in [0.1022, 0.0122, 0.01, 0.0407] {
        loop_transfer = loop_transfer.mul(hp(w, tau));
    }
    assert!(loop_transfer.0 < 0.0);
    near(loop_transfer.1 / loop_transfer.norm(), 0.0, 1e-12);
    assert!((0.0015..=0.0025).contains(&(1.0 / 500.0)));
}

#[test]
fn veg_c42_recharges_in_1_5_seconds_and_gate_removes_current() {
    let c = Components::NOMINAL;
    for rate in rates() {
        let mut v = Veg::new(c);
        v.prepare(rate).unwrap();
        assert_eq!(v.tick(0.0), 0.0);
        for _ in 0..(rate * 0.005) as usize {
            v.tick(11.8);
        }
        assert!(v.capacitor_voltage() < 0.3);
        let peak_current = v.tick(0.0);
        near(peak_current, 47e-6, 2e-6);
        for _ in 0..(rate * 0.05) as usize {
            v.tick(0.0);
        }
        let old = v.capacitor_voltage();
        let n = (rate * 1.5) as usize;
        for _ in 0..n {
            v.tick(0.0);
        }
        near(
            v.capacitor_voltage(),
            c.supply + (old - c.supply) * (-1.0_f64).exp(),
            0.002,
        );
        v.set_gate(false);
        assert_eq!(v.tick(0.0), 0.0);
        v.set_gate(true);
        assert!(v.tick(0.0) > 0.0);
        for _ in 0..(rate * 2.0) as usize {
            v.tick(0.0);
        }
        assert_eq!(v.tick(0.0), 0.0);
        v.reset();
        near(v.capacitor_voltage(), 12.0, 1e-12);
    }
}

fn vca_amplitude(rate: f64, hz: f64, input: VcfSignals, control: f64) -> f64 {
    let mut v = Vca::new(Components::NOMINAL);
    v.prepare(rate).unwrap();
    let mut sin = 0.0;
    let mut cos = 0.0;
    let mut n = 0;
    for i in 0..rate as usize {
        let phase = TAU * hz * i as f64 / rate;
        let wave = phase.sin();
        let y = v.tick(
            VcfSignals {
                output: input.output * wave,
                resonance_tap: input.resonance_tap * wave,
            },
            control,
            0.0,
        );
        if i >= (rate * 0.5) as usize {
            sin += y * wave;
            cos += y * phase.cos();
            n += 1;
        }
    }
    2.0 * sin.hypot(cos) / f64::from(n)
}

#[test]
fn ota_gm_input_corners_and_resonance_tap_follow_netlist() {
    let c = Components::NOMINAL;
    let load = c.r128 * (c.r127 + c.r129) / (c.r128 + c.r127 + c.r129);
    near(load, 52_525.95155709342, 1e-8);
    for rate in rates() {
        for hz in [40.0, 72.0, 159.0, 1000.0] {
            let w = TAU * hz;
            let y1 = Z(1.0, 0.0).div(Z(c.r121, -1.0 / (w * c.c21)));
            let y2 = Z(1.0, 0.0).div(Z(c.r122, -1.0 / (w * c.c22_vca)));
            let denominator = Z(1.0 / c.r124, 0.0).add(y1).add(y2);
            let gm = 10e-6 / (2.0 * c.thermal_voltage);
            let out = hp(w, c.vr8 * c.c38);
            for (input, path) in [
                (
                    VcfSignals {
                        output: 1e-3,
                        resonance_tap: 0.0,
                    },
                    y1,
                ),
                (
                    VcfSignals {
                        output: 0.0,
                        resonance_tap: 1e-3,
                    },
                    y2,
                ),
            ] {
                let expected = path.div(denominator).mul(out).norm() * gm * load * 1e-3;
                let amplitude = vca_amplitude(rate, hz, input, 10e-6);
                near(amplitude, expected, expected * 0.004);
            }
        }
        let input = VcfSignals {
            output: 1e-3,
            resonance_tap: 0.0,
        };
        let a = vca_amplitude(rate, 1000.0, input, 10e-6);
        let b = vca_amplitude(rate, 1000.0, input, 20e-6);
        near(b / a, 2.0, 1e-8);
        assert_eq!(vca_amplitude(rate, 1000.0, input, 0.0), 0.0);
    }
    let low = vca_amplitude(
        192000.0,
        1000.0,
        VcfSignals {
            output: 1e-3,
            resonance_tap: 0.0,
        },
        1e-6,
    );
    let high = vca_amplitude(
        192000.0,
        1000.0,
        VcfSignals {
            output: 10.0,
            resonance_tap: 0.0,
        },
        1e-6,
    );
    assert!(high < low * 10_000.0 * 0.9); // input-differential tanh compression
}

#[test]
fn vca_accent_current_obeys_loaded_c36_network_and_blocks_below_d27() {
    let c = Components::NOMINAL;
    for rate in rates() {
        let mut v = Vca::new(c);
        v.prepare(rate).unwrap();
        let n = (rate * 0.001).round() as usize;
        for _ in 0..n {
            v.tick(VcfSignals::default(), 0.0, 9.9);
        }
        let threshold = c.diode_drop + c.ota_control_voltage;
        let crossing = -c.r119 * c.c36 * (1.0 - threshold / 9.9).ln();
        let tau = (c.r119 * c.r120 / (c.r119 + c.r120)) * c.c36;
        let steady = (9.9 - threshold) / (c.r119 + c.r120);
        near(
            v.control_current(),
            steady * (1.0 - (-(n as f64 / rate - crossing) / tau).exp()),
            1e-12,
        );
        v.reset();
        for _ in 0..(rate * 0.03) as usize {
            v.tick(VcfSignals::default(), 10e-6, 9.9);
        }
        let expected = 10e-6 + (9.9 - c.diode_drop - c.ota_control_voltage) / (c.r119 + c.r120);
        near(v.control_current(), expected, 1e-12);
        for _ in 0..(rate * 0.03) as usize {
            v.tick(VcfSignals::default(), 0.0, 0.5);
        }
        assert_eq!(v.control_current(), 0.0);
    }
}

fn output_amplitude(rate: f64, volume: f64, vca: f64, mix: f64) -> f64 {
    let mut o = OutputStage::new(Components::NOMINAL);
    o.prepare(rate).unwrap();
    let mut sin = 0.0;
    let mut cos = 0.0;
    for i in 0..rate as usize {
        let p = TAU * 1000.0 * i as f64 / rate;
        let y = o.tick(vca * p.sin(), volume, mix * p.sin());
        if i >= rate as usize / 2 {
            sin += y * p.sin();
            cos += y * p.cos();
        }
    }
    4.0 * sin.hypot(cos) / rate
}

#[test]
fn output_has_inverting_mixer_gains_volume_loading_and_independent_mix_input() {
    let c = Components::NOMINAL;
    for rate in rates() {
        near(
            output_amplitude(rate, 1.0, 0.1, 0.0),
            0.1 * c.r153 / c.r155 * c.r165 / (c.r165 + c.r161),
            3e-5,
        );
        near(
            output_amplitude(rate, 0.0, 0.0, 0.1),
            0.1 * c.r153 / c.r156 * c.r165 / (c.r165 + c.r161),
            2e-5,
        );
        assert_eq!(output_amplitude(rate, 0.0, 0.1, 0.0), 0.0);
        let expected =
            0.1 * 0.1 * c.r153 / (c.r155 + c.vr8 * 0.1 * 0.9) * c.r165 / (c.r165 + c.r161);
        near(output_amplitude(rate, 0.5, 0.1, 0.0), expected, 1e-5);
        let mut o = OutputStage::new(c);
        o.prepare(rate).unwrap();
        assert!(o.tick(0.1, 1.0, 0.0) < 0.0);
        let mut y = 0.0;
        for _ in 0..(rate * 6.0) as usize {
            y = o.tick(0.1, 1.0, 0.2);
        }
        assert!(y.abs() < 1e-12);
    }
    near(1.0 / (TAU * (c.r161 + c.r165) * c.c56), 1.5758, 0.0001);
    near(1.0 / (TAU * c.r155 * c.c59), 3.3863, 0.0001);
    near(1.0 / (TAU * c.r156 * c.c58), 1.5915, 0.0001);
}

#[test]
fn lifecycle_sanitisation_and_extreme_settings_stay_finite() {
    for rate in [44_100.0, 48_000.0, 96_000.0, 192_000.0, 384_000.0] {
        let c = Components::NOMINAL;
        let mut p = TriggerPulse::new(c);
        let mut m = Meg::new(c);
        let mut a = Accent::new(c);
        let mut s = AccentSweep::new(c);
        let mut f = FilterControl::new(c);
        let mut v = Veg::new(c);
        let mut ota = Vca::new(c);
        let mut o = OutputStage::new(c);
        macro_rules! lifecycle {($($stage:ident),*)=>{$(
            assert!($stage.prepare(f64::NAN).is_err());$stage.prepare(rate).unwrap();
        )*};}
        lifecycle!(p, m, a, s, f, v, ota, o);
        for i in 0..200_000 {
            let knob = if i % 97 == 0 {
                f64::NAN
            } else {
                f64::from(i % 2)
            };
            let pulse = p.tick(i % 997 < 500);
            let meg = m.tick(pulse, i % 3 == 0, knob);
            let accent = a.tick(i % 3 == 0, meg, knob);
            let sweep = s.tick(accent, knob);
            let current = f.ladder_current(knob, knob, meg, sweep);
            v.set_gate(i % 997 < 500);
            let veg = v.tick(pulse);
            let audio = if i % 127 == 0 { f64::INFINITY } else { 1e300 };
            let y = ota.tick(
                VcfSignals {
                    output: audio,
                    resonance_tap: -audio,
                },
                veg,
                accent,
            );
            let output = o.tick(y, knob, -audio);
            for x in [pulse, meg, accent, sweep, current, veg, y, output] {
                assert!(x.is_finite() && !x.is_subnormal());
            }
        }
        assert!(m.diagnostics().non_finite_inputs > 0);
        assert!(ota.diagnostics().non_finite_inputs > 0);
        m.reset();
        assert_eq!(m.diagnostics().non_finite_inputs, 0);
    }
    let mut c = Components::NOMINAL;
    c.c62 = f64::NAN;
    c.r123 = -1.0;
    c.thermal_voltage = 0.0;
    assert!(Meg::new(c).tick(12.0, true, 1.0).is_finite());
    assert!(Veg::new(c).tick(12.0).is_finite());
    assert!(
        Vca::new(c)
            .tick(VcfSignals::default(), 1e-6, 1.0)
            .is_finite()
    );
}
