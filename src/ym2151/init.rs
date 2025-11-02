//! YM2151 channel initialization
//!
//! Provides initialization sequences for YM2151 channels.

use crate::ym2151::Ym2151Event;

/// Generate initialization events for a YM2151 channel
///
/// # Arguments
/// * `channel` - Channel number (0-7)
/// * `time` - Sample time for the events
///
/// # Returns
/// Vector of initialization events
///
/// # Example
/// ```
/// use smf_to_ym2151log::ym2151::initialize_channel_events;
/// let events = initialize_channel_events(0, 0);
/// assert!(!events.is_empty());
/// ```
pub fn initialize_channel_events(channel: u8, time: u32) -> Vec<Ym2151Event> {
    let mut events = Vec::new();

    // RL_FB_CONNECT: Stereo output and feedback configuration
    // 0xC7 = Both L/R enabled, feedback level 3, connection algorithm 7
    events.push(Ym2151Event {
        time,
        addr: format!("0x{:02X}", 0x20 + channel),
        data: "0xC7".to_string(),
    });

    // PMS/AMS: Phase and amplitude modulation sensitivity
    // 0x00 = No modulation
    events.push(Ym2151Event {
        time,
        addr: format!("0x{:02X}", 0x38 + channel),
        data: "0x00".to_string(),
    });

    // Configure all 4 operators
    for op in 0..4 {
        let slot = channel + (op * 8);

        // DT1/MUL: Detune and frequency multiplier
        // 0x01 = No detune, 1x frequency multiplier
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0x40 + slot),
            data: "0x01".to_string(),
        });

        // TL: Total Level (volume)
        // Operator 0 (carrier) = max volume (0x00)
        // Other operators (modulators) = silent (0x7F)
        let tl_value = if op == 0 { 0x00 } else { 0x7F };
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0x60 + slot),
            data: format!("0x{:02X}", tl_value),
        });

        // KS/AR: Key Scale and Attack Rate
        // 0x1F = Max attack rate (fast attack)
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0x80 + slot),
            data: "0x1F".to_string(),
        });

        // AMS/D1R: Amplitude modulation sensitivity and first decay rate
        // 0x05 = Moderate decay
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0xA0 + slot),
            data: "0x05".to_string(),
        });

        // DT2/D2R: Second detune and second decay rate
        // 0x05 = Moderate second decay
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0xC0 + slot),
            data: "0x05".to_string(),
        });

        // D1L/RR: First decay level and release rate
        // 0xF7 = Fast release, high sustain level
        events.push(Ym2151Event {
            time,
            addr: format!("0x{:02X}", 0xE0 + slot),
            data: "0xF7".to_string(),
        });
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_channel_events_count() {
        let events = initialize_channel_events(0, 0);
        // Expected: 2 channel registers + 4 operators * 6 registers = 26 events
        assert_eq!(events.len(), 26);
    }

    #[test]
    fn test_initialize_channel_events_time() {
        let time = 12345;
        let events = initialize_channel_events(0, time);
        // All events should have the specified time
        for event in events {
            assert_eq!(event.time, time);
        }
    }

    #[test]
    fn test_initialize_channel_events_different_channels() {
        // Test that different channels get different register addresses
        let events_ch0 = initialize_channel_events(0, 0);
        let events_ch1 = initialize_channel_events(1, 0);

        // First event should be RL_FB_CONNECT register
        assert_eq!(events_ch0[0].addr, "0x20");
        assert_eq!(events_ch1[0].addr, "0x21");
    }

    #[test]
    fn test_initialize_channel_events_operator_tl() {
        let events = initialize_channel_events(0, 0);
        // Find TL (Total Level) register writes (0x60 base)
        // TL addresses for operators 0-3: 0x60, 0x68, 0x70, 0x78
        let tl_addresses = vec!["0x60", "0x68", "0x70", "0x78"];
        let mut tl_events: Vec<&Ym2151Event> = events
            .iter()
            .filter(|e| tl_addresses.contains(&e.addr.as_str()))
            .collect();

        // Sort by address to ensure correct order
        tl_events.sort_by(|a, b| a.addr.cmp(&b.addr));

        assert_eq!(tl_events.len(), 4); // 4 operators

        // First operator (carrier) should have max volume (0x00)
        assert_eq!(tl_events[0].data, "0x00");

        // Other operators (modulators) should be silent (0x7F)
        assert_eq!(tl_events[1].data, "0x7F");
        assert_eq!(tl_events[2].data, "0x7F");
        assert_eq!(tl_events[3].data, "0x7F");
    }
}
