//! src/commands/splice_insert.rs

use crate::bitbin::BitRead;
use crate::commands::utils::SpliceEvent;

/// Table 10 - splice_insert()
pub struct SpliceInsert {
    splice_event: SpliceEvent,
}

impl SpliceInsert {
    pub fn from(bytes: &[u8]) -> Self {
        let (splice_event, _) = SpliceEvent::from_insert(BitRead::from(bytes));
        Self { splice_event }
    }
}
