//! src/commands/time_signal.rs

use crate::bitbin::BitRead;

use crate::commands::utils::SpliceTime;

/// Table 11 - time_signal()
pub struct TimeSignal {
    splice_time: SpliceTime,
}

impl TimeSignal {
    pub fn from(bytes: &[u8]) -> Self {
        let (splice_time, _) = SpliceTime::from(BitRead::from(bytes));
        Self { splice_time }
    }
}
