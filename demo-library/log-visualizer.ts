type YmLogEvent = {
	time: number;
	addr: string;
	data: string;
};

type LaneElements = {
	root: HTMLElement;
	track: HTMLElement;
};

type NoteOnState = { time: number; kc: number; kf: number };

type NoteSegment = {
	startTime: number;
	endTime: number;
	kc: number;
	kf: number;
	ch: number;
};

export type LogVisualizer = {
	renderFromJson: (jsonText: string | null | undefined) => void;
	clear: () => void;
};

const DEFAULT_CHANNELS = 8;
const MIN_TRACK_WIDTH = 640;
const MAX_TRACK_WIDTH = 6400;
const PIXELS_PER_SECOND = 180;
const EVENT_WIDTH = 4;
const KC_REGISTER_BASE = 0x28;
const KF_REGISTER_BASE = 0x30;
const TRACK_HEIGHT = 80;
const NOTE_BAR_HEIGHT = 8;
const MIN_NOTE_WIDTH = 2;
const NOTE_WIDTH_GAP = 1;
const UNCLOSED_NOTE_EXTENSION_S = 0.1;
// Minimum segment duration to keep DOM node count manageable.
// Segments shorter than one pixel at the current scale are coalesced into their successor.
const MIN_SEGMENT_SECONDS = MIN_NOTE_WIDTH / PIXELS_PER_SECOND;

/**
 * Maps YM2151 note code (low nibble of KC byte) to a linear semitone index (0–11).
 * YM2151 note codes start at C# (code 0) and end at C (code 14).
 * Codes 3, 7, 11, and 15 are unused by the note table and are mapped to the
 * nearest lower valid semitone so that the resulting pitch is still monotonic.
 */
const NOTE_CODE_TO_SEMITONE: readonly number[] = [
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

function parseHexByte(value: string): number | null {
	const match = /^0x([0-9a-fA-F]{1,2})$/.exec(value.trim());
	if (!match) return null;
	const parsed = parseInt(match[1], 16);
	return Number.isNaN(parsed) ? null : parsed;
}

function detectChannel(
	addrHex: string,
	dataHex: string,
	channelCount: number,
): number | null {
	const addr = parseHexByte(addrHex);
	if (addr === null) return null;

	if (addr === 0x08) {
		const data = parseHexByte(dataHex);
		if (data !== null) {
			return data & 0x07;
		}
	}

	if (addr >= 0x20) {
		const channel = addr & 0x07;
		return channel < channelCount ? channel : null;
	}

	return null;
}

function buildNoteSegments(
	events: YmLogEvent[],
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
function notePitch(kc: number, kf: number): number {
	const octave = (kc >> 4) & 0x07;
	const noteCode = kc & 0x0f;
	const semitone = NOTE_CODE_TO_SEMITONE[noteCode] ?? noteCode;
	const linearSemitone = octave * 12 + semitone;
	const fine = kf & 0x3f; // KF is 6-bit, 0-63
	return linearSemitone * 64 + fine;
}

function computePitchRange(segments: NoteSegment[]): {
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

function noteYPosition(
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

function normalizeEvents(parsed: unknown): YmLogEvent[] {
	if (!parsed || typeof parsed !== "object") return [];
	const rawEvents = (parsed as { events?: unknown }).events;
	if (!Array.isArray(rawEvents)) return [];

	return rawEvents
		.map((event) => {
			if (!event || typeof event !== "object") return null;
			const e = event as { time?: unknown; addr?: unknown; data?: unknown };
			const time =
				typeof e.time === "number"
					? e.time
					: typeof e.time === "string"
						? Number(e.time)
						: Number.NaN;
			const addr = typeof e.addr === "string" ? e.addr : "";
			const data = typeof e.data === "string" ? e.data : "";
			if (!Number.isFinite(time) || !addr || !data) return null;
			return { time, addr, data };
		})
		.filter((e): e is YmLogEvent => Boolean(e));
}

function laneColor(index: number | null): string {
	if (index === null) return "#8a8a8a";
	const hue = (index * 37) % 360;
	return `hsl(${hue}, 70%, 55%)`;
}

function createLane(label: string, trackWidth: number): LaneElements {
	const root = document.createElement("div");
	root.className = "log-visualizer-lane";

	const labelEl = document.createElement("div");
	labelEl.className = "log-visualizer-label";
	labelEl.textContent = label;
	root.appendChild(labelEl);

	const track = document.createElement("div");
	track.className = "log-visualizer-track";
	track.style.width = `${trackWidth}px`;
	root.appendChild(track);

	return { root, track };
}

function computeTrackWidth(events: YmLogEvent[]): number {
	const maxTime = events.reduce((max, e) => Math.max(max, e.time), 0);
	const width = maxTime * PIXELS_PER_SECOND + 40;
	return Math.min(MAX_TRACK_WIDTH, Math.max(MIN_TRACK_WIDTH, width));
}

export function createLogVisualizer(
	container: HTMLElement | null,
	options?: { channelCount?: number },
): LogVisualizer {
	if (!container) {
		return {
			renderFromJson: () => {
				/* no-op */
			},
			clear: () => {
				/* no-op */
			},
		};
	}

	const channelCount = Math.max(
		1,
		Math.min(16, options?.channelCount ?? DEFAULT_CHANNELS),
	);

	const renderEmpty = (message: string) => {
		container.classList.add("log-visualizer", "log-visualizer--empty");
		container.innerHTML = "";
		const empty = document.createElement("div");
		empty.className = "log-visualizer-empty";
		empty.textContent = message;
		container.appendChild(empty);
	};

	const renderFromJson = (jsonText: string | null | undefined) => {
		if (!jsonText || jsonText.trim().length === 0) {
			renderEmpty("変換結果がまだありません。");
			return;
		}

		let events: YmLogEvent[] = [];
		try {
			const parsed = JSON.parse(jsonText);
			events = normalizeEvents(parsed);
		} catch {
			renderEmpty("ログ JSON を解釈できませんでした。");
			return;
		}

		if (events.length === 0) {
			renderEmpty("描画できるイベントがありません。");
			return;
		}

		const trackWidth = computeTrackWidth(events);
		const segments = buildNoteSegments(events, channelCount);
		const { min: minPitch, max: maxPitch } = computePitchRange(segments);

		container.classList.add("log-visualizer");
		container.classList.remove("log-visualizer--empty");
		container.innerHTML = "";

		const lanes: Record<string, LaneElements> = {};

		for (let ch = 0; ch < channelCount; ch += 1) {
			const lane = createLane(`CH ${ch}`, trackWidth);
			container.appendChild(lane.root);
			lanes[ch.toString()] = lane;
		}

		let globalLane: LaneElements | null = null;
		const ensureGlobalLane = () => {
			if (globalLane) return globalLane;
			globalLane = createLane("GLOBAL", trackWidth);
			container.appendChild(globalLane.root);
			return globalLane;
		};

		// Render other events as small background dots (rendered first so note bars appear on top)
		events.forEach((event, index) => {
			const addr = parseHexByte(event.addr);
			// Suppress KC (0x28-0x2F), KF (0x30-0x37), and KEY ON/OFF (0x08) — these are shown via note bars
			const isKcKfOrKeyOn =
				addr !== null &&
				((addr >= KC_REGISTER_BASE &&
					addr < KC_REGISTER_BASE + DEFAULT_CHANNELS) ||
					(addr >= KF_REGISTER_BASE &&
						addr < KF_REGISTER_BASE + DEFAULT_CHANNELS) ||
					addr === 0x08);
			if (isKcKfOrKeyOn) return;

			const channel = detectChannel(event.addr, event.data, channelCount);
			const lane =
				channel !== null && channel >= 0 && channel < channelCount
					? lanes[channel.toString()]
					: ensureGlobalLane();
			const marker = document.createElement("div");
			marker.className = "log-visualizer-event";
			marker.style.left = `${Math.max(0, Math.min(trackWidth - EVENT_WIDTH, event.time * PIXELS_PER_SECOND))}px`;
			marker.style.backgroundColor = laneColor(channel);
			marker.title = `t=${event.time.toFixed(3)}s addr=${event.addr} data=${event.data} (#${index})`;
			lane.track.appendChild(marker);
		});

		// Render note bars on top (piano-roll style: keyon/off + KC pitch)
		for (const seg of segments) {
			const lane = lanes[seg.ch.toString()];
			if (!lane) continue;
			const bar = document.createElement("div");
			bar.className = "log-visualizer-note";
			const x = Math.max(
				0,
				Math.min(
					trackWidth - MIN_NOTE_WIDTH,
					seg.startTime * PIXELS_PER_SECOND,
				),
			);
			const w = Math.max(
				MIN_NOTE_WIDTH,
				(seg.endTime - seg.startTime) * PIXELS_PER_SECOND - NOTE_WIDTH_GAP,
			);
			const y = noteYPosition(seg.kc, seg.kf, minPitch, maxPitch);
			bar.style.left = `${x}px`;
			bar.style.width = `${w}px`;
			bar.style.top = `${y}px`;
			bar.style.backgroundColor = laneColor(seg.ch);
			bar.title = `CH${seg.ch} KC=0x${seg.kc.toString(16).padStart(2, "0")} KF=0x${seg.kf.toString(16).padStart(2, "0")} t=${seg.startTime.toFixed(3)}-${seg.endTime.toFixed(3)}s`;
			lane.track.appendChild(bar);
		}
	};

	renderEmpty("YM2151 ログを変換するとここに描画します。");

	return {
		renderFromJson,
		clear: () => renderEmpty("YM2151 ログを変換するとここに描画します。"),
	};
}
