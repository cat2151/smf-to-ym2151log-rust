use super::*;

#[test]
fn test_change_to_next_tone_keep_fields_preserve_mul_and_alg_from_program0() {
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
        events: vec![
            Ym2151Event {
                time: 0.0,
                addr: "0x20".to_string(),
                data: "0x02".to_string(), // ALG/CON = 2
            },
            Ym2151Event {
                time: 0.0,
                addr: "0x40".to_string(),
                data: "0x03".to_string(), // MUL = 3
            },
            Ym2151Event {
                time: 0.0,
                addr: "0x60".to_string(),
                data: "0x10".to_string(), // TL differs and should still interpolate
            },
        ],
        ..ToneDefinition::default()
    };
    let tone1 = ToneDefinition {
        events: vec![
            Ym2151Event {
                time: 0.0,
                addr: "0x20".to_string(),
                data: "0x07".to_string(), // ALG/CON = 7
            },
            Ym2151Event {
                time: 0.0,
                addr: "0x40".to_string(),
                data: "0x0F".to_string(), // MUL = 15
            },
            Ym2151Event {
                time: 0.0,
                addr: "0x60".to_string(),
                data: "0x30".to_string(),
            },
        ],
        ..ToneDefinition::default()
    };

    let options = ConversionOptions {
        program_attachments: vec![
            ProgramAttachment {
                program_change: 0,
                change_to_next_tone: true,
                change_to_next_tone_time: 5.0,
                change_to_next_tone_keep_fields: vec![
                    ChangeToNextToneKeepField::Mul,
                    ChangeToNextToneKeepField::Alg,
                ],
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

    let alg_values: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x20" && e.time > 0.0)
        .map(|e| e.data.as_str())
        .collect();
    assert!(
        alg_values.iter().all(|&data| data == "0x02"),
        "ALG/CON must stay on program 0 value when keep field is enabled; got {:?}",
        alg_values
    );

    let mul_values: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x40" && e.time > 0.0)
        .map(|e| e.data.as_str())
        .collect();
    assert!(
        mul_values.iter().all(|&data| data == "0x03"),
        "MUL must stay on program 0 value when keep field is enabled; got {:?}",
        mul_values
    );

    let tl_events: Vec<_> = result.events.iter().filter(|e| e.addr == "0x60").collect();
    assert!(
        tl_events.len() > 2,
        "Unrelated fields must still interpolate; got {} TL events",
        tl_events.len()
    );
}
