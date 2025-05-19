//! src/commands.rs

use crate::commands;

/// Table 7 - splice-command-type values
pub enum SpliceCommand {
    SpliceNull(commands::SpliceNull),
    SpliceSchedule(commands::SpliceSchedule),
    SpliceInsert(commands::SpliceInsert),
    TimeSignal(commands::TimeSignal),
    BandwidthReservation(commands::BandwidthReservation),
    PrivateCommand(commands::PrivateCommand),
    Reserved,
}

impl SpliceCommand {
    /// decode a splice_command specified by `id` from byte array `bytes`
    pub fn from(id: u8, bytes: &[u8]) -> Self {
        match id {
            0x00 => SpliceCommand::SpliceNull(commands::SpliceNull::new()),
            0x04 => SpliceCommand::SpliceSchedule(commands::SpliceSchedule::from(bytes)),
            0x05 => SpliceCommand::SpliceInsert(commands::SpliceInsert::from(bytes)),
            0x06 => SpliceCommand::TimeSignal(commands::TimeSignal::from(bytes)),
            0x07 => {
                SpliceCommand::BandwidthReservation(commands::BandwidthReservation::from(bytes))
            }
            0xff => SpliceCommand::PrivateCommand(commands::PrivateCommand::from(bytes)),
            _ => SpliceCommand::Reserved,
        }
    }
}
