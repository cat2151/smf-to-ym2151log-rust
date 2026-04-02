use super::effects::deserialize_delay_vibrato;
use super::{DelayVibratoDefinition, PopNoiseEnvelope, RegisterLfoDefinition};
use crate::ym2151::ToneDefinition;
use serde::Deserialize;

/// Per-program attachment entry used in the new array format.
///
/// Each entry in an attachment JSON array identifies the program it configures
/// via `ProgramChange` and bundles all per-program settings together.
///
/// # Example (new array format)
/// ```json
/// [
///   {
///     "ProgramChange": 0,
///     "DelayVibrato": true,
///     "Tone": { "events": [{ "time": 0, "addr": "0x20", "data": "0xC7" }] }
///   }
/// ]
/// ```
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ProgramAttachment {
    /// Program number (0-127) this entry applies to
    #[serde(rename = "ProgramChange")]
    pub program_change: u8,
    /// Enable delayed vibrato for this program
    #[serde(
        rename = "DelayVibrato",
        default,
        deserialize_with = "deserialize_delay_vibrato"
    )]
    pub delay_vibrato: Option<DelayVibratoDefinition>,
    /// Enable portamento glides between consecutive notes for this program
    #[serde(rename = "Portamento", default)]
    pub portamento: bool,
    /// Optional pre-note envelope overrides to reduce pop noise for this program
    #[serde(rename = "PopNoiseEnvelope", default)]
    pub pop_noise_envelope: Option<PopNoiseEnvelope>,
    /// Optional software LFO definitions for this program
    #[serde(rename = "SoftwareLfo", default)]
    pub software_lfo: Vec<RegisterLfoDefinition>,
    /// Optional inline tone definition for this program
    #[serde(rename = "Tone", default)]
    pub tone: Option<ToneDefinition>,
    /// Enable looping linear interpolation toward the next program's tone (program_change + 1).
    /// When true, register values are continuously morphed from this program's tone
    /// to the next program's tone over `change_to_next_tone_time` seconds, then back,
    /// repeating for the duration of the song.
    #[serde(rename = "ChangeToNextTone", default)]
    pub change_to_next_tone: bool,
    /// Duration in seconds for one interpolation direction (tone N → tone N+1 or back).
    /// Defaults to 5.0 seconds.
    #[serde(
        rename = "ChangeToNextToneTime",
        default = "default_change_to_next_tone_time"
    )]
    pub change_to_next_tone_time: f64,
    /// Optional list of packed YM2151 fields to keep from the current tone while
    /// interpolating toward the next tone. This lets the attachment JSON preserve
    /// parameters such as MUL or ALG/CON without mutating the source `registers`.
    #[serde(rename = "ChangeToNextToneKeepFields", default)]
    pub change_to_next_tone_keep_fields: Vec<ChangeToNextToneKeepField>,
}

/// Packed YM2151 fields that can be preserved from the current tone during
/// ChangeToNextTone interpolation.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ChangeToNextToneKeepField {
    #[serde(alias = "Con", alias = "Connection")]
    Alg,
    Fb,
    Rl,
    Mul,
    Dt1,
    Tl,
    Ks,
    Ar,
    #[serde(alias = "AmsEnable")]
    AmsEn,
    D1r,
    Dt2,
    D2r,
    D1l,
    Rr,
    Pms,
    Ams,
}

fn default_change_to_next_tone_time() -> f64 {
    5.0
}
