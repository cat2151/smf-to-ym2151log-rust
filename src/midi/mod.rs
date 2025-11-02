//! MIDI processing module
//!
//! This module handles MIDI file parsing and event extraction (Pass A).

pub mod events;
pub mod parser;
pub mod utils;

pub use events::*;
pub use parser::*;
pub use utils::*;
