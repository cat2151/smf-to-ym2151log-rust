//! MIDI utility functions
//!
//! Provides conversion functions for MIDI to YM2151 parameters.

/// Convert MIDI note to YM2151 KC (Key Code) and KF (Key Fraction)
///
/// # Arguments
/// * `midi_note` - MIDI note number (0-127)
///
/// # Returns
/// Tuple of (KC, KF) where KC is the key code and KF is the key fraction
///
/// # Example
/// ```ignore
/// use smf_to_ym2151log::midi::midi_to_kc_kf;
/// let (kc, kf) = midi_to_kc_kf(60); // Middle C
/// ```
pub fn midi_to_kc_kf(_midi_note: u8) -> (u8, u8) {
    // TODO: Implement in Phase 3
    unimplemented!("MIDI to KC/KF conversion will be implemented in Phase 3")
}

/// Convert MIDI ticks to seconds
pub fn ticks_to_seconds(_ticks: u32, _ticks_per_beat: u16, _tempo_bpm: f64) -> f64 {
    // TODO: Implement in Phase 3
    unimplemented!("Timing conversion will be implemented in Phase 3")
}

/// Convert seconds to sample count at 55930 Hz
pub fn seconds_to_samples(_seconds: f64) -> u32 {
    // TODO: Implement in Phase 3
    unimplemented!("Timing conversion will be implemented in Phase 3")
}

/// Convert MIDI ticks directly to sample count
pub fn ticks_to_samples(_ticks: u32, _ticks_per_beat: u16, _tempo_bpm: f64) -> u32 {
    // TODO: Implement in Phase 3
    unimplemented!("Timing conversion will be implemented in Phase 3")
}
