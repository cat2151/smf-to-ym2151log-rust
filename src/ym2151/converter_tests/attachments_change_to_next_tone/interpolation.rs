use super::*;

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
