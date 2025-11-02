//! Error types for smf-to-ym2151log library

use thiserror::Error;

/// The main error type for this library
#[derive(Error, Debug)]
pub enum Error {
    /// IO error when reading/writing files
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error parsing MIDI file
    #[error("MIDI parsing error: {0}")]
    MidiParse(String),

    /// Error serializing/deserializing JSON
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Invalid MIDI note or parameter
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// Other errors
    #[error("Error: {0}")]
    Other(String),
}

/// A specialized Result type for this library
pub type Result<T> = std::result::Result<T, Error>;
