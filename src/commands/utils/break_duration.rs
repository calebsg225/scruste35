//! src/commands/utils/break_duration.rs

use crate::bitbin::BitRead;

/// Table 15 - break_duration()
pub struct BreakDuration {
    auto_return: bool,
    duration: u64,
}

impl BreakDuration {
    pub fn from(mut bread: BitRead) -> (Self, usize) {
        let start = bread.get_idx();
        (
            Self {
                auto_return: bread.as_flag(),
                duration: {
                    bread.forward(6); // 6 bits reserved
                    bread.as_int(33)
                },
            },
            start - bread.get_idx(),
        )
    }
}
