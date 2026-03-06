//! Tests for MIDI event processor
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
        attachment_tones: None,
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
