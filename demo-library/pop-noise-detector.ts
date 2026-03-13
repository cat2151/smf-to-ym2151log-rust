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
	// Track the last non-zero sample to handle runs of exact zeros.
	let lastNonZero = 0;
	for (let i = 0; i + 1 < samples.length; i++) {
		const a = samples[i] as number;
		const b = samples[i + 1] as number;
		// Use the last non-zero value when a sample is exactly 0 so that
		// discontinuities landing on an exact zero are still detected.
		const aEff = a !== 0 ? a : lastNonZero;
		// Zero crossing: transition from one side of zero to the other.
		// aEff===0 means we are still in a leading silence; skip until signal starts.
		if (
			aEff !== 0 &&
			((aEff < 0 && b > 0) ||
				(aEff > 0 && b < 0) ||
				(aEff !== 0 &&
					b === 0 &&
					i + 2 < samples.length &&
					(samples[i + 2] as number) * aEff < 0))
		) {
			const magnitude = Math.abs(b - aEff);
			if (magnitude >= threshold) {
				markers.push({ time: i / sampleRate, magnitude });
			}
		}
		if (a !== 0) lastNonZero = a;
	}
	return markers;
}
