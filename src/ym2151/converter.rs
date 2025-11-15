//! YM2151 converter (Pass B)
//!
//! Converts MIDI events to YM2151 register write events.

use crate::error::Result;
use crate::midi::{
    midi_to_kc_kf, ticks_to_samples_with_tempo_map, MidiData, MidiEvent, TempoChange,
};
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

    let mut ym2151_events = Vec::new();

    // Build tempo map from MIDI events
    let mut tempo_map: Vec<TempoChange> = vec![TempoChange {
        tick: 0,
        tempo_bpm: midi_data.tempo_bpm,
    }];

    for event in &midi_data.events {
        if let MidiEvent::Tempo { ticks, tempo_bpm } = event {
            // Only add if it's different from the current tempo
            // or if it's the first explicit tempo event at tick 0
            if tempo_map.is_empty()
                || *ticks > tempo_map.last().unwrap().tick
                || (*ticks == 0 && tempo_map.len() == 1)
            {
                tempo_map.push(TempoChange {
                    tick: *ticks,
                    tempo_bpm: *tempo_bpm,
                });
            }
        }
    }

    // Remove duplicates and sort by tick
    tempo_map.sort_by_key(|t| t.tick);
    tempo_map.dedup_by_key(|t| t.tick);

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

    // First pass: collect which MIDI channels are used
    let mut used_channels = HashSet::new();
    for event in &midi_data.events {
        match event {
            MidiEvent::NoteOn { channel, .. } | MidiEvent::NoteOff { channel, .. } => {
                // Map MIDI channel to YM2151 channel (clamp to 0-7)
                let ym2151_ch = (*channel).min(7);
                used_channels.insert(ym2151_ch);
            }
            // Tempo and ProgramChange events don't affect channel initialization
            _ => {}
        }
    }

    // Initialize all used channels with default parameters
    for &ch in &used_channels {
        ym2151_events.extend(initialize_channel_events(ch, 0));
    }

    // Process MIDI events
    // Track active notes per channel: set of (channel, note) tuples
    let mut active_notes: HashSet<(u8, u8)> = HashSet::new();

    for event in &midi_data.events {
        match event {
            // Tempo events are already in the tempo map, no action needed here
            MidiEvent::Tempo { .. } => {
                // Skip - tempo changes are handled via tempo_map
            }

            // Handle Note On events
            MidiEvent::NoteOn {
                ticks,
                channel,
                note,
                velocity,
                ..
            } => {
                // Skip if velocity is 0 (should already be converted to Note Off in parser)
                if *velocity == 0 {
                    continue;
                }

                // Map MIDI channel to YM2151 channel (clamp to 0-7)
                let ym2151_channel = (*channel).min(7);

                let sample_time =
                    ticks_to_samples_with_tempo_map(*ticks, ticks_per_beat, &tempo_map);
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

                active_notes.insert((ym2151_channel, *note));
            }

            // Handle Note Off events
            MidiEvent::NoteOff {
                ticks,
                channel,
                note,
                ..
            } => {
                // Map MIDI channel to YM2151 channel (clamp to 0-7)
                let ym2151_channel = (*channel).min(7);

                let sample_time =
                    ticks_to_samples_with_tempo_map(*ticks, ticks_per_beat, &tempo_map);

                if active_notes.contains(&(ym2151_channel, *note)) {
                    // Key OFF
                    ym2151_events.push(Ym2151Event {
                        time: sample_time,
                        addr: "0x08".to_string(),
                        data: format!("0x{:02X}", ym2151_channel),
                    });

                    active_notes.remove(&(ym2151_channel, *note));
                }
            }

            // ProgramChange events are not yet implemented for YM2151 conversion
            // Future work: map MIDI programs to YM2151 voice parameters
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

        // Should have only 8 KEY OFF events (no channels used, so no channel init)
        assert_eq!(result.event_count, 8);
        assert_eq!(result.events.len(), 8);
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

    #[test]
    fn test_convert_multi_channel() {
        // Test with notes on different MIDI channels
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                // Channel 0: C (60)
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0,
                    note: 60,
                    velocity: 100,
                },
                // Channel 1: E (64)
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 1,
                    note: 64,
                    velocity: 100,
                },
                // Channel 2: G (67)
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 2,
                    note: 67,
                    velocity: 100,
                },
                // Note offs at tick 480
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 0,
                    note: 60,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 1,
                    note: 64,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 2,
                    note: 67,
                },
            ],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Verify we have events for all 3 channels
        // 8 KEY OFF + (26 * 3 channels) + (3 notes * 3 events each) + (3 note offs) = 8 + 78 + 9 + 3 = 98
        assert_eq!(result.event_count, 98);

        // Verify KC register writes for each channel
        // Channel 0: KC register is 0x28
        let ch0_kc = result
            .events
            .iter()
            .find(|e| e.addr == "0x28" && e.time == 0)
            .expect("Should have KC write for channel 0");
        assert_eq!(ch0_kc.data, "0x3E"); // Middle C

        // Channel 1: KC register is 0x29
        let ch1_kc = result
            .events
            .iter()
            .find(|e| e.addr == "0x29" && e.time == 0)
            .expect("Should have KC write for channel 1");
        assert_eq!(ch1_kc.data, "0x44"); // E (octave 4, note 4)

        // Channel 2: KC register is 0x2A
        let ch2_kc = result
            .events
            .iter()
            .find(|e| e.addr == "0x2A" && e.time == 0)
            .expect("Should have KC write for channel 2");
        assert_eq!(ch2_kc.data, "0x48"); // G (octave 4, note 8)

        // Verify KEY ON events for each channel
        let key_on_ch0 = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x78" && e.time == 0)
            .expect("Should have KEY ON for channel 0");
        assert_eq!(key_on_ch0.data, "0x78");

        let key_on_ch1 = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x79" && e.time == 0)
            .expect("Should have KEY ON for channel 1");
        assert_eq!(key_on_ch1.data, "0x79");

        let key_on_ch2 = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x7A" && e.time == 0)
            .expect("Should have KEY ON for channel 2");
        assert_eq!(key_on_ch2.data, "0x7A");
    }

    #[test]
    fn test_convert_multi_channel_sequential() {
        // Test with notes on different channels played sequentially
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                // Channel 0 plays first
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
                // Channel 1 plays next
                MidiEvent::NoteOn {
                    ticks: 480,
                    channel: 1,
                    note: 64,
                    velocity: 100,
                },
                MidiEvent::NoteOff {
                    ticks: 960,
                    channel: 1,
                    note: 64,
                },
            ],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Should have events for both channels
        // Verify both channels are initialized
        let has_ch0_init = result.events.iter().any(|e| e.addr == "0x20");
        let has_ch1_init = result.events.iter().any(|e| e.addr == "0x21");

        assert!(has_ch0_init, "Channel 0 should be initialized");
        assert!(has_ch1_init, "Channel 1 should be initialized");

        // Verify notes play on correct channels
        let ch0_note = result
            .events
            .iter()
            .find(|e| e.addr == "0x28" && e.time == 0);
        let ch1_note = result
            .events
            .iter()
            .find(|e| e.addr == "0x29" && e.time > 0);

        assert!(ch0_note.is_some(), "Channel 0 should have a note");
        assert!(ch1_note.is_some(), "Channel 1 should have a note");
    }
}
