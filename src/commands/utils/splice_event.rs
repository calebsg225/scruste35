//! src/commands/command_utils/splice_event.rs

use crate::bitbin::BitRead;

pub struct SpliceEvent {
    splice_event_id: u32,
    splice_event_cancel_indicator: bool,
    event_id_compliance_flag: bool,
    out_of_network_indicator: Option<bool>,
    program_splice_flag: Option<bool>,
    duration_flag: Option<bool>,
    utc_splice_time: Option<u32>,
    // TODO: implement BreakDuration
    // break_duration: Option<BreakDuration>,
    unique_program_id: Option<u16>,
    avail_num: Option<u8>,
    avails_expected: Option<u8>,
}

impl SpliceEvent {
    /// decode a `splice_event` from a `splice_schedule()` splice command
    pub fn from_schedule(mut bread: BitRead) -> (Self, BitRead) {
        let mut se = Self {
            splice_event_id: bread.as_int(32) as u32,
            splice_event_cancel_indicator: bread.as_flag(),
            event_id_compliance_flag: bread.as_flag(),
            out_of_network_indicator: None,
            program_splice_flag: None,
            duration_flag: None,
            utc_splice_time: None,
            unique_program_id: None,
            avail_num: None,
            avails_expected: None,
        };
        bread.forward(6); // 6 bits reserved
        if !se.splice_event_cancel_indicator {
            se.out_of_network_indicator = Some(bread.as_flag());
            se.program_splice_flag = Some(bread.as_flag());
            se.duration_flag = Some(bread.as_flag());
            bread.forward(5); // 5 bits reserved
            if let Some(true) = se.program_splice_flag {
                se.utc_splice_time = Some(bread.as_int(32) as u32);
            }
            if let Some(true) = se.duration_flag {
                // TODO: implement BreakDuration
                todo!();
            }
            se.unique_program_id = Some(bread.as_int(16) as u16);
            se.avail_num = Some(bread.as_int(8) as u8);
            se.avails_expected = Some(bread.as_int(8) as u8);
        }
        (se, bread)
    }

    /// decode a `splice_event` from a `splice_insert()` splice command
    pub fn from_insert() {}
}
