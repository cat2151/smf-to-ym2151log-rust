//! Register-side effects
//!
//! Provides software LFO, pop-noise envelope, and attack continuation fix implementations.

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::ym2151::{NoteSegment, Ym2151Event};
use crate::{AttackContinuationFix, PopNoiseEnvelope, RegisterLfoDefinition};

use super::waveform::lfo_waveform_value;

pub(super) const RESTORE_BEFORE_NOTE_EPSILON: f64 = 1e-6;

pub(super) fn append_register_lfo_events(
    lfo_defs: &[RegisterLfoDefinition],
    segments: &[NoteSegment],
    cache: &RegisterStateCache,
    events: &mut Vec<Ym2151Event>,
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
    events: &mut Vec<Ym2151Event>,
) {
    if def.rate_hz <= 0.0 || def.depth.abs() < f64::EPSILON {
        return;
    }

    let start_time = segment.start_time + def.delay_seconds;
    let stop_time = segment.end_time;
    if stop_time <= start_time {
        return;
    }

    let time_step = (1.0 / def.rate_hz.max(f64::EPSILON)) / 8.0;
    if !time_step.is_finite() || time_step <= 0.0 {
        return;
    }

    let addr_str = format!("0x{:02X}", resolved_addr);
    let mut time = start_time;
    let mut last_value: Option<u8> = None;

    while time <= stop_time + f64::EPSILON {
        let elapsed = time - start_time;
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

pub(super) fn append_pop_noise_envelope_events(
    config: &PopNoiseEnvelope,
    segments: &[NoteSegment],
    cache: &RegisterStateCache,
    events: &mut Vec<Ym2151Event>,
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

    for segment in ordered_segments {
        if segment.start_time <= offset || offset <= RESTORE_BEFORE_NOTE_EPSILON {
            continue;
        }
        let apply_time = segment.start_time - offset;
        let restore_time = (segment.start_time - RESTORE_BEFORE_NOTE_EPSILON).max(0.0);

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
            events.push(Ym2151Event {
                time: apply_time,
                addr: addr_str.clone(),
                data: format!("0x{:02X}", override_value),
            });
            events.push(Ym2151Event {
                time: restore_time,
                addr: addr_str,
                data: format!("0x{:02X}", base_value),
            });
        }
    }
}

pub(super) fn append_attack_continuation_fix_events(
    config: &AttackContinuationFix,
    segments: &[NoteSegment],
    cache: &RegisterStateCache,
    events: &mut Vec<Ym2151Event>,
) {
    if !config.enabled || segments.is_empty() {
        return;
    }

    let override_release = config.release_rate;
    let offset = config.offset_seconds.max(0.0);

    let mut ordered_segments = segments.to_vec();
    ordered_segments.sort_by(|a, b| {
        a.start_time
            .partial_cmp(&b.start_time)
            .unwrap_or(Ordering::Equal)
    });

    for segment in ordered_segments {
        if segment.start_time <= offset || offset <= RESTORE_BEFORE_NOTE_EPSILON {
            continue;
        }
        let pre_time = segment.start_time - offset;
        let restore_time = (segment.start_time - RESTORE_BEFORE_NOTE_EPSILON).max(0.0);

        let mut release_registers = Vec::new();
        for op in 0..4 {
            let base_reg = 0xE0u8 + (op * 8);
            let resolved = resolve_register_for_channel(base_reg, segment.ym2151_channel);
            if let Some(base_value) = cache.latest_value(resolved, pre_time) {
                if base_value != override_release {
                    release_registers.push((resolved, base_value));
                }
            }
        }

        if release_registers.is_empty() {
            continue;
        }

        for (addr, _) in &release_registers {
            events.push(Ym2151Event {
                time: pre_time,
                addr: format!("0x{:02X}", *addr),
                data: format!("0x{:02X}", override_release),
            });
        }

        events.push(Ym2151Event {
            time: pre_time,
            addr: "0x08".to_string(),
            data: format!("0x{:02X}", segment.ym2151_channel),
        });

        for (addr, base_value) in &release_registers {
            events.push(Ym2151Event {
                time: restore_time,
                addr: format!("0x{:02X}", *addr),
                data: format!("0x{:02X}", *base_value),
            });
        }
    }
}

pub(super) struct RegisterStateCache {
    by_addr: HashMap<u8, Vec<(f64, u8)>>,
}

pub(super) fn build_register_state_cache(events: &[Ym2151Event]) -> RegisterStateCache {
    let mut by_addr: HashMap<u8, Vec<(f64, u8)>> = HashMap::new();

    for e in events {
        let Some(addr) = parse_hex_byte(&e.addr) else {
            continue;
        };
        let Some(value) = parse_hex_byte(&e.data) else {
            continue;
        };
        by_addr.entry(addr).or_default().push((e.time, value));
    }

    for values in by_addr.values_mut() {
        values.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    }

    RegisterStateCache { by_addr }
}

impl RegisterStateCache {
    fn latest_value(&self, addr: u8, time: f64) -> Option<u8> {
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

fn resolve_register_for_channel(base_register: u8, channel: u8) -> u8 {
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

fn parse_hex_byte(value: &str) -> Option<u8> {
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
