//! Register-side effects
//!
//! Provides software LFO, pop-noise envelope, and tone interpolation implementations.

mod common;
mod pop_noise;
mod register_lfo;
mod state_cache;
mod tone_interpolation;

pub(super) use pop_noise::append_pop_noise_envelope_events;
pub(super) use register_lfo::append_register_lfo_events;
pub(super) use state_cache::build_register_state_cache;
pub(super) use tone_interpolation::append_change_to_next_tone_events;
