/// Software LFO tests for YM2151 converter
use super::*;

#[test]
fn test_register_lfo_triangle_wave_smooth_transitions() {
    // Verify that the triangle LFO produces intermediate values (not just top/center/bottom)
    // and that consecutive register values differ by at most 1.
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
                ticks: 1920, // 2 seconds at 120 BPM
                channel: 0,
                note: 60,
            },
        ],
    };

    let options = ConversionOptions {
        software_lfo: vec![RegisterLfoDefinition {
            base_register: "0x60".to_string(),
            depth: 6.0,
            rate_hz: 4.0,
            delay_seconds: 0.0,
            attack_seconds: 0.0,
            waveform: LfoWaveform::Triangle,
            key_on_sync: true,
        }],
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    let lfo_events: Vec<_> = result.events.iter().filter(|e| e.addr == "0x60").collect();

    assert!(
        !lfo_events.is_empty(),
        "LFO should emit events for the TL register"
    );

    let values: Vec<u8> = lfo_events
        .iter()
        .filter_map(|e| {
            let hex = e.data.strip_prefix("0x")?;
            u8::from_str_radix(hex, 16).ok()
        })
        .collect();

    let unique_count = {
        let mut v = values.clone();
        v.sort_unstable();
        v.dedup();
        v.len()
    };
    assert!(
        unique_count > 3,
        "Triangle LFO with depth=6 should produce more than 3 unique values, got {}",
        unique_count
    );

    // All consecutive LFO value changes should be at most 1 (smooth transitions)
    for window in values.windows(2) {
        let diff = (window[0] as i16 - window[1] as i16).unsigned_abs();
        assert!(
            diff <= 1,
            "Consecutive LFO values should differ by at most 1, got diff={diff} ({} → {})",
            window[0],
            window[1]
        );
    }
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
            key_on_sync: true,
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
fn test_register_lfo_key_on_sync_false_continuous_phase() {
    // With key_on_sync=false, the LFO phase is continuous across notes.
    // Two consecutive notes should produce LFO events where the second note's
    // LFO continues from where the first note ended (phase is not reset).
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
                ticks: 480, // 0.5 seconds
                channel: 0,
                note: 60,
            },
            MidiEvent::NoteOn {
                ticks: 480,
                channel: 0,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 960, // 1.0 seconds
                channel: 0,
                note: 64,
            },
        ],
    };

    // Use key_on_sync=false so LFO runs continuously
    let options = ConversionOptions {
        software_lfo: vec![RegisterLfoDefinition {
            base_register: "0x60".to_string(),
            depth: 4.0,
            rate_hz: 1.0, // 1 Hz: one full period per second
            delay_seconds: 0.0,
            attack_seconds: 0.0,
            waveform: LfoWaveform::Triangle,
            key_on_sync: false,
        }],
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();
    let lfo_events: Vec<_> = result.events.iter().filter(|e| e.addr == "0x60").collect();

    assert!(
        !lfo_events.is_empty(),
        "LFO should emit events for the register"
    );

    // Check that events cover both notes (times from 0 to ~1.0s)
    let first_time = lfo_events.first().unwrap().time;
    let last_time = lfo_events.last().unwrap().time;
    assert!(
        first_time < 0.1,
        "LFO should start near song start, got first_time={first_time}"
    );
    assert!(
        last_time > 0.4,
        "LFO should continue into the second note, got last_time={last_time}"
    );

    // Verify all consecutive LFO value changes are at most 1 (smooth transitions)
    let values: Vec<u8> = lfo_events
        .iter()
        .filter_map(|e| {
            let hex = e.data.strip_prefix("0x")?;
            u8::from_str_radix(hex, 16).ok()
        })
        .collect();
    for window in values.windows(2) {
        let diff = (window[0] as i16 - window[1] as i16).unsigned_abs();
        assert!(
            diff <= 1,
            "Consecutive LFO values should differ by at most 1, got diff={diff} ({} → {})",
            window[0],
            window[1]
        );
    }
}

#[test]
fn test_register_lfo_key_on_sync_default_is_true() {
    // Verify that omitting KeyOnSync from JSON deserializes to key_on_sync=true
    let json = br#"[
      {
        "ProgramChange": 0,
        "SoftwareLfo": [
          {
            "BaseRegister": "0x60",
            "Depth": 4.0,
            "RateHz": 2.0
          }
        ]
      }
    ]"#;
    let opts = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();
    assert_eq!(opts.program_attachments.len(), 1);
    let lfo = &opts.program_attachments[0].software_lfo[0];
    assert!(
        lfo.key_on_sync,
        "Default key_on_sync should be true when not specified in JSON"
    );
}

#[test]
fn test_register_lfo_key_on_sync_false_deserializes() {
    // Verify that KeyOnSync: false deserializes correctly
    let json = br#"[
      {
        "ProgramChange": 0,
        "SoftwareLfo": [
          {
            "BaseRegister": "0x60",
            "Depth": 4.0,
            "RateHz": 2.0,
            "KeyOnSync": false
          }
        ]
      }
    ]"#;
    let opts = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();
    assert_eq!(opts.program_attachments.len(), 1);
    let lfo = &opts.program_attachments[0].software_lfo[0];
    assert!(
        !lfo.key_on_sync,
        "key_on_sync should be false when KeyOnSync: false is specified"
    );
}
