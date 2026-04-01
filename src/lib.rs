//! # smf-to-ym2151log
//!
//! A library for converting Standard MIDI Files (SMF) to YM2151 register write logs.
//!
//! This library provides a 2-pass conversion process:
//! - **Pass A**: Parse MIDI file to intermediate events JSON (intermediate representation)
//! - **Pass B**: Convert events to YM2151 register write log (final output)
//!
//! ## Features
//!
//! - Parse SMF Format 0 and Format 1 files
//! - Convert MIDI notes to YM2151 KC/KF values
//! - Handle tempo changes
//! - Output JSON format compatible with [ym2151-zig-cc](https://github.com/cat2151/ym2151-zig-cc)
//! - Time values in seconds (f64) for simplicity and WebAudio API compatibility
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

mod api;
mod options;

pub mod error;
pub mod midi;
pub mod ym2151;

#[cfg(feature = "wasm")]
pub mod wasm;

#[doc(inline)]
pub use api::{convert_smf_to_ym2151_log, convert_smf_to_ym2151_log_with_options};
pub use error::{Error, Result};
#[doc(inline)]
pub use options::{
    ConversionOptions, DelayVibratoDefinition, LfoWaveform, PopNoiseEnvelope, ProgramAttachment,
    RegisterLfoDefinition, RegisterOverride,
};
