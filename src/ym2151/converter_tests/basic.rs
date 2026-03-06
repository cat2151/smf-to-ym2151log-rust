//! Basic conversion tests for YM2151 converter
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
    // MIDI channel 0 with polyphony 1 gets YM2151 channel 0 (no drum channel present)
    // KC register is at 0x28 for channel 0
    // There should be exactly one KC write from the Note On event
    let kc_events: Vec<&Ym2151Event> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.data == "0x2E")
        .collect();

    assert_eq!(
        kc_events.len(),
        1,
        "Should have exactly one KC register write for Middle C"
    );

    // Middle C (MIDI 60) should map to KC 0x2E (Octave 2, Note C)
    assert_eq!(kc_events[0].data, "0x2E");
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
        .find(|e| e.addr == "0x08" && e.data == "0x78" && e.time < 0.001) // Channel 0 now
        .expect("Should have Note On KEY event at time 0");
    assert!(note_on_event.time < 0.001);
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

    // With polyphony analysis, overlapping notes mean this channel needs 2 voices
    // Init: 8 KEY OFF + (26 * 2 channels) + 2 Note Ons (6) + 2 Note Offs (2)
    //     = 8 + 52 + 6 + 2 = 68
    assert_eq!(result.event_count, 68);
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

    // Find KEY ON event - MIDI channel 0 maps to YM2151 channel 0 (no drums present)
    let key_on = result
        .events
        .iter()
        .find(|e| e.addr == "0x08" && e.data == "0x78")
        .expect("Should have KEY ON event");

    // 0x78 = all operators on, channel 0 (MIDI channel 0)
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
    // MIDI channel 0 maps to YM2151 channel 0 (no drums present)
    let key_off = result
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time > 0.001)
        .find(|e| e.data == "0x00") // Channel 0
        .expect("Should have KEY OFF event");

    // 0x00 = all operators off, channel 0
    assert_eq!(key_off.data, "0x00");
}
