use serde::{Deserialize, Deserializer};

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

pub(crate) fn deserialize_delay_vibrato<'de, D>(
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
