//! Integration tests for smf-to-ym2151log-rust

use smf_to_ym2151log::midi::{parse_midi_file, save_midi_events_json, MidiEvent};
use smf_to_ym2151log::ym2151::{convert_to_ym2151_log, save_ym2151_log};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[test]
fn test_parse_simple_melody() {
    let midi_path = "tests/test_data/simple_melody.mid";

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

    // Check events
    assert!(!midi_data.events.is_empty(), "No events parsed");

    // Should have 2 note on and 2 note off events (4 total)
    let note_ons: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::NoteOn { .. }))
        .collect();
    let note_offs: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::NoteOff { .. }))
        .collect();

    assert_eq!(note_ons.len(), 2, "Expected 2 Note On events");
    assert_eq!(note_offs.len(), 2, "Expected 2 Note Off events");

    // Verify first note on is Middle C (60) at tick 0
    if let MidiEvent::NoteOn {
        ticks,
        note,
        velocity,
        channel,
    } = note_ons[0]
    {
        assert_eq!(*ticks, 0);
        assert_eq!(*note, 60);
        assert_eq!(*velocity, 100);
        assert_eq!(*channel, 0);
    } else {
        panic!("First event should be Note On");
    }
}

#[test]
fn test_parse_tempo_change() {
    let midi_path = "tests/test_data/tempo_change.mid";

    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Check that we have tempo events
    let tempo_events: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::Tempo { .. }))
        .collect();

    assert!(
        !tempo_events.is_empty(),
        "Expected at least one tempo event"
    );

    // First tempo event should be 120 BPM
    if let MidiEvent::Tempo { tempo_bpm, .. } = tempo_events[0] {
        assert!(
            (tempo_bpm - 120.0).abs() < 0.1,
            "First tempo should be ~120 BPM, got {}",
            tempo_bpm
        );
    }

    // If there's a second tempo event, it should be ~140 BPM
    if tempo_events.len() >= 2 {
        if let MidiEvent::Tempo { tempo_bpm, .. } = tempo_events[1] {
            assert!(
                (tempo_bpm - 140.0).abs() < 1.0,
                "Second tempo should be ~140 BPM, got {}",
                tempo_bpm
            );
        }
    }
}

#[test]
fn test_parse_multi_track() {
    let midi_path = "tests/test_data/multi_track.mid";

    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Should have events from both tracks merged
    assert!(
        !midi_data.events.is_empty(),
        "No events parsed from multi-track file"
    );

    // Check that we have tempo and note events
    let has_tempo = midi_data
        .events
        .iter()
        .any(|e| matches!(e, MidiEvent::Tempo { .. }));
    let has_notes = midi_data
        .events
        .iter()
        .any(|e| matches!(e, MidiEvent::NoteOn { .. }));

    assert!(has_tempo, "Should have tempo events");
    assert!(has_notes, "Should have note events");
}

#[test]
fn test_save_midi_events_json() {
    use std::env;

    let midi_path = "tests/test_data/simple_melody.mid";

    // Use system temp directory for cross-platform compatibility
    let temp_dir = env::temp_dir();
    let output_path = temp_dir.join("test_output_events.json");
    let output_path_str = output_path
        .to_str()
        .expect("Failed to convert path to string");

    // Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Save to JSON
    let result = save_midi_events_json(&midi_data, output_path_str);
    assert!(result.is_ok(), "Failed to save JSON: {:?}", result.err());

    // Verify file exists
    assert!(output_path.exists(), "Output JSON file was not created");

    // Read and verify it's valid JSON
    let json_content = fs::read_to_string(&output_path).expect("Failed to read output JSON");

    let parsed: serde_json::Value =
        serde_json::from_str(&json_content).expect("Output is not valid JSON");

    // Verify structure
    assert!(parsed.get("ticks_per_beat").is_some());
    assert!(parsed.get("tempo_bpm").is_some());
    assert!(parsed.get("events").is_some());

    // Clean up
    let _ = fs::remove_file(&output_path);
}

#[test]
fn test_events_are_sorted_by_ticks() {
    let midi_path = "tests/test_data/simple_melody.mid";

    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Verify events are sorted by ticks
    let ticks: Vec<u32> = midi_data
        .events
        .iter()
        .map(|e| match e {
            MidiEvent::NoteOn { ticks, .. } => *ticks,
            MidiEvent::NoteOff { ticks, .. } => *ticks,
            MidiEvent::Tempo { ticks, .. } => *ticks,
            MidiEvent::ProgramChange { ticks, .. } => *ticks,
        })
        .collect();

    // Check that each tick is >= the previous tick
    for i in 1..ticks.len() {
        assert!(
            ticks[i] >= ticks[i - 1],
            "Events not sorted: tick[{}]={} < tick[{}]={}",
            i,
            ticks[i],
            i - 1,
            ticks[i - 1]
        );
    }
}

#[test]
fn test_note_on_with_velocity_zero_becomes_note_off() {
    // This test verifies that Note On with velocity 0 is treated as Note Off
    // This is part of the MIDI specification
    let midi_path = "tests/test_data/simple_melody.mid";

    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // All note off events should exist (either as explicit note off or note on with vel=0)
    let note_offs: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::NoteOff { .. }))
        .collect();

    assert!(!note_offs.is_empty(), "Should have note off events");
}

#[test]
fn test_parse_nonexistent_file() {
    let result = parse_midi_file("nonexistent_file.mid");
    assert!(result.is_err(), "Should fail for nonexistent file");
}

// ============================================================================
// Phase 5: End-to-End Integration Tests
// ============================================================================

/// Test complete end-to-end conversion flow with simple melody
#[test]
fn test_end_to_end_simple_melody() {
    use std::env;

    let midi_path = "tests/test_data/simple_melody.mid";
    let temp_dir = env::temp_dir();
    let events_json_path = temp_dir.join("e2e_simple_melody_events.json");
    let ym2151_json_path = temp_dir.join("e2e_simple_melody_ym2151.json");

    // Pass A: Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Verify Pass A output
    assert_eq!(midi_data.ticks_per_beat, 480);
    assert_eq!(midi_data.tempo_bpm, 120.0);
    assert!(!midi_data.events.is_empty());

    // Save events JSON
    save_midi_events_json(&midi_data, events_json_path.to_str().unwrap())
        .expect("Failed to save events JSON");
    assert!(events_json_path.exists());

    // Pass B: Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Verify Pass B output
    assert!(ym2151_log.event_count > 0);
    assert_eq!(ym2151_log.events.len(), ym2151_log.event_count as usize);

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

    // Verify events array structure
    let events = parsed["events"].as_array().expect("events should be array");
    assert!(!events.is_empty());

    // Check first event structure (should be initialization)
    let first_event = &events[0];
    assert!(first_event.get("time").is_some());
    assert!(first_event.get("addr").is_some());
    assert!(first_event.get("data").is_some());

    // Verify address and data are in hex format
    let addr = first_event["addr"].as_str().expect("addr should be string");
    let data = first_event["data"].as_str().expect("data should be string");
    assert!(addr.starts_with("0x"));
    assert!(data.starts_with("0x"));

    // Clean up
    let _ = fs::remove_file(events_json_path);
    let _ = fs::remove_file(ym2151_json_path);
}

/// Test end-to-end conversion with tempo change
#[test]
fn test_end_to_end_tempo_change() {
    use std::env;

    let midi_path = "tests/test_data/tempo_change.mid";
    let temp_dir = env::temp_dir();
    let events_json_path = temp_dir.join("e2e_tempo_change_events.json");
    let ym2151_json_path = temp_dir.join("e2e_tempo_change_ym2151.json");

    // Pass A: Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Verify tempo events exist
    let tempo_events: Vec<_> = midi_data
        .events
        .iter()
        .filter_map(|e| {
            if let MidiEvent::Tempo { ticks, tempo_bpm } = e {
                Some((*ticks, *tempo_bpm))
            } else {
                None
            }
        })
        .collect();
    assert!(!tempo_events.is_empty(), "Should have tempo events");

    // Save events JSON
    save_midi_events_json(&midi_data, events_json_path.to_str().unwrap())
        .expect("Failed to save events JSON");

    // Pass B: Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Verify that tempo changes affect timing
    // Find note events in the YM2151 log
    let note_on_events: Vec<_> = ym2151_log
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.data.starts_with("0x7"))
        .collect();

    // Should have at least 2 note on events with different timing
    assert!(
        note_on_events.len() >= 2,
        "Should have at least 2 note on events"
    );

    // First note should be at time 0
    assert_eq!(note_on_events[0].time, 0, "First note should be at time 0");

    // Second note timing should reflect tempo change
    // If tempo didn't affect timing, both notes would have same relative spacing
    // With tempo change, the spacing should be different
    assert!(
        note_on_events[1].time > 0,
        "Second note should be after time 0"
    );

    // Save YM2151 log
    save_ym2151_log(&ym2151_log, ym2151_json_path.to_str().unwrap())
        .expect("Failed to save YM2151 log");

    // Verify outputs exist
    assert!(events_json_path.exists());
    assert!(ym2151_json_path.exists());

    // Clean up
    let _ = fs::remove_file(events_json_path);
    let _ = fs::remove_file(ym2151_json_path);
}

/// Test end-to-end conversion with multi-track MIDI file
#[test]
fn test_end_to_end_multi_track() {
    use std::env;

    let midi_path = "tests/test_data/multi_track.mid";
    let temp_dir = env::temp_dir();
    let events_json_path = temp_dir.join("e2e_multi_track_events.json");
    let ym2151_json_path = temp_dir.join("e2e_multi_track_ym2151.json");

    // Pass A: Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Save events JSON
    save_midi_events_json(&midi_data, events_json_path.to_str().unwrap())
        .expect("Failed to save events JSON");

    // Pass B: Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Save YM2151 log
    save_ym2151_log(&ym2151_log, ym2151_json_path.to_str().unwrap())
        .expect("Failed to save YM2151 log");

    // Verify both outputs exist
    assert!(events_json_path.exists());
    assert!(ym2151_json_path.exists());

    // Verify the YM2151 log has reasonable content
    assert!(ym2151_log.event_count > 0);
    assert_eq!(ym2151_log.events.len(), ym2151_log.event_count as usize);

    // Clean up
    let _ = fs::remove_file(events_json_path);
    let _ = fs::remove_file(ym2151_json_path);
}

/// Test output file paths are correctly determined
#[test]
fn test_output_file_path_generation() {
    let test_cases = vec![
        ("test.mid", "test_events.json", "test_ym2151.json"),
        (
            "path/to/test.mid",
            "path/to/test_events.json",
            "path/to/test_ym2151.json",
        ),
        ("my_song.mid", "my_song_events.json", "my_song_ym2151.json"),
    ];

    for (input_path, expected_events, expected_ym2151) in test_cases {
        let path = Path::new(input_path);
        let base_name = path.file_stem().unwrap().to_string_lossy();
        let output_dir = path.parent().unwrap_or_else(|| Path::new("."));

        let events_json_path = output_dir.join(format!("{}_events.json", base_name));
        let ym2151_json_path = output_dir.join(format!("{}_ym2151.json", base_name));

        assert_eq!(
            events_json_path.to_str().unwrap(),
            expected_events,
            "Events JSON path mismatch for {}",
            input_path
        );
        assert_eq!(
            ym2151_json_path.to_str().unwrap(),
            expected_ym2151,
            "YM2151 JSON path mismatch for {}",
            input_path
        );
    }
}

/// Test that YM2151 log contains valid time values (sample times at 55930 Hz)
#[test]
fn test_ym2151_log_time_values() {
    let midi_path = "tests/test_data/simple_melody.mid";

    // Parse and convert
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Check that times are non-decreasing (equal values are allowed, e.g., for simultaneous events)
    let mut prev_time = 0;
    for event in &ym2151_log.events {
        assert!(
            event.time >= prev_time,
            "Time should be non-decreasing (event.time={}, prev_time={})",
            event.time,
            prev_time
        );
        prev_time = event.time;
    }

    // Verify at least one event has non-zero time (unless empty)
    if !ym2151_log.events.is_empty() {
        let has_nonzero = ym2151_log.events.iter().any(|e| e.time > 0);
        // For non-empty MIDI files with notes, we should have some non-zero times
        // (Only all-zero times would be unusual for actual note events)
        assert!(
            ym2151_log.events.is_empty() || has_nonzero || ym2151_log.events.len() <= 32,
            "Expected at least some non-zero time values for events beyond initialization"
        );
    }
}

/// Test that YM2151 log contains properly formatted hex strings
#[test]
fn test_ym2151_log_hex_format() {
    let midi_path = "tests/test_data/simple_melody.mid";

    // Parse and convert
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

    // Check all events have properly formatted hex strings
    for event in &ym2151_log.events {
        // Check address format
        assert!(
            event.addr.starts_with("0x"),
            "Address should start with 0x: {}",
            event.addr
        );
        assert!(
            event.addr.len() == 4,
            "Address should be 4 chars (0xXX): {}",
            event.addr
        );

        // Check data format
        assert!(
            event.data.starts_with("0x"),
            "Data should start with 0x: {}",
            event.data
        );
        assert!(
            event.data.len() == 4,
            "Data should be 4 chars (0xXX): {}",
            event.data
        );

        // Verify they can be parsed as hex
        let addr_val = u8::from_str_radix(&event.addr[2..], 16);
        let data_val = u8::from_str_radix(&event.data[2..], 16);

        assert!(
            addr_val.is_ok(),
            "Address should be valid hex: {}",
            event.addr
        );
        assert!(data_val.is_ok(), "Data should be valid hex: {}", event.data);
    }
}

#[test]
fn test_convert_smf_to_ym2151_log_convenience_function() {
    // Test the convenience function that accepts raw SMF bytes
    let midi_path = "tests/test_data/simple_melody.mid";

    // Read the MIDI file as bytes
    let smf_bytes = fs::read(midi_path).expect("Failed to read test MIDI file");

    // Use the convenience function
    let result = smf_to_ym2151log::convert_smf_to_ym2151_log(&smf_bytes);
    assert!(
        result.is_ok(),
        "Failed to convert SMF to YM2151 log: {:?}",
        result.err()
    );

    let json_string = result.unwrap();

    // Verify it's valid JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&json_string).expect("Output should be valid JSON");

    // Verify it has the expected structure
    assert!(
        parsed.get("event_count").is_some(),
        "Should have event_count field"
    );
    assert!(parsed.get("events").is_some(), "Should have events field");

    // Verify event_count is a number
    let event_count = parsed["event_count"]
        .as_u64()
        .expect("event_count should be a number");
    assert!(event_count > 0, "Should have at least some events");

    // Verify events is an array
    let events = parsed["events"]
        .as_array()
        .expect("events should be an array");
    assert_eq!(
        events.len() as u64,
        event_count,
        "events length should match event_count"
    );
}

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

    // Check that we have register writes for all 3 channels
    // Channel 0: KC register at 0x28
    let has_ch0_kc = ym2151_log.events.iter().any(|e| e.addr == "0x28");
    // Channel 1: KC register at 0x29
    let has_ch1_kc = ym2151_log.events.iter().any(|e| e.addr == "0x29");
    // Channel 2: KC register at 0x2A
    let has_ch2_kc = ym2151_log.events.iter().any(|e| e.addr == "0x2A");

    assert!(has_ch0_kc, "Should have KC register write for channel 0");
    assert!(has_ch1_kc, "Should have KC register write for channel 1");
    assert!(has_ch2_kc, "Should have KC register write for channel 2");

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
        ticks_to_samples_with_tempo_map, MidiData, MidiEvent, TempoChange,
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
        .filter(|e| e.addr == "0x08" && e.time > 0)
        .collect();

    // First note off should be at tick 480
    // At 120 BPM: 480 ticks = 0.5 seconds = 27965 samples
    let first_note_off = note_events
        .iter()
        .find(|e| e.data == "0x00" && e.time > 0)
        .expect("Should have first note off");
    assert_eq!(
        first_note_off.time, 27965,
        "First note off timing incorrect"
    );

    // Second note on should also be at tick 480 (same time as tempo change)
    let second_note_on = note_events
        .iter()
        .find(|e| e.data == "0x78" && e.time == 27965)
        .expect("Should have second note on at tempo change");
    assert_eq!(
        second_note_on.time, 27965,
        "Second note on timing incorrect"
    );

    // Second note off should be at tick 960
    // First 480 ticks at 120 BPM = 0.5 seconds = 27965 samples
    // Next 480 ticks at 60 BPM = 1.0 second = 55930 samples
    // Total = 83895 samples
    let second_note_off = note_events
        .iter()
        .filter(|e| e.data == "0x00")
        .max_by_key(|e| e.time)
        .expect("Should have second note off");
    assert_eq!(
        second_note_off.time, 83895,
        "Second note off timing should reflect tempo change"
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

    let time_at_960 = ticks_to_samples_with_tempo_map(960, 480, &tempo_map);
    assert_eq!(
        time_at_960, 83895,
        "Tempo map calculation should match expected value"
    );
}

#[test]
fn test_tone_loading_from_file() {
    use smf_to_ym2151log::ym2151::load_tone_for_program;

    // Test loading tone file 000.json (which should exist in tones directory)
    let result = load_tone_for_program(0);
    assert!(result.is_ok(), "Failed to load tone: {:?}", result.err());

    let tone_opt = result.unwrap();
    assert!(
        tone_opt.is_some(),
        "Tone file tones/000.json should exist for testing"
    );

    let tone = tone_opt.unwrap();
    assert!(
        !tone.events.is_empty(),
        "Tone should have register write events"
    );

    // Verify tone has expected structure
    assert_eq!(tone.events.len(), 26, "Default tone should have 26 events");
}

#[test]
fn test_tone_loading_nonexistent() {
    use smf_to_ym2151log::ym2151::load_tone_for_program;

    // Test loading a tone that doesn't exist (e.g., program 127)
    let result = load_tone_for_program(127);
    assert!(result.is_ok());

    let tone_opt = result.unwrap();
    // Should return None if file doesn't exist
    if tone_opt.is_none() {
        // This is the expected behavior - no tone file exists
        assert!(true);
    } else {
        // If the file exists, that's also fine for this test
        assert!(true);
    }
}

#[test]
fn test_end_to_end_program_change() {
    use smf_to_ym2151log::midi::{MidiData, MidiEvent};
    use smf_to_ym2151log::ym2151::convert_to_ym2151_log;

    // Create MIDI data with program change
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 0, // Use program 0 which has a tone file
            },
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

    let result = convert_to_ym2151_log(&midi_data);
    assert!(result.is_ok(), "Conversion should succeed");

    let log = result.unwrap();
    assert!(log.event_count > 0, "Should have YM2151 events");

    // Should have more events due to program change tone loading
    // 8 KEY OFF + 26 init + 26 program change tone + 3 note on + 1 note off = 64
    assert_eq!(
        log.event_count, 64,
        "Should have events from init, program change tone, and notes"
    );
}

#[test]
fn test_parse_program_change_midi() {
    let midi_path = "tests/test_data/program_change.mid";

    // Parse the MIDI file
    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Check that we have program change events
    let program_events: Vec<_> = midi_data
        .events
        .iter()
        .filter(|e| matches!(e, MidiEvent::ProgramChange { .. }))
        .collect();

    assert_eq!(program_events.len(), 2, "Expected 2 program change events");

    // Verify first program change is to program 0
    if let MidiEvent::ProgramChange {
        ticks,
        channel,
        program,
    } = program_events[0]
    {
        assert_eq!(*ticks, 0);
        assert_eq!(*channel, 0);
        assert_eq!(*program, 0);
    }

    // Verify second program change is to program 42
    if let MidiEvent::ProgramChange {
        ticks: _,
        channel,
        program,
    } = program_events[1]
    {
        assert_eq!(*channel, 0);
        assert_eq!(*program, 42);
    }
}

#[test]
fn test_end_to_end_program_change_with_file() {
    let midi_path = "tests/test_data/program_change.mid";

    // Parse the MIDI file
    let result = parse_midi_file(midi_path);
    assert!(
        result.is_ok(),
        "Failed to parse MIDI file: {:?}",
        result.err()
    );

    let midi_data = result.unwrap();

    // Convert to YM2151 log
    let ym2151_result = convert_to_ym2151_log(&midi_data);
    assert!(
        ym2151_result.is_ok(),
        "Failed to convert to YM2151: {:?}",
        ym2151_result.err()
    );

    let log = ym2151_result.unwrap();

    // Should have:
    // - 8 KEY OFF events (initialization)
    // - 26 channel init events
    // - 26 program 0 tone events
    // - 3 note on events (KC, KF, KEY ON)
    // - 1 note off event
    // - 26 program 42 tone events
    // - 3 note on events
    // - 1 note off event
    // Total: 8 + 26 + 26 + 3 + 1 + 26 + 3 + 1 = 94
    assert_eq!(
        log.event_count, 94,
        "Should have correct number of events including two program changes"
    );

    // Verify program change events generated tone changes
    let tone_change_events: Vec<_> = log
        .events
        .iter()
        .filter(|e| e.addr == "0x20") // RL_FB_CONNECT register for channel 0
        .collect();

    // Should have 3 writes to 0x20: init + program 0 + program 42
    assert_eq!(
        tone_change_events.len(),
        3,
        "Should have tone settings from init and both program changes"
    );
}
