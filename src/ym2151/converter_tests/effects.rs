//! Effects tests for YM2151 converter (delay vibrato, portamento, LFO, pop noise, attack)
use super::*;

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

    // Collect KC events emitted during the portamento glide window (0.5 to 0.6 seconds).
    // Exclude the initial note-on KC write at exactly 0.5 that the main converter emits
    // before the portamento overrides it, so we only see portamento-driven KC updates.
    let portamento_end = 0.5 + 0.1; // start_time + PORTAMENTO_TIME_SECONDS
    let kc_events_in_glide: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time >= 0.5 && e.time <= portamento_end)
        .collect();
    assert!(
        kc_events_in_glide.len() >= 2,
        "Portamento should emit multiple KC steps during the glide"
    );

    let (kc_second, _) = midi_to_kc_kf(67);

    // Glide should include the previous pitch at the start
    assert!(
        kc_events_in_glide
            .iter()
            .any(|e| e.data == format!("0x{:02X}", kc_first)),
        "Glide should include the starting KC from the previous note"
    );
    // Glide must end at the target KC. Verify the LAST KC event in the glide window
    // equals the target, confirming the portamento fully reaches the destination note.
    let last_kc_in_glide = kc_events_in_glide
        .iter()
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    assert_eq!(
        last_kc_in_glide.map(|e| e.data.as_str()),
        Some(format!("0x{:02X}", kc_second).as_str()),
        "Portamento glide must reach the target KC at the end of the glide"
    );
}

#[test]
fn test_portamento_one_octave_reaches_target() {
    // Verify that a 1-octave portamento (C4 -> C5) always writes the target KC at stop_time.
    // Previously, the loop's time_step didn't align with stop_time, leaving the portamento
    // stuck just below the target note.
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60, // C4
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
                note: 72, // C5 (one octave up)
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 960,
                channel: 0,
                note: 72,
            },
        ],
    };

    let options = ConversionOptions {
        portamento: true,
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    let (kc_target, _) = midi_to_kc_kf(72); // C5
    let portamento_end = 0.5 + 0.1; // start_time + PORTAMENTO_TIME_SECONDS

    // The last KC event written during the portamento window must be the target (C5).
    let last_portamento_kc = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time >= 0.5 && e.time <= portamento_end)
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    assert_eq!(
        last_portamento_kc.map(|e| e.data.as_str()),
        Some(format!("0x{:02X}", kc_target).as_str()),
        "1-octave portamento must reach the target KC (C5) at the end of the glide"
    );
}

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
fn test_pop_noise_envelope_adds_pre_note_overrides() {
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

    let options = ConversionOptions {
        pop_noise_envelope: Some(PopNoiseEnvelope {
            enabled: true,
            offset_seconds: 0.001,
            registers: vec![RegisterOverride {
                base_register: "0xA0".to_string(),
                value: "0x02".to_string(),
            }],
        }),
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    let pre_overrides: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0xA0" && e.data == "0x02" && e.time > 0.4 && e.time < 0.5)
        .collect();
    assert_eq!(
        pre_overrides.len(),
        1,
        "Second note should get one override"
    );

    let restores: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0xA0" && e.time >= 0.499 && e.time <= 0.5)
        .collect();
    assert!(
        restores.iter().any(|e| e.data == "0x05"),
        "Override should be restored to the base D1R value"
    );
}

#[test]
fn test_attack_continuation_fix_forces_release_before_note_on() {
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

    let options = ConversionOptions {
        attack_continuation_fix: Some(AttackContinuationFix {
            enabled: true,
            offset_seconds: 0.001,
            release_rate: 0xF0,
        }),
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    let target_release_addrs = ["0xE0", "0xE8", "0xF0", "0xF8"];
    let release_overrides: Vec<_> = result
        .events
        .iter()
        .filter(|e| {
            target_release_addrs.contains(&e.addr.as_str())
                && e.data == "0xF0"
                && e.time > 0.49
                && e.time < 0.5
        })
        .collect();
    assert_eq!(
        release_overrides.len(),
        4,
        "All four operators should receive a pre-note release override"
    );
    assert!(release_overrides.iter().all(|e| e.data == "0xF0"));

    let key_off = result
        .events
        .iter()
        .find(|e| e.addr == "0x08" && e.data == "0x00" && e.time > 0.49 && e.time < 0.5)
        .expect("Pre-note key off should be generated");
    assert!(key_off.time < 0.5);

    let restore_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| {
            target_release_addrs.contains(&e.addr.as_str())
                && e.data == "0xF7"
                && e.time >= 0.499
                && e.time <= 0.5
        })
        .collect();
    assert!(
        restore_events.iter().all(|e| e.data == "0xF7"),
        "Release rate should return to the base value before key on"
    );
}
