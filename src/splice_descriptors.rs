//! src/splice_descriptors.rs

use crate::descriptors;

/// Table 16 - Splice Descriptor Tags
pub enum SpliceDescriptor {
    AvailDescriptor(),
    DTMFDescriptor(),
    SegmentationDescriptor(),
    TimeDescriptor(),
    AudioDescriptor(),
    Reserved,
}

impl SpliceDescriptor {
    /// decode a splice_descriptor specified by `id` from byte array `bytes`
    pub fn from(id: u8, bytes: &[u8]) -> Self {
        match id {
            0x00 => SpliceDescriptor::AvailDescriptor(),
            0x01 => SpliceDescriptor::DTMFDescriptor(),
            0x02 => SpliceDescriptor::SegmentationDescriptor(),
            0x03 => SpliceDescriptor::TimeDescriptor(),
            0x04 => SpliceDescriptor::AudioDescriptor(),
            _ => SpliceDescriptor::Reserved,
        }
    }
}
