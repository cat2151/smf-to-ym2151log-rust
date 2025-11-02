//! SMF to YM2151 Log Converter
//!
//! Converts Standard MIDI Files to YM2151 register write log in JSON format.
//!
//! Usage:
//!     smf-to-ym2151log-rust <midi_file>

use smf_to_ym2151log::midi::{parse_midi_file, save_midi_events_json};
use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: smf-to-ym2151log-rust <midi_file>");
        eprintln!("  <midi_file>: Path to Standard MIDI File");
        process::exit(1);
    }

    let midi_filename = &args[1];

    println!("smf-to-ym2151log-rust");
    println!("Processing: {}", midi_filename);
    println!();

    // Determine output filename (replace extension with _events.json)
    let path = Path::new(midi_filename);
    let base_name = path.file_stem().unwrap_or_default().to_string_lossy();
    let output_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let events_json_path = output_dir.join(format!("{}_events.json", base_name));

    // Pass A: Parse MIDI file
    println!("Pass A: Parsing MIDI file...");
    let midi_data = match parse_midi_file(midi_filename) {
        Ok(data) => {
            println!("  ✓ Successfully parsed MIDI file");
            println!("  - Ticks per beat: {}", data.ticks_per_beat);
            println!("  - Initial tempo: {:.2} BPM", data.tempo_bpm);
            println!("  - Total events: {}", data.events.len());
            data
        }
        Err(e) => {
            eprintln!("Error parsing MIDI file: {}", e);
            process::exit(1);
        }
    };

    // Save intermediate JSON
    println!();
    println!("Saving intermediate events JSON...");
    if let Err(e) = save_midi_events_json(&midi_data, events_json_path.to_str().unwrap()) {
        eprintln!("Error saving events JSON: {}", e);
        process::exit(1);
    }
    println!("  ✓ Saved: {}", events_json_path.display());

    println!();
    println!("Phase 2 (MIDI Parser) COMPLETED");
    println!();
    println!("Next phases:");
    println!("  Phase 3: MIDI to YM2151 Utilities (TODO)");
    println!("  Phase 4: YM2151 Converter Implementation (TODO)");
    println!("  Phase 5: Main Program Integration (TODO)");
    println!("  Phase 6: Documentation and Polish (TODO)");
}
