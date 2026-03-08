//! Integration tests for tone loading and program change handling

use smf_to_ym2151log::midi::{parse_midi_file, MidiEvent};
use smf_to_ym2151log::ym2151::convert_to_ym2151_log;

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
    } else {
        // If the file exists, that's also fine for this test
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
    // Check for RL_FB_CONNECT register writes (0x20-0x27)
    let tone_change_events: Vec<_> = log
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.addr.len() == 4)
        .collect();

    // Should have writes for init and both program changes
    assert!(
        tone_change_events.len() >= 3,
        "Should have tone settings from init and both program changes"
    );
}

/// Regression test: attachment tone for program 0 must be applied even when
/// the MIDI file contains no explicit Program Change event.
#[test]
fn test_attachment_tone_applied_without_program_change_event() {
    use smf_to_ym2151log::midi::{MidiData, MidiEvent};
    use smf_to_ym2151log::ConversionOptions;
    use smf_to_ym2151log::ym2151::convert_to_ym2151_log_with_options;

    // Build attachment options with a tone for program 0 containing a distinctive register
    let attachment_json = br#"[
      {
        "ProgramChange": 0,
        "Tone": {
          "events": [
            { "time": 0, "addr": "0x20", "data": "0xAB" }
          ]
        }
      }
    ]"#;
    let options = ConversionOptions::from_attachment_bytes(Some(attachment_json)).unwrap();
    // Verify tone was parsed into the options
    assert!(
        options.tones.contains_key(&0),
        "Tone for program 0 should be in tones map"
    );

    // MIDI data without any Program Change event
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

    let result = convert_to_ym2151_log_with_options(&midi_data, &options);
    assert!(result.is_ok(), "Conversion should succeed");

    let log = result.unwrap();

    // The distinctive register value 0xAB must appear in the output log at some
    // channel-adjusted address in the 0x20..=0x27 range (apply_tone_to_channel adjusts
    // the address based on the allocated YM2151 channel).
    let has_distinctive_value = log.events.iter().any(|e| {
        if e.data != "0xAB" {
            return false;
        }
        if let Some(hex) = e.addr.strip_prefix("0x") {
            matches!(u8::from_str_radix(hex, 16), Ok(addr) if (0x20..=0x27).contains(&addr))
        } else {
            false
        }
    });
    assert!(
        has_distinctive_value,
        "Attachment tone register write with data=0xAB at addr=0x20..=0x27 must appear in the log \
         even without an explicit Program Change event"
    );
}
