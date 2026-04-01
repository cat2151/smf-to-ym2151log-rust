use smf_to_ym2151log::{ConversionOptions, DelayVibratoDefinition};

#[test]
fn test_public_conversion_options_reexport_parses_attachment_json() {
    let json = br#"{"DelayVibrato": true}"#;
    let options = ConversionOptions::from_attachment_bytes(Some(json)).unwrap();

    assert_eq!(
        options.delay_vibrato,
        Some(DelayVibratoDefinition::default())
    );
}
