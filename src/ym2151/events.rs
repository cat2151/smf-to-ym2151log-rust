//! YM2151 event structures

use serde::{Deserialize, Serialize};

/// Represents a YM2151 register write event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ym2151Event {
    /// Sample time at 55930 Hz
    pub time: u32,
    /// Register address (hex string format, e.g., "0x08")
    pub addr: String,
    /// Data to write (hex string format, e.g., "0x4E")
    pub data: String,
}

/// YM2151 log container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ym2151Log {
    /// Number of events
    pub event_count: usize,
    /// List of YM2151 register write events
    pub events: Vec<Ym2151Event>,
}
