//! YM2151 converter (Pass B)
//!
//! Converts MIDI events to YM2151 register write events.

use crate::error::Result;
use crate::midi::MidiData;
use crate::ym2151::{
    allocate_channels, analyze_polyphony, build_tempo_map, initialize_channel_events,
    process_event, EventProcessorContext, Ym2151Event, Ym2151Log,
};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

/// Convert MIDI events to YM2151 register write log
///
/// # Arguments
/// * `midi_data` - Parsed MIDI data from Pass A
///
/// # Returns
/// YM2151 log with register write events
///
/// # Errors
/// Returns an error if conversion fails
///
/// # Example
/// ```no_run
/// use smf_to_ym2151log::midi::MidiData;
/// use smf_to_ym2151log::ym2151::convert_to_ym2151_log;
///
/// let midi_data = MidiData {
///     ticks_per_beat: 480,
///     tempo_bpm: 120.0,
///     events: vec![],
/// };
/// let log = convert_to_ym2151_log(&midi_data).unwrap();
/// ```
pub fn convert_to_ym2151_log(midi_data: &MidiData) -> Result<Ym2151Log> {
    let ticks_per_beat = midi_data.ticks_per_beat;

    let mut ym2151_events = Vec::new();

    // Build tempo map from MIDI events
    let tempo_map = build_tempo_map(midi_data);

    // Initialize all channels at time 0
    // Register 0x08 is the Key ON/OFF register
    // Writing channel number turns off that channel
    for ch in 0..8 {
        ym2151_events.push(Ym2151Event {
            time: 0.0,
            addr: "0x08".to_string(),
            data: format!("0x{:02X}", ch),
        });
    }

    // Analyze polyphony requirements for each MIDI channel
    let polyphony = analyze_polyphony(midi_data);

    // Allocate YM2151 channels based on polyphony with drum channel priority
    let mut allocation = allocate_channels(&polyphony);

    // Collect all allocated YM2151 channels for initialization
    let mut used_ym2151_channels = HashSet::new();
    for ym_channels in allocation.midi_to_ym2151.values() {
        for &ym_ch in ym_channels {
            used_ym2151_channels.insert(ym_ch);
        }
    }

    // Initialize all used YM2151 channels with default parameters
    for &ch in &used_ym2151_channels {
        ym2151_events.extend(initialize_channel_events(ch, 0.0));
    }

    // Track the current program (tone) for each YM2151 channel
    let mut channel_programs: HashMap<u8, u8> = HashMap::new();
    for &ch in &used_ym2151_channels {
        channel_programs.insert(ch, 0);
    }

    // Process MIDI events
    // Track active notes per YM2151 channel: set of (ym2151_channel, note) tuples
    let mut active_notes: HashSet<(u8, u8)> = HashSet::new();

    // Create event processor context
    let mut ctx = EventProcessorContext {
        ticks_per_beat,
        tempo_map: &tempo_map,
        allocation: &mut allocation,
        active_notes: &mut active_notes,
        channel_programs: &mut channel_programs,
    };

    for event in &midi_data.events {
        let events = process_event(event, &mut ctx);
        ym2151_events.extend(events);
    }

    Ok(Ym2151Log {
        event_count: ym2151_events.len(),
        events: ym2151_events,
    })
}

/// Save YM2151 log to JSON file
///
/// # Arguments
/// * `log` - YM2151 log to save
/// * `filename` - Path to output JSON file
///
/// # Returns
/// Ok(()) on success
///
/// # Errors
/// Returns an error if file cannot be created or written
pub fn save_ym2151_log(log: &Ym2151Log, filename: &str) -> Result<()> {
    let json = serde_json::to_string_pretty(log)?;
    let mut file = File::create(filename)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

#[cfg(test)]
#[path = "converter_tests.rs"]
mod tests;
