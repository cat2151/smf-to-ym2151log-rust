//! YM2151 converter (Pass B)
//!
//! Converts MIDI events to YM2151 register write events.

mod pitch_effects;
mod register_effects;
mod register_fields;
mod waveform;

use crate::error::Result;
use crate::midi::{ticks_to_seconds_with_tempo_map, MidiData};
use crate::ym2151::{
    allocate_channels, analyze_polyphony, apply_tone_to_channel, build_tempo_map,
    initialize_channel_events, process_event, EventProcessorContext, NoteSegment, Ym2151Event,
    Ym2151Log,
};
use crate::ConversionOptions;
use pitch_effects::{append_delay_vibrato_events, append_portamento_events};
use register_effects::{
    append_attack_continuation_fix_events, append_change_to_next_tone_events,
    append_pop_noise_envelope_events, append_register_lfo_events, build_register_state_cache,
};
use std::cmp::Ordering;
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

    // Collect all allocated YM2151 channels for initialization, sorted for deterministic output
    let used_ym2151_channels: Vec<u8> = {
        let mut set = HashSet::new();
        for ym_channels in allocation.midi_to_ym2151.values() {
            for &ym_ch in ym_channels {
                set.insert(ym_ch);
            }
        }
        let mut v: Vec<u8> = set.into_iter().collect();
        v.sort_unstable();
        v
    };

    // Initialize all used YM2151 channels with default parameters
    for &ch in &used_ym2151_channels {
        ym2151_events.extend(initialize_channel_events(ch, 0.0));
    }

    // Apply initial tone (program 0) from attachment if available.
    // This ensures the attachment tone takes effect even when the MIDI file
    // does not contain an explicit Program Change event.
    if let Some(initial_tone) = options.tones.get(&0) {
        for &ch in &used_ym2151_channels {
            ym2151_events.extend(apply_tone_to_channel(initial_tone, ch, 0.0));
        }
    }

    // Track the current program (tone) for each YM2151 channel
    let mut channel_programs: HashMap<u8, u8> = HashMap::new();
    for &ch in &used_ym2151_channels {
        channel_programs.insert(ch, 0);
    }

    // Process MIDI events
    // Track active notes per YM2151 channel: set of (ym2151_channel, note) tuples
    let mut active_notes: HashSet<(u8, u8)> = HashSet::new();

    // Optional note tracking for vibrato/portamento
    let need_note_segments = options.delay_vibrato
        || options.portamento
        || !options.software_lfo.is_empty()
        || options.pop_noise_envelope.is_some()
        || options.attack_continuation_fix.is_some()
        || options.program_attachments.iter().any(|pa| {
            pa.delay_vibrato
                || pa.portamento
                || !pa.software_lfo.is_empty()
                || pa.pop_noise_envelope.is_some()
                || pa.attack_continuation_fix.is_some()
        });
    let mut vibrato_active_notes = if need_note_segments {
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
            vibrato_completed_notes: if need_note_segments {
                Some(&mut vibrato_segments)
            } else {
                None
            },
            attachment_tones: if options.tones.is_empty() {
                None
            } else {
                Some(&options.tones)
            },
        };

        for event in &midi_data.events {
            let events = process_event(event, &mut ctx);
            ym2151_events.extend(events);
        }
    }

    if need_note_segments {
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
                    program: note_on.program,
                });
            }
        }
    }

    if options.delay_vibrato {
        append_delay_vibrato_events(&vibrato_segments, &mut ym2151_events);
    }

    if options.portamento {
        append_portamento_events(&vibrato_segments, &mut ym2151_events);
    }

    let need_pre_note_events =
        options.pop_noise_envelope.is_some() || options.attack_continuation_fix.is_some();
    let need_register_cache = !options.software_lfo.is_empty() || need_pre_note_events;
    let register_cache = if need_register_cache {
        Some(build_register_state_cache(&ym2151_events))
    } else {
        None
    };

    if !options.software_lfo.is_empty() {
        if let Some(cache) = register_cache.as_ref() {
            append_register_lfo_events(
                &options.software_lfo,
                &vibrato_segments,
                cache,
                &mut ym2151_events,
            );
        }
    }

    if let (Some(config), Some(cache)) = (&options.pop_noise_envelope, register_cache.as_ref()) {
        append_pop_noise_envelope_events(config, &vibrato_segments, cache, &mut ym2151_events);
    }

    if let (Some(config), Some(cache)) = (&options.attack_continuation_fix, register_cache.as_ref())
    {
        append_attack_continuation_fix_events(config, &vibrato_segments, cache, &mut ym2151_events);
    }

    // Apply per-program effects from new array format.
    // Pre-group note segments by program once to avoid O(attachments × segments) scanning.
    let needs_per_program_effects = options.program_attachments.iter().any(|pa| {
        pa.delay_vibrato
            || pa.portamento
            || !pa.software_lfo.is_empty()
            || pa.pop_noise_envelope.is_some()
            || pa.attack_continuation_fix.is_some()
    });
    let segments_by_program: HashMap<u8, Vec<NoteSegment>> = if needs_per_program_effects {
        let mut map: HashMap<u8, Vec<NoteSegment>> = HashMap::new();
        for seg in &vibrato_segments {
            map.entry(seg.program).or_default().push(seg.clone());
        }
        map
    } else {
        HashMap::new()
    };

    // Build the register state cache once (from events before per-program effects)
    // so that all program attachments share the same baseline register state.
    let need_per_program_cache = options.program_attachments.iter().any(|pa| {
        !pa.software_lfo.is_empty()
            || pa.pop_noise_envelope.is_some()
            || pa.attack_continuation_fix.is_some()
    });
    let per_program_cache = if need_per_program_cache {
        Some(build_register_state_cache(&ym2151_events))
    } else {
        None
    };

    for pa in &options.program_attachments {
        // Skip attachments that have no effects enabled (e.g., tone-only entries)
        let has_effects = pa.delay_vibrato
            || pa.portamento
            || !pa.software_lfo.is_empty()
            || pa.pop_noise_envelope.is_some()
            || pa.attack_continuation_fix.is_some();
        if !has_effects {
            continue;
        }

        let program_segments = match segments_by_program.get(&pa.program_change) {
            Some(segs) if !segs.is_empty() => segs,
            _ => continue,
        };

        if pa.delay_vibrato {
            append_delay_vibrato_events(program_segments, &mut ym2151_events);
        }

        if pa.portamento {
            append_portamento_events(program_segments, &mut ym2151_events);
        }

        if !pa.software_lfo.is_empty() {
            if let Some(cache) = per_program_cache.as_ref() {
                append_register_lfo_events(
                    &pa.software_lfo,
                    program_segments,
                    cache,
                    &mut ym2151_events,
                );
            }
        }

        if let (Some(config), Some(cache)) = (&pa.pop_noise_envelope, per_program_cache.as_ref()) {
            append_pop_noise_envelope_events(config, program_segments, cache, &mut ym2151_events);
        }

        if let (Some(config), Some(cache)) =
            (&pa.attack_continuation_fix, per_program_cache.as_ref())
        {
            append_attack_continuation_fix_events(
                config,
                program_segments,
                cache,
                &mut ym2151_events,
            );
        }
    }

    ym2151_events.sort_by(sort_events);

    // Apply looping linear tone interpolation toward the adjacent program tone.
    // This is independent of note segments and runs for the full song duration.
    let has_change_to_next_tone = options
        .program_attachments
        .iter()
        .any(|pa| pa.change_to_next_tone);
    if has_change_to_next_tone && !options.tones.is_empty() {
        let song_end_time = ticks_to_seconds_with_tempo_map(last_tick, ticks_per_beat, &tempo_map);
        append_change_to_next_tone_events(
            &options.program_attachments,
            &options.tones,
            &used_ym2151_channels,
            song_end_time,
            &mut ym2151_events,
        );
        // Re-sort to interleave newly generated events
        ym2151_events.sort_by(sort_events);
    }

    Ok(Ym2151Log {
        event_count: ym2151_events.len(),
        events: ym2151_events,
    })
}

/// Sort comparator for YM2151 events.
///
/// Primary key: time (ascending).
/// Secondary key: at equal time > 0.0, key-on/off writes (addr 0x08) come after other register
/// writes.  This ensures register state (e.g., envelope parameters) is set before key events are
/// processed at runtime (e.g., PopNoiseEnvelope apply_time).
///
/// Exception: at time == 0.0 (initialization), the comparator returns `Equal` so the stable
/// sort preserves insertion order.  The initialization code intentionally writes the "turn off
/// all channels" key-offs first (before channel register init), so they must not be reordered.
fn sort_events(a: &Ym2151Event, b: &Ym2151Event) -> Ordering {
    match a.time.partial_cmp(&b.time).unwrap_or(Ordering::Equal) {
        Ordering::Equal => {
            // At initialization time (t=0.0) preserve insertion order; key-offs were pushed
            // first on purpose to silence all channels before register init writes arrive.
            if a.time == 0.0 {
                return Ordering::Equal;
            }
            let a_is_key = a.addr == "0x08";
            let b_is_key = b.addr == "0x08";
            match (a_is_key, b_is_key) {
                (false, true) => Ordering::Less,
                (true, false) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        }
        other => other,
    }
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
