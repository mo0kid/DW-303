//! The DW-303 processor shared by every plug-in format and the standalone app:
//! sequencer -> circuit voice, with parameters applied sample-accurately.
//!
//! Pattern memory and the original panel editor live on the audio thread.
pub mod link;
pub mod params;
use link::{Button, Editor, Memory, PanelAction, PanelLink};
use std::sync::Arc;

use dw303_sequencer::{ClockSource, Octave, Pattern, Sequencer, Step, Time, VoiceCommand};
use dw303_vcf::Quality;
use dw303_voice::{Knobs, Voice, VoiceInput};
use ravet_foundation::{
    EventData, ParameterState, PrepareError, ProcessBlock, ProcessConfig, ProcessError,
    ProcessStatus, Processor, Tail,
};

/// MIDI key of the voice's lowest C (sequencer note index 0, 65.4 Hz).
pub const LOWEST_C_KEY: u8 = 36;

/// Circuit voice and audio-thread front-panel sequencer.
pub struct Dw303Engine {
    config: Option<ProcessConfig>,
    params: ParameterState<'static, { params::COUNT }>,
    sequencer: Sequencer,
    voice: Voice,
    memory: Memory,
    editor: Editor,
    link: Option<Arc<PanelLink>>,
    memory_generation: u32,
    dirty: bool,
    last_command: VoiceCommand,
    preview_active: bool,
    previews: [VoiceCommand; link::CAPACITY],
    preview_head: usize,
    preview_count: usize,
}

impl Dw303Engine {
    /// Construct with factory memory and no shared panel.
    pub fn new() -> Self {
        Self {
            config: None,
            params: params::schema().expect("valid schema").defaults(),
            sequencer: Sequencer::default(),
            voice: Voice::new(),
            memory: factory_memory(),
            editor: Editor::new(),
            link: None,
            memory_generation: 0,
            dirty: true,
            last_command: VoiceCommand::default(),
            preview_active: false,
            previews: [VoiceCommand::default(); link::CAPACITY],
            preview_head: 0,
            preview_count: 0,
        }
    }

    /// Construct with a shared front-panel exchange.
    pub fn with_link(link: Arc<PanelLink>) -> Self {
        let memory = link.memory();
        Self {
            memory,
            link: Some(link),
            ..Self::new()
        }
    }

    /// Last sequencer/preview command supplied to the voice (MIDI note is pre-offset).
    pub fn last_command(&self) -> VoiceCommand {
        self.last_command
    }

    /// Current parameter values.
    pub fn parameters(&self) -> &ParameterState<'static, { params::COUNT }> {
        &self.params
    }

    /// Selected pattern memory.
    pub fn pattern(&self) -> &Pattern {
        &self.memory.bank.patterns[self.editor.selection().pattern.index().unwrap_or(0)]
    }

    /// Replace the selected pattern.
    pub fn set_pattern(&mut self, pattern: Pattern) {
        self.memory.bank.patterns[self.editor.selection().pattern.index().unwrap_or(0)] = pattern;
        self.dirty = true;
    }

    /// Circuit solver diagnostics.
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
                    if id == params::RUN && (value >= 0.5) != self.editor.running() {
                        self.panel_action(PanelAction::Press(Button::RunStop));
                        self.panel_action(PanelAction::Release(Button::RunStop));
                    }
                    self.update_controls();
                }
            }
            other => {
                self.sequencer.event(other);
                self.reflect_transport();
            }
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
        let changed_source = self.sequencer.clock.source() != source;
        self.sequencer.clock.set_source(source);
        if changed_source && source == ClockSource::Internal && self.editor.running() {
            self.sequencer.clock.start();
        }
        if source == ClockSource::Internal {
            self.sequencer.clock.set_tempo(v(params::TEMPO));
        }
    }

    fn reflect_transport(&mut self) {
        let running = self.sequencer.clock.is_running();
        if running != self.editor.running() {
            let _ = self
                .editor
                .action(&mut self.memory, PanelAction::Press(Button::RunStop));
            let _ = self
                .editor
                .action(&mut self.memory, PanelAction::Release(Button::RunStop));
        }
    }
    fn panel_action(&mut self, action: PanelAction) {
        if let Ok(out) = self.editor.action(&mut self.memory, action) {
            self.dirty |= out.changed;
            if let Some(preview) = out.preview {
                self.preview_active = preview.gate;
                if matches!(
                    action,
                    PanelAction::Mode(_) | PanelAction::Press(Button::RunStop)
                ) {
                    self.preview_head = 0;
                    self.preview_count = 0;
                } else if self.preview_count < link::CAPACITY {
                    self.previews[(self.preview_head + self.preview_count) % link::CAPACITY] =
                        preview;
                    self.preview_count += 1;
                }
            }
            self.editor.apply_transport(&mut self.sequencer, out);
            let _ = self
                .params
                .set(params::RUN, f32::from(self.editor.running()));
        }
    }
    fn panel_block(&mut self) {
        if let Some(link) = self.link.clone() {
            if let Some(memory) = link.adopt_memory(&mut self.memory_generation) {
                self.memory = memory;
                self.preview_active = false;
                self.preview_head = 0;
                self.preview_count = 0;
                self.editor = Editor::new();
                self.sequencer.reset();
                self.voice.reset();
                self.dirty = true;
                let _ = self.params.set(params::RUN, 0.0);
            }
            for _ in 0..link::CAPACITY {
                if self.preview_count == link::CAPACITY {
                    break;
                }
                if let Some(action) = link.pop() {
                    self.panel_action(action);
                } else {
                    break;
                }
            }
            if self.dirty && link.publish_memory(&self.memory) {
                self.dirty = false;
            }
        }
    }

    fn voice_input(&self, command: VoiceCommand, sequencing: bool) -> VoiceInput {
        // Stopped: the command comes from MIDI keys, mapped onto the voice range.
        let note = if sequencing || self.preview_active {
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
        self.panel_block();
        self.sequencer.clock.begin_host_block(block.transport);
        self.reflect_transport();
        let frames = block.audio.frames();
        let events = block.events;
        let mut next = 0;
        for frame in 0..frames {
            while next < events.len() && events[next].frame_offset == frame {
                self.apply(events[next].data);
                next += 1;
            }
            let sequencing = self.sequencer.clock.is_running();
            let mut command = self.editor.tick(&mut self.sequencer, &self.memory);
            let preview = !sequencing && self.preview_count > 0;
            if self.preview_count > 0 {
                if preview {
                    command = self.previews[self.preview_head];
                }
                self.preview_head = (self.preview_head + 1) % link::CAPACITY;
                self.preview_count -= 1;
            }
            self.last_command = command;
            let sample =
                self.voice
                    .tick(self.voice_input(command, sequencing || preview)) as f32;
            for channel in 0..block.audio.output_channels() {
                if let Some(out) = block.audio.output_mut(channel) {
                    out[frame] = sample;
                }
            }
        }
        for event in &events[next..] {
            self.apply(event.data);
        }
        let running = self.sequencer.clock.is_running();
        self.reflect_transport();
        let _ = self.params.set(params::RUN, f32::from(running));
        if let Some(link) = &self.link {
            link.publish_display(&self.editor, &self.memory, running);
        }
        ProcessStatus::Continue
    }

    fn reset(&mut self) {
        self.sequencer.reset();
        if self.editor.running() {
            self.sequencer.clock.start();
        }
        self.voice.reset();

        self.update_controls();
    }

    fn deactivate(&mut self) {
        self.config = None;
    }

    fn tail(&self) -> Tail {
        Tail::Infinite
    }
}

/// Factory pattern in group I, section A, pattern 1.
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

/// Complete factory bank; other patterns and tracks use codec defaults.
pub fn factory_memory() -> Memory {
    let mut memory = Memory::default();
    memory.bank.patterns[0] = demo_pattern();
    memory
}

#[cfg(test)]
mod tests {
    use super::*;
    use link::Mode;
    use ravet_foundation::{AudioBuffer, AudioLayout, Transport};
    fn tap(link: &PanelLink, button: Button) {
        link.push(PanelAction::Press(button)).unwrap();
        link.push(PanelAction::Release(button)).unwrap();
    }
    fn render(engine: &mut Dw303Engine) {
        let mut left = [0.0; 64];
        let mut right = [0.0; 64];
        let mut outputs = [&mut left[..], &mut right[..]];
        let audio = AudioBuffer::separate(&[], &mut outputs, 64).unwrap();
        let mut block = ProcessBlock::new(audio, &[], Transport::default());
        assert_eq!(engine.process(&mut block), ProcessStatus::Continue);
        assert!(left.iter().chain(&right).all(|s| s.is_finite()));
    }
    #[test]
    fn reactivation_keeps_published_memory_and_preview_edges_trigger() {
        let link = Arc::new(PanelLink::new());
        let mut engine = Dw303Engine::with_link(link.clone());
        engine
            .prepare(ProcessConfig::new(48000.0, 64, 128, AudioLayout::STEREO_INSTRUMENT).unwrap())
            .unwrap();
        let mut pattern = demo_pattern();
        pattern.length = 4;
        engine.set_pattern(pattern);
        render(&mut engine);
        assert_eq!(Dw303Engine::with_link(link.clone()).pattern().length, 4);
        link.push(PanelAction::Mode(Mode::PatternWrite)).unwrap();
        tap(&link, Button::PitchMode);
        tap(&link, Button::Pitch(2));
        engine.panel_block();
        assert!(
            engine
                .previews
                .iter()
                .take(engine.preview_count)
                .any(|p| p.gate && p.trigger && p.note == 14)
        );
    }
    #[test]
    fn write_four_steps_then_play_reaches_voice() {
        let link = Arc::new(PanelLink::new());
        let mut engine = Dw303Engine::with_link(link.clone());
        engine
            .prepare(ProcessConfig::new(48000.0, 64, 128, AudioLayout::STEREO_INSTRUMENT).unwrap())
            .unwrap();
        engine.apply(EventData::ParameterValue {
            id: params::SYNC_MODE,
            value: 0.0,
        });
        engine.apply(EventData::ParameterValue {
            id: params::QUALITY,
            value: 0.0,
        });
        link.push(PanelAction::Mode(Mode::PatternWrite)).unwrap();
        link.push(PanelAction::Press(Button::Pitch(0))).unwrap();
        tap(&link, Button::Clear);
        link.push(PanelAction::Release(Button::Pitch(0))).unwrap();
        link.push(PanelAction::Press(Button::Function)).unwrap();
        for _ in 0..4 {
            tap(&link, Button::Step);
        }
        link.push(PanelAction::Release(Button::Function)).unwrap();
        tap(&link, Button::PitchMode);
        for note in [0, 2, 4, 5] {
            tap(&link, Button::Pitch(note));
        }
        tap(&link, Button::TimeMode);
        for _ in 0..4 {
            tap(&link, Button::Down);
        }
        link.push(PanelAction::Mode(Mode::PatternPlay)).unwrap();
        tap(&link, Button::RunStop);
        let mut seen = [false; 4];
        let mut had_gate = false;
        for _ in 0..400 {
            render(&mut engine);
            let c = engine.last_command();
            had_gate |= c.gate;
            for (i, n) in [12, 14, 16, 17].into_iter().enumerate() {
                seen[i] |= c.gate && c.note == n;
            }
        }
        assert!(had_gate && seen.into_iter().all(|s| s));
        assert_eq!(engine.pattern().length, 4);
        assert_eq!(link.memory().bank.patterns[0], *engine.pattern());
        assert_eq!(engine.parameters().get(params::RUN), Some(1.0));
        engine.apply(EventData::ParameterValue {
            id: params::RUN,
            value: 0.0,
        });
        render(&mut engine);
        assert!(!engine.last_command().gate);
        assert!(!link.display().unwrap().running);
    }
}
