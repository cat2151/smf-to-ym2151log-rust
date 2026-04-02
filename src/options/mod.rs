mod attachments;
mod effects;

pub use attachments::{ChangeToNextToneKeepField, ProgramAttachment};
pub use effects::{
    DelayVibratoDefinition, LfoWaveform, PopNoiseEnvelope, RegisterLfoDefinition, RegisterOverride,
};

use crate::ym2151::ToneDefinition;
use crate::Result;
use serde::Deserialize;
use std::collections::HashMap;

/// Optional conversion options supplied via attachment JSON
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConversionOptions {
    /// Enable delayed vibrato generation in the YM2151 log output
    #[serde(
        rename = "DelayVibrato",
        default,
        deserialize_with = "effects::deserialize_delay_vibrato"
    )]
    pub delay_vibrato: Option<DelayVibratoDefinition>,
    /// Enable portamento glides between consecutive notes
    #[serde(rename = "Portamento", default)]
    pub portamento: bool,
    /// Optional pre-note envelope overrides to reduce pop noise
    #[serde(rename = "PopNoiseEnvelope", default)]
    pub pop_noise_envelope: Option<PopNoiseEnvelope>,
    /// Optional software LFO definitions that modulate tone registers
    #[serde(rename = "SoftwareLfo", default)]
    pub software_lfo: Vec<RegisterLfoDefinition>,
    /// Optional YM2151 tone definitions keyed by MIDI program number
    #[serde(rename = "Tones", default)]
    pub tones: HashMap<u8, ToneDefinition>,
    /// Per-program attachment entries (new array format).
    /// Populated when the attachment JSON is an array of `ProgramAttachment` objects.
    #[serde(skip)]
    pub program_attachments: Vec<ProgramAttachment>,
}

impl ConversionOptions {
    /// Build conversion options from an optional attachment JSON payload.
    ///
    /// Accepts two formats:
    /// - **New array format**: an array of [`ProgramAttachment`] objects, each with a
    ///   `ProgramChange` field identifying which program the settings apply to.
    /// - **Legacy object format**: a flat JSON object with top-level fields such as
    ///   `DelayVibrato`, `Portamento`, `Tones`, etc. (still supported for backward compatibility).
    ///
    /// If no payload is provided, or the payload is empty, defaults are used.
    pub fn from_attachment_bytes(attachment_json: Option<&[u8]>) -> Result<Self> {
        match attachment_json {
            Some(bytes) if !bytes.is_empty() => {
                let value: serde_json::Value = serde_json::from_slice(bytes)?;
                if value.is_array() {
                    let attachments: Vec<ProgramAttachment> = serde_json::from_value(value)?;
                    let mut options = ConversionOptions::default();
                    for attachment in &attachments {
                        if let Some(tone) = &attachment.tone {
                            options
                                .tones
                                .insert(attachment.program_change, tone.clone());
                        }
                    }
                    options.program_attachments = attachments;
                    Ok(options)
                } else {
                    let options: ConversionOptions = serde_json::from_value(value)?;
                    Ok(options)
                }
            }
            _ => Ok(ConversionOptions::default()),
        }
    }
}

#[cfg(test)]
mod tests;
