//! MIDI file parser (Pass A)
//!
//! This module parses Standard MIDI Files and extracts relevant events.

use crate::error::{Error, Result};
use crate::midi::events::{MidiData, MidiEvent};
use midly::{MidiMessage, Smf, TrackEventKind};
use std::fs;

/// Default tempo in microseconds per quarter note (120 BPM)
const DEFAULT_TEMPO_USPQN: u32 = 500_000;

/// Parse a MIDI file and extract events
///
/// # Arguments
/// * `path` - Path to the MIDI file
///
/// # Returns
/// Parsed MIDI data with events and metadata
///
/// # Errors
/// Returns an error if the file cannot be read or parsed
pub fn parse_midi_file(path: &str) -> Result<MidiData> {
    // Read the MIDI file
    let data = fs::read(path)?;

    // Parse with midly
    let smf = Smf::parse(&data)
        .map_err(|e| Error::MidiParse(format!("Failed to parse MIDI file: {}", e)))?;

    // Get ticks per beat from the timing
    let ticks_per_beat = match smf.header.timing {
        midly::Timing::Metrical(ticks) => ticks.as_int(),
        midly::Timing::Timecode(fps, subframe) => {
            // For timecode, we'll convert to a reasonable default
            // This is a simplified conversion
            let frames_per_second = match fps {
                midly::Fps::Fps24 => 24,
                midly::Fps::Fps25 => 25,
                midly::Fps::Fps29 => 29,
                midly::Fps::Fps30 => 30,
            };
            (frames_per_second as u16) * (subframe as u16)
        }
    };

    // Extract events from all tracks
    let mut events = Vec::new();

    // Process all tracks and merge events
    for track in smf.tracks.iter() {
        let mut absolute_ticks: u32 = 0;

        for event in track {
            // Convert delta time to absolute ticks
            absolute_ticks = absolute_ticks.saturating_add(event.delta.as_int());

            match &event.kind {
                TrackEventKind::Midi { channel, message } => {
                    let ch = channel.as_int();

                    match message {
                        MidiMessage::NoteOn { key, vel } => {
                            let velocity = vel.as_int();
                            // Note: velocity 0 is treated as Note Off by MIDI spec
                            if velocity == 0 {
                                events.push(MidiEvent::NoteOff {
                                    ticks: absolute_ticks,
                                    channel: ch,
                                    note: key.as_int(),
                                });
                            } else {
                                events.push(MidiEvent::NoteOn {
                                    ticks: absolute_ticks,
                                    channel: ch,
                                    note: key.as_int(),
                                    velocity,
                                });
                            }
                        }
                        MidiMessage::NoteOff { key, vel: _ } => {
                            events.push(MidiEvent::NoteOff {
                                ticks: absolute_ticks,
                                channel: ch,
                                note: key.as_int(),
                            });
                        }
                        MidiMessage::ProgramChange { program } => {
                            events.push(MidiEvent::ProgramChange {
                                ticks: absolute_ticks,
                                channel: ch,
                                program: program.as_int(),
                            });
                        }
                        _ => {
                            // Ignore other MIDI messages for now
                        }
                    }
                }
                TrackEventKind::Meta(midly::MetaMessage::Tempo(tempo)) => {
                    let tempo_uspqn = tempo.as_int();
                    let tempo_bpm = 60_000_000.0 / tempo_uspqn as f64;
                    events.push(MidiEvent::Tempo {
                        ticks: absolute_ticks,
                        tempo_bpm,
                    });
                }
                TrackEventKind::Meta(_) => {
                    // Ignore other meta messages for now
                }
                _ => {
                    // Ignore other event types
                }
            }
        }
    }

    // Sort events by ticks
    events.sort_by_key(|e| match e {
        MidiEvent::NoteOn { ticks, .. } => *ticks,
        MidiEvent::NoteOff { ticks, .. } => *ticks,
        MidiEvent::Tempo { ticks, .. } => *ticks,
        MidiEvent::ProgramChange { ticks, .. } => *ticks,
    });

    // Calculate initial tempo in BPM
    let initial_tempo_bpm = 60_000_000.0 / DEFAULT_TEMPO_USPQN as f64;

    Ok(MidiData {
        ticks_per_beat,
        tempo_bpm: initial_tempo_bpm,
        events,
    })
}

/// Save MIDI data to JSON file
///
/// # Arguments
/// * `midi_data` - Parsed MIDI data to save
/// * `output_path` - Path to output JSON file
///
/// # Errors
/// Returns an error if the file cannot be written
pub fn save_midi_events_json(midi_data: &MidiData, output_path: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(midi_data)?;
    fs::write(output_path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_tempo_conversion() {
        // 120 BPM = 500,000 microseconds per quarter note
        let bpm = 60_000_000.0 / DEFAULT_TEMPO_USPQN as f64;
        assert_eq!(bpm, 120.0);
    }
}
