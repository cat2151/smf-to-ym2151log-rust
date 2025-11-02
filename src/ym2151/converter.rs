//! YM2151 converter (Pass B)
//!
//! Converts MIDI events to YM2151 register write events.

use crate::error::Result;
use crate::midi::{midi_to_kc_kf, ticks_to_samples, MidiData, MidiEvent};
use crate::ym2151::{initialize_channel_events, Ym2151Event, Ym2151Log};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

/// Convert MIDI events to YM2151 register write log
///
/// # Arguments
/// * `midi_data` - Parsed MIDI data from Pass A
///
/// # Returns
/// YM2151 log with register write events
///
/// # Errors
/// Returns an error if conversion fails
///
/// # Example
/// ```no_run
/// use smf_to_ym2151log::midi::MidiData;
/// use smf_to_ym2151log::ym2151::convert_to_ym2151_log;
///
/// let midi_data = MidiData {
///     ticks_per_beat: 480,
///     tempo_bpm: 120.0,
///     events: vec![],
/// };
/// let log = convert_to_ym2151_log(&midi_data).unwrap();
/// ```
pub fn convert_to_ym2151_log(midi_data: &MidiData) -> Result<Ym2151Log> {
    let ticks_per_beat = midi_data.ticks_per_beat;
    let mut current_tempo_bpm = midi_data.tempo_bpm;

    let mut ym2151_events = Vec::new();

    // Initialize all channels at time 0
    // Register 0x08 is the Key ON/OFF register
    // Writing channel number turns off that channel
    for ch in 0..8 {
        ym2151_events.push(Ym2151Event {
            time: 0,
            addr: "0x08".to_string(),
            data: format!("0x{:02X}", ch),
        });
    }

    // Initialize channel 0 with default parameters
    ym2151_events.extend(initialize_channel_events(0, 0));

    // Process MIDI events
    // Track active notes: key is note number only
    let mut active_notes: HashSet<u8> = HashSet::new();
    let ym2151_channel: u8 = 0; // Use YM2151 channel 0 for all notes (mono)

    for event in &midi_data.events {
        match event {
            // Update tempo if tempo change event
            MidiEvent::Tempo { tempo_bpm, .. } => {
                current_tempo_bpm = *tempo_bpm;
            }

            // Handle Note On events
            MidiEvent::NoteOn {
                ticks,
                note,
                velocity,
                ..
            } => {
                // Skip if velocity is 0 (should already be converted to Note Off in parser)
                if *velocity == 0 {
                    continue;
                }

                let sample_time = ticks_to_samples(*ticks, ticks_per_beat, current_tempo_bpm);
                let (kc, kf) = midi_to_kc_kf(*note);

                // Set KC (Key Code)
                ym2151_events.push(Ym2151Event {
                    time: sample_time,
                    addr: format!("0x{:02X}", 0x28 + ym2151_channel),
                    data: format!("0x{:02X}", kc),
                });

                // Set KF (Key Fraction)
                ym2151_events.push(Ym2151Event {
                    time: sample_time,
                    addr: format!("0x{:02X}", 0x30 + ym2151_channel),
                    data: format!("0x{:02X}", kf),
                });

                // Key ON (0x78 = all operators on)
                ym2151_events.push(Ym2151Event {
                    time: sample_time,
                    addr: "0x08".to_string(),
                    data: format!("0x{:02X}", 0x78 | ym2151_channel),
                });

                active_notes.insert(*note);
            }

            // Handle Note Off events
            MidiEvent::NoteOff { ticks, note, .. } => {
                let sample_time = ticks_to_samples(*ticks, ticks_per_beat, current_tempo_bpm);

                if active_notes.contains(note) {
                    // Key OFF
                    ym2151_events.push(Ym2151Event {
                        time: sample_time,
                        addr: "0x08".to_string(),
                        data: format!("0x{:02X}", ym2151_channel),
                    });

                    active_notes.remove(note);
                }
            }

            // Ignore other event types for now
            _ => {}
        }
    }

    Ok(Ym2151Log {
        event_count: ym2151_events.len(),
        events: ym2151_events,
    })
}

/// Save YM2151 log to JSON file
///
/// # Arguments
/// * `log` - YM2151 log to save
/// * `filename` - Path to output JSON file
///
/// # Returns
/// Ok(()) on success
///
/// # Errors
/// Returns an error if file cannot be created or written
pub fn save_ym2151_log(log: &Ym2151Log, filename: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(log)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_empty_midi() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Should have initialization events: 8 KEY OFF + 26 channel init = 34
        assert_eq!(result.event_count, 34);
        assert_eq!(result.events.len(), 34);
    }

    #[test]
    fn test_convert_single_note() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0,
                    note: 60, // Middle C
                    velocity: 100,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 0,
                    note: 60,
                },
            ],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Initialization (34) + Note On (3: KC, KF, KEY ON) + Note Off (1: KEY OFF) = 38
        assert_eq!(result.event_count, 38);

        // Find the KC register write for Note On
        // KC register is at 0x28 for channel 0
        // There should be exactly one KC write from the Note On event
        let kc_events: Vec<&Ym2151Event> = result
            .events
            .iter()
            .filter(|e| e.addr == "0x28" && e.data == "0x3E")
            .collect();

        assert_eq!(
            kc_events.len(),
            1,
            "Should have exactly one KC register write for Middle C"
        );

        // Middle C (MIDI 60) should map to KC 0x3E
        assert_eq!(kc_events[0].data, "0x3E");
    }

    #[test]
    fn test_convert_tempo_change() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0,
                    note: 60,
                    velocity: 100,
                },
                MidiEvent::Tempo {
                    ticks: 240,
                    tempo_bpm: 60.0, // Half speed
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 0,
                    note: 60,
                },
            ],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Should have initialization + Note On + Note Off events
        assert!(result.event_count > 34);

        // Verify Note On happens at time 0
        let note_on_event = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x78" && e.time == 0)
            .expect("Should have Note On KEY event at time 0");
        assert_eq!(note_on_event.time, 0);
    }

    #[test]
    fn test_convert_multiple_notes() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0,
                    note: 60,
                    velocity: 100,
                },
                MidiEvent::NoteOn {
                    ticks: 240,
                    channel: 0,
                    note: 64,
                    velocity: 100,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 0,
                    note: 60,
                },
                MidiEvent::NoteOff {
                    ticks: 720,
                    channel: 0,
                    note: 64,
                },
            ],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Init (34) + 2 Note Ons (6) + 2 Note Offs (2) = 42
        assert_eq!(result.event_count, 42);
    }

    #[test]
    fn test_key_on_register_format() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            }],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Find KEY ON event
        let key_on = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x78")
            .expect("Should have KEY ON event");

        // 0x78 = all operators on, channel 0
        assert_eq!(key_on.data, "0x78");
    }

    #[test]
    fn test_key_off_register_format() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0,
                    note: 60,
                    velocity: 100,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 0,
                    note: 60,
                },
            ],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Find KEY OFF event (should be after initialization)
        let key_off = result
            .events
            .iter()
            .filter(|e| e.addr == "0x08" && e.time > 0)
            .find(|e| e.data == "0x00")
            .expect("Should have KEY OFF event");

        // 0x00 = all operators off, channel 0
        assert_eq!(key_off.data, "0x00");
    }
}
