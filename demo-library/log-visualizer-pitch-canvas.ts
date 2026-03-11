import {
	NOTE_BAR_HEIGHT,
	PIXELS_PER_SECOND,
	TRACK_HEIGHT,
} from "./ym2151-utils";
import {
	type NoteSegment,
	noteYPosition,
} from "./log-visualizer-note-segments";

/**
 * Render note segments for one channel onto a canvas overlay.
 *
 * Iterates one pixel column at a time.  For each column the bar is extended
 * vertically to cover the gap between the previous column's y-position and
 * the current one.  This makes delay-vibrato look like a connected line graph
 * instead of disconnected dots, while static (non-vibrato) notes still look
 * like a piano-roll bar.
 */
export function renderPitchCanvas(
	track: HTMLElement,
	trackWidth: number,
	segments: NoteSegment[],
	minPitch: number,
	maxPitch: number,
	color: string,
): void {
	if (segments.length === 0) return;

	// Canvas dimensions must be integers; ceil so we never clip the last pixel.
	const widthPx = Math.ceil(trackWidth);

	const canvas = document.createElement("canvas");
	canvas.width = widthPx;
	canvas.height = TRACK_HEIGHT;
	canvas.style.position = "absolute";
	canvas.style.top = "0";
	canvas.style.left = "0";
	canvas.style.pointerEvents = "none";

	const ctx = canvas.getContext("2d");
	if (!ctx) return;

	// Only append after confirming we have a valid context so no stray empty
	// canvas elements are left in the DOM on failure.
	track.appendChild(canvas);

	ctx.globalAlpha = 0.9;
	ctx.fillStyle = color;

	// Pre-compute pixel ranges once.
	// Sort by start time so the linear scan with `segIdx` is correct:
	// segments for a single YM2151 channel never overlap (the chip is
	// monophonic per channel), so the sorted order guarantees that once
	// we advance past a segment we will never need to revisit it.
	const pixelSegs = segments
		.slice()
		.sort((a, b) => a.startTime - b.startTime)
		.map((s) => ({
			x0: Math.max(0, Math.floor(s.startTime * PIXELS_PER_SECOND)),
			x1: Math.min(widthPx, Math.ceil(s.endTime * PIXELS_PER_SECOND)),
			y: noteYPosition(s.kc, s.kf, minPitch, maxPitch),
		}));

	let prevY: number | null = null;
	let segIdx = 0;

	for (let x = 0; x < widthPx; x++) {
		// Advance past segments that ended before this pixel column.
		// Safe because pixelSegs is sorted by x0/x1 and segments don't overlap.
		while (segIdx < pixelSegs.length && pixelSegs[segIdx].x1 <= x) {
			segIdx++;
		}

		// Find the segment active at x (if any).
		const curSeg =
			segIdx < pixelSegs.length && pixelSegs[segIdx].x0 <= x
				? pixelSegs[segIdx]
				: null;

		if (!curSeg) {
			// A gap between notes resets the connection so we don't draw lines
			// across rests.
			prevY = null;
			continue;
		}

		const curY = curSeg.y;

		// Extend the bar vertically to bridge the gap from the previous column's
		// y-position.  For the very first pixel of a note prevY is null and we
		// draw just the standard bar height.
		const y0 = prevY !== null ? Math.min(curY, prevY) : curY;
		const rawY1 =
			prevY !== null
				? Math.max(curY, prevY) + NOTE_BAR_HEIGHT
				: curY + NOTE_BAR_HEIGHT;

		const drawY0 = Math.round(y0);
		// Always draw at least NOTE_BAR_HEIGHT pixels tall.
		const drawHeight = Math.max(NOTE_BAR_HEIGHT, Math.round(rawY1) - drawY0);

		ctx.fillRect(x, drawY0, 1, drawHeight);
		prevY = curY;
	}
}
