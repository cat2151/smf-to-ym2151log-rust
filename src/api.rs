use crate::{midi, ym2151, ConversionOptions, Result};

/// Convert Standard MIDI File data to YM2151 register log JSON
///
/// This is a convenience function that combines both passes:
/// - Pass A: Parse MIDI data from bytes
/// - Pass B: Convert to YM2151 register log
///
/// # Arguments
/// * `smf_data` - Raw Standard MIDI File data as bytes
///
/// # Returns
/// YM2151 register log as JSON string
///
/// # Errors
/// Returns an error if parsing or conversion fails
///
/// # Example
/// ```no_run
/// use smf_to_ym2151log::convert_smf_to_ym2151_log;
///
/// let smf_bytes = std::fs::read("song.mid").unwrap();
/// let ym2151_json = convert_smf_to_ym2151_log(&smf_bytes).unwrap();
/// println!("{}", ym2151_json);
/// ```
pub fn convert_smf_to_ym2151_log(smf_data: &[u8]) -> Result<String> {
    let midi_data = midi::parse_midi_from_bytes(smf_data)?;
    let ym2151_log = ym2151::convert_to_ym2151_log(&midi_data)?;
    let json = serde_json::to_string_pretty(&ym2151_log)?;
    Ok(json)
}

/// Convert SMF data to YM2151 register log JSON with optional attachment options
///
/// This variant accepts an optional attachment JSON to control conversion behavior.
pub fn convert_smf_to_ym2151_log_with_options(
    smf_data: &[u8],
    attachment_json: Option<&[u8]>,
) -> Result<String> {
    let midi_data = midi::parse_midi_from_bytes(smf_data)?;
    let options = ConversionOptions::from_attachment_bytes(attachment_json)?;
    let ym2151_log = ym2151::convert_to_ym2151_log_with_options(&midi_data, &options)?;
    let json = serde_json::to_string_pretty(&ym2151_log)?;
    Ok(json)
}
