//! Software LFO tests for YM2151 converter
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
fn test_register_lfo_key_on_sync_false_generates_smooth_events_across_notes() {
    // With key_on_sync=false, the software LFO keeps running while multiple notes
    // are played on the same channel. This test checks that two consecutive notes
    // both receive LFO updates and that those updates change smoothly over time
    // (no large jumps between consecutive TL register values).
    //
    // The note boundary is at 0.25s with rate_hz=1.0, where triangle_wave(0.25)=1.0.
    // This makes the test discriminative: if key_on_sync=false works correctly, the
    // second note's first LFO value is near base_value+depth (offset≈+4); if the
    // phase were reset it would equal base_value+0 (same as the song start value).
    // The default TL for 0x60 (channel 0, operator 0) is 0x00, so values are 4 vs 0.
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
                ticks: 240, // 0.25 seconds at 120 BPM
                channel: 0,
                note: 60,
            },
            MidiEvent::NoteOn {
                ticks: 240,
                channel: 0,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 480, // 0.5 seconds at 120 BPM
                channel: 0,
                note: 64,
            },
        ],
    };

    // Use key_on_sync=false so LFO runs continuously.
    // rate_hz=1.0 with boundary at 0.25s → phase=0.25, triangle_wave=1.0, offset=+depth.
    // A key_on_sync=true reset restarts at phase=0 (offset=0), giving a different value.
    let options = ConversionOptions {
        software_lfo: vec![RegisterLfoDefinition {
            base_register: "0x60".to_string(),
            depth: 4.0,
            rate_hz: 1.0,
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

    // Check that events cover both notes (times from 0 to ~0.5s)
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

    // Verify all consecutive LFO value changes are at most 1 (smooth transitions).
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

    // Discriminative check: at the second note start (t=0.25s), the continuous LFO
    // has phase=0.25 and triangle_wave(0.25)=1.0, so the offset is approximately +depth.
    // If the phase were reset on key-on (key_on_sync=true), the value would equal
    // base_value+0 (triangle=0 at phase=0) — the same as the very first event.
    // With key_on_sync=false the second note should start near value 4, not near 0.
    let note1_start_val = values[0] as i16;
    let note2_first_val = lfo_events
        .iter()
        .filter(|e| e.time >= 0.24)
        .filter_map(|e| {
            let hex = e.data.strip_prefix("0x")?;
            u8::from_str_radix(hex, 16).ok()
        })
        .next()
        .expect("Should have LFO events in the second note") as i16;
    assert!(
        note2_first_val > note1_start_val,
        "With key_on_sync=false, LFO at second note start (phase=0.25, triangle=1.0) \
         should be above the song-start value (phase=0.0, triangle=0.0); \
         got note2_first_val={note2_first_val}, note1_start_val={note1_start_val}"
    );
}

#[test]
fn test_register_lfo_key_on_sync_false_emits_at_segment_start_with_gap() {
    // When key_on_sync=false and there is a rest/gap between note segments,
    // the LFO should emit an event at segment.start_time so the register has
    // the correct phase-continuous value at note-on, not a stale value held
    // until the next grid boundary.
    //
    // Note 1: ticks 0-100 (~0.104s), gap, Note 2: ticks 200-400 (~0.208-0.417s).
    // With rate_hz=1.0, time_step=1/16=0.0625s, note 2 starts at 200/960≈0.2083s
    // which falls between grid points 0.1875 and 0.25. The fix ensures an event
    // is emitted exactly at 0.2083s rather than waiting until 0.25s.
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
                ticks: 100, // ~0.1042s
                channel: 0,
                note: 60,
            },
            MidiEvent::NoteOn {
                ticks: 200, // ~0.2083s (between grid points 0.1875 and 0.25)
                channel: 0,
                note: 64,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 400, // ~0.4167s
                channel: 0,
                note: 64,
            },
        ],
    };

    let options = ConversionOptions {
        software_lfo: vec![RegisterLfoDefinition {
            base_register: "0x60".to_string(),
            depth: 4.0,
            rate_hz: 1.0,
            delay_seconds: 0.0,
            attack_seconds: 0.0,
            waveform: LfoWaveform::Triangle,
            key_on_sync: false,
        }],
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();
    let lfo_events: Vec<_> = result.events.iter().filter(|e| e.addr == "0x60").collect();

    assert!(!lfo_events.is_empty(), "LFO should emit events");

    // Note 2 starts at t=200/960s. With the fix, there must be an event at exactly
    // this time so the register is set to the correct in-phase value at note-on.
    // Without the fix the next event after note 1 would be at the grid point 0.25s.
    let note2_start_time = 200.0_f64 / 960.0;
    let has_event_at_note2_start = lfo_events
        .iter()
        .any(|e| (e.time - note2_start_time).abs() < 1e-9);
    assert!(
        has_event_at_note2_start,
        "Should have an LFO event exactly at note 2 start (t≈{:.6}s) to ensure \
         correct phase at note-on; events near that time: {:?}",
        note2_start_time,
        lfo_events
            .iter()
            .filter(|e| e.time > 0.15 && e.time < 0.3)
            .collect::<Vec<_>>()
    );
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
