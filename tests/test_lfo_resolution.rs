use smf_to_ym2151log::{ConversionOptions, RegisterLfoDefinition, LfoWaveform};
use smf_to_ym2151log::midi::MidiData;
use smf_to_ym2151log::midi::MidiEvent;
use smf_to_ym2151log::ym2151::convert_to_ym2151_log_with_options;

#[test]
fn lfo_output_resolution_analysis() {
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
            rate_hz: 1.0,
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

    println!("\n=== LFO Output Resolution Analysis ===");
    println!("Configuration:");
    println!("  Base Register: 0x60");
    println!("  Depth: 10.0");
    println!("  Rate: 1.0 Hz (1 second period)");
    println!("  Attack: 0.0 seconds");
    println!("  Base Value: 0");
    println!("  Expected range: 0 to 10 (but could go negative to -10)");

    println!("\nGenerated Events:");
    for (i, event) in lfo_events.iter().enumerate() {
        let data_val = event.data.strip_prefix("0x").unwrap_or(&event.data);
        let val = u8::from_str_radix(data_val, 16).unwrap_or(0);
        println!("{:2}: t={:.4}s, value={:3}", i, event.time, val);
    }

    let unique_values: std::collections::HashSet<_> = lfo_events.iter().map(|e| &e.data).collect();
    
    println!("\nAnalysis:");
    println!("  Total events emitted: {}", lfo_events.len());
    println!("  Unique values: {}", unique_values.len());
    println!("  Values: {:?}", unique_values);

    println!("\nProblem Identified:");
    println!("  The LFO is using time_step = (1.0 / rate_hz) / 8.0");
    println!("  This results in: time_step = 1.0 / 8.0 = 0.125 seconds");
    println!("  Only 4 samples per second with 1 Hz LFO rate");
    println!("  Events are only emitted when (base_value + depth*waveform).round() changes");
    println!("  This causes discrete jumps instead of smooth interpolation");

    println!("\nExpected with proper interpolation:");
    println!("  Much higher sampling rate (e.g., 1000+ Hz)");
    println!("  Smooth curve from 0->10->0->-10->0");
    println!("  Many more intermediate values between min and max");
}
