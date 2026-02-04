//! Simple MML (Music Macro Language) parser for WASM
//!
//! This is a simplified MML parser that works without external dependencies
//! like tree-sitter, making it suitable for WASM compilation.
//!
//! ## Supported MML Commands
//!
//! - `c d e f g a b` - Notes (C, D, E, F, G, A, B)
//! - `r` - Rest
//! - `o<n>` - Set octave (e.g., `o4`, `o5`)
//! - `>` - Increase octave by 1
//! - `<` - Decrease octave by 1
//! - `l<n>` - Set default note length (e.g., `l4` for quarter note, `l8` for eighth note)
//! - `v<n>` - Set volume/velocity (0-15, where 15 is loudest)
//! - `;` - Channel separator (for multi-channel)
//!
//! ## Note
//!
//! Tempo is fixed at 120 BPM in the current implementation.
//!
//! ## Examples
//!
//! ```
//! use smf_to_ym2151log::mml::parse_mml;
//!
//! // Simple melody
//! let mml = "cdefgab";
//! let midi_bytes = parse_mml(mml).unwrap();
//!
//! // With octave and length
//! let mml = "o5 l4 cdefgab";
//! let midi_bytes = parse_mml(mml).unwrap();
//!
//! // Multi-channel (chord)
//! let mml = "c;e;g";
//! let midi_bytes = parse_mml(mml).unwrap();
//! ```

use crate::error::{Error, Result};
use midly::{Format, Header, MetaMessage, MidiMessage, Timing, TrackEvent, TrackEventKind};

/// Parse MML string and convert to SMF (Standard MIDI File) bytes
///
/// # Arguments
/// * `mml` - MML string to parse
///
/// # Returns
/// SMF data as bytes
///
/// # Errors
/// Returns an error if the MML string is invalid or cannot be converted
pub fn parse_mml(mml: &str) -> Result<Vec<u8>> {
    let channels = split_channels(mml);

    // Handle empty MML string
    if channels.is_empty() {
        // Create a minimal valid MIDI file with a single empty track
        let header = Header::new(Format::SingleTrack, Timing::Metrical(480.into()));
        let track = vec![TrackEvent {
            delta: 0.into(),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        }];
        let smf = midly::Smf {
            header,
            tracks: vec![track],
        };
        let mut bytes = Vec::new();
        smf.write(&mut bytes)
            .map_err(|e| Error::MidiParse(format!("Failed to write SMF: {}", e)))?;
        return Ok(bytes);
    }

    let has_multiple_channels = channels.len() > 1;

    let format = if has_multiple_channels {
        Format::Parallel
    } else {
        Format::SingleTrack
    };
    let header = Header::new(format, Timing::Metrical(480.into()));

    let mut tracks = Vec::new();

    if has_multiple_channels {
        // Format 1: Create tempo track + channel tracks
        let tempo_track = vec![
            TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::Tempo(500000.into())), // 120 BPM
            },
            TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
            },
        ];
        tracks.push(tempo_track);

        // Create a track for each channel
        for (channel_idx, channel_mml) in channels.iter().enumerate() {
            let track = parse_channel(channel_mml, channel_idx as u8)?;
            tracks.push(track);
        }
    } else {
        // Format 0: Single track with tempo meta event
        let mut track = parse_channel(&channels[0], 0)?;
        // Insert tempo event at the beginning
        track.insert(
            0,
            TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::Tempo(500000.into())), // 120 BPM
            },
        );
        tracks.push(track);
    }

    // Convert to SMF bytes
    let smf = midly::Smf { header, tracks };
    let mut bytes = Vec::new();
    smf.write(&mut bytes)
        .map_err(|e| Error::MidiParse(format!("Failed to write SMF: {}", e)))?;

    Ok(bytes)
}

/// Split MML string by channel separator (;)
fn split_channels(mml: &str) -> Vec<String> {
    mml.split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Parse a single channel's MML and return MIDI track events
fn parse_channel(mml: &str, channel: u8) -> Result<Vec<TrackEvent<'static>>> {
    let mut events = Vec::new();
    let mut time = 0u32;
    let mut octave = 5u8; // Default octave (C5 = MIDI 60)
    let mut default_length = 4u32; // Default quarter note
    let mut velocity = 100u8; // Default velocity

    let chars: Vec<char> = mml.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        match ch {
            // Whitespace
            ' ' | '\t' | '\n' | '\r' => {
                i += 1;
            }
            // Notes
            'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b' | 'C' | 'D' | 'E' | 'F' | 'G' | 'A' | 'B' => {
                let note_offset = match ch.to_ascii_lowercase() {
                    'c' => 0,
                    'd' => 2,
                    'e' => 4,
                    'f' => 5,
                    'g' => 7,
                    'a' => 9,
                    'b' => 11,
                    _ => 0,
                };
                let midi_note = octave * 12 + note_offset;

                // Check for sharp (#) or flat (-)
                i += 1;
                let mut modified_note = midi_note;
                if i < chars.len() {
                    match chars[i] {
                        '+' | '#' => {
                            modified_note = modified_note.saturating_add(1);
                            i += 1;
                        }
                        '-' => {
                            modified_note = modified_note.saturating_sub(1);
                            i += 1;
                        }
                        _ => {}
                    }
                }

                // Clamp to valid MIDI note range (0-127)
                let clamped_note = modified_note.min(127);

                // Check for note length
                let (length, new_i) = parse_number(&chars, i);
                i = new_i;
                let duration = calculate_duration(length.unwrap_or(default_length));

                // Note on
                events.push(TrackEvent {
                    delta: time.into(),
                    kind: TrackEventKind::Midi {
                        channel: channel.into(),
                        message: MidiMessage::NoteOn {
                            key: clamped_note.into(),
                            vel: velocity.into(),
                        },
                    },
                });
                time = 0;

                // Note off
                events.push(TrackEvent {
                    delta: duration.into(),
                    kind: TrackEventKind::Midi {
                        channel: channel.into(),
                        message: MidiMessage::NoteOff {
                            key: clamped_note.into(),
                            vel: 0.into(),
                        },
                    },
                });
            }
            // Rest
            'r' | 'R' => {
                i += 1;
                let (length, new_i) = parse_number(&chars, i);
                i = new_i;
                let duration = calculate_duration(length.unwrap_or(default_length));
                time += duration;
            }
            // Octave set
            'o' | 'O' => {
                i += 1;
                let (oct, new_i) = parse_number(&chars, i);
                i = new_i;
                if let Some(oct_val) = oct {
                    octave = oct_val.clamp(0, 10) as u8;
                }
            }
            // Octave up
            '>' => {
                octave = octave.saturating_add(1).min(10);
                i += 1;
            }
            // Octave down
            '<' => {
                octave = octave.saturating_sub(1);
                i += 1;
            }
            // Default length
            'l' | 'L' => {
                i += 1;
                let (len, new_i) = parse_number(&chars, i);
                i = new_i;
                if let Some(len_val) = len {
                    default_length = len_val.max(1);
                }
            }
            // Volume/Velocity
            'v' | 'V' => {
                i += 1;
                let (vol, new_i) = parse_number(&chars, i);
                i = new_i;
                if let Some(vol_val) = vol {
                    // MML volume is typically 0-15, convert to MIDI velocity 0-127
                    velocity = ((vol_val.min(15) * 127 / 15) as u8).max(1);
                }
            }
            // Note: Tempo commands (t) are ignored in this simple parser
            // Tempo is fixed at 120 BPM
            't' | 'T' => {
                i += 1;
                let (_tempo, new_i) = parse_number(&chars, i);
                i = new_i;
            }
            _ => {
                i += 1;
            }
        }
    }

    // Add end of track
    events.push(TrackEvent {
        delta: time.into(),
        kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
    });

    Ok(events)
}

/// Parse a number from character array starting at position i
/// Returns (Some(number), new_position) or (None, original_position)
fn parse_number(chars: &[char], start: usize) -> (Option<u32>, usize) {
    let mut i = start;
    let mut num_str = String::new();

    while i < chars.len() && chars[i].is_ascii_digit() {
        num_str.push(chars[i]);
        i += 1;
    }

    if num_str.is_empty() {
        (None, start)
    } else {
        (num_str.parse::<u32>().ok(), i)
    }
}

/// Calculate duration in ticks for a note length
/// Assumes 480 ticks per quarter note
fn calculate_duration(length: u32) -> u32 {
    let whole_note_ticks = 1920; // 480 * 4
    whole_note_ticks / length.max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_melody() {
        let mml = "cdefgab";
        let result = parse_mml(mml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_octave() {
        let mml = "o5 cde o4 cde";
        let result = parse_mml(mml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_length() {
        let mml = "l4 c l8 d e";
        let result = parse_mml(mml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_multi_channel() {
        let mml = "c;e;g";
        let result = parse_mml(mml);
        assert!(result.is_ok());
        // Should create Format 1 MIDI
    }

    #[test]
    fn test_parse_with_sharps_flats() {
        let mml = "c# d- e";
        let result = parse_mml(mml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_rests() {
        let mml = "c r d r e";
        let result = parse_mml(mml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_with_volume() {
        let mml = "v15 c v8 d v0 e";
        let result = parse_mml(mml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_octave_up_down() {
        let mml = "c > c > c < c";
        let result = parse_mml(mml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_string() {
        let mml = "";
        let result = parse_mml(mml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_note_lengths() {
        let mml = "c4 d8 e16 f2";
        let result = parse_mml(mml);
        assert!(result.is_ok());
    }
}
