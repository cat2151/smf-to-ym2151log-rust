//! Pitch-related effects
//!
//! Provides delay vibrato and portamento implementations for YM2151 conversion.

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::midi::{midi_note_to_frequency, midi_note_with_offset_to_kc_kf};
use crate::ym2151::{NoteSegment, Ym2151Event};
use crate::DelayVibratoDefinition;

use super::event_accumulator::EventAccumulator;
use super::waveform::lfo_waveform_value;

const PORTAMENTO_TIME_SECONDS: f64 = 0.1;
const MIN_VIBRATO_SAMPLES_PER_PERIOD: f64 = 16.0;
const MAX_VIBRATO_SAMPLES_PER_SECOND: f64 = 512.0;
const DEPTH_TO_SAMPLES_MULTIPLIER: f64 = 4.0;
const MIN_VIBRATO_RATE_HZ: f64 = 0.01;
const VIBRATO_TIME_LOOP_EPSILON: f64 = 1e-9;

pub(super) fn append_delay_vibrato_events(
    segments: &[NoteSegment],
    config: &DelayVibratoDefinition,
    render_duration_seconds: f64,
    events: &mut EventAccumulator,
) {
    append_delay_vibrato_events_matching(
        segments,
        config,
        render_duration_seconds,
        |_| true,
        events,
    );
}

pub(super) fn append_delay_vibrato_events_for_program(
    segments: &[NoteSegment],
    program: u8,
    config: &DelayVibratoDefinition,
    render_duration_seconds: f64,
    events: &mut EventAccumulator,
) {
    append_delay_vibrato_events_matching(
        segments,
        config,
        render_duration_seconds,
        |segment| segment.program == program,
        events,
    );
}

fn append_delay_vibrato_events_matching<F>(
    segments: &[NoteSegment],
    config: &DelayVibratoDefinition,
    render_duration_seconds: f64,
    should_apply: F,
    events: &mut EventAccumulator,
) where
    F: Fn(&NoteSegment) -> bool,
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

    for segment_list in segments_by_channel.values_mut() {
        segment_list.sort_by(|a, b| {
            a.start_time
                .partial_cmp(&b.start_time)
                .unwrap_or(Ordering::Equal)
        });
    }

    for segment_list in segments_by_channel.values() {
        for (idx, segment) in segment_list.iter().enumerate() {
            if !should_apply(segment) {
                continue;
            }

            let next_start = segment_list.get(idx + 1).map(|s| s.start_time);
            let (stop_time, include_stop_time) = match next_start {
                Some(next) => (next, false),
                None => (render_duration_seconds, false),
            };

            append_vibrato_for_segment(segment, stop_time, include_stop_time, config, events);
        }
    }
}

pub(super) fn append_portamento_events(segments: &[NoteSegment], events: &mut EventAccumulator) {
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
    events: &mut EventAccumulator,
) {
    if prev_note == next_note {
        return;
    }

    let delta_cents = (next_note as f64 - prev_note as f64) * 100.0;
    let time_step = 1.0 / midi_note_to_frequency(next_note).max(f64::EPSILON);
    let mut time = start_time;
    let mut last_values: Option<(u8, u8)> = None;

    while time <= stop_time + VIBRATO_TIME_LOOP_EPSILON {
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

    // Always emit the target pitch at stop_time to ensure the portamento reaches the target note.
    // The loop above may stop just before stop_time when time_step doesn't evenly divide the
    // portamento duration, leaving the pitch slightly short of the target.
    let (kc_end, kf_end) = midi_note_with_offset_to_kc_kf(prev_note, delta_cents);
    if Some((kc_end, kf_end)) != last_values {
        events.push(Ym2151Event {
            time: stop_time,
            addr: format!("0x{:02X}", 0x28 + ym2151_channel),
            data: format!("0x{:02X}", kc_end),
        });
        events.push(Ym2151Event {
            time: stop_time,
            addr: format!("0x{:02X}", 0x30 + ym2151_channel),
            data: format!("0x{:02X}", kf_end),
        });
    }
}

fn append_vibrato_for_segment(
    segment: &NoteSegment,
    stop_time: f64,
    include_stop_time: bool,
    config: &DelayVibratoDefinition,
    events: &mut EventAccumulator,
) {
    if config.rate_hz <= 0.0 || config.depth_cents.abs() < f64::EPSILON {
        return;
    }

    let vibrato_start = segment.start_time + config.delay_seconds;
    if stop_time <= vibrato_start {
        return;
    }

    let time_step = delay_vibrato_time_step(config);
    if !time_step.is_finite() {
        return;
    }

    let mut time = vibrato_start;
    let mut last_values: Option<(u8, u8)> = None;

    while is_vibrato_time_before_stop(time, stop_time, include_stop_time) {
        let elapsed_from_delay = time - vibrato_start;
        let depth_ratio = if config.attack_seconds <= 0.0 {
            1.0
        } else {
            (elapsed_from_delay / config.attack_seconds).clamp(0.0, 1.0)
        };
        let phase = (elapsed_from_delay * config.rate_hz) % 1.0;
        let waveform = lfo_waveform_value(config.waveform, phase);
        let offset_cents = config.depth_cents * depth_ratio * waveform;
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

fn is_vibrato_time_before_stop(time: f64, stop_time: f64, include_stop_time: bool) -> bool {
    if include_stop_time {
        time <= stop_time + VIBRATO_TIME_LOOP_EPSILON
    } else {
        time < stop_time - VIBRATO_TIME_LOOP_EPSILON
    }
}

fn delay_vibrato_time_step(config: &DelayVibratoDefinition) -> f64 {
    let period = 1.0 / config.rate_hz.max(MIN_VIBRATO_RATE_HZ);
    // A triangle vibrato reaches its maximum slope by traversing the full depth
    // over a quarter-period, so 4×depth gives roughly 1 cent of change per sample
    // before the max-samples-per-second cap is applied.
    let samples_per_period = (DEPTH_TO_SAMPLES_MULTIPLIER * config.depth_cents.abs())
        .max(MIN_VIBRATO_SAMPLES_PER_PERIOD)
        .ceil();
    let uncapped_step = period / samples_per_period;
    uncapped_step.max(1.0 / MAX_VIBRATO_SAMPLES_PER_SECOND)
}
