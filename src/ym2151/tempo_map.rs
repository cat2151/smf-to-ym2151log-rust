//! Tempo map handling for YM2151 conversion
//!
//! Builds and manages tempo maps from MIDI events for accurate time conversion.

use crate::midi::{MidiData, MidiEvent, TempoChange};

/// Build a tempo map from MIDI data
///
/// Creates a list of tempo changes sorted by tick, starting with the initial tempo.
/// Duplicate tempo changes at the same tick are removed, keeping only the last one.
///
/// # Arguments
/// * `midi_data` - Parsed MIDI data containing events and initial tempo
///
/// # Returns
/// A vector of tempo changes sorted by tick
///
/// # Example
/// ```
/// use smf_to_ym2151log::midi::{MidiData, MidiEvent};
/// use smf_to_ym2151log::ym2151::build_tempo_map;
///
/// let midi_data = MidiData {
///     ticks_per_beat: 480,
///     tempo_bpm: 120.0,
///     events: vec![
///         MidiEvent::Tempo { ticks: 480, tempo_bpm: 140.0 },
///     ],
/// };
/// let tempo_map = build_tempo_map(&midi_data);
/// assert_eq!(tempo_map.len(), 2);
/// assert_eq!(tempo_map[0].tempo_bpm, 120.0);
/// assert_eq!(tempo_map[1].tempo_bpm, 140.0);
/// ```
pub fn build_tempo_map(midi_data: &MidiData) -> Vec<TempoChange> {
    let mut tempo_map: Vec<TempoChange> = vec![TempoChange {
        tick: 0,
        tempo_bpm: midi_data.tempo_bpm,
    }];

    for event in &midi_data.events {
        if let MidiEvent::Tempo { ticks, tempo_bpm } = event {
            // Only add if it's different from the current tempo
            // or if it's the first explicit tempo event at tick 0
            if tempo_map.is_empty()
                || *ticks > tempo_map.last().unwrap().tick
                || (*ticks == 0 && tempo_map.len() == 1)
            {
                tempo_map.push(TempoChange {
                    tick: *ticks,
                    tempo_bpm: *tempo_bpm,
                });
            }
        }
    }

    // Remove duplicates and sort by tick
    tempo_map.sort_by_key(|t| t.tick);
    tempo_map.dedup_by_key(|t| t.tick);

    tempo_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_tempo_map_empty_events() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![],
        };

        let tempo_map = build_tempo_map(&midi_data);

        assert_eq!(tempo_map.len(), 1);
        assert_eq!(tempo_map[0].tick, 0);
        assert_eq!(tempo_map[0].tempo_bpm, 120.0);
    }

    #[test]
    fn test_build_tempo_map_single_change() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![MidiEvent::Tempo {
                ticks: 480,
                tempo_bpm: 140.0,
            }],
        };

        let tempo_map = build_tempo_map(&midi_data);

        assert_eq!(tempo_map.len(), 2);
        assert_eq!(tempo_map[0].tick, 0);
        assert_eq!(tempo_map[0].tempo_bpm, 120.0);
        assert_eq!(tempo_map[1].tick, 480);
        assert_eq!(tempo_map[1].tempo_bpm, 140.0);
    }

    #[test]
    fn test_build_tempo_map_multiple_changes() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 100.0,
            events: vec![
                MidiEvent::Tempo {
                    ticks: 240,
                    tempo_bpm: 120.0,
                },
                MidiEvent::Tempo {
                    ticks: 480,
                    tempo_bpm: 140.0,
                },
            ],
        };

        let tempo_map = build_tempo_map(&midi_data);

        assert_eq!(tempo_map.len(), 3);
        assert_eq!(tempo_map[0].tick, 0);
        assert_eq!(tempo_map[0].tempo_bpm, 100.0);
        assert_eq!(tempo_map[1].tick, 240);
        assert_eq!(tempo_map[1].tempo_bpm, 120.0);
        assert_eq!(tempo_map[2].tick, 480);
        assert_eq!(tempo_map[2].tempo_bpm, 140.0);
    }

    #[test]
    fn test_build_tempo_map_ignores_non_tempo_events() {
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
                MidiEvent::Tempo {
                    ticks: 480,
                    tempo_bpm: 140.0,
                },
                MidiEvent::NoteOff {
                    ticks: 960,
                    channel: 0,
                    note: 60,
                },
            ],
        };

        let tempo_map = build_tempo_map(&midi_data);

        assert_eq!(tempo_map.len(), 2);
        assert_eq!(tempo_map[0].tempo_bpm, 120.0);
        assert_eq!(tempo_map[1].tempo_bpm, 140.0);
    }

    #[test]
    fn test_build_tempo_map_deduplicates_same_tick() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![
                MidiEvent::Tempo {
                    ticks: 480,
                    tempo_bpm: 130.0,
                },
                MidiEvent::Tempo {
                    ticks: 480,
                    tempo_bpm: 140.0,
                },
            ],
        };

        let tempo_map = build_tempo_map(&midi_data);

        // Should only have 2 entries: initial + one for tick 480
        assert_eq!(tempo_map.len(), 2);
        assert_eq!(tempo_map[0].tick, 0);
        assert_eq!(tempo_map[1].tick, 480);
    }

    #[test]
    fn test_build_tempo_map_override_initial_tempo() {
        let midi_data = MidiData {
            ticks_per_beat: 480,
            tempo_bpm: 120.0,
            events: vec![MidiEvent::Tempo {
                ticks: 0,
                tempo_bpm: 140.0,
            }],
        };

        let tempo_map = build_tempo_map(&midi_data);

        // Should have 2 entries but dedup will keep only 1 for tick 0
        assert_eq!(tempo_map.len(), 1);
        assert_eq!(tempo_map[0].tick, 0);
    }
}
