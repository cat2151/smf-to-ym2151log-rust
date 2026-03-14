/**
 * Waveform viewer for pop-noise visualization.
 *
 * Renders the actual PCM audio produced by web-ym2151 so the canvas
 * faithfully reflects the conversion result. Note-boundary markers show
 * key-on events for the selected channel, making amplitude discontinuities
 * (pop-noise) visible.
 *
 * Internal responsibilities are split across:
 *   - waveform-canvas.ts : canvas drawing (drawEmpty / drawWaveform)
 *   - shared-demo.ts     : WebYmApi (generateAudioFromJson / freeAudioBuffer)
 */

import { type WaveformData } from "./waveform-canvas";
import type { WebYmApi } from "./shared-demo";
import { drawEmpty, drawWaveform } from "./waveform-canvas";
import { downloadWav } from "./wav-exporter";
import {
	PIXELS_PER_SECOND,
	parseHexByte,
	OPM_SAMPLE_RATE,
} from "./ym2151-utils";
import { detectPopNoise } from "./pop-noise-detector";

// --- Constants ---

/** Normalize waveform peak amplitude to this fraction of the display range. */
const NORMALIZE_AMPLITUDE = 0.95;

// --- UI string constants ---
const MSG_INITIAL = "YM2151 ログを変換するとここに描画します。";
const MSG_EMPTY = "変換結果がまだありません。";
const MSG_LOADING = "波形を生成中... (web-ym2151)";
const MSG_PARSE_ERROR = "ログ JSON を解析できませんでした。";

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
	exportWav: (filename: string) => void;
};

/**
 * Extract key-on timestamps for a specific YM2151 channel from a parsed
 * events array. Used to mark note boundaries on the canvas without running
 * the audio synthesizer.
 */
function extractNoteBoundaries(
	events: Array<{ time: number; addr: string; data: string }>,
	ch: number,
): number[] {
	const boundaries: number[] = [];
	for (const ev of events) {
		const addr = parseHexByte(ev.addr);
		const data = parseHexByte(ev.data);
		if (addr === null || data === null) continue;
		if (addr === 0x08) {
			const evCh = data & 0x07;
			const ops = (data >> 3) & 0x0f;
			if (evCh === ch && ops !== 0) {
				boundaries.push(ev.time);
			}
		}
	}
	return boundaries;
}

/**
 * Scale PCM samples so the peak absolute value equals targetPeak.
 * Returns a new Float32Array; original is not modified.
 * If the peak is zero (silence), the original array is returned unchanged.
 */
function normalizeAmplitude(
	samples: Float32Array,
	targetPeak: number,
): Float32Array {
	let peak = 0;
	for (let i = 0; i < samples.length; i++) {
		const abs = Math.abs(samples[i]);
		if (abs > peak) peak = abs;
	}
	if (peak === 0) return samples;
	const scale = targetPeak / peak;
	const normalized = new Float32Array(samples.length);
	for (let i = 0; i < samples.length; i++) {
		normalized[i] = samples[i] * scale;
	}
	return normalized;
}

export function createWaveformViewer(
	canvas: HTMLCanvasElement | null,
	controls: WaveformViewerControls,
	webYmApiPromise: Promise<WebYmApi>,
): WaveformViewer {
	if (!canvas)
		return { renderFromJson: () => {}, clear: () => {}, exportWav: () => {} };
	const ctx = canvas.getContext("2d");
	if (!ctx)
		return { renderFromJson: () => {}, clear: () => {}, exportWav: () => {} };

	// ctx is guaranteed non-null from here on; alias as a non-nullable type.
	const ctx2d: CanvasRenderingContext2D = ctx;
	const W = canvas.width;
	const H = canvas.height;

	let waveformData: WaveformData | null = null;
	// Parsed events array, retained so channel changes can update boundaries
	// without re-running the synthesizer.
	let parsedEvents: Array<{ time: number; addr: string; data: string }> = [];
	// Raw JSON text retained for re-synthesis if needed.
	let currentJsonText: string | null = null;
	let viewStart = 0;
	// Initial zoom=1 matches slider value="0" (2000^0=1) and label "1x" in HTML.
	let zoom = 1;
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
		return W / (PIXELS_PER_SECOND * zoom);
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

	function updateBoundariesAndRender(): void {
		if (waveformData) {
			waveformData.noteBoundaries = extractNoteBoundaries(
				parsedEvents,
				selectedChannel,
			);
		}
		render();
	}

	async function synthesizeAndRender(): Promise<void> {
		if (!currentJsonText) {
			waveformData = null;
			render();
			return;
		}
		// Show loading placeholder while waiting for synthesis.
		drawEmpty(ctx2d, W, H, MSG_LOADING);

		try {
			const api = await webYmApiPromise;
			const audioData = api.generateAudioFromJson(currentJsonText);
			if (!audioData) {
				waveformData = null;
				render();
				return;
			}

			waveformData = {
				waveformSamples: normalizeAmplitude(
					audioData.left,
					NORMALIZE_AMPLITUDE,
				),
				sampleRate: OPM_SAMPLE_RATE,
				durationS: audioData.duration,
				noteBoundaries: extractNoteBoundaries(parsedEvents, selectedChannel),
				popNoiseMarkers: detectPopNoise(audioData.left, OPM_SAMPLE_RATE),
			};

			// Release the WASM-side buffer; the Float32Array copy remains valid.
			api.freeAudioBuffer();

			// Auto-scroll to the first note boundary for the selected channel.
			if (waveformData.noteBoundaries.length > 0) {
				const firstNote = waveformData.noteBoundaries[0];
				viewStart = clampViewStart(firstNote - getWindowDurS() * 0.3);
			} else {
				viewStart = 0;
			}
			render();
		} catch {
			waveformData = null;
			render();
		}
	}

	function setZoom(newZoom: number, anchorFraction = 0.5): void {
		const oldWindowDur = getWindowDurS();
		zoom = Math.max(1, Math.min(2000, newZoom));
		const newWindowDur = getWindowDurS();
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

	// Channel selector: update which channel's key-ons are highlighted.
	// The waveform itself is always the full mixed audio; no re-synthesis needed.
	if (channelSelect) {
		channelSelect.addEventListener("change", () => {
			selectedChannel = Number(channelSelect.value);
			updateBoundariesAndRender();
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
		const pixPerSec = PIXELS_PER_SECOND * zoom;
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
				currentJsonText = null;
				parsedEvents = [];
				waveformData = null;
				drawEmpty(ctx2d, W, H, MSG_EMPTY);
				return;
			}
			try {
				const parsed = JSON.parse(jsonText) as unknown;
				if (!parsed || typeof parsed !== "object") {
					currentJsonText = null;
					parsedEvents = [];
					waveformData = null;
					render();
					return;
				}
				const arr = (parsed as { events?: unknown }).events;
				if (!Array.isArray(arr)) {
					currentJsonText = null;
					parsedEvents = [];
					waveformData = null;
					render();
					return;
				}
				parsedEvents = arr
					.map((e) => {
						if (!e || typeof e !== "object") return null;
						const ev = e as { time?: unknown; addr?: unknown; data?: unknown };
						const time = typeof ev.time === "number" ? ev.time : Number.NaN;
						const addr = typeof ev.addr === "string" ? ev.addr : "";
						const data = typeof ev.data === "string" ? ev.data : "";
						if (!Number.isFinite(time) || !addr || !data) return null;
						return { time, addr, data };
					})
					.filter(
						(e): e is { time: number; addr: string; data: string } =>
							e !== null,
					);
				currentJsonText = jsonText;
				void synthesizeAndRender();
			} catch {
				currentJsonText = null;
				parsedEvents = [];
				waveformData = null;
				drawEmpty(ctx2d, W, H, MSG_PARSE_ERROR);
			}
		},
		clear() {
			currentJsonText = null;
			parsedEvents = [];
			waveformData = null;
			drawEmpty(ctx2d, W, H, MSG_INITIAL);
		},
		exportWav(filename: string) {
			if (!waveformData || waveformData.waveformSamples.length === 0) return;
			downloadWav(
				waveformData.waveformSamples,
				waveformData.sampleRate,
				filename,
			);
		},
	};
}
