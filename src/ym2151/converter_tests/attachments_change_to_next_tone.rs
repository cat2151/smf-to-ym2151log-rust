//! ChangeToNextTone attachment tests for YM2151 converter
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
fn test_change_to_next_tone_interpolates_fields_independently() {
    // Verifies the bug fix: registers with multiple packed fields must be interpolated
    // field-by-field, not as a raw byte.
    //
    // KS_AR register (0x80-0x9F) packs:
    //   AR (bits 0-4, mask 0x1F): Attack Rate
    //   KS (bits 6-7, mask 0xC0): Key Scaling
    //
    // tone0: KS_AR register = 0x1F  → AR=31, KS=0
    // tone1: KS_AR register = 0x40  → AR=0,  KS=1
    //
    // Correct per-field midpoint (t=0.5): AR≈16, KS=1  → byte ≈ 0x50
    // Wrong raw-byte midpoint          : (0x1F+0x40)/2 → byte ≈ 0x30  (KS=0 — incorrect!)
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
            // 10 seconds at 120 BPM
            MidiEvent::NoteOff {
                ticks: 9600,
                channel: 0,
                note: 60,
            },
        ],
    };

    // Both tones carry a KS_AR register entry for channel 0 operator 0 (0x80).
    let tone0 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x80".to_string(), // KS_AR, op0, ch0
            data: "0x1F".to_string(), // AR=31, KS=0
        }],
        ..ToneDefinition::default()
    };
    let tone1 = ToneDefinition {
        events: vec![Ym2151Event {
            time: 0.0,
            addr: "0x80".to_string(),
            data: "0x40".to_string(), // AR=0, KS=1
        }],
        ..ToneDefinition::default()
    };

    let options = ConversionOptions {
        program_attachments: vec![
            ProgramAttachment {
                program_change: 0,
                change_to_next_tone: true,
                // Use a period that ensures a sample lands very close to t=0.5.
                // steps = max(AR_steps=31, KS_steps=1) = 31; time_step = 5.0/31 ≈ 0.161s
                // At t=0.5 (time=2.5s): sample index ≈ 2.5/0.161 ≈ 15.5 → sample 15 or 16
                // Just verify that all emitted values are valid field combinations.
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

    // Collect all KS_AR writes and parse their byte values.
    let ks_ar_values: Vec<u8> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x80")
        .filter_map(|e| {
            e.data
                .strip_prefix("0x")
                .and_then(|h| u8::from_str_radix(h, 16).ok())
        })
        .collect();

    assert!(
        !ks_ar_values.is_empty(),
        "KS_AR register must receive interpolation writes"
    );

    // Every emitted byte must be a valid (AR, KS) combination:
    //   AR ∈ [0, 31] (bits 0-4 only)  → bits 5 must always be 0
    //   KS ∈ [0, 3]  (bits 6-7 only)  → no contamination from adjacent fields
    // Any byte with bit5 set (0x20) would indicate raw-byte cross-field contamination.
    let has_contaminated_byte = ks_ar_values.iter().any(|&v| v & 0x20 != 0);
    assert!(
        !has_contaminated_byte,
        "KS_AR values must not have bit5 set (raw-byte interpolation cross-field contamination)"
    );

    // With field-based interpolation the byte at the midpoint (t≈0.5) should have KS=1.
    // KS transitions 0→1 over 5 seconds; at t=0.5 it rounds to 1 (Rust rounds half away from 0).
    // So somewhere in the sequence we must see a byte with KS=1 (bit7 set, i.e. ≥ 0x40).
    let has_ks1 = ks_ar_values.iter().any(|&v| (v & 0xC0) >> 6 == 1);
    assert!(
        has_ks1,
        "KS field must reach 1 during interpolation; got values: {:?}",
        ks_ar_values
            .iter()
            .map(|v| format!("0x{v:02X}"))
            .collect::<Vec<_>>()
    );
}
