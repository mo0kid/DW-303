//! macOS capture operator CLI. All native operations stay off the audio callback.
//! VERIFY: first-sample epoch and independent input latency require hardware
//! calibration; RAVET input capture exposes no hardware timestamps.
//! VERIFY: panel clock positions assume 300-degree travel, 7 to 5 o'clock.
//! VERIFY: onset threshold includes analog attack and must be checked at the
//! actual interface gain, noise level and filter settings.
use dw303_capture::{Manifest, WavWriter, json, quote};
use ravet_device::cpal;
use std::{
    fs::{self, OpenOptions},
    io::{self, BufWriter},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};
type Error = Box<dyn std::error::Error + Send + Sync>;

fn option<'a>(args: &'a [String], name: &str) -> Result<&'a str, Error> {
    args.iter()
        .position(|s| s == name)
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .ok_or_else(|| format!("missing {name}").into())
}
fn sleep_until(deadline: Instant, stop: &AtomicBool) -> bool {
    while let Some(left) = deadline.checked_duration_since(Instant::now()) {
        if stop.load(Ordering::Relaxed) {
            return false;
        }
        thread::sleep(left.min(Duration::from_millis(10)));
    }
    !stop.load(Ordering::Relaxed)
}
fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("list") {
        for d in ravet_device::input_devices()? {
            println!("audio {}: {}", d.id, d.name);
            for c in d.configurations {
                println!("  {c:?}");
            }
        }
        for p in ravet_midi_device::output_ports()? {
            println!("MIDI {}: {}", p.id, p.name);
        }
        return Ok(());
    }
    if !matches!(args.first().map(String::as_str), Some("run" | "loopback")) {
        return Err("usage: dw303-capture list | run MANIFEST --capture ID --audio-device ID --channel N --midi-out NAME --rate 96000 --out DIRECTORY | loopback MANIFEST [same options; capture defaults to loopback]".into());
    }
    let manifest = Manifest::parse(&fs::read_to_string(
        args.get(1).ok_or("manifest path required")?,
    )?)?;
    let id = if args[0] == "loopback" {
        "loopback"
    } else {
        option(&args, "--capture")?
    };
    let capture = manifest
        .captures
        .iter()
        .find(|c| c.id == id)
        .ok_or("unknown capture ID")?;
    let rate: u32 = option(&args, "--rate")?.parse()?;
    if rate != 96000 {
        return Err("protocol requires 96000 Hz".into());
    }
    let channel: usize = option(&args, "--channel")?.parse()?;
    let latency_ms: f64 = if args.iter().any(|s| s == "--latency-ms") {
        option(&args, "--latency-ms")?.parse()?
    } else {
        0.0
    };
    if !latency_ms.is_finite() || latency_ms.abs() > 500.0 {
        return Err("latency compensation must be within +/-500 ms".into());
    }
    if args[0] == "loopback" && latency_ms != 0.0 {
        return Err("loopback requires uncompensated schedule".into());
    }
    let input_latency_ms: Option<f64> = if args.iter().any(|s| s == "--input-latency-ms") {
        Some(option(&args, "--input-latency-ms")?.parse()?)
    } else {
        None
    };
    if input_latency_ms.is_some_and(|v| !v.is_finite() || v < 0.0) {
        return Err("invalid input latency estimate".into());
    }
    let audio_id = option(&args, "--audio-device")?;
    let info = ravet_device::input_devices()?
        .into_iter()
        .find(|d| d.id == audio_id)
        .ok_or("unknown audio ID")?;
    let config = info
        .configurations
        .iter()
        .find(|c| {
            c.sample_format() == cpal::SampleFormat::F32
                && c.min_sample_rate() <= rate
                && c.max_sample_rate() >= rate
                && channel > 0
                && channel <= usize::from(c.channels())
        })
        .ok_or("device lacks requested f32 rate/channel")?;
    let channels = usize::from(config.channels());
    let midi_name = option(&args, "--midi-out")?;
    let ports: Vec<_> = ravet_midi_device::output_ports()?
        .into_iter()
        .filter(|p| p.name == midi_name)
        .collect();
    if ports.len() != 1 {
        return Err("MIDI name absent or ambiguous; choose a unique output name".into());
    }
    let out = std::path::Path::new(option(&args, "--out")?);
    fs::create_dir_all(out)?;
    let wav_path = out.join(format!("{id}.wav"));
    let sidecar_path = out.join(format!("{id}.json"));
    // Reserve both paths before starting hardware; never overwrite measurements.
    let wav_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(wav_path)?;
    let mut sidecar = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(sidecar_path)?;
    println!("Capture {id}: {}", capture.instructions);
    for (key, v) in capture.settings.as_table().ok_or("settings table")? {
        if let Some(n) = v.as_float().or_else(|| v.as_integer().map(|n| n as f64)) {
            let clock = (7.0 + 10.0 * n) % 12.0;
            let h = clock.floor() as u32;
            let minute = ((clock - clock.floor()) * 60.0).round() as u32;
            println!(
                "  {key}: legend {:.2}/10, approximately {}:{minute:02} (VERIFY knob travel)",
                n * 10.0,
                if h == 0 { 12 } else { h }
            );
        } else {
            println!("  {key}: {v:?}");
        }
    }
    println!(
        "Warm up >=30 min. Neutralise distortion/extra features; confirm session metadata. Enter room temperature in C (blank leaves null):"
    );
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature)?;
    let temperature = if temperature.trim().is_empty() {
        "null".into()
    } else {
        let t: f64 = temperature.trim().parse()?;
        if !t.is_finite() {
            return Err("invalid temperature".into());
        }
        t.to_string()
    };
    println!(
        "Press Enter when panel and routing match. Capture channel {channel}, {} -> {}.",
        info.name, ports[0].name
    );
    io::stdin().read_line(&mut String::new())?;
    let device = ravet_device::input_device(Some(audio_id))?.ok_or("device disappeared")?;
    let (stream, mut consumer) = ravet_device::open_input(
        &device,
        cpal::StreamConfig {
            channels: channels as u16,
            sample_rate: rate,
            buffer_size: cpal::BufferSize::Default,
        },
        65536,
    )?;
    let mut midi = ravet_midi_device::open_output(&ports[0].id)?;
    let schedule = capture.schedule(rate, &manifest.mapping);
    let total = ((capture.seconds + 1.5) * f64::from(rate)).round() as u64;
    let stop = Arc::new(AtomicBool::new(false));
    let writer_stop = stop.clone();
    let (tx, rx) = mpsc::sync_channel(1);
    let writer = thread::spawn(move || -> Result<(u64, Option<u64>, u64), Error> {
        let result = (|| {
            let mut wav = WavWriter::new(BufWriter::new(wav_file), rate)?;
            let mut buffer = vec![0.0; channels * 4096];
            let mut frames = 0u64;
            let mut peak_count = 0;
            let mut onset = None;
            let mut noise_sum = 0.0;
            let mut noise_n = 0u64;
            let mut last_data = Instant::now();
            while frames < total && !writer_stop.load(Ordering::Relaxed) {
                let count = consumer
                    .pop(&mut buffer)
                    .map_err(|e| format!("ring: {e:?}"))?;
                if count == 0 {
                    if last_data.elapsed() > Duration::from_secs(5) {
                        return Err("input stalled".into());
                    }
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }
                last_data = Instant::now();
                if frames == 0 {
                    // VERIFY: arrival minus batch duration estimates first sample epoch;
                    // RAVET supplies no CPAL capture timestamp or input-latency estimate.
                    let epoch = last_data - Duration::from_secs_f64(count as f64 / f64::from(rate));
                    tx.send(epoch)?;
                }
                for frame in buffer[..count * channels]
                    .chunks_exact(channels)
                    .take((total - frames) as usize)
                {
                    let sample = frame[channel - 1];
                    let magnitude = f64::from(sample.abs());
                    if frames < u64::from(rate) / 2 && magnitude.is_finite() {
                        noise_sum += magnitude * magnitude;
                        noise_n += 1;
                    }
                    let threshold = 0.01_f64.max(8.0 * (noise_sum / noise_n.max(1) as f64).sqrt());
                    if frames >= u64::from(rate) / 2 && onset.is_none() && magnitude > threshold {
                        onset = Some(frames);
                    }
                    if magnitude >= 1.0 {
                        peak_count += 1;
                    }
                    wav.sample(sample)?;
                    frames += 1;
                }
            }
            wav.finish()?;
            Ok((frames, onset, peak_count))
        })();
        if result.is_err() {
            writer_stop.store(true, Ordering::Relaxed);
        }
        result
    });
    if let Err(e) = stream.play() {
        stop.store(true, Ordering::Relaxed);
        let _ = writer.join();
        return Err(e.into());
    }
    let epoch = match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(t) => t,
        Err(e) => {
            stop.store(true, Ordering::Relaxed);
            let _ = stream.pause();
            let _ = writer.join();
            return Err(e.into());
        }
    };
    // Dedicated timer thread, no cumulative sleeps: every deadline uses epoch.
    let scheduled = schedule.clone();
    let scheduler_stop = stop.clone();
    let scheduler = thread::spawn(move || -> Result<Vec<String>, Error> {
        let result = (|| {
            let mut log = Vec::with_capacity(scheduled.len());
            for event in &scheduled {
                let send_target = event.sample as f64 / f64::from(rate) - latency_ms / 1000.0;
                if !sleep_until(
                    epoch + Duration::from_secs_f64(send_target),
                    &scheduler_stop,
                ) {
                    return Err("capture stopped or input/writer failed".into());
                }
                let actual = Instant::now().duration_since(epoch).as_secs_f64();
                midi.send(&event.bytes)?;
                log.push(format!(
                    "{{\"sample\":{},\"scheduled_seconds\":{},\"send_target_seconds\":{},\"send_seconds\":{},\"bytes\":{:?}}}",
                    event.sample,
                    event.sample as f64 / f64::from(rate),
                    send_target,
                    actual,
                    event.bytes
                ));
            }
            Ok(log)
        })();
        // Always release notes and transport, including failed sends.
        let _ = midi.send(&[0xb0 | manifest.mapping.channel, 123, 0]);
        let _ = midi.send(&[0xfc]);
        result
    });
    let midi_log = scheduler.join().map_err(|_| "scheduler thread failed")?;
    if midi_log.is_err() {
        stop.store(true, Ordering::Relaxed);
    }
    let written = writer.join().map_err(|_| "writer thread failed")?;
    stream.pause()?;
    let stats = stream.statistics();
    let (frames, onset, clipped) = written?;
    let midi_log = midi_log?;
    let first_on = schedule
        .iter()
        .find(|e| e.bytes[0] & 0xf0 == 0x90)
        .map(|e| e.sample);
    let latency = onset
        .zip(first_on)
        .map(|(a, b)| (a as f64 - b as f64) / f64::from(rate));
    use std::io::Write;
    writeln!(
        sidecar,
        "{{\"capture_id\":{},\"sample_rate\":{rate},\"bits\":24,\"channel\":{channel},\"frames\":{frames},\"settings\":{},\"tempo_bpm\":{},\"clock\":{},\"capture_seconds\":{},\"operator_instructions\":{},\"session\":{},\"room_temperature_c\":{temperature},\"audio_device\":{},\"audio_device_id\":{},\"midi_output\":{},\"midi_output_id\":{},\"midi_mapping\":{},\"first_sample_epoch_method\":\"VERIFY: first ring arrival minus batch duration; no hardware timestamp\",\"latency_compensation_seconds\":{},\"input_latency_estimate_seconds\":{},\"midi_to_audio_latency_estimate_seconds\":{},\"onset_sample\":{},\"clipped_samples\":{clipped},\"dropped_frames\":{},\"backend_errors\":{},\"invalid_buffers\":{},\"valid\":{},\"midi_schedule\":[{}]}}",
        quote(id),
        json(&capture.settings),
        capture.tempo,
        capture.clock,
        capture.seconds,
        quote(&capture.instructions),
        json(
            manifest
                .raw
                .get("session")
                .unwrap_or(&toml::Value::Table(Default::default()))
        ),
        quote(&info.name),
        quote(audio_id),
        quote(&ports[0].name),
        quote(&ports[0].id),
        json(&manifest.raw["midi"]),
        latency_ms / 1000.0,
        input_latency_ms.map_or("null".into(), |v| (v / 1000.0).to_string()),
        latency.map_or("null".into(), |v| v.to_string()),
        onset.map_or("null".into(), |v| v.to_string()),
        stats.dropped_frames,
        stats.backend_errors,
        stats.invalid_buffers,
        stats.dropped_frames == 0
            && stats.backend_errors == 0
            && stats.invalid_buffers == 0
            && frames == total
            && clipped == 0,
        midi_log.join(",")
    )?;
    println!(
        "Recorded {frames} frames; dropped {}, clipped {clipped}; MIDI-to-audio onset estimate {latency:?} s.",
        stats.dropped_frames
    );
    if stats.dropped_frames > 0
        || stats.backend_errors > 0
        || stats.invalid_buffers > 0
        || frames != total
        || clipped > 0
    {
        return Err("capture invalid; inspect sidecar and repeat at calibrated level".into());
    }
    Ok(())
}
