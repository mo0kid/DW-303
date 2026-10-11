use dw303_env::{AccentSweep, Components, FilterControl, Meg, OutputStage, StageError, Vca, Veg};

fn meg_curve(rate: f64, decay: f64, accent: bool) -> (f64, f64) {
    let mut meg = Meg::new(Components::NOMINAL);
    meg.prepare(rate).unwrap();
    let attack = (rate * 0.004).round() as usize;
    let mut peak = 0.0;
    for n in 0..attack {
        peak = meg.tick(n == 0, accent, decay);
    }
    let mut end = 0.0;
    for _ in 0..rate as usize {
        end = meg.tick(false, accent, decay);
    }
    (peak, end)
}

#[test]
fn decay_is_component_rc_and_accent_shorts_pot() {
    let c = Components::NOMINAL;
    for rate in [48_000.0, 96_000.0, 192_000.0] {
        for decay in [0.0, 0.25, 0.5, 1.0] {
            let (peak, end) = meg_curve(rate, decay, false);
            let tau = (c.meg_decay_resistance + decay * c.decay_pot_resistance) * c.meg_capacitance;
            assert!((end / peak - (-1.0 / tau).exp()).abs() < 1e-9);
            let accented = meg_curve(rate, decay, true);
            let minimum = meg_curve(rate, 0.0, false);
            assert!((accented.1 - minimum.1).abs() < 1e-12);
        }
        let (peak, _) = meg_curve(rate, 0.0, false);
        assert!((peak - c.meg_source_voltage * (1.0 - (-4.0_f64).exp())).abs() < 1e-9);
    }
}

fn sweep_train(rate: f64, resonance: f64) -> [f64; 64] {
    let mut sweep = AccentSweep::new(Components::NOMINAL);
    let mut meg = Meg::new(Components::NOMINAL);
    sweep.prepare(rate).unwrap();
    meg.prepare(rate).unwrap();
    let mut peaks = [0.0_f64; 64];
    for peak in &mut peaks {
        for n in 0..(rate * 0.125) as usize {
            let m = meg.tick(n == 0, true, 1.0);
            let out = sweep.tick((n as f64) < rate * 0.075, m, 1.0, resonance);
            *peak = peak.max(out);
        }
    }
    peaks
}

#[test]
fn accent_accumulates_saturates_and_tracks_resonance_gang() {
    let high = sweep_train(48_000.0, 1.0);
    for n in 1..8 {
        assert!(high[n] > high[n - 1]);
    }
    assert!((high[63] - high[62]).abs() < 1e-7);
    assert!(high[63] < Components::NOMINAL.meg_source_voltage);
    let low = sweep_train(48_000.0, 0.0);
    for n in 0..64 {
        assert!((low[n] / high[n] - 0.25).abs() < 1e-12);
    }
    for rate in [96_000.0, 192_000.0] {
        let other = sweep_train(rate, 1.0);
        for n in 0..8 {
            assert!((other[n] - high[n]).abs() < 0.002);
        }
    }
}

#[test]
fn veg_fixed_decay_and_gate_off_release_are_rate_independent() {
    let c = Components::NOMINAL;
    for rate in [48_000.0, 96_000.0, 192_000.0] {
        let mut veg = Veg::new(c);
        veg.prepare(rate).unwrap();
        let mut peak = 0.0;
        for n in 0..(rate * c.veg_charge_pulse) as usize {
            peak = veg.tick(true, n == 0);
        }
        let mut held = 0.0;
        for _ in 0..(rate * 0.1) as usize {
            held = veg.tick(true, false);
        }
        assert!(
            (held / peak - (-0.1 / (c.veg_capacitance * c.veg_decay_resistance)).exp()).abs()
                < 1e-9
        );
        let mut released = 0.0;
        for _ in 0..(rate * 0.05) as usize {
            released = veg.tick(false, true);
        }
        assert!(
            (released / held - (-0.05 / (c.veg_capacitance * c.veg_release_resistance)).exp())
                .abs()
                < 1e-9
        );
        assert!(released < held * 0.01);
    }
}

#[test]
fn converter_amperes_and_bipolar_env_mod_law() {
    let f = FilterControl::new(Components::NOMINAL);
    assert!((f.bias_current(0.5, 0.0, 0.0, 0.0) - 1e-6).abs() < 1e-15);
    assert!((f.bias_current(0.625, 0.0, 0.0, 0.0) / 1e-6 - 2.0).abs() < 1e-12);
    assert!(f.bias_current(0.5, 1.0, 0.0, 0.0) < 1e-6);
    assert!(f.bias_current(0.5, 1.0, 5.0, 0.0) > 1e-6);
    assert_eq!(f.bias_current(1.0, 1.0, 12.0, 12.0), 1e-3);
}

#[test]
fn vca_control_gain_is_linear_above_junction_and_exponential_below() {
    let mut vca = Vca::new(Components::NOMINAL);
    assert!((vca.gain(5.0) - 1.00001).abs() < 1e-12);
    let leakage = Components::NOMINAL.vca_leakage;
    let low_ratio = (vca.gain(0.2) - leakage) / (vca.gain(0.174) - leakage);
    assert!((low_ratio - core::f64::consts::E).abs() < 1e-5);
    assert!(((vca.gain(2.8) - leakage) / (vca.gain(1.7) - leakage) - 2.0).abs() < 1e-12);
    let ordinary = vca.tick(0.01, 4.0, false, 1.0);
    let accented = vca.tick(0.01, 4.0, true, 1.0);
    assert!(accented > ordinary);
    assert!(vca.tick(0.0, 4.0, false, 0.0) > 0.0);
}

#[test]
fn output_removes_dc_and_audio_taper_has_exact_endpoints() {
    for rate in [48_000.0, 96_000.0, 192_000.0] {
        let mut output = OutputStage::new(Components::NOMINAL);
        output.prepare(rate).unwrap();
        let first = output.tick(1.0, 1.0);
        let mut end = first;
        for _ in 1..rate as usize {
            end = output.tick(1.0, 1.0);
        }
        assert!(end.abs() < 3e-9);
        output.reset();
        assert_eq!(output.tick(1.0, 0.0), 0.0);
        output.reset();
        assert!((output.tick(1.0, 0.5) / first - 0.1).abs() < 1e-12);
    }
}

#[test]
fn lifecycle_sanitisation_and_long_run_stability() {
    let c = Components::NOMINAL;
    let mut meg = Meg::new(c);
    let mut veg = Veg::new(c);
    let mut sweep = AccentSweep::new(c);
    let mut filter = FilterControl::new(c);
    let mut vca = Vca::new(c);
    let mut output = OutputStage::new(c);
    for rate in [f64::NAN, f64::INFINITY, 0.0, -1.0, 44_099.0, 384_001.0] {
        assert_eq!(meg.prepare(rate), Err(StageError::InvalidSampleRate));
        assert_eq!(veg.prepare(rate), Err(StageError::InvalidSampleRate));
        assert_eq!(sweep.prepare(rate), Err(StageError::InvalidSampleRate));
        assert_eq!(filter.prepare(rate), Err(StageError::InvalidSampleRate));
        assert_eq!(vca.prepare(rate), Err(StageError::InvalidSampleRate));
        assert_eq!(output.prepare(rate), Err(StageError::InvalidSampleRate));
    }
    meg.tick(false, false, f64::NAN);
    sweep.tick(true, f64::NAN, f64::INFINITY, f64::NEG_INFINITY);
    filter.bias_current(f64::NAN, f64::NAN, f64::NAN, f64::NAN);
    vca.tick(f64::NAN, f64::NAN, false, f64::NAN);
    output.tick(f64::NAN, f64::NAN);
    assert_eq!(meg.diagnostics().non_finite_inputs, 1);
    assert_eq!(sweep.diagnostics().non_finite_inputs, 3);
    assert_eq!(filter.diagnostics().non_finite_inputs, 4);
    assert_eq!(vca.diagnostics().non_finite_inputs, 3);
    assert_eq!(output.diagnostics().non_finite_inputs, 2);
    for rate in [44_100.0, 48_000.0, 96_000.0, 192_000.0, 384_000.0] {
        meg.prepare(rate).unwrap();
        veg.prepare(rate).unwrap();
        sweep.prepare(rate).unwrap();
        filter.prepare(rate).unwrap();
        vca.prepare(rate).unwrap();
        output.prepare(rate).unwrap();
        for n in 0..500_000 {
            let gate = n % 6000 < 4000;
            let knob = if n % 12000 < 6000 { 0.0 } else { 1.0 };
            let m = meg.tick(n % 6000 == 0, knob > 0.0, knob);
            let v = veg.tick(gate, n % 6000 == 0);
            let a = sweep.tick(gate, m, knob, knob);
            let i = filter.bias_current(knob, knob, m, a);
            let x = vca.tick(if n % 2 == 0 { f64::MAX } else { -f64::MAX }, v, gate, knob);
            let y = output.tick(x, knob);
            for value in [m, v, a, i, x, y] {
                assert!(value.is_finite());
                assert!(!value.is_subnormal());
            }
        }
    }
    meg.reset();
    veg.reset();
    sweep.reset();
    filter.reset();
    vca.reset();
    output.reset();
    assert_eq!(meg.tick(false, false, 0.0), 0.0);
    assert_eq!(veg.tick(false, false), 0.0);
    assert_eq!(sweep.tick(false, 0.0, 1.0, 1.0), 0.0);
    assert_eq!(output.tick(0.0, 1.0), 0.0);
    assert_eq!(filter.diagnostics().non_finite_inputs, 0);
}
