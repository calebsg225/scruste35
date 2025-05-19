//! src/commands/splice_schedule.rs

use crate::bitbin::BitRead;
use crate::commands::utils::SpliceEvent;

/// Table 9 - splice_schedule()
pub struct SpliceSchedule {
    splice_count: u8,
    splice_events: Vec<SpliceEvent>,
}

impl SpliceSchedule {
    pub fn from(bytes: &[u8]) -> Self {
        let mut bread = BitRead::from(bytes);
        let splice_count = bread.as_int(8) as u8;
        let mut splice_events: Vec<SpliceEvent> = Vec::new();

        for _ in 0..splice_count {
            let splice_event: SpliceEvent;
            // NOTE: currently passing same instance of `bread` to and from each `SpliceEvent`.
            // TODO: find a better way? keep track of the bits decoded instead?
            (splice_event, bread) = SpliceEvent::from_schedule(bread);
            splice_events.push(splice_event);
        }

        Self {
            splice_count,
            splice_events,
        }
    }
}
