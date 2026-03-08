/**
 * Shared utilities for YM2151 log visualization components.
 * Used by log-visualizer.ts and waveform-viewer.ts.
 */

/** A single YM2151 register-write event from the converter JSON output. */
export type YmLogEvent = {
	time: number;
	addr: string;
	data: string;
};

/** Pixels per second used for time-axis layout across all visualizer components. */
export const PIXELS_PER_SECOND = 180;

/**
 * Maps YM2151 note code (low nibble of KC byte) to a linear semitone index (0–11).
 * YM2151 note codes start at C# (code 0) and end at C (code 14).
 * Codes 3, 7, 11, and 15 are unused by the note table and are mapped to the
 * nearest lower valid semitone so that the resulting pitch is still monotonic.
 */
export const NOTE_CODE_TO_SEMITONE: readonly number[] = [
	0, // 0  = C#
	1, // 1  = D
	2, // 2  = D#
	2, // 3  = (unused, treated as D#)
	3, // 4  = E
	4, // 5  = F
	5, // 6  = F#
	5, // 7  = (unused, treated as F#)
	6, // 8  = G
	7, // 9  = G#
	8, // 10 = A
	8, // 11 = (unused, treated as A)
	9, // 12 = A#
	10, // 13 = B
	11, // 14 = C
	11, // 15 = (unused, treated as C)
];

/** Parse a hex string of the form `"0xNN"` into a number, or return `null` on failure. */
export function parseHexByte(value: string): number | null {
	const match = /^0x([0-9a-fA-F]{1,2})$/.exec(value.trim());
	if (!match) return null;
	const parsed = Number.parseInt(match[1], 16);
	return Number.isNaN(parsed) ? null : parsed;
}
