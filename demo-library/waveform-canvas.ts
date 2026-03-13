/**
 * Canvas rendering for the waveform viewer.
 *
 * Provides drawEmpty (placeholder text) and drawWaveform (waveform with
 * note-boundary markers and time labels).
 */

import { PIXELS_PER_SECOND } from "./ym2151-utils";
import type { PopNoiseMarker } from "./pop-noise-detector";

// Pop noise marker visual constants
const POP_NOISE_MARKER_COLOR = "rgba(220, 0, 0, 0.8)";
const POP_NOISE_LABEL_COLOR = "rgba(180, 0, 0, 0.9)";
const POP_NOISE_LABEL_Y = 25;

export type WaveformData = {
	/** PCM waveform samples (left channel from web-ym2151 real synthesis). */
	waveformSamples: Float32Array;
	sampleRate: number;
	durationS: number;
	/** Timestamps (seconds) of key-on events for the selected channel. */
	noteBoundaries: number[];
	/** Pop noise candidates detected by zero-crossing amplitude analysis. */
	popNoiseMarkers: PopNoiseMarker[];
};

/** Render a placeholder message on the canvas (no data loaded yet). */
export function drawEmpty(
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

/** Render the waveform for the visible time window. */
export function drawWaveform(
	ctx: CanvasRenderingContext2D,
	width: number,
	height: number,
	data: WaveformData,
	viewStart: number,
	zoom: number,
): void {
	const pixelsPerSec = PIXELS_PER_SECOND * zoom;
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

	// Compute sample range for visible window (with edge padding for anti-aliasing)
	const viewStartSample = viewStart * data.sampleRate;
	const startSample = Math.max(0, Math.floor(viewStartSample) - 1);
	const endSample = Math.min(
		data.waveformSamples.length - 1,
		Math.ceil(viewEnd * data.sampleRate) + 1,
	);

	if (startSample >= endSample) return;

	// Use the view window duration (not data length) for pixel-to-sample mapping,
	// so waveform positions align correctly with time labels and note-boundary markers.
	// Anchor pixel 0 to viewStartSample exactly (not startSample) to avoid the -1
	// edge-padding offset causing a horizontal shift at high zoom levels.
	const samplesPerPixel = (viewDurationS * data.sampleRate) / width;

	// Draw waveform (blue) using min/max per pixel column for correct anti-aliasing
	ctx.strokeStyle = "#2196F3";
	ctx.lineWidth = 1.5;
	ctx.beginPath();
	let firstWave = true;
	for (let px = 0; px < width; px++) {
		const sStart = Math.max(
			startSample,
			Math.floor(viewStartSample + px * samplesPerPixel),
		);
		const sEnd = Math.min(
			endSample,
			Math.floor(viewStartSample + (px + 1) * samplesPerPixel),
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

	// Pop noise markers drawn after waveform so they always overlay it.
	for (const marker of data.popNoiseMarkers) {
		const t = marker.time;
		if (t < viewStart - 0.001 || t > viewEnd + 0.001) continue;
		const x = Math.round((t - viewStart) * pixelsPerSec);
		ctx.strokeStyle = POP_NOISE_MARKER_COLOR;
		ctx.lineWidth = 2;
		ctx.beginPath();
		ctx.moveTo(x, 0);
		ctx.lineTo(x, height);
		ctx.stroke();
		ctx.fillStyle = POP_NOISE_LABEL_COLOR;
		ctx.font = "10px monospace";
		ctx.fillText(
			`pop ${(marker.magnitude * 100).toFixed(0)}%`,
			x + 3,
			POP_NOISE_LABEL_Y,
		);
	}

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
	ctx.fillStyle = POP_NOISE_MARKER_COLOR;
	ctx.fillRect(70, height - 28, 14, 3);
	ctx.fillStyle = "#555";
	ctx.fillText("ポップノイズ", 88, height - 24);
}
