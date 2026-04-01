//! Waveform generation utilities
//!
//! Provides waveform generation helpers for LFO effects.

use crate::LfoWaveform;

pub(super) fn lfo_waveform_value(waveform: LfoWaveform, phase: f64) -> f64 {
    match waveform {
        LfoWaveform::Triangle => triangle_wave(phase),
        LfoWaveform::Sine => sine_wave(phase),
    }
}

pub(super) fn triangle_wave(phase: f64) -> f64 {
    let wrapped = phase - phase.floor();
    if wrapped < 0.25 {
        wrapped / 0.25
    } else if wrapped < 0.5 {
        1.0 - ((wrapped - 0.25) / 0.25)
    } else if wrapped < 0.75 {
        -((wrapped - 0.5) / 0.25)
    } else {
        -1.0 + ((wrapped - 0.75) / 0.25)
    }
}

pub(super) fn sine_wave(phase: f64) -> f64 {
    let wrapped = phase - phase.floor();
    (wrapped * std::f64::consts::TAU).sin()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_wave_matches_cardinal_points() {
        assert!(sine_wave(0.0).abs() < 1e-12);
        assert!((sine_wave(0.25) - 1.0).abs() < 1e-12);
        assert!(sine_wave(0.5).abs() < 1e-12);
        assert!((sine_wave(0.75) + 1.0).abs() < 1e-12);
    }
}
