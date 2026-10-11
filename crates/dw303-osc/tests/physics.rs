use dw303_osc::{Components, PitchCv, SquareShaper, StageError, Vco, cv_for_note};
use std::f64::consts::TAU;

fn cents(ratio: f64) -> f64 {
    1200.0 * ratio.log2()
}

#[test]
fn dac_and_slide_are_interval_and_rate_independent() {
    for note in 0..=255 {
        assert_eq!(cv_for_note(note), f64::from(note) / 12.0);
    }
    let c = Components::NOMINAL;
    let tau = c.slide_resistance * c.slide_capacitance;
    for rate in [44_100.0, 48_000.0, 96_000.0, 192_000.0, 384_000.0] {
        for (from, to) in [(0, 12), (0, 60), (60, 0), (24, 25)] {
            let mut cv = PitchCv::new(c);
            cv.prepare(rate).unwrap();
            cv.tick(from, false);
            let n = (tau * rate).round() as usize;
            let mut value = 0.0;
            for _ in 0..n {
                value = cv.tick(to, true);
            }
            let remaining = (value - cv_for_note(to)) / (cv_for_note(from) - cv_for_note(to));
            let expected = (-(n as f64) / (rate * tau)).exp();
            assert!((remaining - expected).abs() < 1e-11);
            assert_eq!(cv.tick(to, false), cv_for_note(to));
            cv.reset();
            assert_eq!(cv.tick(0, true), 0.0);
        }
    }
}

fn measured_frequency(vco: &mut Vco, rate: f64, cv: f64, tuning: f64) -> f64 {
    let expected = vco.frequency_hz(cv, tuning);
    let samples = (rate * 80.0 / expected).ceil() as usize;
    let mut previous = vco.tick(cv, tuning).saw;
    let mut first = None;
    let mut last = 0.0;
    let mut periods = 0;
    // Crossing away from the reset/kernel ringing.
    let threshold = 2.5;
    for n in 1..samples {
        let x = vco.tick(cv, tuning).saw;
        if previous < threshold && x >= threshold {
            let t = n as f64 - 1.0 + (threshold - previous) / (x - previous);
            if first.is_some() {
                periods += 1;
            } else {
                first = Some(t);
            }
            last = t;
        }
        previous = x;
    }
    periods as f64 * rate / (last - first.unwrap())
}

#[test]
fn measured_pitch_octaves_and_tuning_follow_expo_law() {
    for rate in [44_100.0, 48_000.0, 96_000.0, 192_000.0, 384_000.0] {
        let mut vco = Vco::new(Components::NOMINAL);
        vco.prepare(rate).unwrap();
        for note in (0..=60).step_by(12) {
            vco.reset();
            let cv = cv_for_note(note);
            let f = measured_frequency(&mut vco, rate, cv, 0.5);
            assert!(
                cents(f / vco.frequency_hz(cv, 0.5)).abs() < 1.0,
                "rate={rate}, note={note}, f={f}"
            );
            assert!(
                cents(vco.frequency_hz(cv + 1.0, 0.5) / vco.frequency_hz(cv, 0.5) / 2.0).abs()
                    < 1e-9
            );
        }
        for tuning in [0.0, 0.5, 1.0] {
            vco.reset();
            let f = measured_frequency(&mut vco, rate, 2.0, tuning);
            assert!(cents(f / vco.frequency_hz(2.0, tuning)).abs() < 1.0);
        }
        assert!(
            (cents(vco.frequency_hz(0.0, 1.0) / vco.frequency_hz(0.0, 0.0)) - 1200.0).abs() < 1e-9
        );
    }
    let nominal = Vco::new(Components::NOMINAL);
    let hot = Vco::new(Components {
        temperature_kelvin: 320.0,
        ..Components::NOMINAL
    });
    let expected = (2.0_f64).powf(298.15 / 320.0);
    assert!((hot.frequency_hz(1.0, 0.5) / hot.frequency_hz(0.0, 0.5) - expected).abs() < 1e-12);
    assert!(nominal.charging_current_a(0.0, 0.5) > 0.0);
    assert!(
        (nominal.charging_current_a(1.0, 0.5) / nominal.charging_current_a(0.0, 0.5) - 2.0).abs()
            < 1e-12
    );
}

#[test]
fn saw_and_pnp_have_asymmetric_curved_shapes() {
    let rate = 192_000.0;
    let mut vco = Vco::new(Components::NOMINAL);
    vco.prepare(rate).unwrap();
    let frequency = 200.0;
    let cv = (frequency / Components::NOMINAL.reference_hz).log2();
    for _ in 0..96_000 {
        vco.tick(cv, 0.5);
    }
    let period = (rate / frequency) as usize;
    let mut saw = Vec::new();
    let mut square = Vec::new();
    for _ in 0..period {
        let out = vco.tick(cv, 0.5);
        saw.push(out.saw);
        square.push(out.square);
    }
    let minimum = saw.iter().copied().fold(f64::INFINITY, f64::min);
    let maximum = saw.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    assert!(minimum < 0.8 && maximum > 4.7);
    let rising = saw.windows(2).filter(|w| w[1] > w[0]).count();
    assert!(rising as f64 / period as f64 > 0.85);
    // Find a ramp stretch away from the reset; finite Early voltage increases slope.
    let low_slopes: Vec<_> = saw
        .windows(2)
        .filter(|w| w[0] > 1.0 && w[0] < 1.5)
        .map(|w| w[1] - w[0])
        .collect();
    let high_slopes: Vec<_> = saw
        .windows(2)
        .filter(|w| w[0] > 4.0 && w[0] < 4.5)
        .map(|w| w[1] - w[0])
        .collect();
    let avg = |xs: &[f64]| xs.iter().sum::<f64>() / xs.len() as f64;
    assert!(avg(&high_slopes) > avg(&low_slopes));
    let duty = square.iter().filter(|&&x| x > 0.0).count() as f64 / period as f64;
    assert!((0.25..0.75).contains(&duty));
    assert!((duty - 0.5).abs() > 0.015, "duty {duty}");
    let positive: Vec<_> = square.iter().copied().filter(|&x| x > 1.0).collect();
    assert!(positive.len() > period / 5);
    let spread = positive.iter().copied().fold(f64::NEG_INFINITY, f64::max)
        - positive.iter().copied().fold(f64::INFINITY, f64::min);
    assert!(spread > 0.1);
    let mut shaper = SquareShaper::new(Components::NOMINAL);
    shaper.prepare(rate).unwrap();
    let first = shaper.tick(2.0);
    for _ in 0..96_000 {
        shaper.tick(2.0);
    }
    assert!(shaper.tick(2.0).abs() < 1e-6);
    assert!(first.is_finite());
}

#[test]
fn stability_sanitisation_and_prepare_errors() {
    for rate in [44_100.0, 48_000.0, 96_000.0, 192_000.0, 384_000.0] {
        let mut vco = Vco::new(Components::NOMINAL);
        vco.prepare(rate).unwrap();
        for n in 0..100_000 {
            let cv = [-16.0, 0.0, 3.0, 16.0][(n / 25_000) % 4];
            let out = vco.tick(cv, if n % 2 == 0 { 0.0 } else { 1.0 });
            assert!(out.saw.is_finite() && out.square.is_finite());
            assert!(out.saw.abs() < 20.0 && out.square.abs() < 20.0);
        }
        for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let out = vco.tick(x, x);
            assert!(out.saw.is_finite() && out.square.is_finite());
        }
        assert_eq!(vco.diagnostics().non_finite_inputs, 6);
        assert!(vco.diagnostics().frequency_limited_samples > 0);
        for bad in [0.0, -1.0, 1000.0, f64::NAN, f64::INFINITY] {
            assert_eq!(vco.prepare(bad), Err(StageError::InvalidSampleRate));
        }
        vco.reset();
        assert_eq!(vco.diagnostics().non_finite_inputs, 0);
        let mut pitch = PitchCv::new(Components::NOMINAL);
        assert_eq!(pitch.prepare(0.0), Err(StageError::InvalidSampleRate));
        let mut square = SquareShaper::new(Components::NOMINAL);
        square.tick(f64::NAN);
        assert_eq!(square.diagnostics().non_finite_inputs, 1);
        assert_eq!(square.prepare(f64::NAN), Err(StageError::InvalidSampleRate));
        square.reset();
        assert_eq!(square.diagnostics().non_finite_inputs, 0);
    }
    let mut bad = Components::NOMINAL;
    bad.square_supply = f64::NAN;
    bad.saw_high = -1.0;
    bad.slide_capacitance = f64::INFINITY;
    assert!(Vco::new(bad).tick(f64::MAX, f64::MAX).saw.is_finite());
    assert!(PitchCv::new(bad).tick(255, true).is_finite());
}

// In-place radix-2 FFT, only in tests. Ideal decimation is selection of bins
// below host Nyquist; rectangular integration is coherent (integer periods).
fn fft(xs: &mut [(f64, f64)]) {
    let n = xs.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            xs.swap(i, j);
        }
    }
    let mut length = 2;
    while length <= n {
        let angle = -TAU / length as f64;
        let step = (angle.cos(), angle.sin());
        for start in (0..n).step_by(length) {
            let mut w = (1.0, 0.0);
            for offset in 0..length / 2 {
                let a = xs[start + offset];
                let b = xs[start + offset + length / 2];
                let t = (w.0 * b.0 - w.1 * b.1, w.0 * b.1 + w.1 * b.0);
                xs[start + offset] = (a.0 + t.0, a.1 + t.1);
                xs[start + offset + length / 2] = (a.0 - t.0, a.1 - t.1);
                w = (w.0 * step.0 - w.1 * step.1, w.0 * step.1 + w.1 * step.0);
            }
        }
        length *= 2;
    }
}

#[test]
fn saw_alias_energy_after_ideal_fourfold_decimation_below_minus_80_db() {
    let rate = 192_000.0;
    let n = 65_536;
    let bin = 341;
    let frequency = rate * bin as f64 / n as f64; // 999.0234375 Hz, coherent ~1 kHz.
    let c = Components::NOMINAL;
    let cv = (frequency / c.reference_hz).log2();
    let mut vco = Vco::new(c);
    vco.prepare(rate).unwrap();
    let mut spectrum: Vec<_> = (0..n).map(|_| (vco.tick(cv, 0.5).saw, 0.0)).collect();
    fft(&mut spectrum);
    let power = |x: (f64, f64)| x.0 * x.0 + x.1 * x.1;
    let fundamental = power(spectrum[bin]);
    // Avoid a vacuous pass on silence or a sine-only substitute: ramp fundamental
    // is approximately span/pi peak, and the second harmonic must be substantial.
    assert!(fundamental > (n as f64 * 0.6).powi(2));
    assert!(power(spectrum[2 * bin]) / fundamental > 0.15);
    let alias: f64 = spectrum
        .iter()
        .enumerate()
        .take(n / 8)
        .skip(1)
        .filter(|(k, _)| k % bin != 0)
        .map(|(_, &x)| power(x))
        .sum();
    let db = 10.0 * (alias / fundamental).log10();
    println!("saw alias energy after ideal 192k→48k decimation: {db:.2} dB");
    assert!(db < -80.0, "alias energy {db} dB");
}

#[test]
fn long_run_retains_finite_state_and_reset_is_deterministic() {
    let mut a = Vco::new(Components::NOMINAL);
    a.prepare(48_000.0).unwrap();
    // 41.7 seconds, well beyond every coupling time constant.
    for _ in 0..2_000_000 {
        let out = a.tick(3.0, 0.5);
        assert!(out.saw.is_finite() && out.square.is_finite());
    }
    a.reset();
    let mut b = Vco::new(Components::NOMINAL);
    for _ in 0..1024 {
        let x = a.tick(2.0, 0.5);
        let y = b.tick(2.0, 0.5);
        assert_eq!(x.saw, y.saw);
        assert_eq!(x.square, y.square);
    }
}
