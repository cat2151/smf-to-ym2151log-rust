//! YM2151 converter (Pass B)
//!
//! Converts MIDI events to YM2151 register write events.

use crate::error::Result;
use crate::midi::{
    midi_note_to_frequency, midi_note_with_offset_to_kc_kf, ticks_to_seconds_with_tempo_map,
    MidiData,
};
use crate::ym2151::{
    allocate_channels, analyze_polyphony, build_tempo_map, initialize_channel_events,
    process_event, EventProcessorContext, NoteSegment, Ym2151Event, Ym2151Log,
};
use crate::ConversionOptions;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

const DELAY_VIBRATO_DELAY_SECONDS: f64 = 0.2;
const DELAY_VIBRATO_ATTACK_SECONDS: f64 = 0.3;
const DELAY_VIBRATO_DEPTH_CENTS: f64 = 100.0;
const DELAY_VIBRATO_RATE_HZ: f64 = 6.0;
const VIBRATO_RELEASE_TAIL_SECONDS: f64 = 0.5;

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
    convert_to_ym2151_log_with_options(midi_data, &ConversionOptions::default())
}

/// Convert MIDI events to YM2151 register write log with conversion options
pub fn convert_to_ym2151_log_with_options(
    midi_data: &MidiData,
    options: &ConversionOptions,
) -> Result<Ym2151Log> {
    let ticks_per_beat = midi_data.ticks_per_beat;

    let mut ym2151_events = Vec::new();

    // Build tempo map from MIDI events
    let tempo_map = build_tempo_map(midi_data);
    let last_tick = midi_data
        .events
        .iter()
        .map(|event| match event {
            crate::midi::MidiEvent::NoteOn { ticks, .. } => *ticks,
            crate::midi::MidiEvent::NoteOff { ticks, .. } => *ticks,
            crate::midi::MidiEvent::Tempo { ticks, .. } => *ticks,
            crate::midi::MidiEvent::ProgramChange { ticks, .. } => *ticks,
        })
        .max()
        .unwrap_or(0);

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

    // Optional vibrato tracking
    let mut vibrato_active_notes = if options.delay_vibrato {
        Some(HashMap::new())
    } else {
        None
    };
    let mut vibrato_segments: Vec<NoteSegment> = Vec::new();

    {
        // Create event processor context
        let mut ctx = EventProcessorContext {
            ticks_per_beat,
            tempo_map: &tempo_map,
            allocation: &mut allocation,
            active_notes: &mut active_notes,
            channel_programs: &mut channel_programs,
            vibrato_active_notes: vibrato_active_notes.as_mut(),
            vibrato_completed_notes: if options.delay_vibrato {
                Some(&mut vibrato_segments)
            } else {
                None
            },
        };

        for event in &midi_data.events {
            let events = process_event(event, &mut ctx);
            ym2151_events.extend(events);
        }
    }

    if options.delay_vibrato {
        if let Some(active_map) = vibrato_active_notes {
            let end_time = ticks_to_seconds_with_tempo_map(last_tick, ticks_per_beat, &tempo_map);
            for ((ym_ch, note), note_on) in active_map.into_iter() {
                vibrato_segments.push(NoteSegment {
                    ym2151_channel: ym_ch,
                    note,
                    start_tick: note_on.start_tick,
                    end_tick: last_tick,
                    start_time: note_on.start_time,
                    end_time,
                });
            }
        }

        append_delay_vibrato_events(&vibrato_segments, &mut ym2151_events);
    }

    ym2151_events.sort_by(|a, b| {
        a.time
            .partial_cmp(&b.time)
            .unwrap_or(Ordering::Equal)
            .then_with(|| a.addr.cmp(&b.addr))
            .then_with(|| a.data.cmp(&b.data))
    });

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

fn append_delay_vibrato_events(segments: &[NoteSegment], events: &mut Vec<Ym2151Event>) {
    if segments.is_empty() {
        return;
    }

    let mut segments_by_channel: HashMap<u8, Vec<&NoteSegment>> = HashMap::new();
    for segment in segments {
        segments_by_channel
            .entry(segment.ym2151_channel)
            .or_default()
            .push(segment);
    }

    for segment_list in segments_by_channel.values_mut() {
        segment_list.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(Ordering::Equal)
        });
    }

    for segment_list in segments_by_channel.values() {
        for (idx, segment) in segment_list.iter().enumerate() {
            let next_start = segment_list.get(idx + 1).map(|s| s.start_time);
            let natural_end = segment.end_time + VIBRATO_RELEASE_TAIL_SECONDS;
            let stop_time = match next_start {
                Some(next) => natural_end.min(next),
                None => natural_end,
            };

            append_vibrato_for_segment(segment, stop_time, events);
        }
    }
}

fn append_vibrato_for_segment(
    segment: &NoteSegment,
    stop_time: f64,
    events: &mut Vec<Ym2151Event>,
) {
    let vibrato_start = segment.start_time + DELAY_VIBRATO_DELAY_SECONDS;
    if stop_time <= vibrato_start {
        return;
    }

    let freq = midi_note_to_frequency(segment.note);
    if freq <= f64::EPSILON {
        return;
    }

    let time_step = 1.0 / freq;
    let mut time = vibrato_start;
    let mut last_values: Option<(u8, u8)> = None;

    while time <= stop_time {
        let elapsed_from_delay = time - vibrato_start;
        let depth_ratio = (elapsed_from_delay / DELAY_VIBRATO_ATTACK_SECONDS).clamp(0.0, 1.0);
        let phase = (elapsed_from_delay * DELAY_VIBRATO_RATE_HZ) % 1.0;
        let waveform = triangle_wave(phase);
        let offset_cents = DELAY_VIBRATO_DEPTH_CENTS * depth_ratio * waveform;
        let (kc, kf) = midi_note_with_offset_to_kc_kf(segment.note, offset_cents);
        let values = (kc, kf);

        if Some(values) != last_values {
            events.push(Ym2151Event {
                time,
                addr: format!("0x{:02X}", 0x28 + segment.ym2151_channel),
                data: format!("0x{:02X}", kc),
            });
            events.push(Ym2151Event {
                time,
                addr: format!("0x{:02X}", 0x30 + segment.ym2151_channel),
                data: format!("0x{:02X}", kf),
            });

            last_values = Some(values);
        }

        time += time_step;
    }
}

fn triangle_wave(phase: f64) -> f64 {
    let wrapped = phase - phase.floor();
    if wrapped < 0.25 {
        wrapped / 0.25
    } else if wrapped < 0.5 {
        1.0 - ((wrapped - 0.25) / 0.25)
    } else if wrapped < 0.75 {
        -((wrapped - 0.5) / 0.25)
    } else {
        -1.0 + ((wrapped - 0.75) / 0.25)
    }
}

#[cfg(test)]
#[path = "converter_tests.rs"]
mod tests;
