use std::collections::{BTreeMap, HashMap};

use crate::ym2151::Ym2151Event;

/// Small tolerance for time-loop termination conditions to absorb accumulated f64 rounding errors.
pub(super) const TIME_LOOP_EPSILON: f64 = 1e-9;

fn insert_sub_index(counters: &mut HashMap<u64, u64>, time_bits: u64) -> u64 {
    let sub_index = counters.entry(time_bits).or_insert(0);
    let current = *sub_index;
    *sub_index += 1;
    current
}

/// Inserts an event at the tail of its time bucket in the map.
/// `counters` tracks the next sub_index per time bucket, giving O(1) insertion.
/// Callers choose tail insertion to express "this event comes last within this timestamp".
///
/// `event.time` must be finite and non-negative; IEEE 754 bit representation then sorts
/// identically to the float value, making `u64` a valid BTreeMap key for time ordering.
pub(super) fn insert_at_tail_of_time(
    map: &mut BTreeMap<(u64, u64), Ym2151Event>,
    counters: &mut HashMap<u64, u64>,
    event: Ym2151Event,
) {
    let time_bits = event.time.to_bits();
    let sub_index = insert_sub_index(counters, time_bits);
    map.insert((time_bits, sub_index), event);
}

pub(super) fn resolve_register_for_channel(base_register: u8, channel: u8) -> u8 {
    match base_register {
        0x20..=0x27 => 0x20 + channel,
        0x28..=0x2F => 0x28 + channel,
        0x30..=0x37 => 0x30 + channel,
        0x38..=0x3F => 0x38 + channel,
        0x40..=0xFF => {
            let base = base_register & 0xE0;
            let slot = base_register & 0x1F;
            let operator = slot / 8;
            let new_slot = channel + (operator * 8);
            base + new_slot
        }
        _ => base_register,
    }
}

/// Returns true if the register address is note-related (KC, KF, or key-on).
///
/// These registers control pitch and key state and must be excluded from tone
/// interpolation so that note playback is not affected by the morphing process.
/// - 0x08: Key Control (key on/off)
/// - 0x28–0x2F: KC (Key Code, one per channel)
/// - 0x30–0x37: KF (Key Fraction, one per channel)
pub(super) fn is_note_register(addr: u8) -> bool {
    matches!(addr, 0x08 | 0x28..=0x2F | 0x30..=0x37)
}

pub(super) fn parse_hex_byte(value: &str) -> Option<u8> {
    let trimmed = value.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        u8::from_str_radix(hex, 16).ok()
    } else {
        trimmed.parse::<u8>().ok()
    }
}
