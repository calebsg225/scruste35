//! src/commands/utils/splice_time.rs

use crate::bitbin::BitRead;

/// Table 14 - splice_time()
pub struct SpliceTime {
    time_specified_flag: bool,
    pts_time: Option<f64>,
}

impl SpliceTime {
    /// build an empty `splice_time()` instance
    pub fn new() -> Self {
        Self {
            time_specified_flag: false,
            pts_time: None,
        }
    }

    /// build a `splice_time()` instance from byte array `bytes`
    pub fn from(bytes: &[u8]) -> (Self, Vec<u8>) {
        let mut st = SpliceTime::new();
        let remaining_bytes = st.decode(bytes);
        (st, remaining_bytes)
    }

    /// decode `splice_time()` from byte array `bytes`
    pub fn decode(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut bread = BitRead::from(bytes);
        self.time_specified_flag = bread.as_flag();
        if self.time_specified_flag {
            bread.forward(6);
            self.pts_time = Some(bread.as_90k(33));
        } else {
            bread.forward(7);
        };
        bread.as_bytes(bread.get_idx())
    }
}
