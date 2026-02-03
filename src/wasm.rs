//! WASM bindings for browser usage
//!
//! This module provides WebAssembly bindings to convert SMF binary data and MML strings to JSON
//! for use in web browsers via JavaScript.

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;

/// Convert SMF binary data to YM2151 register log JSON (WASM entry point)
///
/// This function is exposed to JavaScript for browser usage.
///
/// # Arguments
/// * `smf_data` - Standard MIDI File binary data as bytes
///
/// # Returns
/// YM2151 register log as a JSON string on success, or a JSON string containing an
/// `error` field (e.g. `{"error": "<message>"}`) on failure.
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
#[cfg_attr(all(feature = "wasm", target_arch = "wasm32"), wasm_bindgen)]
pub fn smf_to_ym2151_json(smf_data: &[u8]) -> String {
    match crate::convert_smf_to_ym2151_log(smf_data) {
        Ok(json) => json,
        Err(e) => {
            // Use serde_json to properly escape error messages
            let error_obj = serde_json::json!({
                "error": e.to_string()
            });
            serde_json::to_string(&error_obj)
                .unwrap_or_else(|_| r#"{"error": "Unknown error"}"#.to_string())
        }
    }
}

/// Convert MML string to YM2151 register log JSON (WASM entry point)
///
/// This function is exposed to JavaScript for browser usage.
///
/// # Arguments
/// * `mml` - Music Macro Language string (e.g., "cdefgab", "o5 l4 c;e;g")
///
/// # Returns
/// YM2151 register log as a JSON string on success, or a JSON string containing an
/// `error` field (e.g. `{"error": "<message>"}`) on failure.
///
/// # Example (JavaScript)
/// ```javascript
/// import init, { mml_to_ym2151_json } from './pkg/smf_to_ym2151log.js';
///
/// await init();
/// const mml = "cdefgab";
/// const jsonResult = mml_to_ym2151_json(mml);
/// console.log(jsonResult);
/// ```
#[cfg(feature = "wasm")]
#[cfg_attr(all(feature = "wasm", target_arch = "wasm32"), wasm_bindgen)]
pub fn mml_to_ym2151_json(mml: &str) -> String {
    match crate::convert_mml_to_ym2151_log(mml) {
        Ok(json) => json,
        Err(e) => {
            // Use serde_json to properly escape error messages
            let error_obj = serde_json::json!({
                "error": e.to_string()
            });
            serde_json::to_string(&error_obj)
                .unwrap_or_else(|_| r#"{"error": "Unknown error"}"#.to_string())
        }
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

    #[test]
    #[cfg(feature = "wasm")]
    fn test_wasm_error_json_escaping() {
        // Test that error messages with special characters are properly escaped
        let invalid_data = vec![0x00, 0x01, 0x02];
        let result = smf_to_ym2151_json(&invalid_data);

        // Result should be valid JSON
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&result);
        assert!(
            parsed.is_ok(),
            "Error response should be valid JSON: {}",
            result
        );

        // Should have an error field
        let json = parsed.unwrap();
        assert!(
            json.get("error").is_some(),
            "Error response should have error field"
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_mml_to_ym2151_json_simple() {
        // Test MML conversion with a simple melody
        let mml = "cdefgab";
        let result = mml_to_ym2151_json(mml);

        // Should be valid JSON
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&result);
        assert!(parsed.is_ok(), "Result should be valid JSON: {}", result);

        // Should not be an error
        let json = parsed.unwrap();
        assert!(
            json.get("error").is_none(),
            "Simple MML should not produce error"
        );
        assert!(
            json.get("event_count").is_some(),
            "Result should have event_count"
        );
    }

    #[test]
    #[cfg(feature = "wasm")]
    fn test_mml_to_ym2151_json_empty() {
        // Test MML conversion with empty string
        let mml = "";
        let result = mml_to_ym2151_json(mml);

        // Should be valid JSON
        let parsed: Result<serde_json::Value, _> = serde_json::from_str(&result);
        assert!(parsed.is_ok(), "Result should be valid JSON: {}", result);
    }
}
