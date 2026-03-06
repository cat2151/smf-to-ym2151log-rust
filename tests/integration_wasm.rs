//! Integration tests for WASM interface and convenience functions

use std::fs;

#[test]
#[cfg(feature = "wasm")]
fn test_wasm_smf_to_ym2151_json_valid_midi() {
    use smf_to_ym2151log::wasm::smf_to_ym2151_json;

    let midi_path = "tests/test_data/simple_melody.mid";
    let smf_bytes = fs::read(midi_path).expect("Failed to read test MIDI file");

    let result = smf_to_ym2151_json(&smf_bytes);

    // Parse the result as JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&result).expect("Result should be valid JSON");

    // Should not have an error field
    assert!(
        parsed.get("error").is_none(),
        "Should not have error field for valid MIDI"
    );

    // Should have expected structure
    assert!(
        parsed.get("event_count").is_some(),
        "Should have event_count field"
    );
    assert!(parsed.get("events").is_some(), "Should have events field");

    let event_count = parsed["event_count"]
        .as_u64()
        .expect("event_count should be a number");
    assert!(event_count > 0, "Should have at least some events");
}

#[test]
#[cfg(feature = "wasm")]
fn test_wasm_smf_to_ym2151_json_invalid_midi() {
    use smf_to_ym2151log::wasm::smf_to_ym2151_json;

    // Invalid MIDI data
    let invalid_data = vec![0x00, 0x01, 0x02, 0x03];

    let result = smf_to_ym2151_json(&invalid_data);

    // Parse the result as JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&result).expect("Result should be valid JSON");

    // Should have an error field
    assert!(
        parsed.get("error").is_some(),
        "Should have error field for invalid MIDI"
    );
}

#[test]
#[cfg(feature = "wasm")]
fn test_wasm_smf_to_ym2151_json_empty_data() {
    use smf_to_ym2151log::wasm::smf_to_ym2151_json;

    let empty_data = vec![];

    let result = smf_to_ym2151_json(&empty_data);

    // Parse the result as JSON
    let parsed: serde_json::Value =
        serde_json::from_str(&result).expect("Result should be valid JSON");

    // Should have an error field
    assert!(
        parsed.get("error").is_some(),
        "Should have error field for empty data"
    );
}

#[test]
fn test_convert_smf_to_ym2151_log_end_to_end() {
    // This test is always available and tests the convenience function
    let midi_path = "tests/test_data/simple_melody.mid";
    let smf_bytes = fs::read(midi_path).expect("Failed to read test MIDI file");

    let result = smf_to_ym2151log::convert_smf_to_ym2151_log(&smf_bytes);
    assert!(
        result.is_ok(),
        "Should successfully convert SMF bytes: {:?}",
        result.err()
    );

    let json = result.unwrap();

    // Parse and verify structure
    let parsed: serde_json::Value =
        serde_json::from_str(&json).expect("Result should be valid JSON");

    assert!(parsed.get("event_count").is_some());
    assert!(parsed.get("events").is_some());

    let events = parsed["events"].as_array().expect("events should be array");
    assert!(!events.is_empty(), "Should have events");

    // Verify event structure
    for event in events {
        assert!(event.get("time").is_some(), "Event should have time field");
        assert!(event.get("addr").is_some(), "Event should have addr field");
        assert!(event.get("data").is_some(), "Event should have data field");

        let addr = event["addr"].as_str().expect("addr should be string");
        let data = event["data"].as_str().expect("data should be string");

        assert!(addr.starts_with("0x"), "addr should start with 0x");
        assert!(data.starts_with("0x"), "data should start with 0x");
    }
}
