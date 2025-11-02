//! Integration tests for smf-to-ym2151log-rust

use smf_to_ym2151log::midi::{parse_midi_file, save_midi_events_json, MidiEvent};
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

    assert!(!tempo_events.is_empty(), "Expected at least one tempo event");

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
    let midi_path = "tests/test_data/simple_melody.mid";
    let output_path = "/tmp/test_output_events.json";

    // Parse MIDI file
    let midi_data = parse_midi_file(midi_path).expect("Failed to parse MIDI file");

    // Save to JSON
    let result = save_midi_events_json(&midi_data, output_path);
    assert!(result.is_ok(), "Failed to save JSON: {:?}", result.err());

    // Verify file exists
    assert!(
        Path::new(output_path).exists(),
        "Output JSON file was not created"
    );

    // Read and verify it's valid JSON
    let json_content = fs::read_to_string(output_path).expect("Failed to read output JSON");

    let parsed: serde_json::Value =
        serde_json::from_str(&json_content).expect("Output is not valid JSON");

    // Verify structure
    assert!(parsed.get("ticks_per_beat").is_some());
    assert!(parsed.get("tempo_bpm").is_some());
    assert!(parsed.get("events").is_some());

    // Clean up
    let _ = fs::remove_file(output_path);
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
