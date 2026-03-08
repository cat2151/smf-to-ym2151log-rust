import {
	PIXELS_PER_SECOND,
	parseHexByte,
	EVENT_WIDTH,
	TRACK_HEIGHT,
	type LaneElements,
} from "./ym2151-utils";

/** Config for a single LFO-modulated register (base register in hex, e.g. "0x60"). */
export type LfoRegisterConfig = {
	baseRegister: string;
	label?: string;
};

/**
 * Mirrors the Rust `resolve_register_for_channel` function.
 * Given a base register and YM2151 channel index, returns the per-channel address.
 */
export function resolveRegisterForChannel(
	baseReg: number,
	channel: number,
): number {
	if (baseReg >= 0x20 && baseReg <= 0x27) return 0x20 + channel;
	if (baseReg >= 0x28 && baseReg <= 0x2f) return 0x28 + channel;
	if (baseReg >= 0x30 && baseReg <= 0x37) return 0x30 + channel;
	if (baseReg >= 0x38 && baseReg <= 0x3f) return 0x38 + channel;
	if (baseReg >= 0x40) {
		const base = baseReg & 0xe0;
		const slot = baseReg & 0x1f;
		const operator = Math.floor(slot / 8);
		const newSlot = channel + operator * 8;
		return base + newSlot;
	}
	return baseReg;
}

/**
 * For each LFO base register, collect the set of resolved per-channel addresses
 * (for the 8 YM2151 channels) that appear in the event list.
 */
export function collectLfoEvents(
	events: { time: number; addr: string; data: string }[],
	lfoRegisters: LfoRegisterConfig[],
	channelCount: number,
): Map<
	string,
	{ addr: number; label: string; events: { x: number; data: number }[] }
> {
	const result = new Map<
		string,
		{ addr: number; label: string; events: { x: number; data: number }[] }
	>();

	if (lfoRegisters.length === 0) return result;

	// Build address → (baseKey, label) lookup for all channel-resolved addresses
	const addrToKey = new Map<number, { key: string; label: string }>();
	for (const lfoDef of lfoRegisters) {
		const base = parseHexByte(lfoDef.baseRegister);
		if (base === null) continue;
		const key = lfoDef.baseRegister;
		const label = lfoDef.label ?? `LFO ${lfoDef.baseRegister}`;
		for (let ch = 0; ch < channelCount; ch++) {
			const resolved = resolveRegisterForChannel(base, ch);
			addrToKey.set(resolved, { key, label });
		}
	}

	for (const event of events) {
		const addr = parseHexByte(event.addr);
		const data = parseHexByte(event.data);
		if (addr === null || data === null) continue;
		const entry = addrToKey.get(addr);
		if (!entry) continue;
		if (!result.has(entry.key)) {
			result.set(entry.key, {
				addr: parseHexByte(entry.key) ?? addr,
				label: entry.label,
				events: [],
			});
		}
		result.get(entry.key)!.events.push({ x: event.time, data });
	}

	return result;
}

/**
 * Render an LFO waveform lane below the channel lanes.
 * Events are plotted as dots whose Y position is proportional to the data byte value,
 * scaled between the observed minimum and maximum so the full lane height is used.
 */
export function renderLfoLane(
	container: HTMLElement,
	createLane: (label: string, trackWidth: number) => LaneElements,
	label: string,
	lfoEvts: { x: number; data: number }[],
	trackWidth: number,
): void {
	if (lfoEvts.length === 0) return;

	let minVal = Number.POSITIVE_INFINITY;
	let maxVal = Number.NEGATIVE_INFINITY;
	for (const e of lfoEvts) {
		if (e.data < minVal) minVal = e.data;
		if (e.data > maxVal) maxVal = e.data;
	}

	const lane = createLane(`${label} ${minVal}–${maxVal}`, trackWidth);
	container.appendChild(lane.root);

	const valueRange = maxVal - minVal;
	const usableHeight = TRACK_HEIGHT - EVENT_WIDTH;

	for (const e of lfoEvts) {
		const ratio = valueRange > 0 ? (e.data - minVal) / valueRange : 0.5;
		// High data value → low Y (top of track); low data value → high Y (bottom)
		const top = Math.round((1 - ratio) * usableHeight);

		const dot = document.createElement("div");
		dot.className = "log-visualizer-event log-visualizer-event--lfo";
		dot.style.left = `${Math.max(0, Math.min(trackWidth - EVENT_WIDTH, e.x * PIXELS_PER_SECOND))}px`;
		dot.style.top = `${top}px`;
		dot.title = `t=${e.x.toFixed(3)}s data=0x${e.data.toString(16).padStart(2, "0")}`;
		lane.track.appendChild(dot);
	}
}
