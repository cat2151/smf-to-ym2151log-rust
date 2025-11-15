//! MIDI utility functions
//!
//! Provides conversion functions for MIDI to YM2151 parameters.

use crate::ym2151::note_table::NOTE_TABLE;

/// Convert MIDI note to YM2151 KC (Key Code) and KF (Key Fraction)
///
/// # Arguments
/// * `midi_note` - MIDI note number (0-127)
///
/// # Returns
/// Tuple of (KC, KF) where KC is the key code and KF is the key fraction
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::midi_to_kc_kf;
/// let (kc, kf) = midi_to_kc_kf(60); // Middle C (C4)
/// assert_eq!(kc, 0x3E); // Octave 3, Note C
/// assert_eq!(kf, 0);
/// ```
pub fn midi_to_kc_kf(midi_note: u8) -> (u8, u8) {
    // Adjust MIDI note by -1 to align octaves between MIDI and YM2151 numbering
    let adjusted_midi = if midi_note > 0 { midi_note - 1 } else { 0 };
    let mut midi_octave = (adjusted_midi / 12).saturating_sub(1);

    // Clamp octave to valid range (0-7 for YM2151)
    midi_octave = midi_octave.min(7);

    let note_in_octave = (adjusted_midi % 12) as usize;
    let ym_note = NOTE_TABLE[note_in_octave];

    let kc = (midi_octave << 4) | ym_note;
    let kf = 0; // No fine tuning for now

    (kc, kf)
}

/// Convert MIDI ticks to seconds
///
/// # Arguments
/// * `ticks` - Number of MIDI ticks
/// * `ticks_per_beat` - Ticks per quarter note (from MIDI file)
/// * `tempo_bpm` - Tempo in beats per minute
///
/// # Returns
/// Time in seconds
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::ticks_to_seconds;
/// let seconds = ticks_to_seconds(480, 480, 120.0);
/// assert!((seconds - 0.5).abs() < 0.001); // 1 beat at 120 BPM = 0.5 seconds
/// ```
pub fn ticks_to_seconds(ticks: u32, ticks_per_beat: u16, tempo_bpm: f64) -> f64 {
    let seconds_per_beat = 60.0 / tempo_bpm;
    let seconds_per_tick = seconds_per_beat / ticks_per_beat as f64;
    ticks as f64 * seconds_per_tick
}

/// YM2151 sample rate constant
pub const YM2151_SAMPLE_RATE: u32 = 55930;

/// Convert seconds to sample count at 55930 Hz
///
/// # Arguments
/// * `seconds` - Time in seconds
///
/// # Returns
/// Sample count at 55930 Hz
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::seconds_to_samples;
/// let samples = seconds_to_samples(1.0);
/// assert_eq!(samples, 55930);
/// ```
pub fn seconds_to_samples(seconds: f64) -> u32 {
    (seconds * YM2151_SAMPLE_RATE as f64) as u32
}

/// Convert MIDI ticks directly to sample count
///
/// # Arguments
/// * `ticks` - Number of MIDI ticks
/// * `ticks_per_beat` - Ticks per quarter note (from MIDI file)
/// * `tempo_bpm` - Tempo in beats per minute
///
/// # Returns
/// Sample count at 55930 Hz
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::ticks_to_samples;
/// let samples = ticks_to_samples(480, 480, 120.0);
/// assert_eq!(samples, 27965); // 0.5 seconds at 55930 Hz
/// ```
pub fn ticks_to_samples(ticks: u32, ticks_per_beat: u16, tempo_bpm: f64) -> u32 {
    let seconds = ticks_to_seconds(ticks, ticks_per_beat, tempo_bpm);
    seconds_to_samples(seconds)
}

/// Represents a tempo change at a specific tick
#[derive(Debug, Clone, Copy)]
pub struct TempoChange {
    pub tick: u32,
    pub tempo_bpm: f64,
}

/// Convert MIDI ticks to sample count with tempo changes
///
/// This function correctly handles tempo changes by calculating accumulated time
/// across different tempo segments.
///
/// # Arguments
/// * `target_tick` - The tick to convert to sample time
/// * `ticks_per_beat` - Ticks per quarter note (from MIDI file)
/// * `tempo_map` - Sorted list of tempo changes (by tick)
///
/// # Returns
/// Sample count at 55930 Hz
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::{ticks_to_samples_with_tempo_map, TempoChange};
/// let tempo_map = vec![
///     TempoChange { tick: 0, tempo_bpm: 120.0 },
///     TempoChange { tick: 480, tempo_bpm: 60.0 },
/// ];
/// // First beat at 120 BPM = 27965 samples
/// let samples = ticks_to_samples_with_tempo_map(480, 480, &tempo_map);
/// assert_eq!(samples, 27965);
/// ```
pub fn ticks_to_samples_with_tempo_map(
    target_tick: u32,
    ticks_per_beat: u16,
    tempo_map: &[TempoChange],
) -> u32 {
    if tempo_map.is_empty() {
        // No tempo changes - use default 120 BPM
        return ticks_to_samples(target_tick, ticks_per_beat, 120.0);
    }

    let mut accumulated_seconds = 0.0;
    let mut prev_tick = 0;

    for (i, tempo_change) in tempo_map.iter().enumerate() {
        if target_tick <= tempo_change.tick {
            // Target is before or at this tempo change
            if i == 0 {
                // Target is before the first tempo change
                let ticks_in_segment = target_tick;
                accumulated_seconds +=
                    ticks_to_seconds(ticks_in_segment, ticks_per_beat, tempo_change.tempo_bpm);
            } else {
                // Use the previous tempo for the remaining ticks
                let prev_tempo = tempo_map[i - 1].tempo_bpm;
                let ticks_in_segment = target_tick - prev_tick;
                accumulated_seconds +=
                    ticks_to_seconds(ticks_in_segment, ticks_per_beat, prev_tempo);
            }
            return seconds_to_samples(accumulated_seconds);
        }

        // Calculate time in this tempo segment
        if i > 0 {
            let ticks_in_segment = tempo_change.tick - prev_tick;
            let prev_tempo = tempo_map[i - 1].tempo_bpm;
            accumulated_seconds += ticks_to_seconds(ticks_in_segment, ticks_per_beat, prev_tempo);
        }

        prev_tick = tempo_change.tick;
    }

    // Target is after all tempo changes - use the last tempo
    let last_tempo = tempo_map.last().unwrap().tempo_bpm;
    let ticks_in_segment = target_tick - prev_tick;
    accumulated_seconds += ticks_to_seconds(ticks_in_segment, ticks_per_beat, last_tempo);

    seconds_to_samples(accumulated_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    // MIDI to KC/KF conversion tests
    #[test]
    fn test_midi_to_kc_kf_middle_c() {
        // MIDI note 60 = C4 (Middle C)
        let (kc, kf) = midi_to_kc_kf(60);
        assert_eq!(kc, 0x3E); // Octave 3, Note C
        assert_eq!(kf, 0);
    }

    #[test]
    fn test_midi_to_kc_kf_a440() {
        // MIDI note 69 = A4 (A440)
        let (kc, kf) = midi_to_kc_kf(69);
        assert_eq!(kc, 0x4A); // Octave 4, Note A
        assert_eq!(kf, 0);
    }

    #[test]
    fn test_midi_to_kc_kf_octaves() {
        // Test representative notes across different octaves
        // C notes from different octaves
        let (kc, _) = midi_to_kc_kf(24); // C1
        assert_eq!(kc, 0x0E); // Octave 0, Note C

        let (kc, _) = midi_to_kc_kf(36); // C2
        assert_eq!(kc, 0x1E); // Octave 1, Note C

        let (kc, _) = midi_to_kc_kf(48); // C3
        assert_eq!(kc, 0x2E); // Octave 2, Note C

        let (kc, _) = midi_to_kc_kf(60); // C4
        assert_eq!(kc, 0x3E); // Octave 3, Note C

        let (kc, _) = midi_to_kc_kf(72); // C5
        assert_eq!(kc, 0x4E); // Octave 4, Note C

        let (kc, _) = midi_to_kc_kf(84); // C6
        assert_eq!(kc, 0x5E); // Octave 5, Note C

        let (kc, _) = midi_to_kc_kf(96); // C7
        assert_eq!(kc, 0x6E); // Octave 6, Note C
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
        for midi_note in 108..=127 {
            let (kc, _) = midi_to_kc_kf(midi_note);
            let octave = (kc >> 4) & 0x07;
            assert_eq!(
                octave, 7,
                "Failed to clamp octave for MIDI note {}",
                midi_note
            );
        }
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
}
