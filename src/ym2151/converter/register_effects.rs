//! Register-side effects
//!
//! Provides software LFO, pop-noise envelope, and attack continuation fix implementations.

use std::cmp::Ordering;
use std::collections::HashMap;

use crate::ym2151::{NoteSegment, ToneDefinition, Ym2151Event};
use crate::{AttackContinuationFix, PopNoiseEnvelope, ProgramAttachment, RegisterLfoDefinition};

use super::register_fields::{get_register_fields, interpolate_fields, max_steps_for_fields};
use super::waveform::lfo_waveform_value;

pub(super) const RESTORE_BEFORE_NOTE_EPSILON: f64 = 1e-6;
/// Small tolerance for time-loop termination conditions to absorb accumulated f64 rounding errors.
const TIME_LOOP_EPSILON: f64 = 1e-9;

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
            any_override = true;
        }

        // Move the existing key-off at segment.start_time to apply_time so the
        // envelope has time to decay under the overridden (faster-release) registers
        // before the next key-on.  Only back-to-back notes have a key-off exactly at
        // segment.start_time; skip when the channel was already silent before apply_time.
        //
        // We remove the key-off and re-push it to the *end* of the vector (i.e., after
        // the register overrides that were just pushed above).  A stable sort_by(time)
        // then preserves this insertion order within apply_time, so register writes
        // always precede the key-off without needing a custom sort comparator.
        if any_override {
            if let Some(idx) = events.iter().position(|e| {
                e.addr == "0x08"
                    && e.data == channel_key_off_data
                    && (e.time - segment.start_time).abs() < TIME_LOOP_EPSILON
            }) {
                let mut key_off = events.remove(idx);
                key_off.time = apply_time;
                events.push(key_off);
            }
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

/// Append looping linear interpolation events between adjacent program tones.
///
/// For each `ProgramAttachment` with `change_to_next_tone: true`, this generates register
/// write events that continuously morph from the current program's tone to the next
/// program's tone (program_change + 1) over `change_to_next_tone_time` seconds, then
/// back again, repeating for the duration of the song.
pub(super) fn append_change_to_next_tone_events(
    program_attachments: &[ProgramAttachment],
    tones: &HashMap<u8, ToneDefinition>,
    used_channels: &[u8],
    song_end_time: f64,
    events: &mut Vec<Ym2151Event>,
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
        for ev in &tone_to.events {
            let Some(addr) = parse_hex_byte(&ev.addr) else {
                continue;
            };
            let Some(value) = parse_hex_byte(&ev.data) else {
                continue;
            };
            to_values.insert(addr, value);
        }

        // Collect registers that differ between the two tones.
        // Skip note-related registers (KC, KF, key-on) which must not be interpolated.
        let mut register_changes: Vec<(u8, u8, u8)> = Vec::new(); // (base_addr, from, to)
        for ev in &tone_from.events {
            let Some(base_addr) = parse_hex_byte(&ev.addr) else {
                continue;
            };
            if is_note_register(base_addr) {
                continue;
            }
            let Some(value_from) = parse_hex_byte(&ev.data) else {
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

        for &ch in used_channels {
            for &(base_addr, value_from, value_to) in &register_changes {
                let fields = get_register_fields(base_addr);
                let resolved_addr = resolve_register_for_channel(base_addr, ch);
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

/// Returns true if the register address is note-related (KC, KF, or key-on).
///
/// These registers control pitch and key state and must be excluded from tone
/// interpolation so that note playback is not affected by the morphing process.
/// - 0x08: Key Control (key on/off)
/// - 0x28–0x2F: KC (Key Code, one per channel)
/// - 0x30–0x37: KF (Key Fraction, one per channel)
fn is_note_register(addr: u8) -> bool {
    matches!(addr, 0x08 | 0x28..=0x2F | 0x30..=0x37)
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
