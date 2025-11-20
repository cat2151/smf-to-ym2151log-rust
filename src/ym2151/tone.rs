//! YM2151 tone/voice definitions and loading
//!
//! Handles loading tone settings from external JSON files or using built-in presets.

use crate::error::{Error, Result};
use crate::ym2151::Ym2151Event;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Tone definition compatible with YM2151 register log format
///
/// This structure represents a list of YM2151 register writes that configure
/// a tone/voice for a channel. The events should not include time-dependent
/// registers like KC (Key Code) or KF (Key Fraction), as those are set
/// dynamically based on the note being played.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToneDefinition {
    /// List of YM2151 register write events
    /// Time values are ignored when loading tones
    pub events: Vec<Ym2151Event>,
}

/// Load a tone definition from a JSON file
///
/// # Arguments
/// * `path` - Path to the tone JSON file
///
/// # Returns
/// ToneDefinition if file exists and is valid, None if file doesn't exist
///
/// # Errors
/// Returns an error if the file exists but cannot be parsed
pub fn load_tone_from_file(path: &Path) -> Result<Option<ToneDefinition>> {
    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(path)?;
    let tone: ToneDefinition = serde_json::from_str(&content)
        .map_err(|e| Error::MidiParse(format!("Failed to parse tone JSON: {}", e)))?;

    Ok(Some(tone))
}

/// Load a tone definition for a specific program number
///
/// Attempts to load from `tones/{program:03}.json` in the current directory.
/// Returns None if the file doesn't exist (caller should use default tone).
///
/// # Arguments
/// * `program` - MIDI program number (0-127)
///
/// # Returns
/// ToneDefinition if file exists, None if it doesn't exist
///
/// # Errors
/// Returns an error if the file exists but cannot be parsed
pub fn load_tone_for_program(program: u8) -> Result<Option<ToneDefinition>> {
    let filename = format!("tones/{:03}.json", program);
    let path = Path::new(&filename);
    load_tone_from_file(path)
}

/// Generate default tone events for a channel
///
/// This is a fallback tone used when no external tone file is available.
/// It uses the same initialization as the current implementation.
///
/// # Arguments
/// * `channel` - YM2151 channel number (0-7)
/// * `time` - Time in seconds for the events
///
/// # Returns
/// Vector of YM2151 register write events for the default tone
pub fn default_tone_events(channel: u8, time: f64) -> Vec<Ym2151Event> {
    // Use the existing channel initialization as the default tone
    crate::ym2151::initialize_channel_events(channel, time)
}

/// Apply a tone definition to a specific channel at a specific time
///
/// Converts the tone definition's register writes to use the specified
/// channel and time.
///
/// # Arguments
/// * `tone` - Tone definition to apply
/// * `channel` - Target YM2151 channel (0-7)
/// * `time` - Time in seconds for the tone change
///
/// # Returns
/// Vector of YM2151 events configured for the specified channel and time
pub fn apply_tone_to_channel(tone: &ToneDefinition, channel: u8, time: f64) -> Vec<Ym2151Event> {
    tone.events
        .iter()
        .map(|event| {
            // Parse the address to adjust for the target channel
            let addr_value = if let Some(stripped) = event.addr.strip_prefix("0x") {
                u8::from_str_radix(stripped, 16).unwrap_or(0)
            } else {
                0
            };

            // Determine the register type and adjust the address for the channel
            let new_addr = match addr_value {
                // Channel-specific registers (0x20-0x27, 0x28-0x2F, 0x30-0x37, 0x38-0x3F)
                0x20..=0x27 => format!("0x{:02X}", 0x20 + channel),
                0x28..=0x2F => format!("0x{:02X}", 0x28 + channel),
                0x30..=0x37 => format!("0x{:02X}", 0x30 + channel),
                0x38..=0x3F => format!("0x{:02X}", 0x38 + channel),
                // Operator-specific registers - adjust based on channel
                0x40..=0xFF => {
                    // Calculate base register and operator offset
                    let base = addr_value & 0xE0; // Mask to get base register
                    let slot = addr_value & 0x1F; // Get slot number
                    let operator = slot / 8; // Which operator (0-3)
                    let new_slot = channel + (operator * 8);
                    format!("0x{:02X}", base + new_slot)
                }
                _ => event.addr.clone(),
            };

            Ym2151Event {
                time,
                addr: new_addr,
                data: event.data.clone(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_tone_events() {
        let events = default_tone_events(0, 0.0);
        // Should have 26 events (same as initialize_channel_events)
        assert_eq!(events.len(), 26);
    }

    #[test]
    fn test_load_tone_for_program_nonexistent() {
        // Try to load a program that definitely doesn't exist
        let result = load_tone_for_program(255);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_apply_tone_to_channel() {
        // Create a simple tone definition
        let tone = ToneDefinition {
            events: vec![
                Ym2151Event {
                    time: 0.0,
                    addr: "0x20".to_string(), // RL_FB_CONNECT for channel 0
                    data: "0xC7".to_string(),
                },
                Ym2151Event {
                    time: 0.0,
                    addr: "0x40".to_string(), // DT1/MUL for operator 0, channel 0
                    data: "0x01".to_string(),
                },
            ],
        };

        // Apply to channel 1 at time 1.0 seconds
        let events = apply_tone_to_channel(&tone, 1, 1.0);

        assert_eq!(events.len(), 2);
        // Channel register should be adjusted to 0x21 (channel 1)
        assert_eq!(events[0].addr, "0x21");
        assert!((events[0].time - 1.0).abs() < 0.001);
        // Operator register should be adjusted to 0x41 (operator 0, channel 1)
        assert_eq!(events[1].addr, "0x41");
        assert!((events[1].time - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_apply_tone_to_different_channels() {
        let tone = ToneDefinition {
            events: vec![Ym2151Event {
                time: 0.0,
                addr: "0x60".to_string(), // TL for operator 0, channel 0
                data: "0x00".to_string(),
            }],
        };

        // Apply to channel 0
        let events_ch0 = apply_tone_to_channel(&tone, 0, 0.0);
        assert_eq!(events_ch0[0].addr, "0x60");

        // Apply to channel 3
        let events_ch3 = apply_tone_to_channel(&tone, 3, 0.0);
        assert_eq!(events_ch3[0].addr, "0x63");

        // Apply to channel 7
        let events_ch7 = apply_tone_to_channel(&tone, 7, 0.0);
        assert_eq!(events_ch7[0].addr, "0x67");
    }

    #[test]
    fn test_apply_tone_multiple_operators() {
        let tone = ToneDefinition {
            events: vec![
                Ym2151Event {
                    time: 0.0,
                    addr: "0x60".to_string(), // TL operator 0, channel 0
                    data: "0x00".to_string(),
                },
                Ym2151Event {
                    time: 0.0,
                    addr: "0x68".to_string(), // TL operator 1, channel 0
                    data: "0x7F".to_string(),
                },
                Ym2151Event {
                    time: 0.0,
                    addr: "0x70".to_string(), // TL operator 2, channel 0
                    data: "0x7F".to_string(),
                },
                Ym2151Event {
                    time: 0.0,
                    addr: "0x78".to_string(), // TL operator 3, channel 0
                    data: "0x7F".to_string(),
                },
            ],
        };

        // Apply to channel 2
        let events = apply_tone_to_channel(&tone, 2, 0.0);

        assert_eq!(events.len(), 4);
        assert_eq!(events[0].addr, "0x62"); // TL operator 0, channel 2
        assert_eq!(events[1].addr, "0x6A"); // TL operator 1, channel 2
        assert_eq!(events[2].addr, "0x72"); // TL operator 2, channel 2
        assert_eq!(events[3].addr, "0x7A"); // TL operator 3, channel 2
    }
}
