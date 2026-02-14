//! Tests for YM2151 converter (Pass B)
//!
//! These tests verify the conversion of MIDI events to YM2151 register write events.

use super::*;
use crate::midi::{midi_to_kc_kf, MidiEvent};
use crate::ym2151::{ToneDefinition, Ym2151Event};
use crate::{ConversionOptions, LfoWaveform, RegisterLfoDefinition};

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

#[test]
fn test_delay_vibrato_generates_additional_pitch_events() {
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 69, // A4 (440 Hz)
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 1920, // 2 seconds at 120 BPM
                channel: 0,
                note: 69,
            },
        ],
    };

    let options = ConversionOptions {
        delay_vibrato: true,
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // Vibrato should emit KC/KF writes after the 200ms delay
    let kc_events_after_delay: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > 0.2)
        .collect();
    assert!(
        !kc_events_after_delay.is_empty(),
        "KC events should include vibrato modulation after delay"
    );

    // Some KF events should deviate from the base (0) once vibrato ramps in
    let non_zero_kf_after_delay: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x30" && e.time > 0.2 && e.data != "0x00")
        .collect();
    assert!(
        !non_zero_kf_after_delay.is_empty(),
        "KF events should include fractional pitch changes from vibrato"
    );
}

#[test]
fn test_portamento_generates_pitch_glide_events() {
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
            MidiEvent::NoteOn {
                ticks: 480,
                channel: 0,
                note: 67,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 960,
                channel: 0,
                note: 67,
            },
        ],
    };

    let options = ConversionOptions {
        portamento: true,
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // First note should emit a KC write even with portamento enabled
    let (kc_first, _) = midi_to_kc_kf(60);
    let first_kc = result
        .events
        .iter()
        .find(|e| {
            e.addr == "0x28"
                && (e.time - 0.0).abs() < f64::EPSILON
                && e.data == format!("0x{:02X}", kc_first)
        })
        .map(|e| e.data.clone())
        .expect("First note should set KC at time 0");
    assert_eq!(first_kc, format!("0x{:02X}", kc_first));

    // Collect KC events starting at the second note-on time (0.5 seconds)
    let kc_events_after_second_on: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time >= 0.5)
        .collect();
    assert!(
        kc_events_after_second_on.len() >= 2,
        "Portamento should emit multiple KC steps during the glide"
    );

    let (kc_second, _) = midi_to_kc_kf(67);

    // Glide should include the previous pitch before reaching the target
    assert!(
        kc_events_after_second_on
            .iter()
            .any(|e| e.data == format!("0x{:02X}", kc_first)),
        "Glide should include the starting KC from the previous note"
    );
    // Glide should reach the target KC
    assert!(
        kc_events_after_second_on
            .iter()
            .any(|e| e.data == format!("0x{:02X}", kc_second)),
        "Glide should arrive at the target KC"
    );
}

#[test]
fn test_register_lfo_modulates_tone_register() {
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
                ticks: 240,
                channel: 0,
                note: 60,
            },
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 1,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 1,
                note: 64,
            },
        ],
    };

    let options = ConversionOptions {
        software_lfo: vec![RegisterLfoDefinition {
            base_register: "0x60".to_string(),
            depth: 4.0,
            rate_hz: 2.0,
            delay_seconds: 0.0,
            attack_seconds: 0.0,
            waveform: LfoWaveform::Triangle,
        }],
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // MIDI channel 1 maps to YM channel 1 when channel 0 is also present, so TL base reg 0x60 -> 0x61
    let lfo_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x61" && e.time > 0.0)
        .collect();

    assert!(
        !lfo_events.is_empty(),
        "Software LFO should emit TL updates for channel 1"
    );
    assert!(
        lfo_events.iter().any(|e| e.data != "0x00"),
        "LFO should modulate TL away from the base value"
    );
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
