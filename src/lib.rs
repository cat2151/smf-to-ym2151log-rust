//! # smf-to-ym2151log
//!
//! A library for converting Standard MIDI Files (SMF) to YM2151 register write logs.
//!
//! This library provides a 2-pass conversion process:
//! - **Pass A**: Parse MIDI file to intermediate events JSON
//! - **Pass B**: Convert events to YM2151 register write log JSON
//!
//! ## Usage
//!
//! Full implementation will be done in subsequent phases according to IMPLEMENTATION.md.
//! See the main binary for command-line usage.

pub mod error;
pub mod midi;
pub mod ym2151;

// Re-export commonly used types
pub use error::{Error, Result};
