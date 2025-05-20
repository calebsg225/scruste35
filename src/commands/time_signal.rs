//! src/commands/time_signal.rs

use crate::commands::utils::SpliceTime;

/// Table 11 - time_signal()
pub struct TimeSignal {
    splice_time: SpliceTime,
}

impl TimeSignal {
    /// build an empty `time_signal()` instance
    pub fn new() -> Self {
        Self {
            splice_time: SpliceTime::new(),
        }
    }

    /// decode time_signal from byte array `bytes`
    pub fn from(bytes: &[u8]) -> Self {
        let mut ts = TimeSignal::new();
        ts.decode(bytes);
        ts
    }

    /// decode `time_signal()` from byte array `bytes`
    pub fn decode(&mut self, bytes: &[u8]) {
        (self.splice_time, _) = SpliceTime::from(bytes);
    }
}
