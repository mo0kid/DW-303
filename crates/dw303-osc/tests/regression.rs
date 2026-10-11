#[path = "support/reference.rs"]
mod reference;
use dw303_osc::{Components, PitchCv, SquareShaper, Vco};

fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-6 * b.abs().max(1.0), "{a} != {b}");
}

#[test]
fn every_public_stage_matches_frozen_algorithm() {
    for rate in [48_000.0, 96_000.0, 192_000.0] {
        let mut pitch = PitchCv::new(Components::NOMINAL);
        let mut old_pitch = reference::PitchCv::new(reference::Components::NOMINAL);
        let mut vco = Vco::new(Components::NOMINAL);
        let mut old_vco = reference::Vco::new(reference::Components::NOMINAL);
        let mut square = SquareShaper::new(Components::NOMINAL);
        let mut old_square = reference::SquareShaper::new(reference::Components::NOMINAL);
        for band in [None, Some(20_000.0), Some(1_000.0), Some(f64::NAN)] {
            pitch.prepare(rate).unwrap();
            old_pitch.prepare(rate).unwrap();
            vco.prepare(rate).unwrap();
            old_vco.prepare(rate).unwrap();
            square.prepare(rate).unwrap();
            old_square.prepare(rate).unwrap();
            vco.set_band_limit(band);
            old_vco.set_band_limit(band);
            for n in 0..8192 {
                if n == 4096 {
                    pitch.reset();
                    old_pitch.reset();
                    vco.reset();
                    old_vco.reset();
                    square.reset();
                    old_square.reset();
                }
                let note = [0, 12, 36, 60, 255, 7, 24, 48][(n / 512) % 8];
                let slide = n % 1024 >= 512;
                let cv = pitch.tick(note, slide);
                close(cv, old_pitch.tick(note, slide));
                close(
                    PitchCv::cv_for_note(note),
                    reference::PitchCv::cv_for_note(note),
                );
                let tuning = if n % 1024 < 512 {
                    0.5
                } else {
                    (n % 512) as f64 / 511.0
                };
                let cv = match n % 2048 {
                    0 => f64::NAN,
                    1 => f64::INFINITY,
                    2 => -100.0,
                    3 => 100.0,
                    _ => cv,
                };
                close(
                    vco.frequency_hz(cv, tuning),
                    old_vco.frequency_hz(cv, tuning),
                );
                close(
                    vco.charging_current_a(cv, tuning),
                    old_vco.charging_current_a(cv, tuning),
                );
                let a = vco.tick(cv, tuning);
                let b = old_vco.tick(cv, tuning);
                assert!(
                    (a.saw - b.saw).abs() <= 1e-6 * b.saw.abs().max(1.0),
                    "saw rate {rate} n {n} {a:?} {b:?}"
                );
                assert!(
                    (a.square - b.square).abs() <= 1e-6 * b.square.abs().max(1.0),
                    "square rate {rate} n {n} {a:?} {b:?}"
                );
                // Gate/rest and accent-like amplitude changes, abrupt overdrive,
                // and non-finite signals exercise the independent public shaper.
                let saw = match n % 1024 {
                    0 => f64::NAN,
                    1 => -100.0,
                    2 => 100.0,
                    _ if n % 256 < 32 => 0.0,
                    _ => a.saw * if n % 512 < 256 { 1.0 } else { 2.0 },
                };
                close(square.tick(saw), old_square.tick(saw));
            }
            assert_eq!(
                vco.diagnostics().non_finite_inputs,
                old_vco.diagnostics().non_finite_inputs
            );
            assert_eq!(
                vco.diagnostics().frequency_limited_samples,
                old_vco.diagnostics().frequency_limited_samples
            );
            assert_eq!(
                square.diagnostics().non_finite_inputs,
                old_square.diagnostics().non_finite_inputs
            );
        }
    }
}

/// Manual stage timing, excluding construction and file I/O.
#[test]
#[ignore]
fn performance_comparison() {
    use std::{hint::black_box, time::Instant};
    let mut new = Vco::new(Components::NOMINAL);
    let mut old = reference::Vco::new(reference::Components::NOMINAL);
    new.prepare(192_000.0).unwrap();
    old.prepare(192_000.0).unwrap();
    new.set_band_limit(Some(20_000.0));
    old.set_band_limit(Some(20_000.0));
    let start = Instant::now();
    for n in 0..384_000 {
        black_box(old.tick(black_box((n / 24_000 % 4) as f64), 0.5));
    }
    let before = start.elapsed();
    let start = Instant::now();
    for n in 0..384_000 {
        black_box(new.tick(black_box((n / 24_000 % 4) as f64), 0.5));
    }
    let after = start.elapsed();
    println!(
        "VCO old {before:?}, new {after:?}, {:.2}x",
        before.as_secs_f64() / after.as_secs_f64()
    );
}

#[test]
#[ignore]
fn square_timing() {
    use std::{hint::black_box, time::Instant};
    let mut shaper = SquareShaper::new(Components::NOMINAL);
    shaper.prepare(192_000.0).unwrap();
    let start = Instant::now();
    for n in 0..384_000 {
        black_box(shaper.tick(black_box(0.5 + 4.5 * (n % 2940) as f64 / 2940.0)));
    }
    println!("Square {:?}", start.elapsed());
}
