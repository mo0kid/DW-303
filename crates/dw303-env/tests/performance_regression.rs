//! Compare every public signal stage to the frozen, uncached equations.
#[path = "support/reference.rs"]
mod reference;
use dw303_env::*;
fn old_components(c: Components) -> reference::Components {
    reference::Components {
        meg_capacitance: c.meg_capacitance,
        meg_decay_resistance: c.meg_decay_resistance,
        decay_pot_resistance: c.decay_pot_resistance,
        meg_attack_resistance: c.meg_attack_resistance,
        meg_charge_pulse: c.meg_charge_pulse,
        meg_source_voltage: c.meg_source_voltage,
        veg_capacitance: c.veg_capacitance,
        veg_attack_resistance: c.veg_attack_resistance,
        veg_decay_resistance: c.veg_decay_resistance,
        veg_release_resistance: c.veg_release_resistance,
        veg_charge_pulse: c.veg_charge_pulse,
        veg_source_voltage: c.veg_source_voltage,
        accent_capacitance: c.accent_capacitance,
        accent_charge_resistance: c.accent_charge_resistance,
        accent_discharge_resistance: c.accent_discharge_resistance,
        accent_diode_voltage: c.accent_diode_voltage,
        resonance_sweep_floor: c.resonance_sweep_floor,
        filter_reference_current: c.filter_reference_current,
        cutoff_octaves: c.cutoff_octaves,
        envelope_octaves_per_volt: c.envelope_octaves_per_volt,
        envelope_reference_voltage: c.envelope_reference_voltage,
        accent_octaves_per_volt: c.accent_octaves_per_volt,
        vca_threshold_voltage: c.vca_threshold_voltage,
        thermal_voltage: c.thermal_voltage,
        vca_unity_voltage: c.vca_unity_voltage,
        vca_accent_voltage: c.vca_accent_voltage,
        vca_signal_scale: c.vca_signal_scale,
        vca_leakage: c.vca_leakage,
        vca_feedthrough: c.vca_feedthrough,
        output_capacitance: c.output_capacitance,
        output_load_resistance: c.output_load_resistance,
        output_buffer_gain: c.output_buffer_gain,
    }
}
fn matches(stage: &str, actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-6 * expected.abs().max(1.0),
        "{stage}: {actual} != {expected}"
    );
}
#[test]
fn every_stage_matches_uncached_reference() {
    let mut altered = Components::NOMINAL;
    altered.meg_charge_pulse = 0.003713;
    altered.veg_charge_pulse = 0.001317;
    altered.thermal_voltage = 0.04;
    altered.vca_unity_voltage = 0.9;
    altered.output_capacitance = 2.2e-6;
    let mut guarded = Components::NOMINAL;
    guarded.meg_capacitance = 0.0;
    guarded.veg_attack_resistance = f64::NAN;
    guarded.accent_charge_resistance = -1.0;
    guarded.vca_unity_voltage = f64::INFINITY;
    guarded.output_load_resistance = 0.0;
    for c in [Components::NOMINAL, altered, guarded] {
        let old = old_components(c);
        let (mut meg, mut om) = (Meg::new(c), reference::Meg::new(old));
        let (mut veg, mut ov) = (Veg::new(c), reference::Veg::new(old));
        let (mut sweep, mut os) = (AccentSweep::new(c), reference::AccentSweep::new(old));
        let (mut filter, mut of) = (FilterControl::new(c), reference::FilterControl::new(old));
        let (mut vca, mut oa) = (Vca::new(c), reference::Vca::new(old));
        let (mut output, mut oo) = (OutputStage::new(c), reference::OutputStage::new(old));
        // Preserve live capacitor state across rate changes, then exercise reset.
        for rate in [48_000.0, 96_000.0, 192_000.0] {
            meg.prepare(rate).unwrap();
            om.prepare(rate).unwrap();
            veg.prepare(rate).unwrap();
            ov.prepare(rate).unwrap();
            sweep.prepare(rate).unwrap();
            os.prepare(rate).unwrap();
            filter.prepare(rate).unwrap();
            of.prepare(rate).unwrap();
            vca.prepare(rate).unwrap();
            oa.prepare(rate).unwrap();
            output.prepare(rate).unwrap();
            oo.prepare(rate).unwrap();
            assert!(meg.prepare(f64::NAN).is_err());
            assert!(om.prepare(f64::NAN).is_err());
            for n in 0..30_000 {
                if n == 15_000 {
                    meg.reset();
                    om.reset();
                    veg.reset();
                    ov.reset();
                    sweep.reset();
                    os.reset();
                    filter.reset();
                    of.reset();
                    vca.reset();
                    oa.reset();
                    output.reset();
                    oo.reset();
                }
                let gate = n % 1301 < 853;
                let trigger = n % 1301 == 0 || n % 701 == 0;
                let accent_gate = gate && n % 3903 < 2602;
                let knob = match n % 997 {
                    0 => f64::NAN,
                    1 => f64::INFINITY,
                    2 => -3.0,
                    3 => 5.0,
                    _ => ((n / 97) % 101) as f64 / 100.0,
                };
                let m = meg.tick(trigger, accent_gate, knob);
                let rm = om.tick(trigger, accent_gate, knob);
                matches("MEG", m, rm);
                let v = veg.tick(gate, trigger);
                let rv = ov.tick(gate, trigger);
                matches("VEG", v, rv);
                let analog = match n % 389 {
                    0 => f64::NAN,
                    1 => -100.0,
                    2 => 100.0,
                    _ => m,
                };
                let s = sweep.tick(accent_gate, analog, knob, 1.0 - knob);
                let rs = os.tick(accent_gate, analog, knob, 1.0 - knob);
                matches("accent", s, rs);
                // Relative current comparison prevents the absolute voltage budget
                // from hiding changes to microampere converter outputs.
                let current = filter.bias_current(knob, 1.0 - knob, analog, s);
                let reference_current = of.bias_current(knob, 1.0 - knob, analog, rs);
                assert!((current - reference_current).abs() <= 1e-6 * reference_current);
                let input = match n % 389 {
                    0 => f64::NAN,
                    1 => -100.0,
                    2 => 100.0,
                    _ => 4.0 * libm::sin(n as f64 * 0.037),
                };
                matches("VCA gain", vca.gain(analog), oa.gain(analog));
                let a = vca.tick(input, v, accent_gate, knob);
                let ra = oa.tick(input, rv, accent_gate, knob);
                matches("VCA", a, ra);
                matches("output", output.tick(a, knob), oo.tick(ra, knob));
            }
            assert_eq!(
                meg.diagnostics().non_finite_inputs,
                om.diagnostics().non_finite_inputs
            );
            assert_eq!(
                veg.diagnostics().non_finite_inputs,
                ov.diagnostics().non_finite_inputs
            );
            assert_eq!(
                sweep.diagnostics().non_finite_inputs,
                os.diagnostics().non_finite_inputs
            );
            assert_eq!(
                filter.diagnostics().non_finite_inputs,
                of.diagnostics().non_finite_inputs
            );
            assert_eq!(
                vca.diagnostics().non_finite_inputs,
                oa.diagnostics().non_finite_inputs
            );
            assert_eq!(
                output.diagnostics().non_finite_inputs,
                oo.diagnostics().non_finite_inputs
            );
        }
    }
}

/// Isolates this crate's speedup from concurrently edited voice/filter crates.
#[test]
#[ignore = "manual release timing benchmark"]
fn benchmark_cached_against_reference() {
    macro_rules! run {
        ($c:expr, $meg:ty, $veg:ty, $sweep:ty, $filter:ty, $vca:ty, $output:ty) => {{
            let c = std::hint::black_box($c);
            let mut meg = <$meg>::new(c);
            let mut veg = <$veg>::new(c);
            let mut sweep = <$sweep>::new(c);
            let mut filter = <$filter>::new(c);
            let mut vca = <$vca>::new(c);
            let mut output = <$output>::new(c);
            meg.prepare(192_000.0).unwrap();
            veg.prepare(192_000.0).unwrap();
            sweep.prepare(192_000.0).unwrap();
            filter.prepare(192_000.0).unwrap();
            vca.prepare(192_000.0).unwrap();
            output.prepare(192_000.0).unwrap();
            let start = std::time::Instant::now();
            for n in 0..1_536_000 {
                let gate = n % 24_000 < 16_000;
                let trigger = n % 24_000 == 0;
                let accent = gate && n % 72_000 < 48_000;
                let knob = std::hint::black_box((n / 24_000 % 11) as f64 / 10.0);
                let m = std::hint::black_box(&mut meg).tick(trigger, accent, knob);
                let v = std::hint::black_box(&mut veg).tick(gate, trigger);
                let s = std::hint::black_box(&mut sweep).tick(accent, m, knob, 0.7);
                std::hint::black_box(std::hint::black_box(&filter).bias_current(0.6, knob, m, s));
                let a =
                    std::hint::black_box(&mut vca).tick(std::hint::black_box(0.3), v, accent, knob);
                std::hint::black_box(std::hint::black_box(&mut output).tick(a, 0.8));
            }
            start.elapsed().as_secs_f64()
        }};
    }
    let old = run!(
        old_components(Components::NOMINAL),
        reference::Meg,
        reference::Veg,
        reference::AccentSweep,
        reference::FilterControl,
        reference::Vca,
        reference::OutputStage
    );
    let cached = run!(
        Components::NOMINAL,
        Meg,
        Veg,
        AccentSweep,
        FilterControl,
        Vca,
        OutputStage
    );
    println!(
        "env-only 8 s / 192 kHz: old {old:.6} s ({:.2}% core), cached {cached:.6} s ({:.2}% core), {:.2}x speedup",
        old / 8.0 * 100.0,
        cached / 8.0 * 100.0,
        old / cached
    );
}
