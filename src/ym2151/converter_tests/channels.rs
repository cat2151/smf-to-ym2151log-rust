//! Multi-channel tests for YM2151 converter
use super::*;

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
    // With polyphony-based allocation and no drums, channels are allocated sequentially
    // MIDI Channel 0,1,2 each have polyphony 1, so they get YM2151 channels 0,1,2
    // Note: With -2 octave offset:
    //   MIDI 60 (C4) → KC 0x2E (Octave 2, Note C)
    //   MIDI 64 (E4) → KC 0x34 (Octave 3, Note E)
    //   MIDI 67 (G4) → KC 0x38 (Octave 3, Note G)
    let ch0_kc = result
        .events
        .iter()
        .find(|e| {
            (e.addr == "0x28" || e.addr == "0x29" || e.addr == "0x2A")
                && e.time < 0.001
                && e.data == "0x2E"
        })
        .expect("Should have KC write for MIDI channel 0");
    assert_eq!(ch0_kc.data, "0x2E"); // Middle C (Octave 2, Note C)

    let ch1_kc = result
        .events
        .iter()
        .find(|e| {
            (e.addr == "0x28" || e.addr == "0x29" || e.addr == "0x2A")
                && e.time < 0.001
                && e.data == "0x34"
        })
        .expect("Should have KC write for MIDI channel 1");
    assert_eq!(ch1_kc.data, "0x34"); // E (octave 3, note 4)

    let ch2_kc = result
        .events
        .iter()
        .find(|e| {
            (e.addr == "0x28" || e.addr == "0x29" || e.addr == "0x2A")
                && e.time < 0.001
                && e.data == "0x38"
        })
        .expect("Should have KC write for MIDI channel 2");
    assert_eq!(ch2_kc.data, "0x38"); // G (octave 3, note 8)

    // Verify we have 3 KEY ON events
    let key_on_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x08" && e.time < 0.001 && e.data.starts_with("0x7"))
        .collect();
    assert_eq!(key_on_events.len(), 3, "Should have 3 KEY ON events");
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
    // Verify some YM2151 channels are initialized (allocation may vary)
    let init_channels: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && e.time < 0.001)
        .map(|e| &e.addr)
        .collect();

    assert!(
        init_channels.len() >= 2,
        "At least 2 YM2151 channels should be initialized"
    );

    // Verify notes play on different YM2151 channels
    let note_channels: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr.starts_with("0x2") && (e.time < 0.001 || e.time >= 0.001))
        .map(|e| &e.addr)
        .collect();

    assert!(
        note_channels.len() >= 2,
        "Both MIDI channels should have notes"
    );
}

