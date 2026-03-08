//! Portamento tests for YM2151 converter
use super::*;

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

    // Collect KC events emitted strictly after the note-on time (> 0.5s) and within the
    // portamento glide window (up to 0.6s). Using strictly-greater-than excludes the initial
    // note-on KC write at exactly 0.5s emitted by the main converter, so only
    // portamento-driven KC updates are collected. A small epsilon is added to portamento_end
    // to avoid missing the final event that may be emitted at exactly stop_time.
    let note_on_time = 0.5_f64;
    let portamento_end = note_on_time + 0.1; // start_time + PORTAMENTO_TIME_SECONDS
    let kc_events_in_glide: Vec<_> = result
        .events
        .iter()
        .filter(|e| {
            e.addr == "0x28"
                && e.time > note_on_time
                && e.time <= portamento_end + f64::EPSILON * portamento_end
        })
        .collect();
    assert!(
        kc_events_in_glide.len() >= 2,
        "Portamento should emit multiple KC steps during the glide"
    );

    let (kc_second, kf_second) = midi_to_kc_kf(67);

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

    // The last KF event in the glide window must also match the target pitch exactly.
    let last_kf_in_glide = result
        .events
        .iter()
        .filter(|e| {
            e.addr == "0x30"
                && e.time > note_on_time
                && e.time <= portamento_end + f64::EPSILON * portamento_end
        })
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    assert_eq!(
        last_kf_in_glide.map(|e| e.data.as_str()),
        Some(format!("0x{:02X}", kf_second).as_str()),
        "Portamento glide must reach the target KF at the end of the glide"
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

    let (kc_target, kf_target) = midi_to_kc_kf(72); // C5
    let note_on_time = 0.5_f64;
    let portamento_end = note_on_time + 0.1; // start_time + PORTAMENTO_TIME_SECONDS
                                             // A small epsilon is added to portamento_end to avoid missing the final event
                                             // that is emitted at exactly stop_time due to floating-point boundary effects.
    let portamento_end_with_eps = portamento_end + f64::EPSILON * portamento_end;

    // The last KC event written during the portamento window must be the target (C5).
    let last_portamento_kc = result
        .events
        .iter()
        .filter(|e| e.addr == "0x28" && e.time > note_on_time && e.time <= portamento_end_with_eps)
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    assert_eq!(
        last_portamento_kc.map(|e| e.data.as_str()),
        Some(format!("0x{:02X}", kc_target).as_str()),
        "1-octave portamento must reach the target KC (C5) at the end of the glide"
    );

    // The last KF event must also match, confirming the exact pitch lands on C5 (KF=0).
    let last_portamento_kf = result
        .events
        .iter()
        .filter(|e| e.addr == "0x30" && e.time > note_on_time && e.time <= portamento_end_with_eps)
        .max_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    assert_eq!(
        last_portamento_kf.map(|e| e.data.as_str()),
        Some(format!("0x{:02X}", kf_target).as_str()),
        "1-octave portamento must reach the target KF (C5) at the end of the glide"
    );
}
