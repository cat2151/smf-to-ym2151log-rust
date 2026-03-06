//! Tests for MIDI utility functions
use super::*;

#[test]
fn test_midi_to_kc_kf_middle_c() {
    // MIDI note 60 = C4 (Middle C)
    let (kc, kf) = midi_to_kc_kf(60);
    assert_eq!(kc, 0x2E); // Octave 2, Note C
    assert_eq!(kf, 0);
}

#[test]
fn test_midi_to_kc_kf_a440() {
    // MIDI note 69 = A4 (A440)
    let (kc, kf) = midi_to_kc_kf(69);
    assert_eq!(kc, 0x3A); // Octave 3, Note A
    assert_eq!(kf, 0);
}

#[test]
fn test_midi_to_kc_kf_octaves() {
    // Test representative notes across different octaves
    // C notes from different octaves (YM2151 octave = MIDI octave - 2)
    let (kc, _) = midi_to_kc_kf(24); // C1
    assert_eq!(kc, 0x0E); // Octave 0 (clamped), Note C

    let (kc, _) = midi_to_kc_kf(36); // C2
    assert_eq!(kc, 0x0E); // Octave 0, Note C

    let (kc, _) = midi_to_kc_kf(48); // C3
    assert_eq!(kc, 0x1E); // Octave 1, Note C

    let (kc, _) = midi_to_kc_kf(60); // C4
    assert_eq!(kc, 0x2E); // Octave 2, Note C

    let (kc, _) = midi_to_kc_kf(72); // C5
    assert_eq!(kc, 0x3E); // Octave 3, Note C

    let (kc, _) = midi_to_kc_kf(84); // C6
    assert_eq!(kc, 0x4E); // Octave 4, Note C

    let (kc, _) = midi_to_kc_kf(96); // C7
    assert_eq!(kc, 0x5E); // Octave 5, Note C
}

#[test]
fn test_midi_to_kc_kf_all_notes_in_octave() {
    // Test all 12 notes within an octave (using octave 4 as example)
    // MIDI notes 60-71 map to: C, C#, D, D#, E, F, F#, G, G#, A, A#, B
    // YM2151 note table values for these notes in order
    let base_midi = 60; // C4
    let expected_ym_notes = [14, 0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13];

    for (i, &expected_note) in expected_ym_notes.iter().enumerate() {
        let (kc, kf) = midi_to_kc_kf(base_midi + i as u8);
        let note = kc & 0x0F;
        assert_eq!(note, expected_note, "Failed for note {}", i);
        assert_eq!(kf, 0);
    }
}

#[test]
fn test_midi_to_kc_kf_boundary_minimum() {
    // Test minimum MIDI note (0)
    let (kc, kf) = midi_to_kc_kf(0);
    assert_eq!(kf, 0);
    // Should clamp to octave 0
    let octave = (kc >> 4) & 0x07;
    assert_eq!(octave, 0);
}

#[test]
fn test_midi_to_kc_kf_boundary_maximum() {
    // Test maximum MIDI note (127)
    let (kc, kf) = midi_to_kc_kf(127);
    assert_eq!(kf, 0);
    // Should clamp to octave 7 (maximum for YM2151)
    let octave = (kc >> 4) & 0x07;
    assert_eq!(octave, 7);
}

#[test]
fn test_midi_to_kc_kf_octave_clamping_high() {
    // Test that very high notes clamp to octave 7
    // With -2 octave offset: MIDI 108-119 → octave 6, MIDI 120-127 → octave 7
    for midi_note in 120..=127 {
        let (kc, _) = midi_to_kc_kf(midi_note);
        let octave = (kc >> 4) & 0x07;
        assert_eq!(
            octave, 7,
            "Failed to clamp octave for MIDI note {}",
            midi_note
        );
    }
}

#[test]
fn test_midi_note_to_frequency_a440() {
    let freq = midi_note_to_frequency(69);
    assert!((freq - 440.0).abs() < 0.001);
}

#[test]
fn test_midi_note_with_offset_zero_matches_base() {
    let base = midi_to_kc_kf(60);
    let with_offset = midi_note_with_offset_to_kc_kf(60, 0.0);
    assert_eq!(base, with_offset);
}

#[test]
fn test_midi_note_with_offset_positive_and_negative() {
    let up = midi_note_with_offset_to_kc_kf(60, 100.0);
    let up_expected = midi_to_kc_kf(61);
    assert_eq!(up.0, up_expected.0);

    let down = midi_note_with_offset_to_kc_kf(60, -100.0);
    let down_expected = midi_to_kc_kf(59);
    assert_eq!(down.0, down_expected.0);
}

// Timing conversion tests
#[test]
fn test_ticks_to_seconds_one_beat() {
    // 1 beat at 120 BPM = 0.5 seconds
    let seconds = ticks_to_seconds(480, 480, 120.0);
    assert!((seconds - 0.5).abs() < 0.001);
}

#[test]
fn test_ticks_to_seconds_half_beat() {
    // 0.5 beat at 120 BPM = 0.25 seconds
    let seconds = ticks_to_seconds(240, 480, 120.0);
    assert!((seconds - 0.25).abs() < 0.001);
}

#[test]
fn test_ticks_to_seconds_different_tempo() {
    // 1 beat at 60 BPM = 1.0 second
    let seconds = ticks_to_seconds(480, 480, 60.0);
    assert!((seconds - 1.0).abs() < 0.001);
}

#[test]
fn test_ticks_to_seconds_different_ticks_per_beat() {
    // 1 beat at 120 BPM = 0.5 seconds (with different ticks_per_beat)
    let seconds = ticks_to_seconds(960, 960, 120.0);
    assert!((seconds - 0.5).abs() < 0.001);
}

#[test]
fn test_seconds_to_samples_one_second() {
    let samples = seconds_to_samples(1.0);
    assert_eq!(samples, 55930);
}

#[test]
fn test_seconds_to_samples_half_second() {
    let samples = seconds_to_samples(0.5);
    assert_eq!(samples, 27965);
}

#[test]
fn test_seconds_to_samples_zero() {
    let samples = seconds_to_samples(0.0);
    assert_eq!(samples, 0);
}

#[test]
fn test_ticks_to_samples_one_beat() {
    // 1 beat at 120 BPM = 0.5 seconds = 27965 samples
    let samples = ticks_to_samples(480, 480, 120.0);
    assert_eq!(samples, 27965);
}

#[test]
fn test_ticks_to_samples_zero() {
    let samples = ticks_to_samples(0, 480, 120.0);
    assert_eq!(samples, 0);
}

#[test]
fn test_ticks_to_samples_precision() {
    // Test that the conversion maintains reasonable precision
    let samples1 = ticks_to_samples(1, 480, 120.0);
    let samples2 = ticks_to_samples(2, 480, 120.0);
    // Each tick should produce a consistent increment
    assert_eq!(samples2, samples1 * 2);
}

// Tempo map conversion tests
#[test]
fn test_ticks_to_samples_with_tempo_map_no_changes() {
    // Single tempo - should match regular conversion
    let tempo_map = vec![TempoChange {
        tick: 0,
        tempo_bpm: 120.0,
    }];
    let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
    assert_eq!(samples, 27965); // Same as ticks_to_samples(480, 480, 120.0)
}

#[test]
fn test_ticks_to_samples_with_tempo_map_single_change() {
    // Tempo change at tick 480
    let tempo_map = vec![
        TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        },
        TempoChange {
            tick: 480,
            tempo_bpm: 60.0,
        },
    ];

    // At tick 480 (right at the tempo change)
    // Should be: 480 ticks at 120 BPM = 0.5 seconds = 27965 samples
    let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
    assert_eq!(samples, 27965);

    // At tick 960 (480 ticks after tempo change)
    // Should be: 480 ticks at 120 BPM + 480 ticks at 60 BPM
    // = 0.5 seconds + 1.0 second = 1.5 seconds = 83895 samples
    let samples = ticks_to_samples_with_tempo_map(960, 480, &tempo_map);
    assert_eq!(samples, 83895);
}

#[test]
fn test_ticks_to_samples_with_tempo_map_multiple_changes() {
    // Multiple tempo changes
    let tempo_map = vec![
        TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        },
        TempoChange {
            tick: 240,
            tempo_bpm: 60.0,
        },
        TempoChange {
            tick: 480,
            tempo_bpm: 180.0,
        },
    ];

    // At tick 0
    let samples = ticks_to_samples_with_tempo_map(0, 480, &tempo_map);
    assert_eq!(samples, 0);

    // At tick 240 (at first tempo change)
    // 240 ticks at 120 BPM = 0.25 seconds = 13982 samples
    let samples = ticks_to_samples_with_tempo_map(240, 480, &tempo_map);
    assert_eq!(samples, 13982);

    // At tick 480 (at second tempo change)
    // 240 ticks at 120 BPM + 240 ticks at 60 BPM
    // = 0.25 + 0.5 = 0.75 seconds = 41947 samples
    let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
    assert_eq!(samples, 41947);

    // At tick 720 (after second tempo change)
    // 240 ticks at 120 BPM + 240 ticks at 60 BPM + 240 ticks at 180 BPM
    // = 0.25 + 0.5 + 0.167 = 0.917 seconds ≈ 51269 samples
    let samples = ticks_to_samples_with_tempo_map(720, 480, &tempo_map);
    assert_eq!(samples, 51269); // Adjusted for rounding
}

#[test]
fn test_ticks_to_samples_with_tempo_map_empty() {
    // Empty tempo map - should use default 120 BPM
    let tempo_map = vec![];
    let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
    assert_eq!(samples, 27965); // Same as 120 BPM
}

#[test]
fn test_ticks_to_samples_with_tempo_map_before_first_change() {
    // Tempo change at tick 480, but we want time at tick 240
    let tempo_map = vec![TempoChange {
        tick: 480,
        tempo_bpm: 60.0,
    }];

    // At tick 240 (before the tempo change)
    // Should use the tempo from the first entry (60 BPM)
    // 240 ticks at 60 BPM = 0.5 seconds = 27965 samples
    let samples = ticks_to_samples_with_tempo_map(240, 480, &tempo_map);
    assert_eq!(samples, 27965);
}

// ticks_to_seconds_with_tempo_map tests
#[test]
fn test_ticks_to_seconds_with_tempo_map_no_changes() {
    // Single tempo - should match regular conversion
    let tempo_map = vec![TempoChange {
        tick: 0,
        tempo_bpm: 120.0,
    }];
    let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
    assert!((seconds - 0.5).abs() < 0.001); // Same as ticks_to_seconds(480, 480, 120.0)
}

#[test]
fn test_ticks_to_seconds_with_tempo_map_single_change() {
    // Tempo change at tick 480
    let tempo_map = vec![
        TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        },
        TempoChange {
            tick: 480,
            tempo_bpm: 60.0,
        },
    ];

    // At tick 480 (right at the tempo change)
    // Should be: 480 ticks at 120 BPM = 0.5 seconds
    let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
    assert!((seconds - 0.5).abs() < 0.001);

    // At tick 960 (480 ticks after tempo change)
    // Should be: 480 ticks at 120 BPM + 480 ticks at 60 BPM
    // = 0.5 seconds + 1.0 second = 1.5 seconds
    let seconds = ticks_to_seconds_with_tempo_map(960, 480, &tempo_map);
    assert!((seconds - 1.5).abs() < 0.001);
}

#[test]
fn test_ticks_to_seconds_with_tempo_map_multiple_changes() {
    // Multiple tempo changes
    let tempo_map = vec![
        TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        },
        TempoChange {
            tick: 240,
            tempo_bpm: 60.0,
        },
        TempoChange {
            tick: 480,
            tempo_bpm: 180.0,
        },
    ];

    // At tick 0
    let seconds = ticks_to_seconds_with_tempo_map(0, 480, &tempo_map);
    assert!((seconds - 0.0).abs() < 0.001);

    // At tick 240 (at first tempo change)
    // 240 ticks at 120 BPM = 0.25 seconds
    let seconds = ticks_to_seconds_with_tempo_map(240, 480, &tempo_map);
    assert!((seconds - 0.25).abs() < 0.001);

    // At tick 480 (at second tempo change)
    // 240 ticks at 120 BPM + 240 ticks at 60 BPM
    // = 0.25 + 0.5 = 0.75 seconds
    let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
    assert!((seconds - 0.75).abs() < 0.001);

    // At tick 720 (after second tempo change)
    // 240 ticks at 120 BPM + 240 ticks at 60 BPM + 240 ticks at 180 BPM
    // = 0.25 + 0.5 + 0.167 ≈ 0.917 seconds
    let seconds = ticks_to_seconds_with_tempo_map(720, 480, &tempo_map);
    assert!((seconds - 0.9166666).abs() < 0.001);
}

#[test]
fn test_ticks_to_seconds_with_tempo_map_empty() {
    // Empty tempo map - should use default 120 BPM
    let tempo_map = vec![];
    let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
    assert!((seconds - 0.5).abs() < 0.001); // Same as 120 BPM
}

#[test]
fn test_ticks_to_seconds_with_tempo_map_before_first_change() {
    // Tempo change at tick 480, but we want time at tick 240
    let tempo_map = vec![TempoChange {
        tick: 480,
        tempo_bpm: 60.0,
    }];

    // At tick 240 (before the tempo change)
    // Should use the tempo from the first entry (60 BPM)
    // 240 ticks at 60 BPM = 0.5 seconds
    let seconds = ticks_to_seconds_with_tempo_map(240, 480, &tempo_map);
    assert!((seconds - 0.5).abs() < 0.001);
}
