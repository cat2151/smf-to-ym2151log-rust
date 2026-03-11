//! Program attachment tests for YM2151 converter
use super::*;

#[test]
fn test_change_to_next_tone_skips_kc_kf_key_on_registers() {
    // Tone events that include KC (0x28), KF (0x30), and key-on (0x08) along with
    // a real tone register (TL 0x60).  The KC/KF/key-on differences must NOT produce
    // interpolation events; only TL should be interpolated.
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
                ticks: 4800,
                channel: 0,
                note: 60,
            },
        ],
    };

    let make_tone = |tl: &str, key_on: &str, kc: &str, kf: &str| ToneDefinition {
        events: vec![
            // Key-on register — must be ignored even when values differ between tones
            Ym2151Event {
                time: 0.0,
                addr: "0x08".to_string(),
                data: key_on.to_string(),
            },
            // KC register — must be ignored
            Ym2151Event {
                time: 0.0,
                addr: "0x28".to_string(),
                data: kc.to_string(),
            },
            // KF register — must be ignored
            Ym2151Event {
                time: 0.0,
                addr: "0x30".to_string(),
                data: kf.to_string(),
            },
            // TL register — must be interpolated
            Ym2151Event {
                time: 0.0,
                addr: "0x60".to_string(),
                data: tl.to_string(),
            },
        ],
        ..ToneDefinition::default()
    };

    // Use different key-on values (0x78 vs 0x38) so the 0x08 register would have been
    // interpolated before the fix, verifying that is_note_register(0x08) is exercised.
    let tone0 = make_tone("0x10", "0x78", "0x4E", "0x00"); // TL=0x10, key-on=0x78, KC=A4, KF=0
    let tone1 = make_tone("0x30", "0x38", "0x5E", "0x20"); // TL=0x30, key-on=0x38 (different!), KC different, KF different

    let options = ConversionOptions {
        program_attachments: vec![
            ProgramAttachment {
                program_change: 0,
                change_to_next_tone: true,
                change_to_next_tone_time: 5.0,
                ..ProgramAttachment::default()
            },
            ProgramAttachment {
                program_change: 1,
                ..ProgramAttachment::default()
            },
        ],
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone0);
            m.insert(1, tone1);
            m
        },
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // TL (0x60) differences must still be interpolated
    let tl_events: Vec<_> = result.events.iter().filter(|e| e.addr == "0x60").collect();
    assert!(
        tl_events.len() > 2,
        "TL register must still be interpolated; got {} events",
        tl_events.len()
    );

    // KC (0x28) must NOT receive interpolation events — it is note-related
    let kc_extra: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > 0.01)
        .collect();
    assert!(
        kc_extra.is_empty(),
        "KC register must NOT be interpolated; got {} unexpected KC events",
        kc_extra.len()
    );

    // 0x08 (key-on) must NOT receive interpolation events — values differ between
    // tone0 (0x78) and tone1 (0x38), so without the is_note_register guard the
    // interpolation loop would emit writes between those values.
    // Only the initial key-on write at t=0 and the key-off write are expected.
    let key_on_interpolated: Vec<_> = result
        .events
        .iter()
        .filter(|e| {
            if e.addr != "0x08" {
                return false;
            }
            // Any value strictly between 0x38 and 0x78 would be an interpolated write
            if let Some(hex) = e.data.strip_prefix("0x") {
                if let Ok(v) = u8::from_str_radix(hex, 16) {
                    return v > 0x38 && v < 0x78;
                }
            }
            false
        })
        .collect();
    assert!(
        key_on_interpolated.is_empty(),
        "Key-on register (0x08) must NOT be interpolated; got {} unexpected writes: {:?}",
        key_on_interpolated.len(),
        key_on_interpolated
    );

    // KF (0x30) must NOT receive interpolation events
    let kf_extra: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x30" && e.time > 0.01)
        .collect();
    assert!(
        kf_extra.is_empty(),
        "KF register must NOT be interpolated; got {} unexpected KF events",
        kf_extra.len()
    );
}

#[test]
fn test_change_to_next_tone_generates_interpolation_events() {
    // A 10-second song with program 0 and program 1 tones that differ in TL (0x60).
    // changeToNextTone should produce continuously changing register writes.
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
            // Last event at tick 9600 = 10 seconds at 120 BPM
            MidiEvent::NoteOff {
                ticks: 9600,
                channel: 0,
                note: 60,
            },
        ],
    };

    // Program 0 tone: TL operator 0 = 0x10; Program 1 tone: TL = 0x30 (delta = 32)
    let tone_program0 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(), // TL op0, ch0
            data: "0x10".to_string(),
        }],
        ..ToneDefinition::default()
    };
    let tone_program1 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(),
            data: "0x30".to_string(),
        }],
        ..ToneDefinition::default()
    };

    let options = ConversionOptions {
        program_attachments: vec![
            ProgramAttachment {
                program_change: 0,
                change_to_next_tone: true,
                change_to_next_tone_time: 5.0,
                ..ProgramAttachment::default()
            },
            ProgramAttachment {
                program_change: 1,
                ..ProgramAttachment::default()
            },
        ],
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone_program0);
            m.insert(1, tone_program1);
            m
        },
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // TL register 0x60 (channel 0) should have multiple writes as values interpolate
    let tl_events: Vec<_> = result.events.iter().filter(|e| e.addr == "0x60").collect();

    assert!(
        tl_events.len() > 2,
        "changeToNextTone must produce multiple TL register writes; got {}",
        tl_events.len()
    );

    // Values should include the start (0x10) and progress toward the end (0x30)
    let has_start_value = tl_events.iter().any(|e| e.data == "0x10");
    let has_mid_value = tl_events
        .iter()
        .any(|e| e.data != "0x10" && e.data != "0x30");
    assert!(has_start_value, "First interpolation step must write 0x10");
    assert!(
        has_mid_value,
        "Intermediate interpolated values must be written"
    );
}

#[test]
fn test_change_to_next_tone_disabled_produces_no_extra_events() {
    // When change_to_next_tone is false, no interpolation events should be generated.
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
                ticks: 9600,
                channel: 0,
                note: 60,
            },
        ],
    };

    let tone0 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(),
            data: "0x10".to_string(),
        }],
        ..ToneDefinition::default()
    };
    let tone1 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(),
            data: "0x30".to_string(),
        }],
        ..ToneDefinition::default()
    };

    let base_options = ConversionOptions {
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone0.clone());
            m.insert(1, tone1.clone());
            m
        },
        ..ConversionOptions::default()
    };
    let interpolation_options = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 0,
            change_to_next_tone: false, // explicitly disabled
            ..ProgramAttachment::default()
        }],
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone0);
            m.insert(1, tone1);
            m
        },
        ..ConversionOptions::default()
    };

    let result_base = convert_to_ym2151_log_with_options(&midi_data, &base_options).unwrap();
    let result_disabled =
        convert_to_ym2151_log_with_options(&midi_data, &interpolation_options).unwrap();

    assert_eq!(
        result_base.event_count, result_disabled.event_count,
        "Disabling changeToNextTone must not add extra events"
    );
}

#[test]
fn test_change_to_next_tone_requires_both_tones() {
    // If tone N+1 is not defined, no interpolation events should be generated.
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
                ticks: 9600,
                channel: 0,
                note: 60,
            },
        ],
    };

    let tone0 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x60".to_string(),
            data: "0x10".to_string(),
        }],
        ..ToneDefinition::default()
    };

    // Only tone 0 defined; tone 1 is missing
    let options_missing_next = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 0,
            change_to_next_tone: true,
            change_to_next_tone_time: 5.0,
            ..ProgramAttachment::default()
        }],
        tones: {
            let mut m = std::collections::HashMap::new();
            m.insert(0, tone0.clone());
            // No tone 1
            m
        },
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options_missing_next).unwrap();

    // Without tone 1, no interpolation events should be added.
    // The only TL writes should be from initialization (0x00) and tone0 application (0x10).
    // There should be no intermediate interpolated values between 0x10 and 0x30.
    let has_interpolated_values = result.events.iter().any(|e| {
        if e.addr != "0x60" {
            return false;
        }
        if let Some(hex) = e.data.strip_prefix("0x") {
            if let Ok(v) = u8::from_str_radix(hex, 16) {
                return v > 0x10 && v < 0x30;
            }
        }
        false
    });
    assert!(
        !has_interpolated_values,
        "Missing tone N+1 must not produce interpolation events with intermediate values"
    );
}

#[test]
fn test_program_attachment_delay_vibrato_applies_only_to_matching_program() {
    // Notes under program 0 should get vibrato; notes under program 1 should not.
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            // Program 0 note (2 seconds long — long enough for vibrato to activate)
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 0,
            },
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 69,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 1920, // 2 seconds at 120 BPM
                channel: 0,
                note: 69,
            },
            // Program 1 note on a second channel
            MidiEvent::ProgramChange {
                ticks: 1920,
                channel: 1,
                program: 1,
            },
            MidiEvent::NoteOn {
                ticks: 1920,
                channel: 1,
                note: 60,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 3840, // another 2 seconds
                channel: 1,
                note: 60,
            },
        ],
    };

    let options = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 0,
            delay_vibrato: true,
            ..ProgramAttachment::default()
        }],
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    // Program 0 note (note 69 / A4, channel 0 → YM KC register 0x28) should have
    // KC writes after the 200ms vibrato delay.
    let kc_ch0_after_delay: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > 0.2)
        .collect();
    assert!(
        !kc_ch0_after_delay.is_empty(),
        "Program 0 note should receive vibrato KC modulation"
    );

    // Program 1 note (channel 1 → YM KC register 0x29) must NOT have any KC writes
    // after the note starts at 2 s — vibrato is not enabled for program 1.
    let kc_ch1_after_start: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x29" && e.time > 2.0 + 0.2)
        .collect();
    assert!(
        kc_ch1_after_start.is_empty(),
        "Program 1 note must not receive vibrato KC modulation"
    );
}

#[test]
fn test_program_attachment_no_effects_entry_produces_no_extra_events() {
    // A ProgramAttachment with all effect flags at their defaults (no delay_vibrato,
    // portamento, software_lfo, pop_noise_envelope, or attack_continuation_fix) must
    // not crash and must not generate any vibrato/LFO/etc events.
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 5,
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

    // Attachment entry with no effects enabled (all flags remain at default)
    let options = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 5,
            ..ProgramAttachment::default()
        }],
        ..ConversionOptions::default()
    };

    // Should succeed without panicking
    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();
    assert!(result.event_count > 0);

    // No extra KC events from vibrato should be present
    let vibrato_kc: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > 0.2)
        .collect();
    assert!(
        vibrato_kc.is_empty(),
        "No-effects attachment must not produce vibrato events"
    );
}

#[test]
fn test_program_attachment_unmatched_program_produces_no_extra_events() {
    // An attachment for program 99 should do nothing when only program 0 is used.
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::ProgramChange {
                ticks: 0,
                channel: 0,
                program: 0,
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

    let options_with_attachment = ConversionOptions {
        program_attachments: vec![ProgramAttachment {
            program_change: 99, // not used in the MIDI
            delay_vibrato: true,
            ..ProgramAttachment::default()
        }],
        ..ConversionOptions::default()
    };

    let result_with =
        convert_to_ym2151_log_with_options(&midi_data, &options_with_attachment).unwrap();
    let result_without = convert_to_ym2151_log(&midi_data).unwrap();

    // Both outputs should have the same event count — unmatched attachment is a no-op.
    assert_eq!(
        result_with.event_count, result_without.event_count,
        "Unmatched program attachment must not add extra events"
    );
}
