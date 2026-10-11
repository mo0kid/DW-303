//! DW-303 CLAP instrument: stereo output, MIDI note input, DW-303 parameters
//! and complete parameter state. Pattern state and the editor follow in later
//! phases (see PLAN.md).
use clack_extensions::{audio_ports::*, note_ports::*, params::*, state::*, tail::*};
use clack_plugin::{events::event_types::ParamValueEvent, prelude::*, utils::Cookie};
use dw303_engine::{Dw303Engine, params};
use ravet_clap::{MAXIMUM_EVENTS, MainBusBridge};
use ravet_foundation::{AudioLayout, ParameterId, ParameterState, ProcessConfig};
use std::{
    ffi::CStr,
    fmt::Write,
    io::{Read, Write as IoWrite},
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
};

pub const PLUGIN_ID: &str = "co.djwaudio.dw303";
const STATE_PRODUCT: [u8; 8] = *b"DJWDW303";
const STATE_VERSION: u32 = 1;

pub struct Dw303Plugin;

impl Plugin for Dw303Plugin {
    type AudioProcessor<'a> = Dw303Audio<'a>;
    type Shared<'a> = Dw303Shared;
    type MainThread<'a> = Dw303Main<'a>;
    fn declare_extensions(builder: &mut PluginExtensions<Self>, _: Option<&Dw303Shared>) {
        builder
            .register::<PluginAudioPorts>()
            .register::<PluginNotePorts>()
            .register::<PluginParams>()
            .register::<PluginState>()
            .register::<PluginTail>();
    }
}

impl DefaultPluginFactory for Dw303Plugin {
    fn get_descriptor() -> PluginDescriptor {
        use clack_plugin::plugin::features::{INSTRUMENT, MONO, SYNTHESIZER};
        PluginDescriptor::new(PLUGIN_ID, "DW-303")
            .with_vendor("DJW Audio")
            .with_version(env!("CARGO_PKG_VERSION"))
            .with_description("Circuit-modelled bass line synthesizer")
            .with_features([INSTRUMENT, SYNTHESIZER, MONO])
    }
    fn new_shared(_: HostSharedHandle<'_>) -> Result<Dw303Shared, PluginError> {
        Ok(Dw303Shared {
            values: params::DESCRIPTORS.map(|d| AtomicU32::new(d.default.to_bits())),
            generation: AtomicU32::new(0),
            fault: AtomicU32::new(0),
            parameter_refresh: AtomicBool::new(false),
        })
    }
    fn new_main_thread<'a>(
        host: HostMainThreadHandle<'a>,
        shared: &'a Dw303Shared,
    ) -> Result<Dw303Main<'a>, PluginError> {
        Ok(Dw303Main { host, shared })
    }
}

/// Parameter values shared between threads. A state restore bumps
/// `generation`; the audio thread then re-sends every value to the engine.
pub struct Dw303Shared {
    values: [AtomicU32; params::COUNT],
    generation: AtomicU32,
    fault: AtomicU32,
    parameter_refresh: AtomicBool,
}

impl PluginShared<'_> for Dw303Shared {}

impl Dw303Shared {
    fn value(&self, index: usize) -> f32 {
        f32::from_bits(self.values[index].load(Ordering::Acquire))
    }
    fn store(&self, index: usize, value: f32) -> bool {
        self.values[index].swap(value.to_bits(), Ordering::AcqRel) != value.to_bits()
    }
    pub fn fault_code(&self) -> u32 {
        self.fault.load(Ordering::Relaxed)
    }
}

fn index_of(id: ClapId) -> Option<usize> {
    params::DESCRIPTORS
        .iter()
        .position(|d| d.id == ParameterId(id.get()))
}

pub struct Dw303Main<'a> {
    host: HostMainThreadHandle<'a>,
    shared: &'a Dw303Shared,
}

impl<'a> PluginMainThread<'a, Dw303Shared> for Dw303Main<'a> {
    fn on_main_thread(&self) {
        if self.shared.parameter_refresh.swap(false, Ordering::AcqRel)
            && let Some(params) = self.host.get_extension::<HostParams>()
        {
            params.rescan(&self.host, ParamRescanFlags::VALUES);
        }
    }
}

impl PluginAudioPortsImpl for Dw303Main<'_> {
    fn count(&self, input: bool) -> u32 {
        u32::from(!input)
    }
    fn get(&self, index: u32, input: bool, writer: &mut AudioPortInfoWriter) {
        if !input && index == 0 {
            writer.set(&AudioPortInfo {
                id: ClapId::new(0),
                name: b"Output",
                channel_count: 2,
                flags: AudioPortFlags::IS_MAIN,
                port_type: Some(AudioPortType::STEREO),
                in_place_pair: None,
            });
        }
    }
}

impl PluginNotePortsImpl for Dw303Main<'_> {
    fn count(&self, input: bool) -> u32 {
        u32::from(input)
    }
    fn get(&self, index: u32, input: bool, writer: &mut NotePortInfoWriter) {
        if input && index == 0 {
            writer.set(&NotePortInfo {
                id: ClapId::new(0),
                name: b"MIDI input",
                supported_dialects: NoteDialects::MIDI,
                preferred_dialect: Some(NoteDialect::Midi),
            });
        }
    }
}

impl PluginMainThreadParams for Dw303Main<'_> {
    fn count(&self) -> u32 {
        params::COUNT as u32
    }
    fn get_info(&self, index: u32, writer: &mut ParamInfoWriter) {
        let Some(d) = params::DESCRIPTORS.get(index as usize) else {
            return;
        };
        let mut flags = ParamInfoFlags::empty();
        if d.automatable {
            flags |= ParamInfoFlags::IS_AUTOMATABLE;
        }
        if d.range.intervals().is_some() {
            flags |= ParamInfoFlags::IS_STEPPED;
        }
        writer.set(&ParamInfo {
            id: ClapId::new(d.id.0),
            flags,
            cookie: Cookie::empty(),
            name: d.name.as_bytes(),
            module: b"",
            min_value: f64::from(d.range.minimum()),
            max_value: f64::from(d.range.maximum()),
            default_value: f64::from(d.default),
        });
    }
    fn get_value(&self, id: ClapId) -> Option<f64> {
        index_of(id).map(|i| f64::from(self.shared.value(i)))
    }
    fn value_to_text(
        &self,
        id: ClapId,
        value: f64,
        writer: &mut ParamDisplayWriter,
    ) -> std::fmt::Result {
        if !value.is_finite() {
            return Err(std::fmt::Error);
        }
        match ParameterId(id.get()) {
            params::TEMPO => write!(writer, "{value:.1} BPM"),
            params::WAVEFORM => writer.write_str(if value < 0.5 { "Saw" } else { "Square" }),
            params::SYNC_MODE => writer.write_str(match value.round() as i32 {
                0 => "Internal",
                1 => "Host",
                _ => "MIDI clock",
            }),
            params::QUALITY => writer.write_str(if value < 0.5 { "Economy" } else { "Full" }),
            _ if index_of(id).is_some() => write!(writer, "{:.1}", value * 10.0),
            _ => Err(std::fmt::Error),
        }
    }
    fn text_to_value(&self, id: ClapId, text: &CStr) -> Option<f64> {
        let d = params::DESCRIPTORS[index_of(id)?];
        let text = text.to_str().ok()?.trim();
        let choice = match (d.id, text) {
            (params::WAVEFORM, "Saw")
            | (params::SYNC_MODE, "Internal")
            | (params::QUALITY, "Economy") => Some(0.0),
            (params::WAVEFORM, "Square")
            | (params::SYNC_MODE, "Host")
            | (params::QUALITY, "Full") => Some(1.0),
            (params::SYNC_MODE, "MIDI clock") => Some(2.0),
            _ => None,
        };
        if let Some(choice) = choice {
            return Some(choice);
        }
        let number: f64 = text.trim_end_matches("BPM").trim().parse().ok()?;
        // Knobs display 0–10 like the panel legend.
        let value = if d.unit.is_empty() && d.range.intervals().is_none() {
            number / 10.0
        } else {
            number
        };
        d.range.canonicalize(value as f32).map(f64::from)
    }
    fn flush(&self, input: &InputEvents, _: &mut OutputEvents) {
        for event in input {
            if let Some(event) = event.as_event::<ParamValueEvent>()
                && let Some(id) = event.param_id()
                && let Some(index) = index_of(id)
                && let Some(value) = params::DESCRIPTORS[index]
                    .range
                    .canonicalize(event.value() as f32)
            {
                self.shared.store(index, value);
            }
        }
    }
}

impl PluginStateImpl for Dw303Main<'_> {
    fn save(&self, output: &mut clack_plugin::stream::OutputStream) -> Result<(), PluginError> {
        let schema = params::schema().map_err(|_| PluginError::Message("schema"))?;
        let values = core::array::from_fn(|i| self.shared.value(i));
        let state = schema
            .state(values)
            .map_err(|_| PluginError::Message("invalid parameter state"))?;
        let mut bytes = [0; 16 + params::COUNT * 8];
        state
            .encode(STATE_PRODUCT, STATE_VERSION, &mut bytes)
            .map_err(|_| PluginError::Message("state encode"))?;
        output.write_all(&bytes)?;
        Ok(())
    }
    fn load(&self, input: &mut clack_plugin::stream::InputStream) -> Result<(), PluginError> {
        let mut bytes = Vec::new();
        input.take(64 * 1024).read_to_end(&mut bytes)?;
        let schema = params::schema().map_err(|_| PluginError::Message("schema"))?;
        let state = ParameterState::decode(schema, STATE_PRODUCT, STATE_VERSION, &bytes)
            .map_err(|_| PluginError::Message("invalid DW-303 state"))?;
        for (i, value) in state.values().iter().enumerate() {
            self.shared.store(i, *value);
        }
        self.shared.generation.fetch_add(1, Ordering::AcqRel);
        self.host.request_process();
        if let Some(params) = self.host.get_extension::<HostParams>() {
            params.rescan(&self.host, ParamRescanFlags::VALUES);
        }
        Ok(())
    }
}

pub struct Dw303Audio<'a> {
    host: HostAudioProcessorHandle<'a>,
    shared: &'a Dw303Shared,
    bridge: MainBusBridge<Dw303Engine>,
    generation: u32,
}

impl Dw303Audio<'_> {
    /// After a restore, send every stored value to the engine.
    fn synchronize(&mut self) {
        let generation = self.shared.generation.load(Ordering::Acquire);
        if generation == self.generation {
            return;
        }
        let events: [ParamValueEvent; params::COUNT] = core::array::from_fn(|i| {
            ParamValueEvent::new(
                0,
                ClapId::new(params::DESCRIPTORS[i].id.0),
                Pckn::match_all(),
                f64::from(self.shared.value(i)),
            )
        });
        let result = self.bridge.flush(&InputEvents::from_buffer(&events));
        debug_assert!(result.is_ok());
        self.bridge.reset();
        self.generation = generation;
    }

    /// Publish automation applied on the audio thread back to the host view.
    fn reflect(&self) {
        let values = self.bridge.processor().parameters().values();
        let mut changed = false;
        for (i, value) in values.iter().enumerate() {
            changed |= self.shared.store(i, *value);
        }
        if changed && !self.shared.parameter_refresh.swap(true, Ordering::AcqRel) {
            self.host.request_callback();
        }
    }

    fn record(&self, ok: bool) {
        self.shared.fault.store(u32::from(!ok), Ordering::Relaxed);
    }
}

impl<'a> PluginAudioProcessor<'a, Dw303Shared, Dw303Main<'a>> for Dw303Audio<'a> {
    fn activate(
        host: HostAudioProcessorHandle<'a>,
        _: &Dw303Main<'a>,
        shared: &'a Dw303Shared,
        config: PluginAudioConfiguration,
    ) -> Result<Self, PluginError> {
        let config = ProcessConfig::new(
            config.sample_rate,
            config.max_frames_count as usize,
            MAXIMUM_EVENTS,
            AudioLayout::STEREO_INSTRUMENT,
        )?;
        let bridge = MainBusBridge::prepare(Dw303Engine::new(), config, Some(0))
            .map_err(|_| PluginError::Message("invalid DW-303 activation"))?;
        let mut audio = Self {
            host,
            shared,
            bridge,
            generation: shared.generation.load(Ordering::Acquire).wrapping_sub(1),
        };
        audio.synchronize();
        Ok(audio)
    }
    fn process(
        &mut self,
        process: Process,
        mut audio: Audio,
        events: Events,
    ) -> Result<ProcessStatus, PluginError> {
        self.synchronize();
        let result = self
            .bridge
            .process(&mut audio, events.input, process.transport);
        if result.is_ok() {
            self.reflect();
        }
        self.record(result.is_ok());
        Ok(ProcessStatus::Continue)
    }
    fn reset(&mut self) {
        self.synchronize();
        self.bridge.reset();
    }
    fn deactivate(mut self, _: &Dw303Main<'a>) {
        self.bridge.deactivate();
    }
}

impl PluginAudioProcessorParams for Dw303Audio<'_> {
    fn flush(&mut self, input: &InputEvents, _: &mut OutputEvents) {
        self.synchronize();
        let result = self.bridge.flush(input);
        if result.is_ok() {
            self.reflect();
        }
        self.record(result.is_ok());
    }
}

impl PluginTailImpl for Dw303Audio<'_> {
    fn get(&self) -> TailLength {
        TailLength::Infinite
    }
}

#[allow(unsafe_code)]
mod export {
    clack_plugin::clack_export_entry!(clack_plugin::entry::SinglePluginEntry<super::Dw303Plugin>);
}
