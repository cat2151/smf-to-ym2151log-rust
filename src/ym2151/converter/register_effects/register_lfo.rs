use std::cmp::Ordering;

use crate::ym2151::{NoteSegment, Ym2151Event};
use crate::RegisterLfoDefinition;

use super::common::{parse_hex_byte, resolve_register_for_channel, TIME_LOOP_EPSILON};
use super::state_cache::RegisterStateCache;
use crate::ym2151::converter::event_accumulator::EventAccumulator;
use crate::ym2151::converter::waveform::lfo_waveform_value;

pub(in crate::ym2151::converter) fn append_register_lfo_events(
    lfo_defs: &[RegisterLfoDefinition],
    segments: &[NoteSegment],
    cache: &RegisterStateCache,
    events: &mut EventAccumulator,
) {
    if lfo_defs.is_empty() || segments.is_empty() {
        return;
    }

    let mut ordered_segments = segments.to_vec();
    ordered_segments.sort_by(|a, b| {
        a.start_time
            .partial_cmp(&b.start_time)
            .unwrap_or(Ordering::Equal)
    });

    for segment in &ordered_segments {
        for def in lfo_defs {
            let Some(base_reg) = parse_hex_byte(&def.base_register) else {
                continue;
            };
            let resolved_addr = resolve_register_for_channel(base_reg, segment.ym2151_channel);
            let Some(base_value) = cache.latest_value(resolved_addr, segment.start_time) else {
                continue;
            };

            append_register_lfo_for_segment(def, segment, resolved_addr, base_value, events);
        }
    }
}

fn append_register_lfo_for_segment(
    def: &RegisterLfoDefinition,
    segment: &NoteSegment,
    resolved_addr: u8,
    base_value: u8,
    events: &mut EventAccumulator,
) {
    if def.rate_hz <= 0.0 || def.depth.abs() < f64::EPSILON {
        return;
    }

    // Determine the LFO origin: the reference time from which phase and attack are measured.
    // key_on_sync=true (default): LFO restarts on each note-on; origin is note-on + delay.
    // key_on_sync=false: LFO runs continuously from song start; origin is delay from time 0.
    let lfo_origin = if def.key_on_sync {
        segment.start_time + def.delay_seconds
    } else {
        def.delay_seconds
    };

    // Active range: the time window in this segment where LFO events are generated.
    let active_start = if def.key_on_sync {
        lfo_origin
    } else {
        // When the LFO started before the segment, begin at the segment boundary.
        segment.start_time.max(lfo_origin)
    };
    let active_stop = segment.end_time;
    if active_stop <= active_start {
        return;
    }

    // Use enough samples per period so consecutive values differ by at most 1 integer step.
    // A triangle wave with amplitude `depth` has a max slope of 4*depth per period,
    // so we need at least 4*depth samples to avoid stepping by more than 1.
    let samples_per_period = (4.0 * def.depth.abs()).max(8.0).ceil();
    let time_step = (1.0 / def.rate_hz.max(f64::EPSILON)) / samples_per_period;
    if !time_step.is_finite() || time_step <= 0.0 {
        return;
    }

    let addr_str = format!("0x{:02X}", resolved_addr);

    let mut last_value: Option<u8> = None;

    // For key_on_sync=false, when segment.start_time falls between two grid-aligned
    // sample times (e.g., after a rest/gap between note segments), the register would
    // hold a stale value from the end of the previous segment until the next grid point.
    // Emit the correct LFO value at segment.start_time immediately so the register
    // is in phase at note-on.
    if !def.key_on_sync && active_start > lfo_origin {
        let dist_from_grid = (active_start - lfo_origin) % time_step;
        if dist_from_grid > TIME_LOOP_EPSILON && dist_from_grid < time_step - TIME_LOOP_EPSILON {
            let elapsed = active_start - lfo_origin;
            let attack_ratio = if def.attack_seconds <= 0.0 {
                1.0
            } else {
                (elapsed / def.attack_seconds).clamp(0.0, 1.0)
            };
            let phase = (elapsed * def.rate_hz) % 1.0;
            let waveform_val = lfo_waveform_value(def.waveform, phase);
            let offset = def.depth * attack_ratio * waveform_val;
            let value = ((base_value as f64) + offset).round().clamp(0.0, 255.0) as u8;
            events.push(Ym2151Event {
                time: active_start,
                addr: addr_str.clone(),
                data: format!("0x{:02X}", value),
            });
            last_value = Some(value);
        }
    }

    // For key_on_sync=true the first sample is always at lfo_origin (= active_start).
    // For key_on_sync=false we find the first sample boundary >= active_start that is
    // aligned to lfo_origin so the phase is continuous across note segments.
    let mut time = if def.key_on_sync {
        lfo_origin
    } else {
        let n = ((active_start - lfo_origin) / time_step).ceil() as u64;
        lfo_origin + (n as f64) * time_step
    };

    while time <= active_stop + f64::EPSILON {
        let elapsed = (time - lfo_origin).max(0.0);
        let attack_ratio = if def.attack_seconds <= 0.0 {
            1.0
        } else {
            (elapsed / def.attack_seconds).clamp(0.0, 1.0)
        };

        let phase = (elapsed * def.rate_hz) % 1.0;
        let waveform = lfo_waveform_value(def.waveform, phase);
        let offset = def.depth * attack_ratio * waveform;
        let value = ((base_value as f64) + offset).round().clamp(0.0, 255.0) as u8;

        if Some(value) != last_value {
            events.push(Ym2151Event {
                time,
                addr: addr_str.clone(),
                data: format!("0x{:02X}", value),
            });
            last_value = Some(value);
        }

        time += time_step;
    }
}
