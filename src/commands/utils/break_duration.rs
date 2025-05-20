//! src/commands/utils/break_duration.rs

use crate::bitbin::BitRead;

/// Table 15 - break_duration()
pub struct BreakDuration {
    auto_return: bool,
    duration: u64,
}

impl BreakDuration {
    /// build an empty `break_duration()` instance
    pub fn new() -> Self {
        Self {
            auto_return: false,
            duration: 0,
        }
    }

    /// build a `break_duration()` instance from byte array `bytes`
    pub fn from(bytes: &[u8]) -> (Self, Vec<u8>) {
        let mut bd = BreakDuration::new();
        let remaining_bytes = bd.decode(bytes);
        (bd, remaining_bytes)
    }

    /// decode `break_duration()` from byte array `bytes`
    pub fn decode(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut bread = BitRead::from(bytes);
        self.auto_return = bread.as_flag();
        bread.forward(6); // 6 bits reserved
        self.duration = bread.as_int(33);
        bread.as_bytes(bread.get_idx())
    }
}
