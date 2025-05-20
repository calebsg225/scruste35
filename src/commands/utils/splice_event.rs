//! src/commands/utils/splice_event.rs

use crate::bitbin::BitRead;
use crate::commands::utils::BreakDuration;
use crate::commands::utils::SpliceTime;

pub struct SpliceEvent {
    splice_event_id: u32,
    splice_event_cancel_indicator: bool,
    event_id_compliance_flag: Option<bool>,
    out_of_network_indicator: Option<bool>,
    program_splice_flag: Option<bool>,
    duration_flag: Option<bool>,
    splice_immediate_flag: Option<bool>,
    utc_splice_time: Option<u32>,
    splice_time: Option<SpliceTime>,
    break_duration: Option<BreakDuration>,
    unique_program_id: Option<u16>,
    avail_num: Option<u8>,
    avails_expected: Option<u8>,
}

impl SpliceEvent {
    /// builds an empty 'splice_event()` instance
    pub fn new() -> Self {
        Self {
            splice_event_id: 0,
            splice_event_cancel_indicator: false,
            event_id_compliance_flag: None,
            out_of_network_indicator: None,
            program_splice_flag: None,
            duration_flag: None,
            splice_immediate_flag: None,
            utc_splice_time: None,
            splice_time: None,
            break_duration: None,
            unique_program_id: None,
            avail_num: None,
            avails_expected: None,
        }
    }

    /// build a `splice_event()` instance in `splice_insert()` from `bytes`
    pub fn from_insert(mut bytes: &[u8]) -> (Self, Vec<u8>) {
        let mut sei = SpliceEvent::new();
        let remaining_bytes = sei.decode_insert(bytes);
        (sei, remaining_bytes)
    }

    /// build a `splice_event()` instance in `splice_schedule()` from `bytes`
    pub fn from_schedule(mut bytes: &[u8]) -> (Self, Vec<u8>) {
        let mut ses = SpliceEvent::new();
        let remaining_bytes = ses.decode_schedule(bytes);
        (ses, remaining_bytes)
    }

    /// decode a `splice_event()` in `splice_insert()` from `bytes`
    pub fn decode_insert(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut bread = BitRead::from(bytes);
        self.splice_event_id = bread.as_int(32) as u32;
        self.splice_event_cancel_indicator = bread.as_flag();
        bread.forward(7); // 7 bits reserved
        if self.splice_event_cancel_indicator {
            self.out_of_network_indicator = Some(bread.as_flag());
            let program_splice_flag = bread.as_flag();
            self.program_splice_flag = Some(program_splice_flag);
            self.duration_flag = Some(bread.as_flag());
            let splice_immediate_flag = bread.as_flag();
            self.splice_immediate_flag = Some(splice_immediate_flag);
            self.event_id_compliance_flag = Some(bread.as_flag());
            bread.forward(3); // 3 bits reserved
            if program_splice_flag && !splice_immediate_flag {
                let (splice_time, remaining_bytes) =
                    SpliceTime::from(&bread.as_bytes(bread.get_idx()));
                self.splice_time = Some(splice_time);
                bread = BitRead::from(&remaining_bytes);
            }
            if let Some(true) = self.duration_flag {
                let (break_duration, remaining_bytes) =
                    BreakDuration::from(&bread.as_bytes(bread.get_idx()));
                self.break_duration = Some(break_duration);
                bread = BitRead::from(&remaining_bytes);
            }
            self.unique_program_id = Some(bread.as_int(16) as u16);
            self.avail_num = Some(bread.as_int(8) as u8);
            self.avails_expected = Some(bread.as_int(8) as u8);
        };
        bread.as_bytes(bread.get_idx())
    }

    pub fn decode_schedule(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut bread = BitRead::from(bytes);
        self.splice_event_id = bread.as_int(32) as u32;
        self.splice_event_cancel_indicator = bread.as_flag();
        self.event_id_compliance_flag = Some(bread.as_flag());
        bread.forward(6); // 6 bits reserved
        if !self.splice_event_cancel_indicator {
            self.out_of_network_indicator = Some(bread.as_flag());
            self.program_splice_flag = Some(bread.as_flag());
            self.duration_flag = Some(bread.as_flag());
            bread.forward(5); // 5 bits reserved
            if let Some(true) = self.program_splice_flag {
                self.utc_splice_time = Some(bread.as_int(32) as u32);
            }
            if let Some(true) = self.duration_flag {
                let (break_duration, remaining_bytes) =
                    BreakDuration::from(&bread.as_bytes(bread.get_idx()));
                self.break_duration = Some(break_duration);
                bread = BitRead::from(&remaining_bytes);
            }
            self.unique_program_id = Some(bread.as_int(16) as u16);
            self.avail_num = Some(bread.as_int(8) as u8);
            self.avails_expected = Some(bread.as_int(8) as u8);
        };
        bread.as_bytes(bread.get_idx())
    }
}
