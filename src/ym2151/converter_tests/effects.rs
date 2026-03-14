//! Effects tests for YM2151 converter (delay vibrato, pop noise, attack)
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
fn test_pop_noise_envelope_adds_pre_note_overrides() {
    // Use back-to-back notes so that the key-off for note 1 is at the same
    // time as the key-on for note 2 (t=0.5s).  PopNoiseEnvelope should move
    // that key-off to apply_time so the envelope decays before note 2 starts.
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
                ticks: 480, // ends at t=0.5s — same tick as note 2 start (back-to-back)
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
                ticks: 960,
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

    // Register override should appear at apply_time (≈ 0.499)
    let pre_overrides: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0xA0" && e.data == "0x02" && e.time > 0.498 && e.time < 0.5)
        .collect();
    assert_eq!(
        pre_overrides.len(),
        1,
        "Second note should get one override at apply_time"
    );

    // Register restore should appear at segment.start_time (0.5)
    let restores: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0xA0" && (e.time - 0.5_f64).abs() < 1e-6)
        .collect();
    assert!(
        restores.iter().any(|e| e.data != "0x02"),
        "Override should be restored to the base value at segment.start_time"
    );

    // The existing key-off (from note 1 ending at t=0.5) must be moved to apply_time
    // so the envelope decays with the overridden registers before note 2's key-on.
    let _pre_note_key_off = result
        .events
        .iter()
        .find(|e| e.addr == "0x08" && e.data == "0x00" && e.time > 0.498 && e.time < 0.5)
        .expect("Key-off should be moved to apply_time for pop noise mitigation");

    // No key-off should remain at segment.start_time (t=0.5) — it was moved.
    let key_off_at_note_start = result
        .events
        .iter()
        .find(|e| e.addr == "0x08" && e.data == "0x00" && (e.time - 0.5_f64).abs() < 1e-9);
    assert!(
        key_off_at_note_start.is_none(),
        "Key-off should have been moved away from segment.start_time"
    );

    // At apply_time (= segment.start_time - offset_seconds = 0.5 - 0.001 ≈ 0.499) the
    // register override must come before the key-off in the sorted output.
    // This ensures the envelope begins decaying with the overridden (faster-release) registers
    // from the very first sample after key-off.
    // The window (0.498, 0.5) captures apply_time while excluding the segment.start_time events.
    let apply_time_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.time > 0.498 && e.time < 0.5)
        .collect();
    let key_off_idx = apply_time_events
        .iter()
        .position(|e| e.addr == "0x08")
        .expect("Key-off must be present at apply_time");
    let reg_override_idx = apply_time_events
        .iter()
        .position(|e| e.addr == "0xA0" && e.data == "0x02")
        .expect("Register override must be present at apply_time");
    assert!(
        reg_override_idx < key_off_idx,
        "Register override must appear before key-off at the same timestamp"
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
