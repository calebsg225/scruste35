//! src/commands/private_command.rs

use crate::bitbin::BitRead;

/// Table 13 - private_command()
pub struct PrivateCommand {
    identifier: u32,
    private_bytes: Vec<u8>,
}

impl PrivateCommand {
    /// build an empty `private_command` instance
    pub fn new() -> Self {
        Self {
            identifier: 0,
            private_bytes: Vec::new(),
        }
    }
    /// build a `private_command()` from byte array `bytes`
    pub fn from(bytes: &[u8]) -> Self {
        let mut pc = PrivateCommand::new();
        pc.decode(bytes);
        pc
    }

    /// decode `private_command()` from bytes array `bytes`
    pub fn decode(&mut self, bytes: &[u8]) {
        let mut bread = BitRead::from(bytes);
        self.identifier = bread.as_int(32) as u32;
        self.private_bytes = bread.as_bytes(bread.get_idx());
    }
}
