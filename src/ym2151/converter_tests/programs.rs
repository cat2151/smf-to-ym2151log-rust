//! Program change and drum channel tests for YM2151 converter
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
        .filter(|e| e.addr == "0x28" && e.time < 0.001)
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
        .find(|e| e.addr == "0x08" && e.data == "0x78" && e.time < 0.001)
        .expect("Should have KEY ON for channel 0");
    assert_eq!(key_on.data, "0x78"); // 0x78 = all operators on, channel 0
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
        .find(|e| e.addr == "0x28" && e.time < 0.001)
        .expect("Drum should use YM2151 channel 0");
    assert!(drum_kc.data.starts_with("0x"));

    // Verify MIDI channel 0 uses YM2151 channel 1
    let ch0_kc = result
        .events
        .iter()
        .find(|e| e.addr == "0x29" && e.time < 0.001)
        .expect("MIDI ch 0 should use YM2151 channel 1");
    assert!(ch0_kc.data.starts_with("0x"));

    // Verify MIDI channel 1 uses YM2151 channel 2
    let ch1_kc = result
        .events
        .iter()
        .find(|e| e.addr == "0x2A" && e.time < 0.001)
        .expect("MIDI ch 1 should use YM2151 channel 2");
    assert!(ch1_kc.data.starts_with("0x"));

    // Verify KEY ON events are in the correct order (drum first)
    let key_on_events: Vec<&Ym2151Event> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time < 0.001 && e.data.starts_with("0x7"))
        .collect();

    // Should have 3 KEY ON events
    assert_eq!(key_on_events.len(), 3);

    // First KEY ON should be channel 0 (drum)
    assert_eq!(key_on_events[0].data, "0x78"); // Channel 0
}
