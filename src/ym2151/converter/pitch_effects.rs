//! Pitch-related effects
//!
//! Provides delay vibrato and portamento implementations for YM2151 conversion.

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::midi::{midi_note_to_frequency, midi_note_with_offset_to_kc_kf};
use crate::ym2151::{NoteSegment, Ym2151Event};

use super::waveform::triangle_wave;

const DELAY_VIBRATO_DELAY_SECONDS: f64 = 0.2;
const DELAY_VIBRATO_ATTACK_SECONDS: f64 = 0.3;
const DELAY_VIBRATO_DEPTH_CENTS: f64 = 100.0;
const DELAY_VIBRATO_RATE_HZ: f64 = 6.0;
const VIBRATO_RELEASE_TAIL_SECONDS: f64 = 0.5;
const PORTAMENTO_TIME_SECONDS: f64 = 0.1;

pub(super) fn append_delay_vibrato_events(segments: &[NoteSegment], events: &mut Vec<Ym2151Event>) {
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

pub(super) fn append_portamento_events<F>(
    segments: &[NoteSegment],
    should_apply: F,
    events: &mut Vec<Ym2151Event>,
) where
    F: Fn(&NoteSegment, &NoteSegment) -> bool,
{
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

    for list in segments_by_channel.values_mut() {
        list.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(Ordering::Equal)
        });
    }

    for list in segments_by_channel.values() {
        for pair in list.windows(2) {
            let prev = pair[0];
            let next = pair[1];
            if !should_apply(prev, next) {
                continue;
            }
            let stop_time = (next.start_time + PORTAMENTO_TIME_SECONDS).min(next.end_time);
            if stop_time <= next.start_time {
                continue;
            }
            append_portamento_glide(
                prev.note,
                next.note,
                next.ym2151_channel,
                next.start_time,
                stop_time,
                events,
            );
        }
    }
}

fn append_portamento_glide(
    prev_note: u8,
    next_note: u8,
    ym2151_channel: u8,
    start_time: f64,
    stop_time: f64,
    events: &mut Vec<Ym2151Event>,
) {
    if prev_note == next_note {
        return;
    }

    let delta_cents = (next_note as f64 - prev_note as f64) * 100.0;
    let time_step = 1.0 / midi_note_to_frequency(next_note).max(f64::EPSILON);
    let mut time = start_time;
    let mut last_values: Option<(u8, u8)> = None;

    while time <= stop_time + f64::EPSILON {
        let progress = ((time - start_time) / (stop_time - start_time)).clamp(0.0, 1.0);
        let (kc, kf) = midi_note_with_offset_to_kc_kf(prev_note, delta_cents * progress);
        let values = (kc, kf);

        if Some(values) != last_values {
            events.push(Ym2151Event {
                time,
                addr: format!("0x{:02X}", 0x28 + ym2151_channel),
                data: format!("0x{:02X}", kc),
            });
            events.push(Ym2151Event {
                time,
                addr: format!("0x{:02X}", 0x30 + ym2151_channel),
                data: format!("0x{:02X}", kf),
            });
            last_values = Some(values);
        }

        time += time_step;
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
