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

export type NoteOnState = {
	time: number;
	kc: number;
	kf: number;
	releasedAt: number | null;
	hasReleasePitchEvents: boolean;
};

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
const TIME_EPSILON = 1e-9;

function keyOnTimeKey(ch: number, time: number): string {
	return `${ch}:${time}`;
}

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
	const keyOnTimes = new Set<string>();
	const pushSegment = (
		ch: number,
		noteOn: NoteOnState,
		endTime: number,
	): void => {
		if (endTime - noteOn.time >= MIN_SEGMENT_SECONDS) {
			segments.push({
				startTime: noteOn.time,
				endTime,
				kc: noteOn.kc,
				kf: noteOn.kf,
				ch,
			});
		}
	};
	const closeForNextKeyOn = (
		ch: number,
		noteOn: NoteOnState,
		nextKeyOnTime: number,
	): void => {
		const endTime =
			noteOn.releasedAt !== null && !noteOn.hasReleasePitchEvents
				? noteOn.releasedAt
				: nextKeyOnTime;
		pushSegment(ch, noteOn, endTime);
	};

	for (const event of events) {
		const addr = parseHexByte(event.addr);
		const data = parseHexByte(event.data);
		if (addr !== 0x08 || data === null) continue;
		const ch = data & 0x07;
		const operators = (data >> 3) & 0x0f;
		if (operators !== 0 && ch >= 0 && ch < channelCount) {
			keyOnTimes.add(keyOnTimeKey(ch, event.time));
		}
	}

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
				if (
					noteOn.releasedAt !== null &&
					keyOnTimes.has(keyOnTimeKey(ch, event.time))
				) {
					closeForNextKeyOn(ch, noteOn, event.time);
					channelNoteOn[ch] = null;
					channelKC[ch] = newKC;
					continue;
				}
				if (
					noteOn.releasedAt !== null &&
					event.time <= noteOn.releasedAt + TIME_EPSILON
				) {
					pushSegment(ch, noteOn, noteOn.releasedAt);
					channelNoteOn[ch] = null;
					channelKC[ch] = newKC;
					continue;
				}
				const duration = event.time - noteOn.time;
				const isReleasePitchEvent =
					noteOn.releasedAt !== null &&
					event.time > noteOn.releasedAt + TIME_EPSILON;
				if (duration >= MIN_SEGMENT_SECONDS) {
					pushSegment(ch, noteOn, event.time);
					channelNoteOn[ch] = {
						time: event.time,
						kc: newKC,
						kf: channelKF[ch],
						releasedAt: noteOn.releasedAt,
						hasReleasePitchEvents:
							noteOn.hasReleasePitchEvents || isReleasePitchEvent,
					};
				} else {
					// Too short to be individually visible; update pitch without splitting.
					channelNoteOn[ch] = {
						...noteOn,
						kc: newKC,
						kf: channelKF[ch],
						hasReleasePitchEvents:
							noteOn.hasReleasePitchEvents || isReleasePitchEvent,
					};
				}
			}
			channelKC[ch] = newKC;
		}

		// KF register (0x30-0x37): fine pitch is stored in bits 7..2
		// as 1/64 semitone steps.
		// Track changes alongside KC to show continuous vibrato motion.
		if (addr >= KF_REGISTER_BASE && addr < KF_REGISTER_BASE + ymChannelCount) {
			const ch = addr - KF_REGISTER_BASE;
			const newKF = data;
			if (channelNoteOn[ch] && channelKF[ch] !== newKF) {
				const noteOn = channelNoteOn[ch] as NoteOnState;
				if (
					noteOn.releasedAt !== null &&
					keyOnTimes.has(keyOnTimeKey(ch, event.time))
				) {
					closeForNextKeyOn(ch, noteOn, event.time);
					channelNoteOn[ch] = null;
					channelKF[ch] = newKF;
					continue;
				}
				if (
					noteOn.releasedAt !== null &&
					event.time <= noteOn.releasedAt + TIME_EPSILON
				) {
					pushSegment(ch, noteOn, noteOn.releasedAt);
					channelNoteOn[ch] = null;
					channelKF[ch] = newKF;
					continue;
				}
				// Avoid creating a zero-duration segment when KC and KF updates
				// occur at the same timestamp (the converter emits them as a pair).
				// Also skip splitting when the accumulated segment is too short to render.
				const isReleasePitchEvent =
					noteOn.releasedAt !== null &&
					event.time > noteOn.releasedAt + TIME_EPSILON;
				if (
					noteOn.time !== event.time &&
					event.time - noteOn.time >= MIN_SEGMENT_SECONDS
				) {
					pushSegment(ch, noteOn, event.time);
					channelNoteOn[ch] = {
						time: event.time,
						kc: channelKC[ch],
						kf: newKF,
						releasedAt: noteOn.releasedAt,
						hasReleasePitchEvents:
							noteOn.hasReleasePitchEvents || isReleasePitchEvent,
					};
				} else {
					// Coalesce into the current segment (same-timestamp or sub-pixel).
					channelNoteOn[ch] = {
						...noteOn,
						kf: newKF,
						hasReleasePitchEvents:
							noteOn.hasReleasePitchEvents || isReleasePitchEvent,
					};
				}
			}
			channelKF[ch] = newKF;
		}

		if (addr === 0x08) {
			const ch = data & 0x07;
			const operators = (data >> 3) & 0x0f;
			if (ch >= 0 && ch < channelCount) {
				if (operators !== 0) {
					const previous = channelNoteOn[ch];
					if (previous) {
						closeForNextKeyOn(ch, previous, event.time);
					}
					channelNoteOn[ch] = {
						time: event.time,
						kc: channelKC[ch],
						kf: channelKF[ch],
						releasedAt: null,
						hasReleasePitchEvents: false,
					};
				} else {
					const noteOn = channelNoteOn[ch];
					if (noteOn) {
						channelNoteOn[ch] =
							noteOn.releasedAt === null
								? { ...noteOn, releasedAt: event.time }
								: noteOn;
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
			const endTime =
				noteOn.releasedAt !== null && !noteOn.hasReleasePitchEvents
					? noteOn.releasedAt
					: lastTime;
			pushSegment(ch, noteOn, endTime);
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
 * We decode KC into a linear semitone index and then add KF's upper 6 bits
 * (fine pitch, 1/64 semitone per step) so that the result is monotonic with
 * actual pitch and suitable for Y-axis placement.
 */
export function notePitch(kc: number, kf: number): number {
	const octave = (kc >> 4) & 0x07;
	const noteCode = kc & 0x0f;
	const semitone = NOTE_CODE_TO_SEMITONE[noteCode] ?? noteCode;
	const linearSemitone = octave * 12 + semitone;
	const fine = (kf >> 2) & 0x3f;
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
