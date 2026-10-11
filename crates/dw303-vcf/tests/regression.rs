#[path = "support/reference.rs"]
mod reference;
use dw303_vcf::{Components, Quality, Vcf};

#[test]
fn both_qualities_match_frozen_algorithm() {
    for rate in [48_000.0, 96_000.0, 192_000.0] {
        let mut stage = Vcf::new(Components::NOMINAL);
        let mut old = reference::Vcf::new(reference::Components::NOMINAL);
        stage.prepare(rate).unwrap();
        old.prepare(rate).unwrap();
        for n in 0..32768 {
            if n % 8192 == 0 {
                let full = n % 16384 == 0;
                stage.set_quality(if full {
                    Quality::Full
                } else {
                    Quality::Economy
                });
                old.set_quality(if full {
                    reference::Quality::Full
                } else {
                    reference::Quality::Economy
                });
            }
            if n == 16384 {
                stage.reset();
                old.reset();
            }
            let gate = n % 1024 < 768;
            let accent = n % 2048 < 1024;
            let wave = libm::sin(n as f64 * 0.031) * if accent { 6.0 } else { 3.0 };
            let input = match n % 4096 {
                0 => f64::NAN,
                1 => 100.0,
                2 => -100.0,
                _ => {
                    if gate {
                        wave
                    } else {
                        0.0
                    }
                }
            };
            let bias = [0.0, 1e-9, 0.1e-6, 10e-6, 50e-6, 1e-3, f64::INFINITY, -1.0][(n / 1024) % 8];
            let resonance = (n % 1024) as f64 / 1023.0;
            let a = stage.tick(input, bias, resonance);
            let b = old.tick(input, bias, resonance);
            assert!(
                (a - b).abs() <= 1e-6 * b.abs().max(1.0),
                "rate {rate}, sample {n}: {a} != {b}"
            );
        }
        assert_eq!(
            stage.diagnostics().non_convergence,
            old.diagnostics().non_convergence
        );
        assert_eq!(
            stage.diagnostics().sanitized_inputs,
            old.diagnostics().sanitized_inputs
        );
    }
}

/// Manual stage timing, excluding construction and file I/O.
#[test]
#[ignore]
fn performance_comparison() {
    use std::{hint::black_box, time::Instant};
    let mut new = Vcf::new(Components::NOMINAL);
    let mut old = reference::Vcf::new(reference::Components::NOMINAL);
    let start = Instant::now();
    for n in 0..384_000 {
        black_box(old.tick(black_box(libm::sin(n as f64 * 0.02) * 3.0), 20e-6, 0.7));
    }
    let before = start.elapsed();
    let start = Instant::now();
    for n in 0..384_000 {
        black_box(new.tick(black_box(libm::sin(n as f64 * 0.02) * 3.0), 20e-6, 0.7));
    }
    let after = start.elapsed();
    println!(
        "VCF old {before:?}, new {after:?}, {:.2}x",
        before.as_secs_f64() / after.as_secs_f64()
    );
}

#[test]
fn sustained_self_oscillation_matches_frozen_algorithm() {
    for rate in [48_000.0, 96_000.0, 192_000.0] {
        let mut stage = Vcf::new(Components::NOMINAL);
        let mut old = reference::Vcf::new(reference::Components::NOMINAL);
        stage.prepare(rate).unwrap();
        old.prepare(rate).unwrap();
        for n in 0..192_000 {
            let input = if n < 512 { 0.05 } else { 0.0 };
            let a = stage.tick(input, 10e-6, 1.0);
            let b = old.tick(input, 10e-6, 1.0);
            assert!(
                (a - b).abs() <= 1e-6 * b.abs().max(1.0),
                "rate {rate}, sample {n}: {a} != {b}"
            );
        }
        assert_eq!(stage.diagnostics().non_convergence, 0);
    }
}
