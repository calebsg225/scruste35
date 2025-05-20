//! src/info_section.rs

use crate::bitbin::BitRead;
use crate::splice_commands::SpliceCommand;

/// Table 6 - sap_type values
pub fn sap_type_values(id: &str) -> String {
    match id {
        "0x00" => "Type 1 Closed GOP with no leading pictures",
        "0x01" => "Type 2 Closed GOP with leading pictures",
        "0x02" => "Type 3 Open GOP",
        "0x03" => "No Sap Type",
        _ => {
            eprintln!("Invalid id: {}.", id);
            ""
        }
    }
    .to_string()
}

/// Table 5 - splice_info_section()
pub struct SpliceInfoSection {
    table_id: String,
    section_syntax_indicator: bool,
    private_indicator: bool,
    sap_type: String,
    sap_description: String,
    section_length: u16,
    protocol_version: u8,
    encrypted_packet: bool,
    encryption_algorithm: u8,
    pts_adjustment: f64,
    cw_index: String,
    tier: String,
    splice_command_length: u16,
    splice_command_type: u8,
    descriptor_loop_length: u16,
    // splice_descriptor: SpliceDescriptor,
    // alignment_stuffing: u8, // not u8...
    e_crc_32: Option<u32>,
    crc_32: u32,
}

impl SpliceInfoSection {
    /// build an empty `splice_info_section()` instance
    pub fn new() -> Self {
        Self {
            table_id: String::new(),
            section_syntax_indicator: false,
            private_indicator: false,
            sap_type: String::new(),
            sap_description: String::new(),
            section_length: 0,
            protocol_version: 0,
            encrypted_packet: false,
            encryption_algorithm: 0,
            pts_adjustment: 0.,
            cw_index: String::new(),
            tier: String::new(),
            splice_command_length: 0,
            splice_command_type: 0,
            descriptor_loop_length: 0,
            e_crc_32: None,
            crc_32: 0,
        }
    }

    /// build a `splice_info_section()` instance from byte array `bytes`
    pub fn from(bytes: &[u8]) -> (Self, SpliceCommand) {
        let mut sis = SpliceInfoSection::new();
        let splice_command = sis.decode(bytes);
        (sis, splice_command)
    }

    /// decode `splice_info_section()` from byte array `bytes`
    pub fn decode(&mut self, bytes: &[u8]) -> SpliceCommand {
        let mut bread = BitRead::from(bytes);
        self.table_id = bread.as_hex(8);
        self.section_syntax_indicator = bread.as_flag();
        self.private_indicator = bread.as_flag();
        self.sap_type = bread.as_hex(2);
        self.sap_description = sap_type_values(&self.sap_type);
        self.section_length = bread.as_int(12) as u16;
        self.protocol_version = bread.as_int(8) as u8;
        self.encrypted_packet = bread.as_flag();
        self.encryption_algorithm = bread.as_int(6) as u8;
        self.pts_adjustment = bread.as_90k(33);
        self.cw_index = bread.as_hex(8);
        self.tier = bread.as_hex(12);
        self.splice_command_length = bread.as_int(12) as u16;
        self.splice_command_type = bread.as_int(8) as u8;
        let splice_command = SpliceCommand::from(
            self.splice_command_type,
            &bread.as_bytes((self.splice_command_length << 3) as usize),
        );
        self.descriptor_loop_length = bread.as_int(16) as u16;
        // TODO: SpliceDescriptor(s)
        // needs to be in order
        // TODO: alignment_stuffing
        // needs to be in order
        if self.encrypted_packet {
            self.e_crc_32 = Some(bread.as_int(32) as u32);
        }
        self.crc_32 = bread.as_int(32) as u32;
        splice_command
    }
}
