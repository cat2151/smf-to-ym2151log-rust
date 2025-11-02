//! YM2151 note table
//!
//! Contains the note mapping table for YM2151 chip.

/// YM2151 note table (C# to C)
/// Maps note within octave to YM2151 note code
pub const NOTE_TABLE: [u8; 12] = [
    0,  // C#
    1,  // D
    2,  // D#
    4,  // E
    5,  // F
    6,  // F#
    8,  // G
    9,  // G#
    10, // A
    12, // A#
    13, // B
    14, // C
];
