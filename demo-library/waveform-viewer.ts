/**
 * Waveform viewer for pop-noise visualization.
 *
 * Orchestrates the envelope simulator, canvas renderer, and UI controls.
 * Simulates the YM2151 envelope generator (ADSR per operator) and renders
 * a canvas waveform so users can scroll to note boundaries, zoom to a single
 * waveform period, and visually confirm whether pop-noise has been reduced.
 *
 * Internal responsibilities are split across:
 *   - envelope-generator.ts  : YM2151 ADSR simulation
 *   - waveform-simulator.ts  : per-channel sample production
 *   - waveform-canvas.ts     : canvas drawing (drawEmpty / drawWaveform)
 */

import { type YmLogEvent, PIXELS_PER_SECOND } from "./ym2151-utils";
import { type WaveformData, simulateWaveform } from "./waveform-simulator";
import { drawEmpty, drawWaveform } from "./waveform-canvas";
import { downloadWav } from "./wav-exporter";

// --- UI string constants ---
const MSG_INITIAL = "YM2151 ログを変換するとここに描画します。";
const MSG_EMPTY = "変換結果がまだありません。";
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

export function createWaveformViewer(
	canvas: HTMLCanvasElement | null,
	controls: WaveformViewerControls,
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
	let rawEvents: YmLogEvent[] = [];
	let viewStart = 0; // seconds from start of log
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
