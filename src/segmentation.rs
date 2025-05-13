//! /src/segmentation.rs
//!
//! SCTE35 Segmentation Descriptor Tables

#![allow(unused)]

/// Table 21 - device_restrictions
pub fn device_restrictions(id: u8) -> String {
    match id {
        0x00 => "Restrict Group 0",
        0x01 => "Restrict Group 1",
        0x02 => "Restrict Group 2",
        0x03 => "None",
        _ => panic!("Invalid Device Restrictions Id: {id}"),
    }
    .to_string()
}

/// Table 23 - segmentation_type_id
pub fn segmentation_type_id(id: u8) -> String {
    match id {
        0x00 => "Not Indicated",
        0x01 => "Content Identification",
        // 0x02 Private
        // 0x03-0x0F Reserved
        0x10 => "Program Start",
        0x11 => "Program End",
        0x12 => "Program Early Termination",
        0x13 => "Program Breakaway",
        0x14 => "Program Resumption",
        0x15 => "Program Runover Planned",
        0x16 => "Program Runover Unplanned",
        0x17 => "Program Overlap Start",
        0x18 => "Program Blackout Override",
        0x19 => "Program Join",
        0x1A => "Program Immediate Resumption",
        // 0x1B-0x1F Reserved
        0x20 => "Chapter Start",
        0x21 => "Chapter End",
        0x22 => "Break Start",
        0x23 => "Break End",
        // 0x24 => "Opening Credit Start", // Depreciated
        // 0x25 => "Opening Credit End", // Depreciated
        // 0x26 => "Closing Credit Start", // Depreciated
        // 0x27 => "Closing Credit End", // Depreciated
        // 0x28-0x2F Reserved
        0x30 => "Provider Advertisement Start",
        0x31 => "Provider Advertisement End",
        0x32 => "Distributor Advertisement Start",
        0x33 => "Distributor Advertisement End",
        0x34 => "Provider Placement Opportunity Start",
        0x35 => "Provider Placement Opportunity End",
        0x36 => "Distributor Placement Opportunity Start",
        0x37 => "Distributor Placement Opportunity End",
        0x38 => "Provider Overlay Placement Opportunity Start",
        0x39 => "Provider Overlay Placement Opportunity End",
        0x3A => "Distributor Overlay Placement Opportunity Start",
        0x3B => "Distributor Overlay Placement Opportunity End",
        0x3C => "Provider Promo Start",
        0x3D => "Provider Promo End",
        0x3E => "Distributor Promo Start",
        0x3F => "Distributor Promo End",
        0x40 => "Unscheduled Event Start",
        0x41 => "Unscheduled Event End",
        0x42 => "Alternate Content Opportunity Start",
        0x43 => "Alternate Content Opportunity End",
        0x44 => "Provider Ad Block Start",
        0x45 => "Provider Ad Block End",
        0x46 => "Distributor Ad Block End",
        0x47 => "Distributor Ad Block End",
        // 0x48-0x4F Reserved
        0x50 => "Network Start",
        0x51 => "Network End",
        // 0x52-0xFF Reserved
        _ => panic!("Invalid, Reserved, or Depreciated Segmentation Type: {id}"),
    }
    .to_string()
}
