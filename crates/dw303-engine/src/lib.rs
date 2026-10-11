//! The DW-303 processor shared by every plug-in format and the standalone app.
//!
//! Phase 1 scaffold: parameters are received and stored; the circuit voice and
//! sequencer are integrated once `dw303-voice` and `dw303-sequencer` land.
#![no_std]

pub mod params;

use ravet_foundation::{
    EventData, ParameterState, PrepareError, ProcessBlock, ProcessConfig, ProcessError,
    ProcessStatus, Processor, Tail,
};

pub struct Dw303Engine {
    config: Option<ProcessConfig>,
    params: ParameterState<'static, { params::COUNT }>,
}

impl Dw303Engine {
    pub fn new() -> Self {
        Self {
            config: None,
            params: params::schema().expect("valid schema").defaults(),
        }
    }

    pub fn parameters(&self) -> &ParameterState<'static, { params::COUNT }> {
        &self.params
    }

    fn apply(&mut self, data: EventData) {
        if let EventData::ParameterValue { id, value } = data
            && let Some(index) = params::schema().ok().and_then(|s| s.index(id))
        {
            let range = params::DESCRIPTORS[index].range;
            if let Some(value) = range.canonicalize(value) {
                let _ = self.params.set(id, value);
            }
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
        for event in block.events {
            self.apply(event.data);
        }
        block.audio.clear_outputs();
        ProcessStatus::Continue
    }

    fn reset(&mut self) {}

    fn deactivate(&mut self) {
        self.config = None;
    }

    fn tail(&self) -> Tail {
        Tail::Infinite
    }
}
