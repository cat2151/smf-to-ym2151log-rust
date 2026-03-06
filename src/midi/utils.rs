//! MIDI utility functions
//!
//! Provides conversion functions for MIDI to YM2151 parameters.

use crate::ym2151::note_table::NOTE_TABLE;

/// Convert MIDI note number to frequency in Hz (A4 = 440 Hz)
pub fn midi_note_to_frequency(midi_note: u8) -> f64 {
    440.0 * 2_f64.powf((midi_note as f64 - 69.0) / 12.0)
}

/// Convert MIDI note with a cent offset to YM2151 KC (Key Code) and KF (Key Fraction)
pub fn midi_note_with_offset_to_kc_kf(midi_note: u8, cents_offset: f64) -> (u8, u8) {
    // Convert cents to fractional MIDI note offset
    let target_note = (midi_note as f64) + cents_offset / 100.0;
    let clamped_note = target_note.clamp(0.0, 127.0);

    // Align with the existing -1 MIDI offset used for YM2151 mapping
    let adjusted = (clamped_note - 1.0).max(0.0);
    let base_note = adjusted.floor() as u8;
    let fractional = adjusted - base_note as f64;

    let note_in_octave = (base_note % 12) as usize;
    let ym_octave = ((base_note / 12) as i8 - 2).clamp(0, 7) as u8;
    let ym_note = NOTE_TABLE[note_in_octave];
    let kc = (ym_octave << 4) | ym_note;

    // KF steps are 1/64 of a semitone on YM2151
    let kf = (fractional * 64.0).round().clamp(0.0, 63.0) as u8;

    (kc, kf)
}

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
/// assert_eq!(kc, 0x2E); // Octave 2, Note C
/// assert_eq!(kf, 0);
/// ```
pub fn midi_to_kc_kf(midi_note: u8) -> (u8, u8) {
    // Adjust MIDI note by -1 to align octaves between MIDI and YM2151 numbering
    let adjusted_midi = if midi_note > 0 { midi_note - 1 } else { 0 };
    let note_in_octave = (adjusted_midi % 12) as usize;

    let ym_octave = ((adjusted_midi / 12) as i8 - 2).clamp(0, 7) as u8;
    let ym_note = NOTE_TABLE[note_in_octave];
    let kc = (ym_octave << 4) | ym_note;
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

/// Convert MIDI ticks to seconds with tempo changes
///
/// This function correctly handles tempo changes by calculating accumulated time
/// across different tempo segments.
///
/// # Arguments
/// * `target_tick` - The tick to convert to seconds
/// * `ticks_per_beat` - Ticks per quarter note (from MIDI file)
/// * `tempo_map` - Sorted list of tempo changes (by tick)
///
/// # Returns
/// Time in seconds (f64)
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::{ticks_to_seconds_with_tempo_map, TempoChange};
/// let tempo_map = vec![
///     TempoChange { tick: 0, tempo_bpm: 120.0 },
///     TempoChange { tick: 480, tempo_bpm: 60.0 },
/// ];
/// // First beat at 120 BPM = 0.5 seconds
/// let seconds = ticks_to_seconds_with_tempo_map(480, 480, &tempo_map);
/// assert!((seconds - 0.5).abs() < 0.001);
/// ```
pub fn ticks_to_seconds_with_tempo_map(
    target_tick: u32,
    ticks_per_beat: u16,
    tempo_map: &[TempoChange],
) -> f64 {
    if tempo_map.is_empty() {
        // No tempo changes - use default 120 BPM
        return ticks_to_seconds(target_tick, ticks_per_beat, 120.0);
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
            return accumulated_seconds;
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

    accumulated_seconds
}

#[cfg(test)]
#[path = "utils_tests.rs"]
mod tests;
