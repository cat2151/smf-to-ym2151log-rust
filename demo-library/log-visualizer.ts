import {
	type YmLogEvent,
	PIXELS_PER_SECOND,
	parseHexByte,
	DEFAULT_CHANNELS,
	KC_REGISTER_BASE,
	KF_REGISTER_BASE,
	EVENT_WIDTH,
	type LaneElements,
} from "./ym2151-utils";
import {
	buildNoteSegments,
	computePitchRange,
} from "./log-visualizer-note-segments";
import { renderPitchCanvas } from "./log-visualizer-pitch-canvas";
import {
	type LfoRegisterConfig,
	collectLfoEvents,
	renderLfoLane,
	resolveRegisterForChannel,
} from "./log-visualizer-lfo";

export type { LfoRegisterConfig };

export type LogVisualizer = {
	renderFromJson: (jsonText: string | null | undefined) => void;
	clear: () => void;
	/** Provide LFO register config so the visualizer can draw waveform lanes. */
	setLfoRegisters: (registers: LfoRegisterConfig[]) => void;
};

const MIN_TRACK_WIDTH = 640;
const MAX_TRACK_WIDTH = 6400;

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

		// Render note pitch on a per-channel canvas overlay.
		// Groups segments by channel and renders each group with a connected
		// line-graph so that vibrato/delay-vibrato looks continuous.
		const segsByChannel = new Map<number, typeof segments>();
		for (const seg of segments) {
			if (!segsByChannel.has(seg.ch)) segsByChannel.set(seg.ch, []);
			segsByChannel.get(seg.ch)!.push(seg);
		}
		for (const [ch, chSegs] of segsByChannel) {
			const lane = lanes[ch.toString()];
			if (!lane) continue;
			renderPitchCanvas(
				lane.track,
				trackWidth,
				chSegs,
				minPitch,
				maxPitch,
				laneColor(ch),
			);
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
