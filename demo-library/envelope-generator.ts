/**
 * YM2151 per-operator envelope generator simulation.
 *
 * Provides the EnvelopeGenerator class and the YM2151 pitch-to-frequency
 * conversion used by the waveform simulator.
 */

import { NOTE_CODE_TO_SEMITONE } from "./ym2151-utils";

/** YM2151 internal sample rate (55.93 kHz, per the OPM datasheet §4). */
export const YM_SAMPLE_RATE = 55930;

/** Number of FM operators per YM2151 channel. */
export const NUM_OPERATORS = 4;

/**
 * Convert YM2151 KC and KF register values to a frequency in Hz.
 * Reference: C#4 = 277.18 Hz at linear semitone 48 (octave 4, note code 0).
 */
export function kcToFrequency(kc: number, kf: number): number {
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
export function ampStepPerSample(effectiveRate: number): number {
	if (effectiveRate <= 0) return 0;
	const r = Math.min(62, effectiveRate);
	const traversalSeconds = 0.000086 * Math.pow(2, 62 - r);
	return 1.0 / (traversalSeconds * YM_SAMPLE_RATE);
}

type EnvelopePhase = "off" | "attack" | "decay1" | "decay2" | "release";

/**
 * Simplified YM2151 per-operator envelope generator.
 * amplitude is NOT reset on key-on by design: if the previous note's release
 * phase left a non-zero amplitude, the new attack starts from that level,
 * creating a discontinuity that manifests as pop-noise in the audio output.
 */
export class EnvelopeGenerator {
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
