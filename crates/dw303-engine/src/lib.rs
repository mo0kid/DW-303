//! The DW-303 processor shared by every plug-in format and the standalone app:
//! sequencer -> circuit voice, with parameters applied sample-accurately.
//!
//! Pattern memory is held here; persistence and the front-panel edit state
//! machine arrive in phase 6 (see PLAN.md).
pub mod params;

use dw303_sequencer::{ClockSource, Octave, Pattern, Sequencer, Step, Time, VoiceCommand};
use dw303_vcf::Quality;
use dw303_voice::{Knobs, Voice, VoiceInput};
use ravet_foundation::{
    EventData, ParameterState, PrepareError, ProcessBlock, ProcessConfig, ProcessError,
    ProcessStatus, Processor, Tail,
};

/// MIDI key of the voice's lowest C (sequencer note index 0, 65.4 Hz).
pub const LOWEST_C_KEY: u8 = 36;

pub struct Dw303Engine {
    config: Option<ProcessConfig>,
    params: ParameterState<'static, { params::COUNT }>,
    sequencer: Sequencer,
    voice: Voice,
    pattern: Pattern,
    running: bool,
}

impl Dw303Engine {
    pub fn new() -> Self {
        Self {
            config: None,
            params: params::schema().expect("valid schema").defaults(),
            sequencer: Sequencer::default(),
            voice: Voice::new(),
            pattern: demo_pattern(),
            running: false,
        }
    }

    pub fn parameters(&self) -> &ParameterState<'static, { params::COUNT }> {
        &self.params
    }

    pub fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = pattern;
    }

    pub fn vcf_diagnostics(&self) -> dw303_vcf::VcfDiagnostics {
        self.voice.vcf_diagnostics()
    }

    fn apply(&mut self, data: EventData) {
        match data {
            EventData::ParameterValue { id, value } => {
                if let Some(index) = params::schema().ok().and_then(|s| s.index(id))
                    && let Some(value) = params::DESCRIPTORS[index].range.canonicalize(value)
                {
                    let _ = self.params.set(id, value);
                    self.update_controls();
                }
            }
            other => self.sequencer.event(other),
        }
    }

    /// Push parameter values to the voice and clock.
    fn update_controls(&mut self) {
        let params = self.params;
        let v = |id| f64::from(params.get(id).unwrap_or_default());
        self.voice.set_knobs(Knobs {
            tuning: v(params::TUNING),
            cutoff: v(params::CUTOFF),
            resonance: v(params::RESONANCE),
            env_mod: v(params::ENV_MOD),
            decay: v(params::DECAY),
            accent: v(params::ACCENT),
            volume: v(params::VOLUME),
            square: v(params::WAVEFORM) >= 0.5,
        });
        self.voice.set_quality(if v(params::QUALITY) >= 0.5 {
            Quality::Full
        } else {
            Quality::Economy
        });
        let source = match v(params::SYNC_MODE).round() as i32 {
            0 => ClockSource::Internal,
            1 => ClockSource::Host,
            _ => ClockSource::Midi,
        };
        let clock = &mut self.sequencer.clock;
        clock.set_source(source);
        if source == ClockSource::Internal {
            clock.set_tempo(v(params::TEMPO));
            let run = v(params::RUN) >= 0.5;
            if run != self.running {
                if run {
                    clock.start();
                } else {
                    clock.stop();
                }
            }
            self.running = run;
        } else {
            self.running = false;
        }
    }

    fn voice_input(&self, command: VoiceCommand, sequencing: bool) -> VoiceInput {
        // Stopped: the command comes from MIDI keys, mapped onto the voice range.
        let note = if sequencing {
            command.note
        } else {
            command.note.saturating_sub(LOWEST_C_KEY)
        };
        VoiceInput {
            note,
            gate: command.gate,
            trigger: command.trigger,
            accent: command.accent,
            slide: command.slide,
        }
    }
}

impl Default for Dw303Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor for Dw303Engine {
    fn prepare(&mut self, config: ProcessConfig) -> Result<(), PrepareError> {
        let rate = config.sample_rate();
        self.voice
            .prepare(rate)
            .map_err(|_| PrepareError::InvalidSampleRate)?;
        self.sequencer
            .prepare(rate)
            .map_err(|_| PrepareError::InvalidSampleRate)?;
        self.running = false;
        self.update_controls();
        self.voice.reset();
        self.config = Some(config);
        Ok(())
    }

    fn process(&mut self, block: &mut ProcessBlock<'_, '_, '_>) -> ProcessStatus {
        let Some(config) = self.config else {
            block.clear_outputs();
            return ProcessStatus::Fault(ProcessError::NotPrepared);
        };
        if let Err(error) = config.validate_block(block) {
            block.clear_outputs();
            return ProcessStatus::Fault(error);
        }
        self.sequencer.clock.begin_host_block(block.transport);
        let frames = block.audio.frames();
        let events = block.events;
        let mut next = 0;
        for frame in 0..frames {
            while next < events.len() && events[next].frame_offset == frame {
                self.apply(events[next].data);
                next += 1;
            }
            let sequencing = self.sequencer.clock.is_running();
            let command = self.sequencer.tick(&self.pattern);
            let sample = self.voice.tick(self.voice_input(command, sequencing)) as f32;
            for channel in 0..block.audio.output_channels() {
                if let Some(out) = block.audio.output_mut(channel) {
                    out[frame] = sample;
                }
            }
        }
        for event in &events[next..] {
            self.apply(event.data);
        }
        ProcessStatus::Continue
    }

    fn reset(&mut self) {
        self.sequencer.reset();
        self.voice.reset();
        self.running = false;
        self.update_controls();
    }

    fn deactivate(&mut self) {
        self.config = None;
    }

    fn tail(&self) -> Tail {
        Tail::Infinite
    }
}

/// A placeholder bass line so the instrument plays before pattern editing exists.
pub fn demo_pattern() -> Pattern {
    let notes: [(u8, Octave, bool, bool); 16] = [
        (0, Octave::Mid, true, false),
        (0, Octave::Mid, false, false),
        (12, Octave::Mid, false, true),
        (0, Octave::Mid, false, false),
        (3, Octave::Mid, true, false),
        (0, Octave::Mid, false, false),
        (7, Octave::Mid, false, true),
        (5, Octave::Mid, false, false),
        (0, Octave::Mid, true, false),
        (10, Octave::Mid, false, false),
        (0, Octave::Up, false, true),
        (0, Octave::Mid, false, false),
        (3, Octave::Mid, true, false),
        (7, Octave::Down, false, false),
        (5, Octave::Mid, false, true),
        (7, Octave::Mid, true, false),
    ];
    let mut pattern = Pattern::default();
    for (step, (pitch, octave, accent, slide)) in pattern.steps.iter_mut().zip(notes) {
        *step = Step {
            pitch,
            octave,
            accent,
            slide,
        };
    }
    pattern.time = [Time::Note; 16];
    pattern.time[5] = Time::Rest;
    pattern.time[11] = Time::Tie;
    pattern
}
