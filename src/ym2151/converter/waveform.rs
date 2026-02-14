//! Waveform generation utilities
//!
//! Provides waveform generation helpers for LFO effects.

use crate::LfoWaveform;

pub(super) fn lfo_waveform_value(waveform: LfoWaveform, phase: f64) -> f64 {
    match waveform {
        LfoWaveform::Triangle => triangle_wave(phase),
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
