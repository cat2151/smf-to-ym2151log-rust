use crate::ym2151::ToneDefinition;
use crate::Result;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

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
}

/// Optional conversion options supplied via attachment JSON
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConversionOptions {
    /// Enable delayed vibrato generation in the YM2151 log output
    #[serde(
        rename = "DelayVibrato",
        default,
        deserialize_with = "deserialize_delay_vibrato"
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

/// Defines a software LFO targeting a YM2151 tone register (per channel/operator)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterLfoDefinition {
    /// Base register address (channel 0 / operator base, e.g. "0x60")
    pub base_register: String,
    /// Peak modulation amount applied around the base register value
    #[serde(default)]
    pub depth: f64,
    /// Oscillation rate in Hz
    pub rate_hz: f64,
    /// Delay before the LFO starts.
    /// When `key_on_sync` is true (default), the delay is from each note-on.
    /// When `key_on_sync` is false, the delay is from the beginning of the song.
    #[serde(default)]
    pub delay_seconds: f64,
    /// Attack time before reaching full depth.
    /// When `key_on_sync` is true (default), the attack restarts on each note-on.
    /// When `key_on_sync` is false, the attack runs once from song start.
    #[serde(default)]
    pub attack_seconds: f64,
    /// Waveform shape
    #[serde(default = "default_lfo_waveform")]
    pub waveform: LfoWaveform,
    /// When true (default), the LFO phase and attack reset on each note-on (key-on sync).
    /// When false, the LFO is triggered once at the start of the song and runs continuously
    /// across all notes without resetting.
    #[serde(default = "default_key_on_sync")]
    pub key_on_sync: bool,
}

/// Defines a pitch vibrato applied after note-on with configurable depth and waveform
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DelayVibratoDefinition {
    /// Delay before the vibrato starts from note-on
    #[serde(default = "default_delay_vibrato_delay_seconds")]
    pub delay_seconds: f64,
    /// Attack time before reaching full depth
    #[serde(default = "default_delay_vibrato_attack_seconds")]
    pub attack_seconds: f64,
    /// Peak modulation amount in cents
    #[serde(default = "default_delay_vibrato_depth_cents")]
    pub depth_cents: f64,
    /// Oscillation rate in Hz
    #[serde(default = "default_delay_vibrato_rate_hz")]
    pub rate_hz: f64,
    /// Waveform shape
    #[serde(default = "default_lfo_waveform")]
    pub waveform: LfoWaveform,
}

/// Register override applied before a note-on to soften envelope transitions
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterOverride {
    /// Base register address (channel 0 / operator base, e.g. "0xA0")
    pub base_register: String,
    /// Override value written before restoring the base register
    pub value: String,
}

/// Pop-noise mitigation settings applied just before note-on
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PopNoiseEnvelope {
    /// Whether to apply pop-noise mitigation
    #[serde(default)]
    pub enabled: bool,
    /// How far before the note-on to apply the temporary envelope
    #[serde(default = "default_pre_note_offset")]
    pub offset_seconds: f64,
    /// Registers to override temporarily before restoring base values
    #[serde(default)]
    pub registers: Vec<RegisterOverride>,
}

/// Supported software LFO waveforms
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LfoWaveform {
    Triangle,
    Sine,
}

fn default_lfo_waveform() -> LfoWaveform {
    LfoWaveform::Triangle
}

fn default_key_on_sync() -> bool {
    true
}

fn default_delay_vibrato_delay_seconds() -> f64 {
    0.2
}

fn default_delay_vibrato_attack_seconds() -> f64 {
    0.3
}

fn default_delay_vibrato_depth_cents() -> f64 {
    100.0
}

fn default_delay_vibrato_rate_hz() -> f64 {
    6.0
}

fn default_change_to_next_tone_time() -> f64 {
    5.0
}

fn default_pre_note_offset() -> f64 {
    0.001
}

impl Default for DelayVibratoDefinition {
    fn default() -> Self {
        Self {
            delay_seconds: default_delay_vibrato_delay_seconds(),
            attack_seconds: default_delay_vibrato_attack_seconds(),
            depth_cents: default_delay_vibrato_depth_cents(),
            rate_hz: default_delay_vibrato_rate_hz(),
            waveform: default_lfo_waveform(),
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum DelayVibratoValue {
    Enabled(bool),
    Definition(DelayVibratoDefinition),
}

fn deserialize_delay_vibrato<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<DelayVibratoDefinition>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<DelayVibratoValue>::deserialize(deserializer)?;
    Ok(match value {
        Some(DelayVibratoValue::Enabled(true)) => Some(DelayVibratoDefinition::default()),
        Some(DelayVibratoValue::Enabled(false)) | None => None,
        Some(DelayVibratoValue::Definition(definition)) => Some(definition),
    })
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
mod tests {
    use super::*;

    #[test]
    fn test_from_attachment_bytes_empty() {
        let opts = ConversionOptions::from_attachment_bytes(None).unwrap();
        assert!(opts.delay_vibrato.is_none());
        assert!(opts.program_attachments.is_empty());
    }

    #[test]
    fn test_from_attachment_bytes_legacy_flat_object() {
        let json = br#"{"DelayVibrato": true, "Portamento": false}"#;
        let opts = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();
        assert_eq!(opts.delay_vibrato, Some(DelayVibratoDefinition::default()));
        assert!(!opts.portamento);
        assert!(opts.program_attachments.is_empty());
    }

    #[test]
    fn test_from_attachment_bytes_new_array_format() {
        let json = br#"[
          { "ProgramChange": 0, "DelayVibrato": true },
          { "ProgramChange": 1, "Portamento": true }
        ]"#;
        let opts = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();
        assert!(opts.delay_vibrato.is_none());
        assert!(!opts.portamento);
        assert_eq!(opts.program_attachments.len(), 2);
        assert_eq!(opts.program_attachments[0].program_change, 0);
        assert_eq!(
            opts.program_attachments[0].delay_vibrato,
            Some(DelayVibratoDefinition::default())
        );
        assert_eq!(opts.program_attachments[1].program_change, 1);
        assert!(opts.program_attachments[1].portamento);
    }

    #[test]
    fn test_from_attachment_bytes_delay_vibrato_object_with_sine_waveform() {
        let json = br#"{
          "DelayVibrato": {
            "DelaySeconds": 0.05,
            "AttackSeconds": 0.1,
            "DepthCents": 25.0,
            "RateHz": 5.0,
            "Waveform": "sine"
          }
        }"#;
        let opts = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();
        assert_eq!(
            opts.delay_vibrato,
            Some(DelayVibratoDefinition {
                delay_seconds: 0.05,
                attack_seconds: 0.1,
                depth_cents: 25.0,
                rate_hz: 5.0,
                waveform: LfoWaveform::Sine,
            })
        );
    }

    #[test]
    fn test_from_attachment_bytes_array_delay_vibrato_object() {
        let json = br#"[
          {
            "ProgramChange": 0,
            "DelayVibrato": {
              "DelaySeconds": 0.05,
              "AttackSeconds": 0.1,
              "DepthCents": 25.0,
              "RateHz": 5.0,
              "Waveform": "sine"
            }
          }
        ]"#;
        let opts = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();
        assert_eq!(opts.program_attachments.len(), 1);
        assert_eq!(
            opts.program_attachments[0].delay_vibrato,
            Some(DelayVibratoDefinition {
                delay_seconds: 0.05,
                attack_seconds: 0.1,
                depth_cents: 25.0,
                rate_hz: 5.0,
                waveform: LfoWaveform::Sine,
            })
        );
    }

    #[test]
    fn test_from_attachment_bytes_array_with_inline_tone() {
        let json = br#"[
          {
            "ProgramChange": 5,
            "Tone": {
              "events": [
                { "time": 0, "addr": "0x20", "data": "0xC7" }
              ]
            }
          }
        ]"#;
        let opts = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();
        assert_eq!(opts.program_attachments.len(), 1);
        assert_eq!(opts.program_attachments[0].program_change, 5);
        assert!(
            opts.tones.contains_key(&5),
            "Tone for program 5 should be in tones map"
        );
        assert_eq!(opts.tones[&5].events.len(), 1);
        assert_eq!(opts.tones[&5].events[0].addr, "0x20");
    }

    #[test]
    fn test_from_attachment_bytes_array_empty() {
        let json = b"[]";
        let opts = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();
        assert!(opts.program_attachments.is_empty());
    }

    #[test]
    fn test_from_attachment_bytes_change_to_next_tone_fields() {
        let json = br#"[
          {
            "ProgramChange": 0,
            "ChangeToNextTone": true,
            "ChangeToNextToneTime": 3.5,
            "Tone": { "events": [] }
          },
          {
            "ProgramChange": 1,
            "Tone": { "events": [] }
          }
        ]"#;
        let opts = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();
        assert_eq!(opts.program_attachments.len(), 2);
        assert!(opts.program_attachments[0].change_to_next_tone);
        assert!((opts.program_attachments[0].change_to_next_tone_time - 3.5).abs() < 1e-9);
        assert!(!opts.program_attachments[1].change_to_next_tone);
        assert!((opts.program_attachments[1].change_to_next_tone_time - 5.0).abs() < 1e-9);
    }
}
