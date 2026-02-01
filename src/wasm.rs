//! WASM bindings for browser usage
//!
//! This module provides WebAssembly bindings to convert SMF binary data to JSON
//! for use in web browsers via JavaScript.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// Convert SMF binary data to YM2151 register log JSON (WASM entry point)
///
/// This function is exposed to JavaScript for browser usage.
///
/// # Arguments
/// * `smf_data` - Standard MIDI File binary data as bytes
///
/// # Returns
/// YM2151 register log as JSON string on success, or error message on failure
///
/// # Example (JavaScript)
/// ```javascript
/// import init, { smf_to_ym2151_json } from './pkg/smf_to_ym2151log.js';
///
/// await init();
/// const midiBytes = new Uint8Array([...]); // Your MIDI file bytes
/// const jsonResult = smf_to_ym2151_json(midiBytes);
/// console.log(jsonResult);
/// ```
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn smf_to_ym2151_json(smf_data: &[u8]) -> String {
    match crate::convert_smf_to_ym2151_log(smf_data) {
        Ok(json) => json,
        Err(e) => format!("{{\"error\": \"{}\"}}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "wasm")]
    fn test_wasm_function_exists() {
        // This test just ensures the function compiles when wasm feature is enabled
        let empty_smf: Vec<u8> = vec![];
        let result = smf_to_ym2151_json(&empty_smf);
        assert!(result.contains("error") || result.contains("events"));
    }
}
