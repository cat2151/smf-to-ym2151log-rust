//! Event accumulator with BTreeMap-based ordering.
//!
//! Replaces the previous `Vec<Ym2151Event>` + `sort_by(time)` pattern.
//! Events are stored in a `BTreeMap` keyed by `(time_bits, sub_index)`, giving
//! deterministic time ordering without a post-insertion sort.
//!
//! `time_bits` is the IEEE 754 bit representation of `event.time` (finite,
//! non-negative), which sorts identically to the float value and is therefore a
//! valid `u64` BTreeMap key.  `sub_index` encodes insertion order within a
//! timestamp, so callers express intra-timestamp sequencing by controlling the
//! order in which they call [`EventAccumulator::push`].

use crate::ym2151::Ym2151Event;
use std::collections::{BTreeMap, HashMap};

/// Accumulates `Ym2151Event`s in deterministic `(time, insertion-order)` sequence.
///
/// Unlike a plain `Vec`, no post-insertion sort is required: the BTreeMap key
/// `(time_bits, sub_index)` guarantees that iterating the accumulator always
/// yields events ordered first by timestamp, then by the order they were
/// inserted.
pub(in crate::ym2151::converter) struct EventAccumulator {
    map: BTreeMap<(u64, u64), Ym2151Event>,
    /// Next sub_index for each timestamp bucket (keyed by `time.to_bits()`).
    counters: HashMap<u64, u64>,
}

impl EventAccumulator {
    pub(in crate::ym2151::converter) fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            counters: HashMap::new(),
        }
    }

    /// Insert `event` after all previously inserted events at the same timestamp.
    ///
    /// Callers express "this event must come after X" simply by calling `push`
    /// after X has been pushed, rather than relying on an external sort step.
    pub(in crate::ym2151::converter) fn push(&mut self, event: Ym2151Event) {
        let time_bits = event.time.to_bits();
        let sub_index = self.counters.entry(time_bits).or_insert(0);
        self.map.insert((time_bits, *sub_index), event);
        *sub_index += 1;
    }

    /// Push each event in `iter` in iteration order, preserving relative ordering.
    pub(in crate::ym2151::converter) fn extend(
        &mut self,
        iter: impl IntoIterator<Item = Ym2151Event>,
    ) {
        for event in iter {
            self.push(event);
        }
    }

    /// Find and remove the first event for which `predicate` returns `true`.
    ///
    /// Used by pop-noise processing to relocate an existing key-off event to an
    /// earlier timestamp without an external sort step.  The removed entry's
    /// sub_index slot is simply left vacant; the BTreeMap skips it during
    /// iteration.
    pub(in crate::ym2151::converter) fn remove_matching<F>(
        &mut self,
        predicate: F,
    ) -> Option<Ym2151Event>
    where
        F: Fn(&Ym2151Event) -> bool,
    {
        let key = self.map.iter().find(|(_, v)| predicate(v)).map(|(k, _)| *k);
        key.and_then(|k| self.map.remove(&k))
    }

    /// Iterate over all events in `(time, sub_index)` order.
    pub(in crate::ym2151::converter) fn iter(&self) -> impl Iterator<Item = &Ym2151Event> {
        self.map.values()
    }

    /// Consume the accumulator, returning events in `(time, sub_index)` order.
    pub(in crate::ym2151::converter) fn into_vec(self) -> Vec<Ym2151Event> {
        self.map.into_values().collect()
    }
}
