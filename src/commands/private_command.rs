//! src/commands/private_command.rs

use crate::bitbin::BitRead;

pub struct PrivateCommand {}

impl PrivateCommand {
    pub fn from(bread: &mut BitRead) -> Self {
        Self {}
    }
}
