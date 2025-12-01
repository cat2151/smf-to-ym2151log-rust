//! YM2151 channel allocation
//!
//! Handles allocation of YM2151 channels based on MIDI polyphony requirements.

use crate::midi::{MidiData, MidiEvent};
use std::collections::{HashMap, HashSet};

/// Channel allocation information
#[derive(Debug, Clone)]
pub struct ChannelAllocation {
    /// Maps MIDI channel to list of allocated YM2151 channels
    pub midi_to_ym2151: HashMap<u8, Vec<u8>>,
    /// Tracks which YM2151 channels are currently in use for each MIDI channel
    pub current_voice: HashMap<u8, usize>,
}

/// Analyze polyphony requirements for each MIDI channel
///
/// Measures the maximum number of simultaneous notes per MIDI channel
/// by tracking note on/off events.
///
/// # Arguments
/// * `midi_data` - MIDI data containing events to analyze
///
/// # Returns
/// HashMap mapping MIDI channel number to its maximum polyphony count
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::MidiData;
/// use smf_to_ym2151log::ym2151::analyze_polyphony;
///
/// let midi_data = MidiData {
///     ticks_per_beat: 480,
///     tempo_bpm: 120.0,
///     events: vec![],
/// };
/// let polyphony = analyze_polyphony(&midi_data);
/// ```
pub fn analyze_polyphony(midi_data: &MidiData) -> HashMap<u8, usize> {
    let mut active_notes: HashMap<u8, HashSet<u8>> = HashMap::new();
    let mut max_polyphony: HashMap<u8, usize> = HashMap::new();

    for event in &midi_data.events {
        match event {
            MidiEvent::NoteOn {
                channel,
                note,
                velocity,
                ..
            } => {
                if *velocity > 0 {
                    active_notes.entry(*channel).or_default().insert(*note);
                    let current_poly = active_notes[channel].len();
                    max_polyphony
                        .entry(*channel)
                        .and_modify(|max| *max = (*max).max(current_poly))
                        .or_insert(current_poly);
                }
            }
            MidiEvent::NoteOff { channel, note, .. } => {
                if let Some(notes) = active_notes.get_mut(channel) {
                    notes.remove(note);
                }
            }
            _ => {}
        }
    }

    max_polyphony
}

/// Allocate YM2151 channels based on polyphony requirements with drum channel priority
///
/// 1. First allocates channels based on polyphony requirements
/// 2. Then reorders to prioritize drum channel (MIDI ch 9) to YM2151 ch 0
///
/// # Arguments
/// * `polyphony` - HashMap mapping MIDI channel to its polyphony requirement
///
/// # Returns
/// ChannelAllocation with MIDI to YM2151 channel mappings
///
/// # Example
/// ```
/// use smf_to_ym2151log::ym2151::allocate_channels;
/// use std::collections::HashMap;
///
/// let mut polyphony = HashMap::new();
/// polyphony.insert(0, 2);  // MIDI channel 0 needs 2 voices
/// let allocation = allocate_channels(&polyphony);
/// ```
pub fn allocate_channels(polyphony: &HashMap<u8, usize>) -> ChannelAllocation {
    let mut allocation = HashMap::new();
    let mut next_ym2151_channel = 0u8;

    // Sort MIDI channels by polyphony requirement (descending) for initial allocation
    let mut channels: Vec<(u8, usize)> = polyphony.iter().map(|(k, v)| (*k, *v)).collect();
    channels.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    // Allocate YM2151 channels based on polyphony
    for (midi_ch, poly) in channels {
        let mut ym2151_channels = Vec::new();
        for _ in 0..poly {
            if next_ym2151_channel < 8 {
                ym2151_channels.push(next_ym2151_channel);
                next_ym2151_channel += 1;
            } else {
                // Overflow - reuse last channel
                ym2151_channels.push(7);
                break;
            }
        }
        allocation.insert(midi_ch, ym2151_channels);
    }

    // Apply drum channel priority reordering
    // If MIDI channel 9 (drums) is allocated, ensure it uses YM2151 channel 0
    if let Some(drum_channels) = allocation.get(&9) {
        if !drum_channels.is_empty() && drum_channels[0] != 0 {
            // Need to reorder - swap the channel that has YM2151 ch 0 with drums
            let drum_first = drum_channels[0];

            // Find which MIDI channel has YM2151 ch 0
            for (midi_ch, ym_channels) in allocation.iter_mut() {
                if *midi_ch != 9 {
                    for ym_ch in ym_channels.iter_mut() {
                        if *ym_ch == 0 {
                            *ym_ch = drum_first;
                        }
                    }
                }
            }

            // Update drum channel to use YM2151 ch 0
            if let Some(drum_channels) = allocation.get_mut(&9) {
                drum_channels[0] = 0;
            }
        }
    }

    ChannelAllocation {
        midi_to_ym2151: allocation,
        current_voice: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_polyphony_single_note() {
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

        let polyphony = analyze_polyphony(&midi_data);
        assert_eq!(polyphony.get(&0), Some(&1));
    }

    #[test]
    fn test_analyze_polyphony_chord() {
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
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0,
                    note: 64,
                    velocity: 100,
                },
                MidiEvent::NoteOn {
                    ticks: 0,
                    channel: 0,
                    note: 67,
                    velocity: 100,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 0,
                    note: 60,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 0,
                    note: 64,
                },
                MidiEvent::NoteOff {
                    ticks: 480,
                    channel: 0,
                    note: 67,
                },
            ],
        };

        let polyphony = analyze_polyphony(&midi_data);
        assert_eq!(polyphony.get(&0), Some(&3));
    }

    #[test]
    fn test_allocate_channels_simple() {
        let mut polyphony = HashMap::new();
        polyphony.insert(0, 1);
        polyphony.insert(1, 1);

        let allocation = allocate_channels(&polyphony);

        // Each channel should get one YM2151 channel
        assert_eq!(allocation.midi_to_ym2151.get(&0).unwrap().len(), 1);
        assert_eq!(allocation.midi_to_ym2151.get(&1).unwrap().len(), 1);
    }

    #[test]
    fn test_allocate_channels_with_drums() {
        let mut polyphony = HashMap::new();
        polyphony.insert(0, 1);
        polyphony.insert(9, 1); // Drum channel

        let allocation = allocate_channels(&polyphony);

        // Drum channel (9) should get YM2151 channel 0
        assert_eq!(allocation.midi_to_ym2151.get(&9).unwrap()[0], 0);
    }

    #[test]
    fn test_allocate_channels_polyphonic() {
        let mut polyphony = HashMap::new();
        polyphony.insert(0, 3); // Needs 3 channels

        let allocation = allocate_channels(&polyphony);

        // Should allocate 3 YM2151 channels
        assert_eq!(allocation.midi_to_ym2151.get(&0).unwrap().len(), 3);
    }
}
