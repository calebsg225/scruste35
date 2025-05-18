//! src/commands.rs

use crate::{bitbin::BitRead, commands};

pub enum SpliceCommand {
    SpliceNull(commands::SpliceNull),
    SpliceSchedule(commands::SpliceSchedule),
    SpliceInsert(commands::SpliceInsert),
    TimeSignal(commands::TimeSignal),
    BandwidthReservation(commands::BandwidthReservation),
    PrivateCommand,
    Reserved,
}

impl SpliceCommand {
    pub fn from(id: u8, &mut bread: BitRead) -> (Self, BitRead) {
        (
            match id {
                0x00 => SpliceCommand::SpliceNull(commands::SpliceNull::new()),
                0x04 => SpliceCommand::SpliceSchedule(commands::SpliceSchedule::from(bread)),
                0x05 => SpliceCommand::SpliceInsert(commands::SpliceInsert::from(bread)),
                0x06 => SpliceCommand::TimeSignal(commands::TimeSignal::from(bread)),
                0x07 => {
                    SpliceCommand::BandwidthReservation(commands::BandwidthReservation::from(bread))
                }
                0xff => SpliceCommand::PrivateCommand,
                _ => SpliceCommand::Reserved,
            },
            bread,
        )
    }
}
