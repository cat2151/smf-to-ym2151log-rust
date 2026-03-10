/**
 * WAV file export utility.
 *
 * Encodes a Float32Array of audio samples into a 16-bit mono PCM WAV Blob
 * and triggers a browser download.
 */

/**
 * Encode Float32Array samples as a 16-bit mono PCM WAV Blob.
 * Samples should be in the range [-1, 1].
 */
export function encodeWav(
	samples: Float32Array,
	sampleRate: number,
): ArrayBuffer {
	const numSamples = samples.length;
	const bytesPerSample = 2; // 16-bit PCM
	const numChannels = 1;
	const dataSize = numSamples * bytesPerSample * numChannels;
	const headerSize = 44;
	const buffer = new ArrayBuffer(headerSize + dataSize);
	const view = new DataView(buffer);

	// RIFF chunk descriptor
	writeAscii(view, 0, "RIFF");
	view.setUint32(4, headerSize - 8 + dataSize, true);
	writeAscii(view, 8, "WAVE");

	// fmt sub-chunk
	writeAscii(view, 12, "fmt ");
	view.setUint32(16, 16, true); // Sub-chunk size: 16 for PCM
	view.setUint16(20, 1, true); // Audio format: 1 = PCM
	view.setUint16(22, numChannels, true);
	view.setUint32(24, sampleRate, true);
	view.setUint32(28, sampleRate * numChannels * bytesPerSample, true); // Byte rate
	view.setUint16(32, numChannels * bytesPerSample, true); // Block align
	view.setUint16(34, 8 * bytesPerSample, true); // Bits per sample

	// data sub-chunk
	writeAscii(view, 36, "data");
	view.setUint32(40, dataSize, true);

	// PCM samples (clamp and convert Float32 to Int16)
	let offset = 44;
	for (let i = 0; i < numSamples; i++) {
		const clamped = Math.max(-1, Math.min(1, samples[i]));
		const int16 = clamped < 0 ? clamped * 32768 : clamped * 32767;
		view.setInt16(offset, Math.round(int16), true);
		offset += 2;
	}

	return buffer;
}

function writeAscii(view: DataView, offset: number, text: string): void {
	for (let i = 0; i < text.length; i++) {
		view.setUint8(offset + i, text.charCodeAt(i));
	}
}

/**
 * Encode samples as WAV and trigger a browser download.
 */
export function downloadWav(
	samples: Float32Array,
	sampleRate: number,
	filename: string,
): void {
	const buffer = encodeWav(samples, sampleRate);
	const blob = new Blob([buffer], { type: "audio/wav" });
	const url = URL.createObjectURL(blob);
	const a = document.createElement("a");
	a.href = url;
	a.download = filename;
	a.click();
	setTimeout(() => URL.revokeObjectURL(url), 100);
}
