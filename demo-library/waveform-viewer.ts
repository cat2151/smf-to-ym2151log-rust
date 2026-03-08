/**
 * Waveform viewer for pop-noise visualization.
 *
 * Simulates the YM2151 envelope generator (ADSR per operator) and renders
 * a canvas waveform so users can scroll to note boundaries, zoom to a single
 * waveform period, and visually confirm whether pop-noise has been reduced.
 */

// YM2151 internal sample rate (55.93 kHz, per the OPM datasheet §4).
const YM_SAMPLE_RATE = 55930;
// Number of FM operators per channel
const NUM_OPERATORS = 4;
// Base pixels-per-second scale (same as log-visualizer)
const BASE_PIXELS_PER_SECOND = 180;
// Maximum simulation length in seconds to keep memory reasonable
const MAX_SIMULATE_SECONDS = 30;

// --- UI string constants ---
const MSG_INITIAL = "YM2151 ログを変換するとここに描画します。";
const MSG_EMPTY = "変換結果がまだありません。";
const MSG_PARSE_ERROR = "ログ JSON を解析できませんでした。";

// --- Helpers ---

function parseHex(s: string): number | null {
	const m = /^0x([0-9a-fA-F]{1,2})$/.exec(s.trim());
	if (!m) return null;
	const n = Number.parseInt(m[1], 16);
	return Number.isNaN(n) ? null : n;
}

// Maps YM2151 note code (low nibble of KC) to semitone offset from C# (0-11).
// Matches the same table used in log-visualizer.ts.
const NOTE_CODE_TO_SEMITONE: readonly number[] = [
	0, 1, 2, 2, 3, 4, 5, 5, 6, 7, 8, 8, 9, 10, 11, 11,
];

/**
 * Convert YM2151 KC and KF register values to a frequency in Hz.
 * Reference: C#4 = 277.18 Hz at linear semitone 48 (octave 4, note code 0).
 */
function kcToFrequency(kc: number, kf: number): number {
	const octave = (kc >> 4) & 0x07;
	const noteCode = kc & 0x0f;
	const semitone = NOTE_CODE_TO_SEMITONE[noteCode] ?? 0;
	// KF bits 7:2 hold the 6-bit key fraction (0–63), each step = 1/64 semitone.
	const fine = ((kf >> 2) & 0x3f) / 64.0;
	const linearSemitones = octave * 12 + semitone + fine;
	return 277.18 * Math.pow(2, (linearSemitones - 48) / 12);
}

/**
 * Amplitude increment per sample for a given effective rate (0–62).
 * At effective rate 62 a full traversal (0→1) takes ~0.086 ms at 55930 Hz.
 */
function ampStepPerSample(effectiveRate: number): number {
	if (effectiveRate <= 0) return 0;
	const r = Math.min(62, effectiveRate);
	const traversalSeconds = 0.000086 * Math.pow(2, 62 - r);
	return 1.0 / (traversalSeconds * YM_SAMPLE_RATE);
}

// --- Envelope Generator ---

type EnvelopePhase = "off" | "attack" | "decay1" | "decay2" | "release";

/**
 * Simplified YM2151 per-operator envelope generator.
 * amplitude is NOT reset on key-on by design: if the previous note's release
 * phase left a non-zero amplitude, the new attack starts from that level,
 * creating a discontinuity that manifests as pop-noise in the audio output.
 */
class EnvelopeGenerator {
	amplitude = 0;
	phase: EnvelopePhase = "off";
	ar = 28; // Attack Rate 0–31
	d1r = 5; // Decay1 Rate 0–31
	d1lLevel = 0.25; // Decay1 Level threshold, 0–1
	d2r = 0; // Decay2 Rate 0–31
	rr = 7; // Release Rate 0–15
	tl = 0; // Total Level attenuation 0–127 (0=loudest)

	keyOn(): void {
		this.phase = "attack";
		// amplitude intentionally kept — non-zero amplitude at key-on = pop noise
	}

	keyOff(): void {
		if (this.phase !== "off") this.phase = "release";
	}

	step(): number {
		switch (this.phase) {
			case "attack": {
				const s = ampStepPerSample(this.ar * 2);
				this.amplitude = Math.min(1.0, this.amplitude + Math.max(s, 1e-6));
				if (this.amplitude >= 1.0) {
					this.amplitude = 1.0;
					this.phase = "decay1";
				}
				break;
			}
			case "decay1": {
				const s = ampStepPerSample(this.d1r * 2);
				this.amplitude = Math.max(this.d1lLevel, this.amplitude - s);
				if (this.amplitude <= this.d1lLevel) this.phase = "decay2";
				break;
			}
			case "decay2": {
				const s = ampStepPerSample(this.d2r * 2);
				this.amplitude = Math.max(0, this.amplitude - s);
				break;
			}
			case "release": {
				const s = ampStepPerSample(this.rr * 4);
				this.amplitude = Math.max(0, this.amplitude - s);
				if (this.amplitude <= 0) {
					this.amplitude = 0;
					this.phase = "off";
				}
				break;
			}
			case "off":
				this.amplitude = 0;
				break;
		}
		// Apply total-level attenuation (0 = no attenuation, 127 = near silence)
		const attenuation = 1.0 - this.tl / 128.0;
		return this.amplitude * attenuation;
	}
}

// --- Waveform simulation ---

type YmLogEvent = { time: number; addr: string; data: string };

type WaveformData = {
	/** Interleaved [envelope, waveform] pairs: index 2i = envelope, 2i+1 = waveform */
	envelopeSamples: Float32Array;
	waveformSamples: Float32Array;
	sampleRate: number;
	durationS: number;
	/** Timestamps (seconds) of key-on events for the simulated channel */
	noteBoundaries: number[];
};

/**
 * Simulate the YM2151 envelope generator for one channel and return sample arrays.
 * Events must be sorted by time (guaranteed by the converter).
 */
function simulateWaveform(rawEvents: YmLogEvent[], ch: number): WaveformData {
	const maxTime = rawEvents.reduce((m, e) => Math.max(m, e.time), 0);
	const durationS = Math.min(maxTime + 1.0, MAX_SIMULATE_SECONDS);
	const totalSamples = Math.ceil(durationS * YM_SAMPLE_RATE);

	const envelopeSamples = new Float32Array(totalSamples);
	const waveformSamples = new Float32Array(totalSamples);
	const noteBoundaries: number[] = [];

	const envGens = Array.from(
		{ length: NUM_OPERATORS },
		() => new EnvelopeGenerator(),
	);
	let kc = 0x4a; // default pitch: A4
	let kf = 0;
	let freq = kcToFrequency(kc, kf);
	let oscPhase = 0;
	let eventIndex = 0;

	for (let i = 0; i < totalSamples; i++) {
		const t = i / YM_SAMPLE_RATE;

		// Process all events at or before current sample time
		while (eventIndex < rawEvents.length && rawEvents[eventIndex].time <= t) {
			const ev = rawEvents[eventIndex++];
			const addr = parseHex(ev.addr);
			const data = parseHex(ev.data);
			if (addr === null || data === null) continue;

			// Key On/Off: register 0x08
			if (addr === 0x08) {
				const evCh = data & 0x07;
				if (evCh !== ch) continue;
				const ops = (data >> 3) & 0x0f;
				if (ops !== 0) {
					for (const gen of envGens) gen.keyOn();
					noteBoundaries.push(ev.time);
				} else {
					for (const gen of envGens) gen.keyOff();
				}
				continue;
			}

			// KC register: 0x28–0x2F (one per channel)
			if (addr >= 0x28 && addr <= 0x2f && addr - 0x28 === ch) {
				kc = data;
				freq = kcToFrequency(kc, kf);
				continue;
			}

			// KF register: 0x30–0x37 (one per channel)
			if (addr >= 0x30 && addr <= 0x37 && addr - 0x30 === ch) {
				kf = data;
				freq = kcToFrequency(kc, kf);
				continue;
			}

			// Operator register helper: slot = operator*8 + channel
			const applyOp = (
				base: number,
				apply: (gen: EnvelopeGenerator, val: number) => void,
			): void => {
				if (addr >= base && addr < base + 32) {
					const slot = addr - base;
					if (slot % 8 !== ch) return;
					const op = Math.floor(slot / 8);
					if (op < NUM_OPERATORS) apply(envGens[op], data);
				}
			};

			applyOp(0x60, (gen, val) => {
				gen.tl = val & 0x7f;
			}); // TL
			applyOp(0x80, (gen, val) => {
				gen.ar = val & 0x1f;
			}); // AR
			applyOp(0xa0, (gen, val) => {
				gen.d1r = val & 0x1f;
			}); // D1R
			applyOp(0xc0, (gen, val) => {
				gen.d2r = val & 0x1f;
			}); // D2R
			applyOp(0xe0, (gen, val) => {
				gen.d1lLevel = ((val >> 4) & 0x0f) / 15.0;
				gen.rr = val & 0x0f;
			}); // D1L/RR
		}

		// Combine all operators: take the loudest envelope value
		let envelope = 0;
		for (const gen of envGens) {
			envelope = Math.max(envelope, gen.step());
		}
		envelopeSamples[i] = envelope;

		// Carrier sine wave multiplied by envelope
		oscPhase += (2 * Math.PI * freq) / YM_SAMPLE_RATE;
		if (oscPhase > 2 * Math.PI) oscPhase -= 2 * Math.PI;
		waveformSamples[i] = envelope * Math.sin(oscPhase);
	}

	return {
		envelopeSamples,
		waveformSamples,
		sampleRate: YM_SAMPLE_RATE,
		durationS,
		noteBoundaries,
	};
}

// --- Canvas rendering ---

function drawEmpty(
	ctx: CanvasRenderingContext2D,
	width: number,
	height: number,
	message: string,
): void {
	ctx.fillStyle = "#f8f8f8";
	ctx.fillRect(0, 0, width, height);
	ctx.fillStyle = "#999";
	ctx.font = "14px Arial, sans-serif";
	ctx.textAlign = "center";
	ctx.fillText(message, width / 2, height / 2);
	ctx.textAlign = "left";
}

function drawWaveform(
	ctx: CanvasRenderingContext2D,
	width: number,
	height: number,
	data: WaveformData,
	viewStart: number,
	zoom: number,
): void {
	const pixelsPerSec = BASE_PIXELS_PER_SECOND * zoom;
	const viewDurationS = width / pixelsPerSec;
	const viewEnd = viewStart + viewDurationS;

	// Background
	ctx.fillStyle = "#f8f8f8";
	ctx.fillRect(0, 0, width, height);

	// Center line
	ctx.strokeStyle = "#ddd";
	ctx.lineWidth = 1;
	ctx.beginPath();
	ctx.moveTo(0, height / 2);
	ctx.lineTo(width, height / 2);
	ctx.stroke();

	// Envelope top/bottom guides at ±1
	const margin = 4;
	const yCenter = height / 2;
	const yScale = yCenter - margin;

	ctx.strokeStyle = "#e8e8e8";
	ctx.lineWidth = 1;
	ctx.setLineDash([4, 4]);
	ctx.beginPath();
	ctx.moveTo(0, margin);
	ctx.lineTo(width, margin);
	ctx.moveTo(0, height - margin);
	ctx.lineTo(width, height - margin);
	ctx.stroke();
	ctx.setLineDash([]);

	// Note boundary markers (key-on events)
	for (const t of data.noteBoundaries) {
		if (t < viewStart - 0.001 || t > viewEnd + 0.001) continue;
		const x = Math.round((t - viewStart) * pixelsPerSec);
		ctx.strokeStyle = "rgba(0, 180, 80, 0.8)";
		ctx.lineWidth = 2;
		ctx.beginPath();
		ctx.moveTo(x, 0);
		ctx.lineTo(x, height);
		ctx.stroke();
		ctx.fillStyle = "rgba(0, 140, 60, 0.9)";
		ctx.font = "10px monospace";
		ctx.fillText(`key-on ${(t * 1000).toFixed(1)}ms`, x + 3, 11);
	}

	// Compute sample range for visible window
	const startSample = Math.max(0, Math.floor(viewStart * data.sampleRate) - 1);
	const endSample = Math.min(
		data.waveformSamples.length - 1,
		Math.ceil(viewEnd * data.sampleRate) + 1,
	);

	if (startSample >= endSample) return;

	const samplesPerPixel = (endSample - startSample + 1) / width;

	// Draw waveform (blue) using min/max per pixel column for correct anti-aliasing
	ctx.strokeStyle = "#2196F3";
	ctx.lineWidth = 1.5;
	ctx.beginPath();
	let firstWave = true;
	for (let px = 0; px < width; px++) {
		const sStart = Math.floor(startSample + px * samplesPerPixel);
		const sEnd = Math.min(
			endSample,
			Math.floor(startSample + (px + 1) * samplesPerPixel),
		);
		let minVal = 0;
		let maxVal = 0;
		for (let s = sStart; s <= sEnd; s++) {
			const v = data.waveformSamples[s] ?? 0;
			if (v < minVal) minVal = v;
			if (v > maxVal) maxVal = v;
		}
		const yTop = yCenter - maxVal * yScale;
		const yBot = yCenter - minVal * yScale;
		if (firstWave) {
			ctx.moveTo(px, (yTop + yBot) / 2);
			firstWave = false;
		}
		if (yBot - yTop > 0.5) {
			ctx.lineTo(px, yTop);
			ctx.lineTo(px, yBot);
		} else {
			ctx.lineTo(px, (yTop + yBot) / 2);
		}
	}
	ctx.stroke();

	// Draw envelope (orange) as an overlay line
	ctx.strokeStyle = "rgba(230, 100, 20, 0.85)";
	ctx.lineWidth = 1.5;
	ctx.beginPath();
	let firstEnv = true;
	for (let px = 0; px < width; px++) {
		const sStart = Math.floor(startSample + px * samplesPerPixel);
		const sEnd = Math.min(
			endSample,
			Math.floor(startSample + (px + 1) * samplesPerPixel),
		);
		let maxEnv = 0;
		for (let s = sStart; s <= sEnd; s++) {
			const v = data.envelopeSamples[s] ?? 0;
			if (v > maxEnv) maxEnv = v;
		}
		const y = yCenter - maxEnv * yScale;
		if (firstEnv) {
			ctx.moveTo(px, y);
			firstEnv = false;
		} else {
			ctx.lineTo(px, y);
		}
	}
	ctx.stroke();

	// Time-axis labels
	const labelCount = Math.min(8, Math.floor(width / 80));
	ctx.fillStyle = "#888";
	ctx.font = "10px monospace";
	for (let i = 0; i <= labelCount; i++) {
		const t = viewStart + (i / labelCount) * viewDurationS;
		const x = Math.round((i / labelCount) * width);
		ctx.fillText(`${(t * 1000).toFixed(2)}ms`, x + 2, height - 2);
	}

	// Legend
	ctx.fillStyle = "#2196F3";
	ctx.fillRect(6, height - 28, 14, 3);
	ctx.fillStyle = "#555";
	ctx.font = "10px sans-serif";
	ctx.fillText("波形", 24, height - 24);
	ctx.fillStyle = "rgba(230, 100, 20, 0.85)";
	ctx.fillRect(60, height - 28, 14, 3);
	ctx.fillStyle = "#555";
	ctx.fillText("エンベロープ", 78, height - 24);
}

// --- Public API ---

export type WaveformViewerControls = {
	zoomSlider: HTMLInputElement | null;
	zoomLabel: HTMLElement | null;
	prevNoteBtn: HTMLButtonElement | null;
	nextNoteBtn: HTMLButtonElement | null;
	channelSelect: HTMLSelectElement | null;
	positionLabel: HTMLElement | null;
};

export type WaveformViewer = {
	renderFromJson: (jsonText: string | null | undefined) => void;
	clear: () => void;
};

export function createWaveformViewer(
	canvas: HTMLCanvasElement | null,
	controls: WaveformViewerControls,
): WaveformViewer {
	if (!canvas) return { renderFromJson: () => {}, clear: () => {} };
	const ctx = canvas.getContext("2d");
	if (!ctx) return { renderFromJson: () => {}, clear: () => {} };

	// ctx is guaranteed non-null from here on; alias as a non-nullable type.
	const ctx2d: CanvasRenderingContext2D = ctx;
	const W = canvas.width;
	const H = canvas.height;

	let waveformData: WaveformData | null = null;
	let rawEvents: YmLogEvent[] = [];
	let viewStart = 0; // seconds from start of log
	let zoom = 50; // current zoom multiplier
	let selectedChannel = 0;

	const {
		zoomSlider,
		zoomLabel,
		prevNoteBtn,
		nextNoteBtn,
		channelSelect,
		positionLabel,
	} = controls;

	function getWindowDurS(): number {
		return W / (BASE_PIXELS_PER_SECOND * zoom);
	}

	function clampViewStart(v: number): number {
		const maxStart = Math.max(
			0,
			(waveformData?.durationS ?? 0) - getWindowDurS(),
		);
		return Math.max(0, Math.min(maxStart, v));
	}

	function updatePositionLabel(): void {
		if (!positionLabel) return;
		const windowMs = getWindowDurS() * 1000;
		positionLabel.textContent = `位置: ${(viewStart * 1000).toFixed(2)} ms　表示幅: ${windowMs.toFixed(2)} ms　CH: ${selectedChannel}`;
	}

	function render(): void {
		if (!waveformData) {
			drawEmpty(ctx2d, W, H, MSG_INITIAL);
			return;
		}
		drawWaveform(ctx2d, W, H, waveformData, viewStart, zoom);
		updatePositionLabel();
	}

	function rebuildAndRender(): void {
		if (rawEvents.length === 0) {
			waveformData = null;
			render();
			return;
		}
		waveformData = simulateWaveform(rawEvents, selectedChannel);
		// Auto-scroll to the first note boundary for the selected channel
		if (waveformData.noteBoundaries.length > 0) {
			const firstNote = waveformData.noteBoundaries[0];
			viewStart = clampViewStart(firstNote - getWindowDurS() * 0.3);
		} else {
			viewStart = 0;
		}
		render();
	}

	function setZoom(newZoom: number, anchorFraction = 0.5): void {
		const oldWindowDur = getWindowDurS();
		zoom = Math.max(1, Math.min(2000, newZoom));
		const newWindowDur = getWindowDurS();
		// Zoom around the anchor point (fraction of canvas width)
		viewStart = clampViewStart(
			viewStart + anchorFraction * (oldWindowDur - newWindowDur),
		);
		if (zoomSlider) {
			zoomSlider.value = String(
				Math.round((Math.log10(zoom) / Math.log10(2000)) * 100),
			);
		}
		if (zoomLabel) {
			zoomLabel.textContent = `${zoom.toFixed(zoom < 10 ? 1 : 0)}x`;
		}
		render();
	}

	// Zoom slider (0–100 → zoom 1–2000 exponentially)
	if (zoomSlider) {
		zoomSlider.addEventListener("input", () => {
			const sliderVal = Number(zoomSlider.value) / 100;
			const newZoom = Math.pow(2000, sliderVal);
			const oldWindowDur = getWindowDurS();
			zoom = Math.max(1, Math.min(2000, newZoom));
			const newWindowDur = getWindowDurS();
			viewStart = clampViewStart(
				viewStart + 0.5 * (oldWindowDur - newWindowDur),
			);
			if (zoomLabel) {
				zoomLabel.textContent = `${zoom.toFixed(zoom < 10 ? 1 : 0)}x`;
			}
			render();
		});
	}

	// Channel selector
	if (channelSelect) {
		channelSelect.addEventListener("change", () => {
			selectedChannel = Number(channelSelect.value);
			rebuildAndRender();
		});
	}

	// Prev note button
	if (prevNoteBtn) {
		prevNoteBtn.addEventListener("click", () => {
			if (!waveformData) return;
			const center = viewStart + getWindowDurS() / 2;
			const prevCandidates = waveformData.noteBoundaries.filter(
				(t) => t < center - 0.001,
			);
			const prev =
				prevCandidates.length > 0
					? prevCandidates[prevCandidates.length - 1]
					: undefined;
			if (prev !== undefined) {
				viewStart = clampViewStart(prev - getWindowDurS() * 0.3);
				render();
			}
		});
	}

	// Next note button
	if (nextNoteBtn) {
		nextNoteBtn.addEventListener("click", () => {
			if (!waveformData) return;
			const center = viewStart + getWindowDurS() / 2;
			const next = waveformData.noteBoundaries.find((t) => t > center + 0.001);
			if (next !== undefined) {
				viewStart = clampViewStart(next - getWindowDurS() * 0.3);
				render();
			}
		});
	}

	// Mouse wheel: zoom (centered on cursor)
	canvas.addEventListener(
		"wheel",
		(e) => {
			e.preventDefault();
			const factor = e.deltaY > 0 ? 0.75 : 1.333;
			const rect = canvas.getBoundingClientRect();
			const anchorFraction = (e.clientX - rect.left) / rect.width;
			setZoom(zoom * factor, anchorFraction);
		},
		{ passive: false },
	);

	// Mouse drag: pan
	let dragState: { startX: number; startViewStart: number } | null = null;
	canvas.addEventListener("mousedown", (e) => {
		dragState = { startX: e.clientX, startViewStart: viewStart };
		canvas.style.cursor = "grabbing";
	});
	canvas.addEventListener("mousemove", (e) => {
		if (!dragState) return;
		const dx = e.clientX - dragState.startX;
		const pixPerSec = BASE_PIXELS_PER_SECOND * zoom;
		viewStart = clampViewStart(dragState.startViewStart - dx / pixPerSec);
		render();
	});
	const endDrag = () => {
		dragState = null;
		canvas.style.cursor = "grab";
	};
	canvas.addEventListener("mouseup", endDrag);
	canvas.addEventListener("mouseleave", endDrag);

	// Initial state
	drawEmpty(ctx2d, W, H, MSG_INITIAL);

	return {
		renderFromJson(jsonText) {
			if (!jsonText || jsonText.trim().length === 0) {
				rawEvents = [];
				waveformData = null;
				drawEmpty(ctx2d, W, H, MSG_EMPTY);
				return;
			}
			try {
				const parsed = JSON.parse(jsonText) as unknown;
				if (!parsed || typeof parsed !== "object") {
					rawEvents = [];
					waveformData = null;
					render();
					return;
				}
				const arr = (parsed as { events?: unknown }).events;
				if (!Array.isArray(arr)) {
					rawEvents = [];
					waveformData = null;
					render();
					return;
				}
				rawEvents = arr
					.map((e) => {
						if (!e || typeof e !== "object") return null;
						const ev = e as { time?: unknown; addr?: unknown; data?: unknown };
						const time = typeof ev.time === "number" ? ev.time : Number.NaN;
						const addr = typeof ev.addr === "string" ? ev.addr : "";
						const data = typeof ev.data === "string" ? ev.data : "";
						if (!Number.isFinite(time) || !addr || !data) return null;
						return { time, addr, data };
					})
					.filter((e): e is YmLogEvent => e !== null);
				rebuildAndRender();
			} catch {
				rawEvents = [];
				waveformData = null;
				drawEmpty(ctx2d, W, H, MSG_PARSE_ERROR);
			}
		},
		clear() {
			rawEvents = [];
			waveformData = null;
			drawEmpty(ctx2d, W, H, MSG_INITIAL);
		},
	};
}
