//! # smf-to-ym2151log
//!
//! A library for converting Standard MIDI Files (SMF) to YM2151 register write logs.
//!
//! This library provides a 2-pass conversion process:
//! - **Pass A**: Parse MIDI file to intermediate events JSON (intermediate representation)
//! - **Pass B**: Convert events to YM2151 register write log JSON (final output)
//!
//! ## Features
//!
//! - Parse SMF Format 0 and Format 1 files
//! - Convert MIDI notes to YM2151 KC/KF values
//! - Handle tempo changes
//! - Output JSON format compatible with [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc)
//! - Sample rate: 55930 Hz (YM2151 standard)
//!
//! ## Example
//!
//! ```no_run
//! use smf_to_ym2151log::midi::parse_midi_file;
//! use smf_to_ym2151log::ym2151::convert_to_ym2151_log;
//!
//! // Parse MIDI file (Pass A)
//! let midi_data = parse_midi_file("song.mid").unwrap();
//!
//! // Convert to YM2151 log (Pass B)
//! let ym2151_log = convert_to_ym2151_log(&midi_data).unwrap();
//!
//! println!("Generated {} YM2151 events", ym2151_log.event_count);
//! ```
//!
//! ## Modules
//!
//! - [`midi`] - MIDI file parsing and event handling (Pass A)
//! - [`ym2151`] - YM2151 conversion and register mapping (Pass B)
//! - [`error`] - Error types and result handling

pub mod error;
pub mod midi;
pub mod ym2151;

// Re-export commonly used types
pub use error::{Error, Result};

/// Convert Standard MIDI File data to YM2151 register log JSON
///
/// This is a convenience function that combines both passes:
/// - Pass A: Parse MIDI data from bytes
/// - Pass B: Convert to YM2151 register log
///
/// # Arguments
/// * `smf_data` - Raw Standard MIDI File data as bytes
///
/// # Returns
/// YM2151 register log as JSON string
///
/// # Errors
/// Returns an error if parsing or conversion fails
///
/// # Example
/// ```no_run
/// use smf_to_ym2151log::convert_smf_to_ym2151_log;
///
/// let smf_bytes = std::fs::read("song.mid").unwrap();
/// let ym2151_json = convert_smf_to_ym2151_log(&smf_bytes).unwrap();
/// println!("{}", ym2151_json);
/// ```
pub fn convert_smf_to_ym2151_log(smf_data: &[u8]) -> Result<String> {
    // Pass A: Parse MIDI data from bytes
    let midi_data = midi::parse_midi_from_bytes(smf_data)?;

    // Pass B: Convert to YM2151 log
    let ym2151_log = ym2151::convert_to_ym2151_log(&midi_data)?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&ym2151_log)?;

    Ok(json)
}
