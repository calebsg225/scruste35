//! src/commands/private_command.rs

use crate::bitbin::BitRead;

/// Table 13 - private_command()
pub struct PrivateCommand {}

impl PrivateCommand {
    pub fn from(bytes: &[u8]) -> Self {
        Self {}
    }
}
