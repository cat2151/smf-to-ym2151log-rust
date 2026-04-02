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
        "ChangeToNextToneKeepFields": ["Mul", "Alg"],
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
    assert_eq!(
        opts.program_attachments[0].change_to_next_tone_keep_fields,
        vec![
            ChangeToNextToneKeepField::Mul,
            ChangeToNextToneKeepField::Alg
        ]
    );
    assert!(!opts.program_attachments[1].change_to_next_tone);
    assert!((opts.program_attachments[1].change_to_next_tone_time - 5.0).abs() < 1e-9);
}
