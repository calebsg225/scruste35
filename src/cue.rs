//! src/cue.rs

use crate::info_section::SpliceInfoSection;
use crate::splice_commands::SpliceCommand;
use crate::splice_descriptors::SpliceDescriptor;

pub enum SCTE35Data {}

pub struct Cue {
    command: SpliceCommand,
    info_section: SpliceInfoSection,
    descriptors: Vec<SpliceDescriptor>,
}

impl Cue {
    /// initialize an empty `Cue` instance
    pub fn new() -> Self {
        Self {
            command: SpliceCommand::SpliceNull,
            info_section: SpliceInfoSection::new(),
            descriptors: Vec::new(),
        }
    }

    /// build a `Cue` instance from data
    pub fn from_data(data: SCTE35Data) -> Self {
        let mut cue = Cue::new();
        cue
    }
}
