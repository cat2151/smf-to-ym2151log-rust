type YmLogEvent = {
	time: number;
	addr: string;
	data: string;
};

type LaneElements = {
	root: HTMLElement;
	track: HTMLElement;
};

type NoteOnState = { time: number; kc: number };

type NoteSegment = {
	startTime: number;
	endTime: number;
	kc: number;
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
const TRACK_HEIGHT = 80;
const NOTE_BAR_HEIGHT = 8;
const MIN_NOTE_WIDTH = 2;
const NOTE_WIDTH_GAP = 1;
const UNCLOSED_NOTE_EXTENSION_S = 0.1;

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
	// YM2151 has exactly 8 channels; KC registers are always 0x28-0x2F.
	const kcChannelCount = Math.min(channelCount, DEFAULT_CHANNELS);
	const channelKC: number[] = Array(channelCount).fill(0);
	const channelNoteOn: Array<NoteOnState | null> =
		Array(channelCount).fill(null);
	const segments: NoteSegment[] = [];

	for (const event of events) {
		const addr = parseHexByte(event.addr);
		const data = parseHexByte(event.data);
		if (addr === null || data === null) continue;

		if (addr >= KC_REGISTER_BASE && addr < KC_REGISTER_BASE + kcChannelCount) {
			const ch = addr - KC_REGISTER_BASE;
			const newKC = data;
			// If KC changes while a note is held, close the current segment and
			// open a new one with the updated pitch (handles portamento/vibrato).
			if (channelNoteOn[ch] && channelKC[ch] !== newKC) {
				const noteOn = channelNoteOn[ch] as NoteOnState;
				segments.push({
					startTime: noteOn.time,
					endTime: event.time,
					kc: noteOn.kc,
					ch,
				});
				channelNoteOn[ch] = { time: event.time, kc: newKC };
			}
			channelKC[ch] = newKC;
		}

		if (addr === 0x08) {
			const ch = data & 0x07;
			const operators = (data >> 3) & 0x0f;
			if (ch >= 0 && ch < channelCount) {
				if (operators !== 0) {
					if (!channelNoteOn[ch]) {
						channelNoteOn[ch] = { time: event.time, kc: channelKC[ch] };
					}
				} else {
					const noteOn = channelNoteOn[ch];
					if (noteOn) {
						segments.push({
							startTime: noteOn.time,
							endTime: event.time,
							kc: noteOn.kc,
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
				ch,
			});
		}
	}

	return segments;
}

function computeKcRange(segments: NoteSegment[]): { min: number; max: number } {
	if (segments.length === 0) return { min: 0, max: 0 };
	let min = segments[0].kc;
	let max = segments[0].kc;
	for (const seg of segments) {
		if (seg.kc < min) min = seg.kc;
		if (seg.kc > max) max = seg.kc;
	}
	return { min, max };
}

function noteYPosition(kc: number, minKC: number, maxKC: number): number {
	const range = maxKC - minKC;
	if (range === 0) return (TRACK_HEIGHT - NOTE_BAR_HEIGHT) / 2;
	return ((maxKC - kc) / range) * (TRACK_HEIGHT - NOTE_BAR_HEIGHT);
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
		const { min: minKC, max: maxKC } = computeKcRange(segments);

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
			// Suppress KC (0x28-0x2F, always 8 channels on YM2151) and KEY ON/OFF (0x08)
			const isKcOrKeyOn =
				addr !== null &&
				((addr >= KC_REGISTER_BASE &&
					addr < KC_REGISTER_BASE + DEFAULT_CHANNELS) ||
					addr === 0x08);
			if (isKcOrKeyOn) return;

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
			const y = noteYPosition(seg.kc, minKC, maxKC);
			bar.style.left = `${x}px`;
			bar.style.width = `${w}px`;
			bar.style.top = `${y}px`;
			bar.style.backgroundColor = laneColor(seg.ch);
			bar.title = `CH${seg.ch} KC=0x${seg.kc.toString(16).padStart(2, "0")} t=${seg.startTime.toFixed(3)}-${seg.endTime.toFixed(3)}s`;
			lane.track.appendChild(bar);
		}
	};

	renderEmpty("YM2151 ログを変換するとここに描画します。");

	return {
		renderFromJson,
		clear: () => renderEmpty("YM2151 ログを変換するとここに描画します。"),
	};
}
