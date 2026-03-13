/**
 * Pop noise detector for YM2151 waveform analysis.
 *
 * Detects the most likely pop noise by finding the zero crossing with the
 * largest amplitude jump in the PCM audio.
 */

export type PopNoiseMarker = {
	/** Time in seconds where the pop noise zero crossing occurs. */
	time: number;
	/** Magnitude of the amplitude jump at this zero crossing (range 0–2). */
	magnitude: number;
};

/**
 * Detect the single most prominent pop noise candidate in PCM samples.
 *
 * Algorithm:
 *   1. Scan all zero crossings (sign changes between consecutive samples).
 *   2. For each zero crossing compute the amplitude jump magnitude.
 *   3. Return only the one zero crossing with the largest magnitude.
 *
 * @param samples    Normalized PCM samples (Float32Array from web-ym2151).
 * @param sampleRate Sample rate in Hz (e.g. 55930 for OPM).
 */
export function detectPopNoise(
	samples: Float32Array,
	sampleRate: number,
): PopNoiseMarker[] {
	let bestIndex = -1;
	let bestMagnitude = 0;
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
				(b === 0 &&
					i + 2 < samples.length &&
					(samples[i + 2] as number) * aEff < 0))
		) {
			const magnitude = Math.abs(b - aEff);
			if (magnitude > bestMagnitude) {
				bestMagnitude = magnitude;
				bestIndex = i;
			}
		}
		if (a !== 0) lastNonZero = a;
	}
	if (bestIndex < 0) return [];
	return [{ time: bestIndex / sampleRate, magnitude: bestMagnitude }];
}
