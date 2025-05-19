//! src/commands/private_command.rs

use crate::bitbin::BitRead;

/// Table 13 - private_command()
pub struct PrivateCommand {
    identifier: u32,
    private_bytes: Vec<u8>,
}

impl PrivateCommand {
    pub fn from(bytes: &[u8]) -> Self {
        let mut bread = BitRead::from(bytes);
        Self {
            identifier: bread.as_int(32) as u32,
            private_bytes: bread.as_bytes(bread.get_idx()),
        }
    }
}
