//! YM2151 register bit-field definitions for per-field interpolation.
//!
//! Many YM2151 registers pack multiple independent parameters into a single byte.
//! For example, register 0x80-0x9F holds both AR (bits 0-4) and KS (bits 6-7).
//! This module defines those sub-fields so that linear interpolation can operate
//! on each parameter independently, rather than blending the raw byte value.

/// A single parameter packed into a YM2151 register byte.
pub(super) struct RegisterFieldDef {
    /// Bitmask of this field's bits in their original register-byte position.
    pub mask: u8,
    /// Bit-position of this field's least-significant bit (right-shift amount).
    pub shift: u8,
}

impl RegisterFieldDef {
    /// Extract this field's value from a register byte.
    pub fn extract(&self, byte: u8) -> u8 {
        (byte & self.mask) >> self.shift
    }

    /// Maximum value this field can hold (all field bits set).
    pub fn max_value(&self) -> u8 {
        self.mask >> self.shift
    }
}

// ── static field tables ──────────────────────────────────────────────────────

static RL_FB_CON_FIELDS: [RegisterFieldDef; 3] = [
    RegisterFieldDef {
        mask: 0x07,
        shift: 0,
    },
    RegisterFieldDef {
        mask: 0x38,
        shift: 3,
    },
    RegisterFieldDef {
        mask: 0xC0,
        shift: 6,
    },
];

static PMS_AMS_FIELDS: [RegisterFieldDef; 2] = [
    RegisterFieldDef {
        mask: 0x03,
        shift: 0,
    },
    RegisterFieldDef {
        mask: 0x70,
        shift: 4,
    },
];

static DT1_MUL_FIELDS: [RegisterFieldDef; 2] = [
    RegisterFieldDef {
        mask: 0x0F,
        shift: 0,
    },
    RegisterFieldDef {
        mask: 0x70,
        shift: 4,
    },
];

static TL_FIELDS: [RegisterFieldDef; 1] = [RegisterFieldDef {
    mask: 0x7F,
    shift: 0,
}];

static KS_AR_FIELDS: [RegisterFieldDef; 2] = [
    RegisterFieldDef {
        mask: 0x1F,
        shift: 0,
    },
    RegisterFieldDef {
        mask: 0xC0,
        shift: 6,
    },
];

static AMSEN_D1R_FIELDS: [RegisterFieldDef; 2] = [
    RegisterFieldDef {
        mask: 0x1F,
        shift: 0,
    },
    RegisterFieldDef {
        mask: 0x80,
        shift: 7,
    },
];

static DT2_D2R_FIELDS: [RegisterFieldDef; 2] = [
    RegisterFieldDef {
        mask: 0x1F,
        shift: 0,
    },
    RegisterFieldDef {
        mask: 0xC0,
        shift: 6,
    },
];

static D1L_RR_FIELDS: [RegisterFieldDef; 2] = [
    RegisterFieldDef {
        mask: 0x0F,
        shift: 0,
    },
    RegisterFieldDef {
        mask: 0xF0,
        shift: 4,
    },
];

static GENERIC_FIELDS: [RegisterFieldDef; 1] = [RegisterFieldDef {
    mask: 0xFF,
    shift: 0,
}];

// ── public helpers ───────────────────────────────────────────────────────────

/// Returns the bit-field definitions for the given YM2151 register address.
pub(super) fn get_register_fields(addr: u8) -> &'static [RegisterFieldDef] {
    match addr {
        0x20..=0x27 => &RL_FB_CON_FIELDS,
        0x38..=0x3F => &PMS_AMS_FIELDS,
        0x40..=0x5F => &DT1_MUL_FIELDS,
        0x60..=0x7F => &TL_FIELDS,
        0x80..=0x9F => &KS_AR_FIELDS,
        0xA0..=0xBF => &AMSEN_D1R_FIELDS,
        0xC0..=0xDF => &DT2_D2R_FIELDS,
        0xE0..=0xFF => &D1L_RR_FIELDS,
        _ => &GENERIC_FIELDS,
    }
}

/// Interpolate a register byte field-by-field at position `t` (0.0 = from, 1.0 = to).
///
/// Each field is independently rounded to the nearest integer, preventing
/// adjacent-bit-field contamination that occurs with raw-byte interpolation.
pub(super) fn interpolate_fields(
    value_from: u8,
    value_to: u8,
    t: f64,
    fields: &[RegisterFieldDef],
) -> u8 {
    let mut result = 0u8;
    for field in fields {
        let from_f = field.extract(value_from);
        let to_f = field.extract(value_to);
        let interp = (from_f as f64 + (to_f as f64 - from_f as f64) * t)
            .round()
            .clamp(0.0, field.max_value() as f64) as u8;
        result |= (interp << field.shift) & field.mask;
    }
    result
}

/// Return the maximum number of integer steps across all fields (used for time-step sizing).
pub(super) fn max_steps_for_fields(
    value_from: u8,
    value_to: u8,
    fields: &[RegisterFieldDef],
) -> usize {
    fields
        .iter()
        .map(|f| {
            let from_f = f.extract(value_from);
            let to_f = f.extract(value_to);
            (from_f as i32 - to_f as i32).unsigned_abs() as usize
        })
        .max()
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_fields_single_field_tl() {
        let fields = get_register_fields(0x60); // TL
                                                // from=0x10 (TL=16), to=0x30 (TL=48), t=0.5 → TL=32
        let result = interpolate_fields(0x10, 0x30, 0.5, fields);
        assert_eq!(result, 0x20, "TL midpoint should be 0x20");
    }

    #[test]
    fn test_interpolate_fields_ks_ar_independent() {
        let fields = get_register_fields(0x80); // KS_AR
                                                // AR: 31→0, KS: 0→1
        let from = 0x1F; // AR=31, KS=0
        let to = 0x40; // AR=0, KS=1

        // At t=0.0 → exact from value
        let v0 = interpolate_fields(from, to, 0.0, fields);
        assert_eq!(v0, from, "t=0 must equal from");

        // At t=1.0 → exact to value
        let v1 = interpolate_fields(from, to, 1.0, fields);
        assert_eq!(v1, to, "t=1 must equal to");

        // At t=0.5: AR interpolates 31→0, midpoint is 15.5 → rounds to 16; KS interpolates
        // 0→1, midpoint is 0.5 → rounds to 1 (Rust's f64::round() uses half-away-from-zero).
        let v_mid = interpolate_fields(from, to, 0.5, fields);
        let ar_mid = v_mid & 0x1F;
        let ks_mid = (v_mid & 0xC0) >> 6;
        assert!(
            ar_mid == 15 || ar_mid == 16,
            "AR at midpoint should be 15 or 16, got {ar_mid}"
        );
        // KS goes from 0→1; at t=0.5 it rounds to 1 (Rust rounds half away from zero, so 0.5→1).
        assert_eq!(
            ks_mid, 1,
            "KS at midpoint should be 1 (0.5 rounds to 1 half-away-from-zero)"
        );

        // Verify raw byte interpolation would give wrong KS:
        // raw = (0x1F + 0x40) / 2 = 47.5 → 48 = 0x30 → KS=(0x30>>6)=0 (wrong, should be 1)
        let raw_mid = (from as f64 + (to as f64 - from as f64) * 0.5)
            .round()
            .clamp(0.0, 255.0) as u8;
        let raw_ks = (raw_mid & 0xC0) >> 6;
        assert_eq!(
            raw_ks, 0,
            "Raw interpolation incorrectly gives KS=0 at midpoint"
        );
        // Field-based gives KS=1, raw gives KS=0 — demonstrating the bug fix
        assert_ne!(
            ks_mid, raw_ks,
            "Field-based and raw must differ to show the fix works"
        );
    }

    #[test]
    fn test_interpolate_fields_d1l_rr_independent() {
        let fields = get_register_fields(0xE0); // D1L_RR
                                                // D1L: 15→0, RR: 0→15
        let from = 0xF0; // D1L=15, RR=0
        let to = 0x0F; // D1L=0, RR=15

        // Raw byte midpoint = (0xF0 + 0x0F) / 2 = 0x7F or 0x80 (wrong mix)
        // Correct per-field midpoint: D1L=7 or 8, RR=7 or 8 → 0x77 or 0x88 etc.
        let v_mid = interpolate_fields(from, to, 0.5, fields);
        let rr_mid = v_mid & 0x0F;
        let d1l_mid = (v_mid & 0xF0) >> 4;

        // Both should be near 7-8, not 15
        assert!(
            rr_mid <= 8 && rr_mid >= 7,
            "RR midpoint should be 7 or 8, got {rr_mid}"
        );
        assert!(
            d1l_mid <= 8 && d1l_mid >= 7,
            "D1L midpoint should be 7 or 8, got {d1l_mid}"
        );

        // Verify raw byte would have given wrong result
        let raw_mid = ((from as f64 + to as f64) / 2.0).round() as u8;
        let raw_rr = raw_mid & 0x0F;
        let _raw_d1l = (raw_mid & 0xF0) >> 4;
        // 0xF0 + 0x0F = 0xFF; 0xFF/2 = 0x7F → RR=15, D1L=7 — D1L looks right but RR=15 is wrong (should be 7-8)
        assert_ne!(
            raw_rr, rr_mid,
            "Raw byte interpolation gives different (wrong) RR"
        );
    }

    #[test]
    fn test_max_steps_for_fields() {
        let fields = get_register_fields(0xE0); // D1L_RR
                                                // D1L: 0→15 (15 steps), RR: 15→0 (15 steps) → max = 15
        let steps = max_steps_for_fields(0x0F, 0xF0, fields);
        assert_eq!(steps, 15);
    }
}
