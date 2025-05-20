//! src/commands/splice_insert.rs

use crate::commands::utils::SpliceEvent;

/// Table 10 - splice_insert()
pub struct SpliceInsert {
    splice_event: SpliceEvent,
}

impl SpliceInsert {
    /// build an empty `splice_insert()` instance
    pub fn new() -> Self {
        Self {
            splice_event: SpliceEvent::new(),
        }
    }

    /// build a `splice_insert()` from byte array `bytes`
    pub fn from(bytes: &[u8]) -> Self {
        let mut si = SpliceInsert::new();
        si.decode(bytes);
        si
    }

    /// decode `splice_insert()` from byte array `bytes`
    pub fn decode(&mut self, bytes: &[u8]) {
        (self.splice_event, _) = SpliceEvent::from_insert(bytes);
    }
}
