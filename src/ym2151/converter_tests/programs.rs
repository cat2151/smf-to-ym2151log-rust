//! Program change tests for YM2151 converter
use super::*;

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
    // Look for RL_FB_CONNECT register writes (0x20-0x27)
    let tone_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.addr.len() == 4 && e.time < 0.001)
        .collect();

    // Should have 2 writes: one from init, one from program change
    assert!(
        tone_events.len() >= 2,
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
fn test_convert_program_change_with_attachment_tone() {
    // Program change should use tone definitions supplied via attachment JSON
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 99,
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

    let mut options = ConversionOptions::default();
    options.tones.insert(
        99,
        ToneDefinition {
            events: vec![Ym2151Event {
                time: 0.0,
                addr: "0x20".to_string(),
                data: "0xAB".to_string(),
            }],
            ..ToneDefinition::default()
        },
    );

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // 8 KEY OFF + 26 init + 1 attachment tone + note on (3) + note off (1) = 39
    assert_eq!(result.event_count, 39);

    let has_custom_tone = result.events.iter().any(|e| e.data == "0xAB");
    assert!(
        has_custom_tone,
        "Attachment tone definition should be applied for program 99"
    );
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
    // Check for RL_FB_CONNECT register writes at time 0
    let tone_events_time_0: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.addr.len() == 4 && e.time < 0.001)
        .collect();
    assert!(
        tone_events_time_0.len() >= 2,
        "Should have init + program 10 tone events"
    ); // init + program 10

    // Second program change should be at a different time
    let tone_events_later: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.addr.len() == 4 && e.time > 0.001)
        .collect();
    assert!(
        !tone_events_later.is_empty(),
        "Should have tone change at later time"
    );
}
