//! Integration tests for multi-channel MIDI parsing and timing accuracy

use smf_to_ym2151log::midi::{parse_midi_file, save_midi_events_json, MidiEvent};
use smf_to_ym2151log::ym2151::{convert_to_ym2151_log, save_ym2151_log};
use std::collections::HashSet;
use std::fs;

#[test]
fn test_parse_midi_from_bytes() {
    // Test parsing MIDI from bytes directly
    let midi_path = "tests/test_data/simple_melody.mid";

    // Read the MIDI file as bytes
    let smf_bytes = fs::read(midi_path).expect("Failed to read test MIDI file");

    // Parse from bytes
    let result = smf_to_ym2151log::midi::parse_midi_from_bytes(&smf_bytes);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI from bytes: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Verify metadata
    assert_eq!(midi_data.ticks_per_beat, 480);
    assert_eq!(midi_data.tempo_bpm, 120.0);

    // Verify we got events
    assert!(!midi_data.events.is_empty(), "Should have parsed events");
}

#[test]
fn test_parse_multi_channel() {
    let midi_path = "tests/test_data/multi_channel.mid";

    // Parse the MIDI file
    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Check metadata
    assert_eq!(midi_data.ticks_per_beat, 480);
    assert_eq!(midi_data.tempo_bpm, 120.0);

    // Should have 6 events (3 note on, 3 note off)
    assert_eq!(midi_data.events.len(), 6);

    // Verify we have notes on different channels
    let note_ons: Vec<_> = midi_data
        .events
        .iter()
        .filter_map(|e| {
            if let MidiEvent::NoteOn { channel, note, .. } = e {
                Some((*channel, *note))
            } else {
                None
            }
        })
        .collect();

    assert_eq!(note_ons.len(), 3, "Should have 3 note on events");

    // Verify channels 0, 1, 2 are present with notes C(60), E(64), G(67)
    assert!(
        note_ons.contains(&(0, 60)),
        "Should have channel 0 with note 60"
    );
    assert!(
        note_ons.contains(&(1, 64)),
        "Should have channel 1 with note 64"
    );
    assert!(
        note_ons.contains(&(2, 67)),
        "Should have channel 2 with note 67"
    );
}

#[test]
fn test_end_to_end_multi_channel() {
    use std::env;

    let midi_path = "tests/test_data/multi_channel.mid";
    let temp_dir = env::temp_dir();
    let events_json_path = temp_dir.join("e2e_multi_channel_events.json");
    let ym2151_json_path = temp_dir.join("e2e_multi_channel_ym2151.json");

    // Pass A: Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Verify we have notes on different channels
    let channels_used: HashSet<u8> = midi_data
        .events
        .iter()
        .filter_map(|e| match e {
            MidiEvent::NoteOn { channel, .. } => Some(*channel),
            _ => None,
        })
        .collect();

    assert_eq!(channels_used.len(), 3, "Should use 3 different channels");
    assert!(channels_used.contains(&0));
    assert!(channels_used.contains(&1));
    assert!(channels_used.contains(&2));

    // Save events JSON
    save_midi_events_json(&midi_data, events_json_path.to_str().unwrap())
        .expect("Failed to save events JSON");
    assert!(events_json_path.exists());

    // Pass B: Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Verify Pass B output has events for all channels
    assert!(ym2151_log.event_count > 0);

    // Check that we have register writes for at least 3 distinct YM2151 channels.
    // We infer chip channels from KC register writes (0x28..0x2F -> channels 0..7).
    let mut ym_channels_with_kc: HashSet<u8> = HashSet::new();
    for e in &ym2151_log.events {
        if e.addr.starts_with("0x2") && e.addr.len() == 4 {
            if let Ok(reg_val) = u8::from_str_radix(&e.addr[2..], 16) {
                if (0x28..=0x2F).contains(&reg_val) {
                    let ch = reg_val - 0x28;
                    ym_channels_with_kc.insert(ch);
                }
            }
        }
    }

    assert!(
        ym_channels_with_kc.len() >= 3,
        "Expected at least 3 distinct YM2151 channels with KC writes, got channels: {:?}",
        ym_channels_with_kc
    );

    // Save YM2151 log JSON
    save_ym2151_log(&ym2151_log, ym2151_json_path.to_str().unwrap())
        .expect("Failed to save YM2151 log");
    assert!(ym2151_json_path.exists());

    // Verify YM2151 JSON structure
    let json_content = fs::read_to_string(&ym2151_json_path).expect("Failed to read YM2151 JSON");
    let parsed: serde_json::Value =
        serde_json::from_str(&json_content).expect("Invalid JSON format");

    assert!(parsed.get("event_count").is_some());
    assert!(parsed.get("events").is_some());

    // Clean up
    let _ = fs::remove_file(events_json_path);
    let _ = fs::remove_file(ym2151_json_path);
}

/// Test that tempo changes are correctly reflected in YM2151 timing
#[test]
fn test_tempo_change_timing_accuracy() {
    use smf_to_ym2151log::midi::{
        ticks_to_seconds_with_tempo_map, MidiData, MidiEvent, TempoChange,
    };
    use smf_to_ym2151log::ym2151::convert_to_ym2151_log;

    // Create a test MIDI file with tempo change
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            // Tempo starts at 120 BPM
            MidiEvent::Tempo {
                ticks: 0,
                tempo_bpm: 120.0,
            },
            // First note at tick 0
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            },
            // First note off at tick 480 (1 beat at 120 BPM = 0.5 seconds)
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 0,
                note: 60,
            },
            // Tempo changes to 60 BPM at tick 480
            MidiEvent::Tempo {
                ticks: 480,
                tempo_bpm: 60.0,
            },
            // Second note at tick 480
            MidiEvent::NoteOn {
                ticks: 480,
                channel: 0,
                note: 62,
                velocity: 100,
            },
            // Second note off at tick 960 (1 beat at 60 BPM = 1.0 second after tempo change)
            MidiEvent::NoteOff {
                ticks: 960,
                channel: 0,
                note: 62,
            },
        ],
    };

    // Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert");

    // Find the note on/off events
    let note_events: Vec<_> = ym2151_log
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time > 0.001)
        .collect();

    // First note off should be at tick 480
    // At 120 BPM: 480 ticks = 0.5 seconds
    // With polyphony analysis, channel allocation may vary - just check for note off events
    let first_note_off = note_events
        .iter()
        .find(|e| e.data.starts_with("0x0") && e.time > 0.001 && e.time <= 0.51)
        .expect("Should have first note off");
    assert!(
        first_note_off.time >= 0.49 && first_note_off.time <= 0.51,
        "First note off timing should be around 0.5 seconds, got {}",
        first_note_off.time
    );

    // Second note on should also be at tick 480 (same time as tempo change)
    let second_note_on = note_events
        .iter()
        .find(|e| e.data.starts_with("0x7") && e.time >= 0.49 && e.time <= 0.51)
        .expect("Should have second note on at tempo change");
    assert!(
        second_note_on.time >= 0.49 && second_note_on.time <= 0.51,
        "Second note on timing should be around 0.5 seconds, got {}",
        second_note_on.time
    );

    // Second note off should be at tick 960
    // First 480 ticks at 120 BPM = 0.5 seconds
    // Next 480 ticks at 60 BPM = 1.0 second
    // Total = 1.5 seconds
    let second_note_off = note_events
        .iter()
        .filter(|e| e.data.starts_with("0x0"))
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap())
        .expect("Should have second note off");
    assert!(
        second_note_off.time >= 1.49 && second_note_off.time <= 1.51,
        "Second note off timing should be around 1.5 seconds, got {}",
        second_note_off.time
    );

    // Verify using the tempo map function directly
    let tempo_map = vec![
        TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        },
        TempoChange {
            tick: 480,
            tempo_bpm: 60.0,
        },
    ];

    let time_at_960 = ticks_to_seconds_with_tempo_map(960, 480, &tempo_map);
    assert!(
        (time_at_960 - 1.5).abs() < 0.001,
        "Tempo map calculation should match expected value of 1.5 seconds, got {}",
        time_at_960
    );
}
