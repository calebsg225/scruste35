//! src/info_section.rs

use crate::bitbin::BitRead;
use crate::splice_commands::SpliceCommand;

pub fn sap_type_values(id: u8) -> String {
    match id {
        0x00 => "Type 1 Closed GOP with no leading pictures",
        0x01 => "Type 2 Closed GOP with leading pictures",
        0x02 => "Type 3 Open GOP",
        0x03 => "No Sap Type",
        _ => {
            eprintln!("Invalid id: {}.", id);
            ""
        }
    }
    .to_string()
}

pub struct SpliceInfoSection {
    table_id: String,
    section_syntax_indicator: bool,
    private_indicator: bool,
    sap_type: String,
    section_length: u16,
    protocol_version: u8,
    encrypted_packet: bool,
    encryption_algorithm: u8,
    pts_adjustment: f64,
    cw_index: String,
    tier: String,
    splice_command_length: u16,
    splice_command_type: u8,
    splice_command: SpliceCommand,
    descriptor_loop_length: u16,
    // splice_descriptor: SpliceDescriptor,
    // alignment_stuffing: u8,
    e_crc_32: Option<u32>,
    crc_32: u32,
}

impl SpliceInfoSection {
    pub fn from(bytes: &[u8]) -> Self {
        let mut bread = BitRead::from(bytes);
        let mut sis = Self {
            table_id: bread.as_hex(8),
            section_syntax_indicator: bread.as_flag(),
            private_indicator: bread.as_flag(),
            sap_type: bread.as_hex(2),
            section_length: bread.as_int(12) as u16,
            protocol_version: bread.as_int(8) as u8,
            encrypted_packet: bread.as_flag(),
            encryption_algorithm: bread.as_int(6) as u8,
            pts_adjustment: bread.as_90k(33),
            cw_index: bread.as_hex(8),
            tier: bread.as_hex(12),
            splice_command_length: bread.as_int(12) as u16,
            splice_command_type: bread.as_int(8) as u8,
            splice_command: SpliceCommand::Reserved,
            descriptor_loop_length: 0,
            e_crc_32: None,
            crc_32: 0,
        };
        // TODO: find a better way to handle bread ownership
        (sis.splice_command, bread) = SpliceCommand::from(sis.splice_command_type, bread);
        sis.descriptor_loop_length = bread.as_int(16) as u16;
        // TODO: SpliceDescriptor(s)
        // needs to be in order
        // TODO: alignment_stuffing
        // needs to be in order
        if sis.encrypted_packet {
            sis.e_crc_32 = Some(bread.as_int(32) as u32);
        }
        sis.crc_32 = bread.as_int(32) as u32;
        sis
    }
}
