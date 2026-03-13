/**
 * Pop noise detector for YM2151 waveform analysis.
 *
 * Detects potential pop noise by finding zero crossings with large amplitude
 * jumps in the PCM audio.  A zero crossing where the sample-to-sample change
 * magnitude (|sample[i+1] − sample[i]|) exceeds the threshold is flagged as
 * a pop noise candidate.
 */

export type PopNoiseMarker = {
	/** Time in seconds where the pop noise zero crossing occurs. */
	time: number;
	/** Magnitude of the amplitude jump at this zero crossing (range 0–2). */
	magnitude: number;
};

/**
 * Detect pop noise candidates in PCM samples.
 *
 * Algorithm:
 *   1. List all zero crossings (sign changes between consecutive samples).
 *   2. For each zero crossing compute the amplitude jump magnitude.
 *   3. Those whose magnitude ≥ threshold are pop noise candidates.
 *
 * @param samples    Normalized PCM samples (Float32Array from web-ym2151).
 * @param sampleRate Sample rate in Hz (e.g. 55930 for OPM).
 * @param threshold  Minimum jump magnitude to flag (default 0.5).
 *                   Normal smooth zero crossings stay well below this level;
 *                   sudden discontinuities from YM2151 attack-continuation
 *                   pop noise typically produce jumps ≥ 0.5.
 */
export function detectPopNoise(
	samples: Float32Array,
	sampleRate: number,
	threshold = 0.5,
): PopNoiseMarker[] {
	const markers: PopNoiseMarker[] = [];
	for (let i = 0; i + 1 < samples.length; i++) {
		const a = samples[i] as number;
		const b = samples[i + 1] as number;
		// Zero crossing: consecutive samples have opposite non-zero signs.
		if ((a < 0 && b > 0) || (a > 0 && b < 0)) {
			const magnitude = Math.abs(b - a);
			if (magnitude >= threshold) {
				markers.push({ time: i / sampleRate, magnitude });
			}
		}
	}
	return markers;
}
