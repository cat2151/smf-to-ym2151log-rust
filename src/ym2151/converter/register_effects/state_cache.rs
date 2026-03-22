use std::collections::HashMap;

use crate::ym2151::Ym2151Event;

use super::common::parse_hex_byte;

pub(in crate::ym2151::converter) struct RegisterStateCache {
    by_addr: HashMap<u8, Vec<(f64, u8)>>,
}

/// Build a register state cache from an iterator of events in time order.
///
/// Callers should pass `accumulator.iter()` so events are visited in the same
/// deterministic `(time, sub_index)` order that the accumulator provides.
/// Because the iterator is already time-ordered, no internal sort is required.
pub(in crate::ym2151::converter) fn build_register_state_cache<'a>(
    events: impl Iterator<Item = &'a Ym2151Event>,
) -> RegisterStateCache {
    let mut by_addr: HashMap<u8, Vec<(f64, u8)>> = HashMap::new();

    for event in events {
        let Some(addr) = parse_hex_byte(&event.addr) else {
            continue;
        };
        let Some(value) = parse_hex_byte(&event.data) else {
            continue;
        };
        by_addr.entry(addr).or_default().push((event.time, value));
    }

    // Events from the accumulator iterator are already in time order,
    // so no post-collection sort is needed.
    RegisterStateCache { by_addr }
}

impl RegisterStateCache {
    pub(super) fn latest_value(&self, addr: u8, time: f64) -> Option<u8> {
        let entries = self.by_addr.get(&addr)?;
        let mut lo = 0;
        let mut hi = entries.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if entries[mid].0 <= time + f64::EPSILON {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo == 0 {
            None
        } else {
            Some(entries[lo - 1].1)
        }
    }
}
