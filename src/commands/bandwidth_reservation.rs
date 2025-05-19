//! src/commands/bandwidth_reservation.rs

use crate::bitbin::BitRead;

/// Table 12 - bandwidth_reservation()
pub struct BandwidthReservation {}

impl BandwidthReservation {
    /// decode bandwidth_reservation from byte array `bytes`
    pub fn from(bytes: &[u8]) -> Self {
        Self {}
    }
}
