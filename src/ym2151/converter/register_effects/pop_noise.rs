use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};

use crate::ym2151::{NoteSegment, Ym2151Event};
use crate::PopNoiseEnvelope;

use super::common::{
    insert_at_tail_of_time, parse_hex_byte, resolve_register_for_channel, TIME_LOOP_EPSILON,
};
use super::state_cache::RegisterStateCache;
use crate::ym2151::converter::event_accumulator::EventAccumulator;

pub(super) const RESTORE_BEFORE_NOTE_EPSILON: f64 = 1e-6;

pub(in crate::ym2151::converter) fn append_pop_noise_envelope_events(
    config: &PopNoiseEnvelope,
    segments: &[NoteSegment],
    cache: &RegisterStateCache,
    events: &mut EventAccumulator,
) {
    if !config.enabled || config.registers.is_empty() || segments.is_empty() {
        return;
    }

    let mut ordered_segments = segments.to_vec();
    ordered_segments.sort_by(|a, b| {
        a.start_time
            .partial_cmp(&b.start_time)
            .unwrap_or(Ordering::Equal)
    });

    let offset = config.offset_seconds.max(0.0);

    // Local map: key = (time_as_u64_bits, sub_index_within_same_time), value = event.
    // The composite key makes ordering intent explicit at insertion time:
    // "insert at tail" means this event comes after all previously inserted events
    // at the same timestamp, with no reliance on Vec push order.
    // `counters` tracks the next sub_index per time bucket for O(1) tail insertion.
    let mut new_events: BTreeMap<(u64, u64), Ym2151Event> = BTreeMap::new();
    let mut counters: HashMap<u64, u64> = HashMap::new();

    for segment in ordered_segments {
        if segment.start_time <= offset || offset <= RESTORE_BEFORE_NOTE_EPSILON {
            continue;
        }
        let apply_time = segment.start_time - offset;
        let restore_time = segment.start_time;

        let channel_key_off_data = format!("0x{:02X}", segment.ym2151_channel);
        let mut any_override = false;
        for reg in &config.registers {
            let Some(base_reg) = parse_hex_byte(&reg.base_register) else {
                continue;
            };
            let Some(override_value) = parse_hex_byte(&reg.value) else {
                continue;
            };
            let resolved_addr = resolve_register_for_channel(base_reg, segment.ym2151_channel);
            let Some(base_value) = cache.latest_value(resolved_addr, restore_time) else {
                continue;
            };
            if base_value == override_value {
                continue;
            }

            let addr_str = format!("0x{:02X}", resolved_addr);
            // Register overrides go first at apply_time (tail insertion, sub_index 0, 1, …)
            insert_at_tail_of_time(
                &mut new_events,
                &mut counters,
                Ym2151Event {
                    time: apply_time,
                    addr: addr_str.clone(),
                    data: format!("0x{:02X}", override_value),
                },
            );
            // Restore to the base value at restore_time (segment.start_time)
            insert_at_tail_of_time(
                &mut new_events,
                &mut counters,
                Ym2151Event {
                    time: restore_time,
                    addr: addr_str,
                    data: format!("0x{:02X}", base_value),
                },
            );
            any_override = true;
        }

        // Move the existing key-off to apply_time, inserted *after* the register overrides.
        // Only back-to-back notes have a key-off exactly at segment.start_time;
        // skip when the channel was already silent before apply_time.
        if any_override {
            if let Some(mut key_off) = events.remove_matching(|event| {
                event.addr == "0x08"
                    && event.data == channel_key_off_data
                    && (event.time - segment.start_time).abs() < TIME_LOOP_EPSILON
            }) {
                key_off.time = apply_time;
                // Tail insertion: key-off sub_index falls after all register overrides at apply_time
                insert_at_tail_of_time(&mut new_events, &mut counters, key_off);
            }
        }
    }

    // Drain local map in (time_bits, sub_index) order into the accumulator.
    // The sub_index values are discarded after this point; their only role was to enforce
    // ordering within the local map. The EventAccumulator inserts each drained event at
    // the tail of its timestamp bucket, preserving the intended ordering
    // (register overrides before key-off) relative to any already-accumulated events.
    for (_, event) in new_events {
        events.push(event);
    }
}
