//! MIDI event structures

use serde::{Deserialize, Serialize};

/// Represents a parsed MIDI event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MidiEvent {
    /// Note On event
    NoteOn {
        ticks: u32,
        channel: u8,
        note: u8,
        velocity: u8,
    },
    /// Note Off event
    NoteOff { ticks: u32, channel: u8, note: u8 },
    /// Tempo change event
    Tempo { ticks: u32, tempo_bpm: f64 },
    /// Program change event (for future use)
    ProgramChange {
        ticks: u32,
        channel: u8,
        program: u8,
    },
}

/// Parsed MIDI data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiData {
    /// Ticks per quarter note
    pub ticks_per_beat: u16,
    /// Initial tempo in BPM
    pub tempo_bpm: f64,
    /// List of MIDI events
    pub events: Vec<MidiEvent>,
}
