import {
	NOTE_CODE_TO_SEMITONE,
	PIXELS_PER_SECOND,
	parseHexByte,
	DEFAULT_CHANNELS,
	KC_REGISTER_BASE,
	KF_REGISTER_BASE,
	TRACK_HEIGHT,
	NOTE_BAR_HEIGHT,
	MIN_NOTE_WIDTH,
} from "./ym2151-utils";

export type NoteOnState = { time: number; kc: number; kf: number };

export type NoteSegment = {
	startTime: number;
	endTime: number;
	kc: number;
	kf: number;
	ch: number;
};

// Minimum segment duration to keep DOM node count manageable.
// Segments shorter than one pixel at the current scale are coalesced into their successor.
const MIN_SEGMENT_SECONDS = MIN_NOTE_WIDTH / PIXELS_PER_SECOND;
const UNCLOSED_NOTE_EXTENSION_S = 0.1;

export function buildNoteSegments(
	events: { time: number; addr: string; data: string }[],
	channelCount: number,
): NoteSegment[] {
	// YM2151 has exactly 8 channels; KC and KF registers are always 0x28-0x2F and 0x30-0x37.
	const ymChannelCount = Math.min(channelCount, DEFAULT_CHANNELS);
	const channelKC: number[] = Array(channelCount).fill(0);
	const channelKF: number[] = Array(channelCount).fill(0);
	const channelNoteOn: Array<NoteOnState | null> =
		Array(channelCount).fill(null);
	const segments: NoteSegment[] = [];

	for (const event of events) {
		const addr = parseHexByte(event.addr);
		const data = parseHexByte(event.data);
		if (addr === null || data === null) continue;

		if (addr >= KC_REGISTER_BASE && addr < KC_REGISTER_BASE + ymChannelCount) {
			const ch = addr - KC_REGISTER_BASE;
			const newKC = data;
			// If KC changes while a note is held, close the current segment and
			// open a new one with the updated pitch (handles portamento/vibrato).
			if (channelNoteOn[ch] && channelKC[ch] !== newKC) {
				const noteOn = channelNoteOn[ch] as NoteOnState;
				const duration = event.time - noteOn.time;
				if (duration >= MIN_SEGMENT_SECONDS) {
					segments.push({
						startTime: noteOn.time,
						endTime: event.time,
						kc: noteOn.kc,
						kf: noteOn.kf,
						ch,
					});
					channelNoteOn[ch] = {
						time: event.time,
						kc: newKC,
						kf: channelKF[ch],
					};
				} else {
					// Too short to be individually visible; update pitch without splitting.
					channelNoteOn[ch] = { ...noteOn, kc: newKC, kf: channelKF[ch] };
				}
			}
			channelKC[ch] = newKC;
		}

		// KF register (0x30-0x37): fine pitch in 1/64 semitone steps.
		// Track changes alongside KC to show continuous vibrato motion.
		if (addr >= KF_REGISTER_BASE && addr < KF_REGISTER_BASE + ymChannelCount) {
			const ch = addr - KF_REGISTER_BASE;
			const newKF = data;
			if (channelNoteOn[ch] && channelKF[ch] !== newKF) {
				const noteOn = channelNoteOn[ch] as NoteOnState;
				// Avoid creating a zero-duration segment when KC and KF updates
				// occur at the same timestamp (the converter emits them as a pair).
				// Also skip splitting when the accumulated segment is too short to render.
				if (
					noteOn.time !== event.time &&
					event.time - noteOn.time >= MIN_SEGMENT_SECONDS
				) {
					segments.push({
						startTime: noteOn.time,
						endTime: event.time,
						kc: noteOn.kc,
						kf: noteOn.kf,
						ch,
					});
					channelNoteOn[ch] = {
						time: event.time,
						kc: channelKC[ch],
						kf: newKF,
					};
				} else {
					// Coalesce into the current segment (same-timestamp or sub-pixel).
					channelNoteOn[ch] = { ...noteOn, kf: newKF };
				}
			}
			channelKF[ch] = newKF;
		}

		if (addr === 0x08) {
			const ch = data & 0x07;
			const operators = (data >> 3) & 0x0f;
			if (ch >= 0 && ch < channelCount) {
				if (operators !== 0) {
					if (!channelNoteOn[ch]) {
						channelNoteOn[ch] = {
							time: event.time,
							kc: channelKC[ch],
							kf: channelKF[ch],
						};
					}
				} else {
					const noteOn = channelNoteOn[ch];
					if (noteOn) {
						segments.push({
							startTime: noteOn.time,
							endTime: event.time,
							kc: noteOn.kc,
							kf: noteOn.kf,
							ch,
						});
						channelNoteOn[ch] = null;
					}
				}
			}
		}
	}

	const lastTime =
		events.length > 0
			? events[events.length - 1].time + UNCLOSED_NOTE_EXTENSION_S
			: 0;
	for (let ch = 0; ch < channelCount; ch++) {
		const noteOn = channelNoteOn[ch];
		if (noteOn) {
			segments.push({
				startTime: noteOn.time,
				endTime: lastTime,
				kc: noteOn.kc,
				kf: noteOn.kf,
				ch,
			});
		}
	}

	return segments;
}

/**
 * Combined pitch value for visualization.
 *
 * YM2151 KC layout:
 *   - high nibble: YM2151 octave (0–7); each octave starts at C#
 *   - low nibble: note code (from NOTE_TABLE), decoded via NOTE_CODE_TO_SEMITONE
 *
 * We decode KC into a linear semitone index and then add KF (fine pitch,
 * 1/64 semitone per step) so that the result is monotonic with actual pitch
 * and suitable for Y-axis placement.
 */
export function notePitch(kc: number, kf: number): number {
	const octave = (kc >> 4) & 0x07;
	const noteCode = kc & 0x0f;
	const semitone = NOTE_CODE_TO_SEMITONE[noteCode] ?? noteCode;
	const linearSemitone = octave * 12 + semitone;
	const fine = kf & 0x3f; // KF is 6-bit, 0-63
	return linearSemitone * 64 + fine;
}

export function computePitchRange(segments: NoteSegment[]): {
	min: number;
	max: number;
} {
	if (segments.length === 0) return { min: 0, max: 0 };
	let min = notePitch(segments[0].kc, segments[0].kf);
	let max = min;
	for (const seg of segments) {
		const pitch = notePitch(seg.kc, seg.kf);
		if (pitch < min) min = pitch;
		if (pitch > max) max = pitch;
	}
	return { min, max };
}

export function noteYPosition(
	kc: number,
	kf: number,
	minPitch: number,
	maxPitch: number,
): number {
	const pitch = notePitch(kc, kf);
	const range = maxPitch - minPitch;
	if (range === 0) return (TRACK_HEIGHT - NOTE_BAR_HEIGHT) / 2;
	return ((maxPitch - pitch) / range) * (TRACK_HEIGHT - NOTE_BAR_HEIGHT);
}
