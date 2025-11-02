//! SMF to YM2151 Log Converter
//!
//! Converts Standard MIDI Files to YM2151 register write log in JSON format.
//!
//! Usage:
//!     smf-to-ym2151log-rust <midi_file>

use smf_to_ym2151log::midi::{parse_midi_file, save_midi_events_json};
use smf_to_ym2151log::ym2151::{convert_to_ym2151_log, save_ym2151_log};
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

    // Determine output filenames
    let path = Path::new(midi_filename);
    let base_name = path.file_stem().unwrap_or_default().to_string_lossy();
    let output_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let events_json_path = output_dir.join(format!("{}_events.json", base_name));
    let ym2151_json_path = output_dir.join(format!("{}_ym2151.json", base_name));

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

    // Pass B: Convert to YM2151 log
    println!();
    println!("Pass B: Converting to YM2151 register log...");
    let ym2151_log = match convert_to_ym2151_log(&midi_data) {
        Ok(log) => {
            println!("  ✓ Successfully converted to YM2151 log");
            println!("  - Total YM2151 events: {}", log.event_count);
            log
        }
        Err(e) => {
            eprintln!("Error converting to YM2151 log: {}", e);
            process::exit(1);
        }
    };

    // Save YM2151 log JSON
    println!();
    println!("Saving YM2151 log JSON...");
    if let Err(e) = save_ym2151_log(&ym2151_log, ym2151_json_path.to_str().unwrap()) {
        eprintln!("Error saving YM2151 log: {}", e);
        process::exit(1);
    }
    println!("  ✓ Saved: {}", ym2151_json_path.display());

    println!();
    println!("=== CONVERSION COMPLETE ===");
    println!();
    println!("Summary:");
    println!("  Input file:  {}", midi_filename);
    println!("  Events JSON: {}", events_json_path.display());
    println!("  YM2151 log:  {}", ym2151_json_path.display());
    println!();
    println!("Implementation Status:");
    println!("  Phase 1-3: Foundation & MIDI Parser (COMPLETED)");
    println!("  Phase 4: YM2151 Converter (COMPLETED)");
    println!("  Phase 5: Main Program Integration (COMPLETED)");
    println!();
    println!("Next phase:");
    println!("  Phase 6: Documentation and Polish (TODO)");
}
