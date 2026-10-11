//! Render the DW-303 through its real engine at a fixed tempo, write a 24-bit
//! mono WAV, and report the real-time factor.
//!
//!   dw303-render [out.wav] [--seconds 8] [--rate 48000] [--tempo 125]
//!                [--set ID=VALUE ...] [--economy]
use dw303_engine::{Dw303Engine, params};
use ravet_foundation::{
    AudioBuffer, AudioLayout, Event, EventData, ParameterId, ProcessBlock, ProcessConfig,
    Processor, Transport,
};
use std::{io::Write, time::Instant};

const BLOCK: usize = 256;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut out = String::from("dw303.wav");
    let mut seconds = 8.0_f64;
    let mut rate = 48_000.0_f64;
    let mut settings: Vec<(ParameterId, f32)> = vec![
        (params::SYNC_MODE, 0.0),
        (params::TEMPO, 125.0),
        (params::RUN, 1.0),
    ];
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--seconds" => seconds = args.next().ok_or("--seconds value")?.parse()?,
            "--rate" => rate = args.next().ok_or("--rate value")?.parse()?,
            "--tempo" => {
                settings.push((params::TEMPO, args.next().ok_or("--tempo value")?.parse()?))
            }
            "--economy" => settings.push((params::QUALITY, 0.0)),
            "--set" => {
                let s = args.next().ok_or("--set ID=VALUE")?;
                let (id, value) = s.split_once('=').ok_or("--set ID=VALUE")?;
                settings.push((ParameterId(id.parse()?), value.parse()?));
            }
            _ => out = arg,
        }
    }

    let mut engine = Dw303Engine::new();
    engine.prepare(ProcessConfig::new(
        rate,
        BLOCK,
        64,
        AudioLayout::MONO_INSTRUMENT,
    )?)?;
    let initial: Vec<Event> = settings
        .iter()
        .map(|&(id, value)| Event {
            frame_offset: 0,
            data: EventData::ParameterValue { id, value },
        })
        .collect();

    let total = (seconds * rate) as usize;
    let mut samples = Vec::with_capacity(total);
    let mut buffer = [0.0_f32; BLOCK];
    let start = Instant::now();
    let mut done = 0;
    while done < total {
        let frames = BLOCK.min(total - done);
        let events: &[Event] = if done == 0 { &initial } else { &[] };
        let mut outputs = [&mut buffer[..frames]];
        let audio = AudioBuffer::separate(&[], &mut outputs, frames)?;
        let mut block = ProcessBlock::new(audio, events, Transport::default());
        let status = engine.process(&mut block);
        if let ravet_foundation::ProcessStatus::Fault(e) = status {
            return Err(format!("engine fault: {e:?}").into());
        }
        samples.extend_from_slice(&buffer[..frames]);
        done += frames;
    }
    let elapsed = start.elapsed().as_secs_f64();

    let peak = samples.iter().fold(0.0_f32, |p, s| p.max(s.abs()));
    let rms = (samples.iter().map(|s| f64::from(*s).powi(2)).sum::<f64>() / total as f64).sqrt();
    write_wav(&out, rate as u32, &samples)?;
    let d = engine.vcf_diagnostics();
    println!("wrote {out}: {seconds} s at {rate} Hz");
    println!(
        "peak {:.2} dBFS, rms {:.2} dBFS",
        20.0 * peak.log10(),
        20.0 * rms.log10()
    );
    println!(
        "render {elapsed:.3} s = {:.1}x real time ({:.1}% of one core)",
        seconds / elapsed,
        100.0 * elapsed / seconds
    );
    println!("vcf diagnostics: {d:?}");
    Ok(())
}

fn write_wav(path: &str, rate: u32, samples: &[f32]) -> std::io::Result<()> {
    let data_len = (samples.len() * 3) as u32;
    let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
    f.write_all(b"RIFF")?;
    f.write_all(&(36 + data_len).to_le_bytes())?;
    f.write_all(b"WAVEfmt ")?;
    f.write_all(&16u32.to_le_bytes())?;
    f.write_all(&1u16.to_le_bytes())?; // PCM
    f.write_all(&1u16.to_le_bytes())?; // mono
    f.write_all(&rate.to_le_bytes())?;
    f.write_all(&(rate * 3).to_le_bytes())?;
    f.write_all(&3u16.to_le_bytes())?;
    f.write_all(&24u16.to_le_bytes())?;
    f.write_all(b"data")?;
    f.write_all(&data_len.to_le_bytes())?;
    for s in samples {
        let v = (f64::from(s.clamp(-1.0, 1.0)) * 8_388_607.0).round() as i32;
        f.write_all(&v.to_le_bytes()[..3])?;
    }
    f.flush()
}
