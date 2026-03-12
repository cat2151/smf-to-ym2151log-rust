/**
 * YM2151 waveform simulation.
 *
 * Replays YM2151 register-write events for a single channel and produces
 * a per-sample waveform array used by the canvas renderer.
 */

import { type YmLogEvent, parseHexByte } from "./ym2151-utils";
import {
	EnvelopeGenerator,
	NUM_OPERATORS,
	YM_SAMPLE_RATE,
	kcToFrequency,
} from "./envelope-generator";

// Maximum simulation length in seconds to keep memory reasonable.
const MAX_SIMULATE_SECONDS = 30;

export type WaveformData = {
	/** Waveform sample (envelope × carrier sine) per sample. */
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
export function simulateWaveform(
	rawEvents: YmLogEvent[],
	ch: number,
): WaveformData {
	const maxTime = rawEvents.reduce((m, e) => Math.max(m, e.time), 0);
	const durationS = Math.min(maxTime + 1.0, MAX_SIMULATE_SECONDS);
	const totalSamples = Math.ceil(durationS * YM_SAMPLE_RATE);

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
			const addr = parseHexByte(ev.addr);
			const data = parseHexByte(ev.data);
			if (addr === null || data === null) continue;

			// Key On/Off: register 0x08
			if (addr === 0x08) {
				const evCh = data & 0x07;
				if (evCh !== ch) continue;
				const ops = (data >> 3) & 0x0f;
				if (ops !== 0) {
					for (const gen of envGens) gen.keyOn();
					noteBoundaries.push(ev.time);
					// Reset oscillator phase on key-on so the carrier sine starts from 0.
					// If the previous note's amplitude is non-zero at this moment, the jump
					// from amplitude×sin(old_phase) to amplitude×sin(0)=0 is the pop-noise
					// click. When pop-noise mitigation (e.g. AttackContinuationFix) brings
					// the amplitude to ~0 before key-on, both sides are ~0 and no click occurs.
					oscPhase = 0;
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

		// Carrier sine wave multiplied by envelope.
		// Phase is sampled first, then incremented, so that oscPhase=0 at key-on
		// causes the very first post-key-on sample to be exactly sin(0)=0.
		waveformSamples[i] = envelope * Math.sin(oscPhase);
		oscPhase += (2 * Math.PI * freq) / YM_SAMPLE_RATE;
		if (oscPhase > 2 * Math.PI) oscPhase -= 2 * Math.PI;
	}

	return {
		waveformSamples,
		sampleRate: YM_SAMPLE_RATE,
		durationS,
		noteBoundaries,
	};
}
