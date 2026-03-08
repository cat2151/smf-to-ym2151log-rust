import {
	type YmLogEvent,
	PIXELS_PER_SECOND,
	parseHexByte,
} from "./ym2151-utils";
import {
	buildNoteSegments,
	computePitchRange,
	noteYPosition,
} from "./log-visualizer-note-segments";
import {
	type LfoRegisterConfig,
	collectLfoEvents,
	renderLfoLane,
	resolveRegisterForChannel,
} from "./log-visualizer-lfo";

export type { LfoRegisterConfig };

type LaneElements = {
	root: HTMLElement;
	track: HTMLElement;
};

export type LogVisualizer = {
	renderFromJson: (jsonText: string | null | undefined) => void;
	clear: () => void;
	/** Provide LFO register config so the visualizer can draw waveform lanes. */
	setLfoRegisters: (registers: LfoRegisterConfig[]) => void;
};

const DEFAULT_CHANNELS = 8;
const MIN_TRACK_WIDTH = 640;
const MAX_TRACK_WIDTH = 6400;
const EVENT_WIDTH = 4;
const KC_REGISTER_BASE = 0x28;
const KF_REGISTER_BASE = 0x30;
const MIN_NOTE_WIDTH = 2;
const NOTE_WIDTH_GAP = 1;

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
			setLfoRegisters: () => {
				/* no-op */
			},
		};
	}

	const channelCount = Math.max(
		1,
		Math.min(16, options?.channelCount ?? DEFAULT_CHANNELS),
	);

	let lfoRegisters: LfoRegisterConfig[] = [];
	let lastJsonText: string | null | undefined = null;

	const renderEmpty = (message: string) => {
		container.classList.add("log-visualizer", "log-visualizer--empty");
		container.innerHTML = "";
		const empty = document.createElement("div");
		empty.className = "log-visualizer-empty";
		empty.textContent = message;
		container.appendChild(empty);
	};

	const renderFromJson = (jsonText: string | null | undefined) => {
		lastJsonText = jsonText;
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
		// Build a set of all LFO-resolved addresses so they can be suppressed in channel lanes
		const lfoAddrSet = new Set<number>();
		for (const lfoDef of lfoRegisters) {
			const base = parseHexByte(lfoDef.baseRegister);
			if (base === null) continue;
			for (let ch = 0; ch < channelCount; ch++) {
				lfoAddrSet.add(resolveRegisterForChannel(base, ch));
			}
		}

		events.forEach((event, index) => {
			const addr = parseHexByte(event.addr);
			// Suppress KC (0x28-0x2F), KF (0x30-0x37), KEY ON/OFF (0x08), and LFO registers —
			// these are shown via note bars or dedicated LFO lanes.
			const isKcKfOrKeyOn =
				addr !== null &&
				((addr >= KC_REGISTER_BASE &&
					addr < KC_REGISTER_BASE + DEFAULT_CHANNELS) ||
					(addr >= KF_REGISTER_BASE &&
						addr < KF_REGISTER_BASE + DEFAULT_CHANNELS) ||
					addr === 0x08);
			if (isKcKfOrKeyOn) return;
			if (addr !== null && lfoAddrSet.has(addr)) return;

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

		// Render LFO waveform lanes (one per configured LFO base register)
		if (lfoRegisters.length > 0) {
			const lfoData = collectLfoEvents(events, lfoRegisters, channelCount);
			for (const [, entry] of lfoData) {
				renderLfoLane(
					container,
					createLane,
					entry.label,
					entry.events,
					trackWidth,
				);
			}
		}
	};

	renderEmpty("YM2151 ログを変換するとここに描画します。");

	const setLfoRegisters = (registers: LfoRegisterConfig[]) => {
		lfoRegisters = registers;
		// Re-render with the new LFO config if we already have data
		if (lastJsonText != null) {
			renderFromJson(lastJsonText);
		}
	};

	return {
		renderFromJson,
		clear: () => {
			lastJsonText = null;
			renderEmpty("YM2151 ログを変換するとここに描画します。");
		},
		setLfoRegisters,
	};
}
