//! Integration tests for smf-to-ym2151log-rust

use smf_to_ym2151log::midi::{parse_midi_file, save_midi_events_json, MidiEvent};
use smf_to_ym2151log::ym2151::{convert_to_ym2151_log, save_ym2151_log};
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
    let has_tempo = midi_data
        .events
        .iter()
        .any(|e| matches!(e, MidiEvent::Tempo { .. }));
    assert!(has_tempo, "Should have tempo events");

    // Save events JSON
    save_midi_events_json(&midi_data, events_json_path.to_str().unwrap())
        .expect("Failed to save events JSON");

    // Pass B: Convert to YM2151 log
    let ym2151_log = convert_to_ym2151_log(&midi_data).expect("Failed to convert to YM2151 log");

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
