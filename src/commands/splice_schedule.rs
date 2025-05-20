//! src/commands/splice_schedule.rs

use crate::commands::utils::SpliceEvent;

/// Table 9 - splice_schedule()
pub struct SpliceSchedule {
    splice_count: u8,
    splice_events: Vec<SpliceEvent>,
}

impl SpliceSchedule {
    /// build an empty `splice_schedule()` instance
    pub fn new() -> Self {
        Self {
            splice_count: 0,
            splice_events: Vec::new(),
        }
    }

    /// build a `splice_schedule()` from byte array `bytes`
    pub fn from(bytes: &[u8]) -> Self {
        let mut ss = SpliceSchedule::new();
        ss.decode(bytes);
        ss
    }

    /// decode `splice_schedule()` from byte array `bytes`
    pub fn decode(&mut self, bytes: &[u8]) {
        let mut remaining_bytes = bytes.to_vec();
        for _ in 0..self.splice_count {
            let se = SpliceEvent::from_schedule(&remaining_bytes);
            self.splice_events.push(se.0);
            remaining_bytes = se.1;
        }
    }
}
