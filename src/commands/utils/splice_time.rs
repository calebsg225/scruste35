//! src/commands/utils/splice_time.rs

use crate::bitbin::BitRead;

/// Table 14 - splice_time()
pub struct SpliceTime {
    time_specified_flag: bool,
    pts_time: Option<f64>,
}

impl SpliceTime {
    /// decode splice_time from parent bitreader `bread`
    pub fn from(mut bread: BitRead) -> (Self, usize) {
        let start = bread.get_idx();
        let time_specified_flag = bread.as_flag();
        let mut pts_time = None;
        if time_specified_flag {
            bread.forward(6);
            pts_time = Some(bread.as_90k(33));
        } else {
            bread.forward(7);
        }
        (
            Self {
                time_specified_flag,
                pts_time,
            },
            start - bread.get_idx(),
        )
    }
}
