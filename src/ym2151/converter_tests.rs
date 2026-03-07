//! Tests for YM2151 converter (Pass B)
//!
//! These tests verify the conversion of MIDI events to YM2151 register write events.
//! Tests are split into submodules by category.

// Re-export items needed by test submodules
pub use super::{convert_to_ym2151_log, convert_to_ym2151_log_with_options};
pub use crate::midi::{midi_to_kc_kf, MidiData, MidiEvent};
pub use crate::ym2151::{ToneDefinition, Ym2151Event};
pub use crate::{
    AttackContinuationFix, ConversionOptions, LfoWaveform, PopNoiseEnvelope, ProgramAttachment,
    RegisterLfoDefinition, RegisterOverride,
};

#[path = "converter_tests/attachments.rs"]
mod attachments;
#[path = "converter_tests/basic.rs"]
mod basic;
#[path = "converter_tests/channels.rs"]
mod channels;
#[path = "converter_tests/drums.rs"]
mod drums;
#[path = "converter_tests/effects.rs"]
mod effects;
#[path = "converter_tests/programs.rs"]
mod programs;
