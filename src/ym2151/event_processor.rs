//! MIDI event processor for YM2151 conversion
//!
//! This module handles the processing of individual MIDI events
//! and converts them to YM2151 register write events.

use crate::midi::{midi_to_kc_kf, ticks_to_seconds_with_tempo_map, MidiEvent, TempoChange};
use crate::ym2151::{
    apply_tone_to_channel, default_tone_events, load_tone_for_program, ChannelAllocation,
    Ym2151Event,
};
use std::collections::{HashMap, HashSet};

/// Tracks a note-on event for later vibrato processing
#[derive(Debug, Clone)]
pub struct NoteOnInfo {
    pub start_tick: u32,
    pub start_time: f64,
}

/// Captures a full note span on a specific YM2151 channel
#[derive(Debug, Clone)]
pub struct NoteSegment {
    pub ym2151_channel: u8,
    pub note: u8,
    pub start_tick: u32,
    pub end_tick: u32,
    pub start_time: f64,
    pub end_time: f64,
}

/// Context for processing MIDI events
pub struct EventProcessorContext<'a> {
    /// Ticks per beat from MIDI file
    pub ticks_per_beat: u16,
    /// Tempo map for timing conversion
    pub tempo_map: &'a [TempoChange],
    /// Channel allocation mapping
    pub allocation: &'a mut ChannelAllocation,
    /// Active notes (YM2151 channel, MIDI note)
    pub active_notes: &'a mut HashSet<(u8, u8)>,
    /// Current program per YM2151 channel
    pub channel_programs: &'a mut HashMap<u8, u8>,
    /// Active note timings for optional vibrato processing
    pub vibrato_active_notes: Option<&'a mut HashMap<(u8, u8), NoteOnInfo>>,
    /// Completed note spans for optional vibrato processing
    pub vibrato_completed_notes: Option<&'a mut Vec<NoteSegment>>,
}

/// Process a Note On MIDI event
///
/// Converts a MIDI Note On event to YM2151 register write events
/// for KC (Key Code), KF (Key Fraction), and Key ON register.
///
/// # Arguments
/// * `ticks` - MIDI tick time
/// * `channel` - MIDI channel
/// * `note` - MIDI note number
/// * `velocity` - Note velocity (0 means Note Off in some files)
/// * `ctx` - Event processor context
///
/// # Returns
/// Vector of YM2151 register write events
pub fn process_note_on(
    ticks: u32,
    channel: u8,
    note: u8,
    velocity: u8,
    ctx: &mut EventProcessorContext,
) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    // Skip if velocity is 0 (should already be converted to Note Off in parser)
    if velocity == 0 {
        return events;
    }

    // Get allocated YM2151 channel(s) for this MIDI channel
    let Some(ym_channels) = ctx.allocation.midi_to_ym2151.get(&channel) else {
        return events;
    };
    if ym_channels.is_empty() {
        return events;
    }

    // Use round-robin voice allocation for polyphony
    let voice_index = ctx.allocation.current_voice.entry(channel).or_insert(0);
    let ym2151_channel = ym_channels[*voice_index % ym_channels.len()];
    *voice_index = (*voice_index + 1) % ym_channels.len();

    let time_seconds = ticks_to_seconds_with_tempo_map(ticks, ctx.ticks_per_beat, ctx.tempo_map);
    let (kc, kf) = midi_to_kc_kf(note);

    // Set KC (Key Code)
    events.push(Ym2151Event {
        time: time_seconds,
        addr: format!("0x{:02X}", 0x28 + ym2151_channel),
        data: format!("0x{:02X}", kc),
    });

    // Set KF (Key Fraction)
    events.push(Ym2151Event {
        time: time_seconds,
        addr: format!("0x{:02X}", 0x30 + ym2151_channel),
        data: format!("0x{:02X}", kf),
    });

    // Key ON (0x78 = all operators on)
    events.push(Ym2151Event {
        time: time_seconds,
        addr: "0x08".to_string(),
        data: format!("0x{:02X}", 0x78 | ym2151_channel),
    });

    ctx.active_notes.insert((ym2151_channel, note));
    if let Some(active_map) = ctx.vibrato_active_notes.as_deref_mut() {
        active_map.insert(
            (ym2151_channel, note),
            NoteOnInfo {
                start_tick: ticks,
                start_time: time_seconds,
            },
        );
    }

    events
}

/// Process a Note Off MIDI event
///
/// Converts a MIDI Note Off event to a YM2151 Key OFF register write.
///
/// # Arguments
/// * `ticks` - MIDI tick time
/// * `channel` - MIDI channel
/// * `note` - MIDI note number
/// * `ctx` - Event processor context
///
/// # Returns
/// Vector of YM2151 register write events
pub fn process_note_off(
    ticks: u32,
    channel: u8,
    note: u8,
    ctx: &mut EventProcessorContext,
) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    // Get allocated YM2151 channel(s) for this MIDI channel
    let Some(ym_channels) = ctx.allocation.midi_to_ym2151.get(&channel) else {
        return events;
    };

    let time_seconds = ticks_to_seconds_with_tempo_map(ticks, ctx.ticks_per_beat, ctx.tempo_map);

    // Find which YM2151 channel has this note active and turn it off
    for &ym2151_channel in ym_channels {
        if ctx.active_notes.contains(&(ym2151_channel, note)) {
            // Key OFF
            events.push(Ym2151Event {
                time: time_seconds,
                addr: "0x08".to_string(),
                data: format!("0x{:02X}", ym2151_channel),
            });

            ctx.active_notes.remove(&(ym2151_channel, note));
            if let (Some(active_map), Some(completed)) = (
                ctx.vibrato_active_notes.as_deref_mut(),
                ctx.vibrato_completed_notes.as_deref_mut(),
            ) {
                if let Some(note_on) = active_map.remove(&(ym2151_channel, note)) {
                    completed.push(NoteSegment {
                        ym2151_channel,
                        note,
                        start_tick: note_on.start_tick,
                        end_tick: ticks,
                        start_time: note_on.start_time,
                        end_time: time_seconds,
                    });
                }
            }
            break; // Only turn off one voice
        }
    }

    events
}

/// Process a Program Change MIDI event
///
/// Converts a MIDI Program Change event to YM2151 tone register writes.
/// Attempts to load a tone file for the program, falling back to default tone.
///
/// # Arguments
/// * `ticks` - MIDI tick time
/// * `channel` - MIDI channel
/// * `program` - Program/patch number
/// * `ctx` - Event processor context
///
/// # Returns
/// Vector of YM2151 register write events
pub fn process_program_change(
    ticks: u32,
    channel: u8,
    program: u8,
    ctx: &mut EventProcessorContext,
) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    // Get allocated YM2151 channel(s) for this MIDI channel
    let Some(ym_channels) = ctx.allocation.midi_to_ym2151.get(&channel) else {
        return events;
    };

    let time_seconds = ticks_to_seconds_with_tempo_map(ticks, ctx.ticks_per_beat, ctx.tempo_map);

    // Apply program change to all allocated YM2151 channels for this MIDI channel
    for &ym2151_channel in ym_channels {
        // Try to load tone from external file, fallback to default
        let tone_events = match load_tone_for_program(program) {
            Ok(Some(tone)) => {
                // Apply the loaded tone to the channel
                apply_tone_to_channel(&tone, ym2151_channel, time_seconds)
            }
            Ok(None) | Err(_) => {
                // Use default tone if file doesn't exist or can't be loaded
                default_tone_events(ym2151_channel, time_seconds)
            }
        };

        // Add the tone change events
        events.extend(tone_events);

        // Update the channel's current program
        ctx.channel_programs.insert(ym2151_channel, program);
    }

    events
}

/// Process a single MIDI event
///
/// Dispatches to the appropriate handler based on event type.
///
/// # Arguments
/// * `event` - The MIDI event to process
/// * `ctx` - Event processor context
///
/// # Returns
/// Vector of YM2151 register write events
pub fn process_event(event: &MidiEvent, ctx: &mut EventProcessorContext) -> Vec<Ym2151Event> {
    match event {
        // Tempo events are handled via tempo_map, no action needed here
        MidiEvent::Tempo { .. } => Vec::new(),

        MidiEvent::NoteOn {
            ticks,
            channel,
            note,
            velocity,
            ..
        } => process_note_on(*ticks, *channel, *note, *velocity, ctx),

        MidiEvent::NoteOff {
            ticks,
            channel,
            note,
            ..
        } => process_note_off(*ticks, *channel, *note, ctx),

        MidiEvent::ProgramChange {
            ticks,
            channel,
            program,
        } => process_program_change(*ticks, *channel, *program, ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ym2151::allocate_channels;
    use std::collections::HashMap;

    fn create_test_context<'a>(
        ticks_per_beat: u16,
        tempo_map: &'a [TempoChange],
        allocation: &'a mut ChannelAllocation,
        active_notes: &'a mut HashSet<(u8, u8)>,
        channel_programs: &'a mut HashMap<u8, u8>,
    ) -> EventProcessorContext<'a> {
        EventProcessorContext {
            ticks_per_beat,
            tempo_map,
            allocation,
            active_notes,
            channel_programs,
            vibrato_active_notes: None,
            vibrato_completed_notes: None,
        }
    }

    #[test]
    fn test_process_note_on_basic() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_note_on(0, 0, 60, 100, &mut ctx);

        // Should produce 3 events: KC, KF, Key ON
        assert_eq!(events.len(), 3);

        // Verify KC event
        assert!(events[0].addr.starts_with("0x2")); // KC register range
        assert_eq!(events[0].data, "0x2E"); // Middle C

        // Verify KF event
        assert!(events[1].addr.starts_with("0x3")); // KF register range
        assert_eq!(events[1].data, "0x00");

        // Verify Key ON event
        assert_eq!(events[2].addr, "0x08");
        assert!(events[2].data.starts_with("0x7")); // All operators on
    }

    #[test]
    fn test_process_note_on_zero_velocity() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_note_on(0, 0, 60, 0, &mut ctx);

        // Zero velocity should produce no events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_note_on_no_allocation() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        // Empty polyphony - no channels allocated
        let polyphony = HashMap::new();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_note_on(0, 0, 60, 100, &mut ctx);

        // No allocation should produce no events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_note_off_basic() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        // First, send a note on
        {
            let mut ctx = create_test_context(
                480,
                &tempo_map,
                &mut allocation,
                &mut active_notes,
                &mut channel_programs,
            );
            process_note_on(0, 0, 60, 100, &mut ctx);
        }

        // Now send note off
        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_note_off(480, 0, 60, &mut ctx);

        // Should produce 1 Key OFF event
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].addr, "0x08");
        // Data should be the channel number with no operators on
        assert!(events[0].data.starts_with("0x0"));
    }

    #[test]
    fn test_process_note_off_no_active_note() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        // Note off without note on
        let events = process_note_off(480, 0, 60, &mut ctx);

        // Should produce no events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_program_change_basic() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_program_change(0, 0, 42, &mut ctx);

        // Should produce tone events (26 events for default tone)
        assert!(!events.is_empty());

        // Channel program should be updated
        assert_eq!(*ctx.channel_programs.get(&0).unwrap(), 42);
    }

    #[test]
    fn test_process_program_change_no_allocation() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = HashMap::new();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let events = process_program_change(0, 0, 42, &mut ctx);

        // No allocation should produce no events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_event_tempo() {
        let tempo_map = vec![];
        let polyphony = HashMap::new();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let event = MidiEvent::Tempo {
            ticks: 0,
            tempo_bpm: 140.0,
        };

        let events = process_event(&event, &mut ctx);

        // Tempo events should produce no YM2151 events
        assert!(events.is_empty());
    }

    #[test]
    fn test_process_event_note_on() {
        let tempo_map = vec![TempoChange {
            tick: 0,
            tempo_bpm: 120.0,
        }];
        let polyphony = [(0u8, 1usize)].into_iter().collect();
        let mut allocation = allocate_channels(&polyphony);
        let mut active_notes = HashSet::new();
        let mut channel_programs = HashMap::new();

        let mut ctx = create_test_context(
            480,
            &tempo_map,
            &mut allocation,
            &mut active_notes,
            &mut channel_programs,
        );

        let event = MidiEvent::NoteOn {
            ticks: 0,
            channel: 0,
            note: 60,
            velocity: 100,
        };

        let events = process_event(&event, &mut ctx);

        // Should produce 3 events: KC, KF, Key ON
        assert_eq!(events.len(), 3);
    }
}
