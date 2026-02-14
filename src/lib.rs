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

pub mod error;
pub mod midi;
pub mod ym2151;

#[cfg(feature = "wasm")]
pub mod wasm;

// Re-export commonly used types
use crate::ym2151::ToneDefinition;
pub use error::{Error, Result};
use serde::Deserialize;
use std::collections::HashMap;

/// Optional conversion options supplied via attachment JSON
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConversionOptions {
    /// Enable delayed vibrato generation in the YM2151 log output
    #[serde(rename = "DelayVibrato", default)]
    pub delay_vibrato: bool,
    /// Enable portamento glides between consecutive notes
    #[serde(rename = "Portamento", default)]
    pub portamento: bool,
    /// Optional pre-note envelope overrides to reduce pop noise
    #[serde(rename = "PopNoiseEnvelope", default)]
    pub pop_noise_envelope: Option<PopNoiseEnvelope>,
    /// Optional release-rate reset to avoid attack continuation
    #[serde(rename = "AttackContinuationFix", default)]
    pub attack_continuation_fix: Option<AttackContinuationFix>,
    /// Optional software LFO definitions that modulate tone registers
    #[serde(rename = "SoftwareLfo", default)]
    pub software_lfo: Vec<RegisterLfoDefinition>,
    /// Optional YM2151 tone definitions keyed by MIDI program number
    #[serde(rename = "Tones", default)]
    pub tones: HashMap<u8, ToneDefinition>,
    /// Optional per-program conversion settings supplied via attachment JSON
    #[serde(skip, default)]
    pub program_settings: HashMap<u8, ProgramConfig>,
}

/// Defines a software LFO targeting a YM2151 tone register (per channel/operator)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterLfoDefinition {
    /// Base register address (channel 0 / operator base, e.g. "0x60")
    pub base_register: String,
    /// Peak modulation amount applied around the base register value
    #[serde(default)]
    pub depth: f64,
    /// Oscillation rate in Hz
    pub rate_hz: f64,
    /// Delay before the LFO starts after the note-on
    #[serde(default)]
    pub delay_seconds: f64,
    /// Attack time before reaching full depth
    #[serde(default)]
    pub attack_seconds: f64,
    /// Waveform shape
    #[serde(default = "default_lfo_waveform")]
    pub waveform: LfoWaveform,
}

/// Register override applied before a note-on to soften envelope transitions
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterOverride {
    /// Base register address (channel 0 / operator base, e.g. "0xA0")
    pub base_register: String,
    /// Override value written before restoring the base register
    pub value: String,
}

/// Pop-noise mitigation settings applied just before note-on
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PopNoiseEnvelope {
    /// Whether to apply pop-noise mitigation
    #[serde(default)]
    pub enabled: bool,
    /// How far before the note-on to apply the temporary envelope
    #[serde(default = "default_pre_note_offset")]
    pub offset_seconds: f64,
    /// Registers to override temporarily before restoring base values
    #[serde(default)]
    pub registers: Vec<RegisterOverride>,
}

/// Attack continuation guard settings (forces a short release before note-on)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AttackContinuationFix {
    /// Whether to apply attack continuation guard
    #[serde(default)]
    pub enabled: bool,
    /// How far before note-on to send the release-rate override + key-off
    #[serde(default = "default_pre_note_offset")]
    pub offset_seconds: f64,
    /// Release rate value to apply during the forced key-off
    #[serde(deserialize_with = "deserialize_u8_hex_or_dec")]
    pub release_rate: u8,
}

/// Supported software LFO waveforms
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LfoWaveform {
    Triangle,
}

/// Per-program conversion settings from attachment JSON
#[derive(Debug, Clone, Default)]
pub struct ProgramConfig {
    /// Enable delayed vibrato for this program
    pub delay_vibrato: bool,
    /// Enable portamento glides for this program
    pub portamento: bool,
    /// Optional pre-note envelope overrides to reduce pop noise
    pub pop_noise_envelope: Option<PopNoiseEnvelope>,
    /// Optional release-rate reset to avoid attack continuation
    pub attack_continuation_fix: Option<AttackContinuationFix>,
    /// Optional software LFO definitions that modulate tone registers
    pub software_lfo: Vec<RegisterLfoDefinition>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ProgramAttachment {
    #[serde(rename = "ProgramChange")]
    program_change: u8,
    #[serde(default)]
    delay_vibrato: bool,
    #[serde(default)]
    portamento: bool,
    #[serde(default)]
    pop_noise_envelope: Option<PopNoiseEnvelope>,
    #[serde(default)]
    attack_continuation_fix: Option<AttackContinuationFix>,
    #[serde(default)]
    software_lfo: Vec<RegisterLfoDefinition>,
    #[serde(default)]
    tone: Option<ToneDefinition>,
}

fn default_lfo_waveform() -> LfoWaveform {
    LfoWaveform::Triangle
}

fn default_pre_note_offset() -> f64 {
    0.001
}

fn deserialize_u8_hex_or_dec<'de, D>(deserializer: D) -> std::result::Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let value: serde_json::Value = serde::Deserialize::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(n) => n
            .as_u64()
            .and_then(|v| u8::try_from(v).ok())
            .ok_or_else(|| D::Error::custom("expected u8 numeric value")),
        serde_json::Value::String(s) => {
            if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
                u8::from_str_radix(hex, 16).map_err(D::Error::custom)
            } else {
                u8::from_str_radix(&s, 10).map_err(D::Error::custom)
            }
        }
        _ => Err(D::Error::custom("expected number or string")),
    }
}

impl ConversionOptions {
    /// Build conversion options from an optional attachment JSON payload.
    ///
    /// If no payload is provided, or the payload is empty, defaults are used.
    pub fn from_attachment_bytes(attachment_json: Option<&[u8]>) -> Result<Self> {
        match attachment_json {
            Some(bytes) if !bytes.is_empty() => {
                if let Ok(programs) = serde_json::from_slice::<Vec<ProgramAttachment>>(bytes) {
                    return Self::from_program_attachments(programs);
                }
                let options: ConversionOptions = serde_json::from_slice(bytes)?;
                Ok(options)
            }
            _ => Ok(ConversionOptions::default()),
        }
    }

    fn from_program_attachments(programs: Vec<ProgramAttachment>) -> Result<Self> {
        let mut options = ConversionOptions::default();

        for entry in programs {
            if entry.program_change > 127 {
                return Err(Error::MidiParse(
                    "ProgramChange must be in the range 0-127".to_string(),
                ));
            }

            if let Some(tone) = entry.tone {
                options.tones.insert(entry.program_change, tone);
            }

            options.program_settings.insert(
                entry.program_change,
                ProgramConfig {
                    delay_vibrato: entry.delay_vibrato,
                    portamento: entry.portamento,
                    pop_noise_envelope: entry.pop_noise_envelope,
                    attack_continuation_fix: entry.attack_continuation_fix,
                    software_lfo: entry.software_lfo,
                },
            );
        }

        Ok(options)
    }
}

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

/// Convert SMF data to YM2151 register log JSON with optional attachment options
///
/// This variant accepts an optional attachment JSON to control conversion behavior.
pub fn convert_smf_to_ym2151_log_with_options(
    smf_data: &[u8],
    attachment_json: Option<&[u8]>,
) -> Result<String> {
    // Pass A: Parse MIDI data from bytes
    let midi_data = midi::parse_midi_from_bytes(smf_data)?;

    // Parse optional conversion options
    let options = ConversionOptions::from_attachment_bytes(attachment_json)?;

    // Pass B: Convert to YM2151 log with options
    let ym2151_log = ym2151::convert_to_ym2151_log_with_options(&midi_data, &options)?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&ym2151_log)?;

    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_attachment_array_parses_into_options() {
        let attachment = br#"[
  {
    "ProgramChange": 7,
    "DelayVibrato": true,
    "Portamento": true,
    "SoftwareLfo": [
      { "BaseRegister": "0x60", "Depth": 2.0, "RateHz": 3.0, "DelaySeconds": 0.0, "AttackSeconds": 0.0, "Waveform": "triangle" }
    ],
    "Tone": {
      "events": [
        { "time": 0.0, "addr": "0x20", "data": "0xAA" }
      ]
    }
  }
]"#;

        let options = ConversionOptions::from_attachment_bytes(Some(attachment)).unwrap();
        let program_cfg = options
            .program_settings
            .get(&7)
            .expect("program settings should include program 7");

        assert!(program_cfg.delay_vibrato);
        assert!(program_cfg.portamento);
        assert_eq!(program_cfg.software_lfo.len(), 1);

        let tone = options
            .tones
            .get(&7)
            .expect("tone definition should be stored for program 7");
        assert_eq!(tone.events.len(), 1);
        assert_eq!(tone.events[0].data, "0xAA");

        // Global options remain unset when supplied via per-program attachments
        assert!(!options.delay_vibrato);
        assert!(options.software_lfo.is_empty());
    }
}
