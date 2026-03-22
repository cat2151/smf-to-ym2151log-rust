use std::collections::HashMap;

use crate::ym2151::{ToneDefinition, Ym2151Event};
use crate::{ym2151::converter::event_accumulator::EventAccumulator, ProgramAttachment};

use super::common::{
    is_note_register, parse_hex_byte, resolve_register_for_channel, TIME_LOOP_EPSILON,
};
use crate::ym2151::converter::register_fields::{
    get_register_fields, interpolate_fields, max_steps_for_fields,
};

/// Append looping linear interpolation events between adjacent program tones.
///
/// For each `ProgramAttachment` with `change_to_next_tone: true`, this generates register
/// write events that continuously morph from the current program's tone to the next
/// program's tone (program_change + 1) over `change_to_next_tone_time` seconds, then
/// back again, repeating for the duration of the song.
pub(in crate::ym2151::converter) fn append_change_to_next_tone_events(
    program_attachments: &[ProgramAttachment],
    tones: &HashMap<u8, ToneDefinition>,
    used_channels: &[u8],
    song_end_time: f64,
    events: &mut EventAccumulator,
) {
    if song_end_time <= 0.0 || used_channels.is_empty() {
        return;
    }

    for attachment in program_attachments {
        if !attachment.change_to_next_tone {
            continue;
        }
        let period = attachment.change_to_next_tone_time;
        if period <= 0.0 {
            continue;
        }

        let Some(next_program) = attachment.program_change.checked_add(1) else {
            continue;
        };
        let Some(tone_from) = tones.get(&attachment.program_change) else {
            continue;
        };
        let Some(tone_to) = tones.get(&next_program) else {
            continue;
        };

        // Build a lookup from tone_to: base_addr -> value
        let mut to_values: HashMap<u8, u8> = HashMap::new();
        for event in &tone_to.events {
            let Some(addr) = parse_hex_byte(&event.addr) else {
                continue;
            };
            let Some(value) = parse_hex_byte(&event.data) else {
                continue;
            };
            to_values.insert(addr, value);
        }

        // Collect registers that differ between the two tones.
        // Skip note-related registers (KC, KF, key-on) which must not be interpolated.
        let mut register_changes: Vec<(u8, u8, u8)> = Vec::new(); // (base_addr, from, to)
        for event in &tone_from.events {
            let Some(base_addr) = parse_hex_byte(&event.addr) else {
                continue;
            };
            if is_note_register(base_addr) {
                continue;
            }
            let Some(value_from) = parse_hex_byte(&event.data) else {
                continue;
            };
            let Some(&value_to) = to_values.get(&base_addr) else {
                continue;
            };
            if value_from != value_to {
                register_changes.push((base_addr, value_from, value_to));
            }
        }

        if register_changes.is_empty() {
            continue;
        }

        let cycle = 2.0 * period;

        for &channel in used_channels {
            for &(base_addr, value_from, value_to) in &register_changes {
                let fields = get_register_fields(base_addr);
                let resolved_addr = resolve_register_for_channel(base_addr, channel);
                let addr_str = format!("0x{:02X}", resolved_addr);

                // Step count is the maximum field-delta so every integer step is covered.
                let steps = max_steps_for_fields(value_from, value_to, fields);

                // One event per integer step; keep time_step fine enough for smooth changes
                let time_step = period / steps.max(1) as f64;

                let mut last_value: Option<u8> = None;
                let mut time = 0.0_f64;

                while time <= song_end_time + TIME_LOOP_EPSILON {
                    let cycle_pos = time % cycle;
                    // Forward ramp [0, period]: t goes 0→1; backward [period, 2*period]: t goes 1→0
                    let t = if cycle_pos <= period {
                        cycle_pos / period
                    } else {
                        2.0 - cycle_pos / period
                    };

                    // Interpolate each packed field independently to avoid cross-field contamination.
                    let value = interpolate_fields(value_from, value_to, t, fields);

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
        }
    }
}
