use smf_to_ym2151log::{ConversionOptions, RegisterLfoDefinition, LfoWaveform};
use smf_to_ym2151log::midi::MidiData;
use smf_to_ym2151log::midi::MidiEvent;
use smf_to_ym2151log::ym2151::convert_to_ym2151_log_with_options;

#[test]
fn test_lfo_smooth_interpolation() {
    let midi_data = MidiData {
        ticks_per_beat: 480,
        tempo_bpm: 120.0,
        events: vec![
            MidiEvent::NoteOn {
                ticks: 0,
                channel: 0,
                note: 60,
                velocity: 100,
            },
            MidiEvent::NoteOff {
                ticks: 480,
                channel: 0,
                note: 60,
            },
        ],
    };

    let options = ConversionOptions {
        software_lfo: vec![RegisterLfoDefinition {
            base_register: "0x60".to_string(),
            depth: 10.0,
            rate_hz: 1.0,  // 1 Hz = 1 complete cycle per second
            delay_seconds: 0.0,
            attack_seconds: 0.0,
            waveform: LfoWaveform::Triangle,
        }],
        ..ConversionOptions::default()
    };

    let result = convert_to_ym2151_log_with_options(&midi_data, &options).unwrap();

    let lfo_events: Vec<_> = result
        .events
        .iter()
        .filter(|e| e.addr == "0x60" && e.time > 0.0 && e.time < 1.1)
        .collect();

    println!("\nLFO Events (first 50):");
    for (i, event) in lfo_events.iter().take(50).enumerate() {
        let data_val = event.data.strip_prefix("0x").unwrap_or(&event.data);
        let val = u8::from_str_radix(data_val, 16).unwrap_or(0);
        println!("{:3}: time={:.4}s, data={:3} (0x{:02X})", i, event.time, val, val);
    }
    
    // Check unique values - should be many if smooth interpolation
    let unique_values: std::collections::HashSet<_> = lfo_events.iter().map(|e| &e.data).collect();
    println!("\nTotal LFO events: {}", lfo_events.len());
    println!("Unique register values: {}", unique_values.len());
    println!("All unique values: {:?}", unique_values);
    
    // With smooth interpolation and depth=10, we should have much more than 3 values
    assert!(
        unique_values.len() > 3,
        "Expected smooth interpolation with many values, but got only {} unique values",
        unique_values.len()
    );
}
