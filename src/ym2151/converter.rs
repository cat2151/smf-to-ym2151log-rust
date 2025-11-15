//! YM2151 converter (Pass B)
//!
//! Converts MIDI events to YM2151 register write events.

use crate::error::Result;
use crate::midi::{
    midi_to_kc_kf, ticks_to_samples_with_tempo_map, MidiData, MidiEvent, TempoChange,
};
use crate::ym2151::{
    apply_tone_to_channel, default_tone_events, initialize_channel_events, load_tone_for_program,
    Ym2151Event, Ym2151Log,
};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

/// Map MIDI channel to YM2151 channel with drum channel (9) priority
///
/// MIDI channel 9 (0-based) is the drum channel in General MIDI.
/// Since drums often have multiple simultaneous note-ons and YM2151 processes
/// channels sequentially, we prioritize the drum channel to YM2151 channel 0
/// for better audio quality.
///
/// Mapping:
/// - MIDI channel 9 → YM2151 channel 0 (drum channel gets priority)
/// - MIDI channel 0 → YM2151 channel 1
/// - MIDI channel 1 → YM2151 channel 2
/// - MIDI channel 2 → YM2151 channel 3
/// - MIDI channel 3 → YM2151 channel 4
/// - MIDI channel 4 → YM2151 channel 5
/// - MIDI channel 5 → YM2151 channel 6
/// - MIDI channel 6 → YM2151 channel 7
/// - MIDI channel 7 → YM2151 channel 7 (overflow)
/// - MIDI channel 8+ → YM2151 channel 7 (overflow)
///
/// # Arguments
/// * `midi_channel` - MIDI channel number (0-15)
///
/// # Returns
/// YM2151 channel number (0-7)
fn map_midi_channel_to_ym2151(midi_channel: u8) -> u8 {
    match midi_channel {
        9 => 0, // Drum channel gets priority
        0 => 1, // Regular channels shifted
        1 => 2,
        2 => 3,
        3 => 4,
        4 => 5,
        5 => 6,
        _ => 7, // Channels 6, 7, 8+ overflow to channel 7
    }
}

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
                // Map MIDI channel to YM2151 channel with drum channel priority
                let ym2151_ch = map_midi_channel_to_ym2151(*channel);
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

    // Track the current program (tone) for each MIDI channel
    // Initialize all channels to program 0 (default)
    let mut channel_programs: HashMap<u8, u8> = HashMap::new();
    for &ch in &used_channels {
        channel_programs.insert(ch, 0);
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

                // Map MIDI channel to YM2151 channel with drum channel priority
                let ym2151_channel = map_midi_channel_to_ym2151(*channel);

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
                // Map MIDI channel to YM2151 channel with drum channel priority
                let ym2151_channel = map_midi_channel_to_ym2151(*channel);

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

            // Handle Program Change events
            MidiEvent::ProgramChange {
                ticks,
                channel,
                program,
            } => {
                // Map MIDI channel to YM2151 channel with drum channel priority
                let ym2151_channel = map_midi_channel_to_ym2151(*channel);

                // Skip if this channel isn't being used
                if !used_channels.contains(&ym2151_channel) {
                    continue;
                }

                let sample_time =
                    ticks_to_samples_with_tempo_map(*ticks, ticks_per_beat, &tempo_map);

                // Try to load tone from external file, fallback to default
                let tone_events = match load_tone_for_program(*program) {
                    Ok(Some(tone)) => {
                        // Apply the loaded tone to the channel
                        apply_tone_to_channel(&tone, ym2151_channel, sample_time)
                    }
                    Ok(None) | Err(_) => {
                        // Use default tone if file doesn't exist or can't be loaded
                        default_tone_events(ym2151_channel, sample_time)
                    }
                };

                // Add the tone change events
                ym2151_events.extend(tone_events);

                // Update the channel's current program
                channel_programs.insert(ym2151_channel, *program);
            }
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
        // MIDI channel 0 now maps to YM2151 channel 1 (due to drum channel priority)
        // KC register is at 0x29 for channel 1
        // There should be exactly one KC write from the Note On event
        let kc_events: Vec<&Ym2151Event> = result
            .events
            .iter()
            .filter(|e| e.addr == "0x29" && e.data == "0x3E")
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
            .find(|e| e.addr == "0x08" && e.data == "0x79" && e.time == 0) // Channel 1 now
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

        // Find KEY ON event - MIDI channel 0 maps to YM2151 channel 1
        let key_on = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x79")
            .expect("Should have KEY ON event");

        // 0x79 = all operators on, channel 1 (MIDI channel 0)
        assert_eq!(key_on.data, "0x79");
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
        // MIDI channel 0 maps to YM2151 channel 1
        let key_off = result
            .events
            .iter()
            .filter(|e| e.addr == "0x08" && e.time > 0)
            .find(|e| e.data == "0x01") // Channel 1
            .expect("Should have KEY OFF event");

        // 0x01 = all operators off, channel 1
        assert_eq!(key_off.data, "0x01");
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
        // MIDI Channel 0 -> YM2151 Channel 1 (KC register 0x29)
        let ch0_kc = result
            .events
            .iter()
            .find(|e| e.addr == "0x29" && e.time == 0)
            .expect("Should have KC write for MIDI channel 0 -> YM2151 channel 1");
        assert_eq!(ch0_kc.data, "0x3E"); // Middle C

        // MIDI Channel 1 -> YM2151 Channel 2 (KC register 0x2A)
        let ch1_kc = result
            .events
            .iter()
            .find(|e| e.addr == "0x2A" && e.time == 0)
            .expect("Should have KC write for MIDI channel 1 -> YM2151 channel 2");
        assert_eq!(ch1_kc.data, "0x44"); // E (octave 4, note 4)

        // MIDI Channel 2 -> YM2151 Channel 3 (KC register 0x2B)
        let ch2_kc = result
            .events
            .iter()
            .find(|e| e.addr == "0x2B" && e.time == 0)
            .expect("Should have KC write for MIDI channel 2 -> YM2151 channel 3");
        assert_eq!(ch2_kc.data, "0x48"); // G (octave 4, note 8)

        // Verify KEY ON events for each channel
        let key_on_ch0 = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x79" && e.time == 0)
            .expect("Should have KEY ON for MIDI channel 0 -> YM2151 channel 1");
        assert_eq!(key_on_ch0.data, "0x79");

        let key_on_ch1 = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x7A" && e.time == 0)
            .expect("Should have KEY ON for MIDI channel 1 -> YM2151 channel 2");
        assert_eq!(key_on_ch1.data, "0x7A");

        let key_on_ch2 = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x7B" && e.time == 0)
            .expect("Should have KEY ON for MIDI channel 2 -> YM2151 channel 3");
        assert_eq!(key_on_ch2.data, "0x7B");
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
        // MIDI channel 0 -> YM2151 channel 1 (RL_FB_CONNECT register 0x21)
        // MIDI channel 1 -> YM2151 channel 2 (RL_FB_CONNECT register 0x22)
        let has_ch1_init = result.events.iter().any(|e| e.addr == "0x21");
        let has_ch2_init = result.events.iter().any(|e| e.addr == "0x22");

        assert!(
            has_ch1_init,
            "YM2151 Channel 1 (MIDI ch 0) should be initialized"
        );
        assert!(
            has_ch2_init,
            "YM2151 Channel 2 (MIDI ch 1) should be initialized"
        );

        // Verify notes play on correct channels
        // MIDI channel 0 -> YM2151 channel 1 (KC register 0x29)
        let ch0_note = result
            .events
            .iter()
            .find(|e| e.addr == "0x29" && e.time == 0);
        // MIDI channel 1 -> YM2151 channel 2 (KC register 0x2A)
        let ch1_note = result
            .events
            .iter()
            .find(|e| e.addr == "0x2A" && e.time > 0);

        assert!(ch0_note.is_some(), "MIDI Channel 0 should have a note");
        assert!(ch1_note.is_some(), "MIDI Channel 1 should have a note");
    }

    #[test]
    fn test_convert_program_change() {
        // Test that program change events trigger tone changes
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                // Program change at the start
                MidiEvent::ProgramChange {
                    ticks: 0,
                    channel: 0,
                    program: 42,
                },
                // Play a note
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

        // Should have initialization + program change tone events + note events
        // 8 KEY OFF + 26 channel init + 26 program change tone + note on (3) + note off (1)
        // = 64 events
        assert_eq!(result.event_count, 64);

        // Verify there are tone setting events at time 0
        // MIDI channel 0 -> YM2151 channel 1
        // Look for the RL_FB_CONNECT register (0x21 for channel 1)
        let tone_events: Vec<_> = result
            .events
            .iter()
            .filter(|e| e.addr == "0x21" && e.time == 0)
            .collect();

        // Should have 2 writes to 0x21: one from init, one from program change
        assert_eq!(
            tone_events.len(),
            2,
            "Should have tone settings from both init and program change"
        );
    }

    #[test]
    fn test_convert_program_change_unused_channel() {
        // Program change on a channel that has no notes should be ignored
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                // Program change on channel 5
                MidiEvent::ProgramChange {
                    ticks: 0,
                    channel: 5,
                    program: 10,
                },
                // But only channel 0 plays a note
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

        // Should only have events for channel 0
        // 8 KEY OFF + 26 channel 0 init + note on (3) + note off (1) = 38
        assert_eq!(result.event_count, 38);
    }

    #[test]
    fn test_convert_multiple_program_changes() {
        // Test multiple program changes on the same channel
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                MidiEvent::ProgramChange {
                    ticks: 0,
                    channel: 0,
                    program: 10,
                },
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0,
                    note: 60,
                    velocity: 100,
                },
                MidiEvent::NoteOff {
                    ticks: 240,
                    channel: 0,
                    note: 60,
                },
                // Change to a different program
                MidiEvent::ProgramChange {
                    ticks: 240,
                    channel: 0,
                    program: 20,
                },
                MidiEvent::NoteOn {
                    ticks: 480,
                    channel: 0,
                    note: 64,
                    velocity: 100,
                },
                MidiEvent::NoteOff {
                    ticks: 720,
                    channel: 0,
                    note: 64,
                },
            ],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // 8 KEY OFF + 26 init + 26 program 10 + note (3) + note off (1)
        // + 26 program 20 + note (3) + note off (1) = 94
        assert_eq!(result.event_count, 94);

        // Verify both program changes generated tone events
        // MIDI channel 0 -> YM2151 channel 1 (RL_FB_CONNECT register 0x21)
        let tone_events_time_0: Vec<_> = result
            .events
            .iter()
            .filter(|e| e.addr == "0x21" && e.time == 0)
            .collect();
        assert_eq!(tone_events_time_0.len(), 2); // init + program 10

        // Second program change should be at a different time
        let tone_events_later: Vec<_> = result
            .events
            .iter()
            .filter(|e| e.addr == "0x21" && e.time > 0)
            .collect();
        assert!(
            !tone_events_later.is_empty(),
            "Should have tone change at later time"
        );
    }

    #[test]
    fn test_map_midi_channel_to_ym2151_drum_channel() {
        // MIDI channel 9 (drum channel) should map to YM2151 channel 0
        assert_eq!(map_midi_channel_to_ym2151(9), 0);
    }

    #[test]
    fn test_map_midi_channel_to_ym2151_regular_channels() {
        // MIDI channel 0 should map to YM2151 channel 1 (shifted due to drum priority)
        assert_eq!(map_midi_channel_to_ym2151(0), 1);

        // MIDI channels 1-5 should map to YM2151 channels 2-6
        assert_eq!(map_midi_channel_to_ym2151(1), 2);
        assert_eq!(map_midi_channel_to_ym2151(2), 3);
        assert_eq!(map_midi_channel_to_ym2151(3), 4);
        assert_eq!(map_midi_channel_to_ym2151(4), 5);
        assert_eq!(map_midi_channel_to_ym2151(5), 6);
    }

    #[test]
    fn test_map_midi_channel_to_ym2151_overflow() {
        // MIDI channels 6, 7, 8, and 10+ should overflow to YM2151 channel 7
        assert_eq!(map_midi_channel_to_ym2151(6), 7);
        assert_eq!(map_midi_channel_to_ym2151(7), 7);
        assert_eq!(map_midi_channel_to_ym2151(8), 7);
        assert_eq!(map_midi_channel_to_ym2151(10), 7);
        assert_eq!(map_midi_channel_to_ym2151(15), 7);
    }

    #[test]
    fn test_convert_drum_channel_note_on_channel_0() {
        // Test that MIDI channel 9 (drum) maps to YM2151 channel 0
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 9, // Drum channel
                    note: 60,
                    velocity: 100,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 9,
                    note: 60,
                },
            ],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Find KC register write for channel 0 (0x28)
        let kc_events: Vec<&Ym2151Event> = result
            .events
            .iter()
            .filter(|e| e.addr == "0x28" && e.time == 0)
            .collect();

        assert_eq!(
            kc_events.len(),
            1,
            "Drum channel should use YM2151 channel 0 (KC register 0x28)"
        );

        // Verify KEY ON uses channel 0
        let key_on = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x78" && e.time == 0)
            .expect("Should have KEY ON for channel 0");
        assert_eq!(key_on.data, "0x78"); // 0x78 = all operators on, channel 0
    }

    #[test]
    fn test_convert_regular_channel_shifted() {
        // Test that MIDI channel 0 maps to YM2151 channel 1 (due to drum priority)
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0, // Regular channel 0
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

        // Find KC register write for channel 1 (0x29)
        let kc_events: Vec<&Ym2151Event> = result
            .events
            .iter()
            .filter(|e| e.addr == "0x29" && e.time == 0)
            .collect();

        assert_eq!(
            kc_events.len(),
            1,
            "MIDI channel 0 should map to YM2151 channel 1 (KC register 0x29)"
        );

        // Verify KEY ON uses channel 1
        let key_on = result
            .events
            .iter()
            .find(|e| e.addr == "0x08" && e.data == "0x79" && e.time == 0)
            .expect("Should have KEY ON for channel 1");
        assert_eq!(key_on.data, "0x79"); // 0x79 = all operators on, channel 1
    }

    #[test]
    fn test_convert_drum_and_regular_channels_together() {
        // Test with both drum channel and regular channels
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                // Drum channel (MIDI 9) at same tick
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 9,
                    note: 36, // Bass drum
                    velocity: 100,
                },
                // Regular channel (MIDI 0) at same tick
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0,
                    note: 60,
                    velocity: 100,
                },
                // Regular channel (MIDI 1) at same tick
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 1,
                    note: 64,
                    velocity: 100,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 9,
                    note: 36,
                },
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
            ],
        };

        let result = convert_to_ym2151_log(&midi_data).unwrap();

        // Verify drum channel uses YM2151 channel 0
        let drum_kc = result
            .events
            .iter()
            .find(|e| e.addr == "0x28" && e.time == 0)
            .expect("Drum should use YM2151 channel 0");
        assert!(drum_kc.data.starts_with("0x"));

        // Verify MIDI channel 0 uses YM2151 channel 1
        let ch0_kc = result
            .events
            .iter()
            .find(|e| e.addr == "0x29" && e.time == 0)
            .expect("MIDI ch 0 should use YM2151 channel 1");
        assert!(ch0_kc.data.starts_with("0x"));

        // Verify MIDI channel 1 uses YM2151 channel 2
        let ch1_kc = result
            .events
            .iter()
            .find(|e| e.addr == "0x2A" && e.time == 0)
            .expect("MIDI ch 1 should use YM2151 channel 2");
        assert!(ch1_kc.data.starts_with("0x"));

        // Verify KEY ON events are in the correct order (drum first)
        let key_on_events: Vec<&Ym2151Event> = result
            .events
            .iter()
            .filter(|e| e.addr == "0x08" && e.time == 0 && e.data.starts_with("0x7"))
            .collect();

        // Should have 3 KEY ON events
        assert_eq!(key_on_events.len(), 3);

        // First KEY ON should be channel 0 (drum)
        assert_eq!(key_on_events[0].data, "0x78"); // Channel 0
    }
}
