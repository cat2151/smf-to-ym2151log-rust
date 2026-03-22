/** URL of the ym2151-tone-editor WASM library used for random tone generation. */
const YM2151_TONE_EDITOR_WASM_URL =
	"https://cat2151.github.io/ym2151-tone-editor/demo-library/pkg/ym2151_wasm.js";

/** MIDI note number used when generating random tones (A4 = 69). */
const DEFAULT_MIDI_NOTE_FOR_RANDOM = 69;

type RandomToneGenerator = (seed: number, midiNote: number) => string;

let toneEditorInitPromise: Promise<RandomToneGenerator> | null = null;

/** Load the ym2151-tone-editor WASM once and return the generation function. */
function getToneEditorGenerator(): Promise<RandomToneGenerator> {
	if (!toneEditorInitPromise) {
		toneEditorInitPromise = (async () => {
			try {
				const mod = await import(
					/* @vite-ignore */ YM2151_TONE_EDITOR_WASM_URL
				);
				await mod.default();
				return mod.generate_random_tone_registers as RandomToneGenerator;
			} catch (error) {
				toneEditorInitPromise = null;
				throw error;
			}
		})();
	}
	return toneEditorInitPromise;
}

/** Generate compact YM2151 tone registers using the random tone editor. */
export async function generateRandomToneRegisters(
	seed = Date.now(),
	midiNote = DEFAULT_MIDI_NOTE_FOR_RANDOM,
): Promise<string> {
	const generate = await getToneEditorGenerator();
	return generate(seed, midiNote);
}

/** Generate a distinct pair of compact YM2151 tone registers for interpolation. */
export async function generateRandomInterpolationPairRegisters(
	seed = Date.now(),
): Promise<[string, string]> {
	const generate = await getToneEditorGenerator();
	return [
		generate(seed, DEFAULT_MIDI_NOTE_FOR_RANDOM),
		generate(seed + 100000, DEFAULT_MIDI_NOTE_FOR_RANDOM),
	];
}

function parseAttachmentEntries(rawAttachment: string): Array<Record<string, unknown>> {
	let parsed: unknown;
	try {
		parsed = JSON.parse(rawAttachment);
	} catch {
		throw new Error("JSON が不正なためランダム音色を適用できません。");
	}
	if (!Array.isArray(parsed)) {
		throw new Error("JSON が配列ではありません。ランダム音色を適用できません。");
	}

	return parsed as Array<Record<string, unknown>>;
}

/**
 * Replace or insert the compact `registers` field for one ProgramChange entry.
 * Existing fields are preserved so the demos keep their attachment-specific
 * settings and user-edited JSON shape. `normalizeAttachmentText()` will still
 * prefer the injected compact registers during conversion, so keeping `Tone`
 * here avoids clobbering the textarea content without changing runtime behavior.
 */
function upsertEntryRegisters(
	entries: Array<Record<string, unknown>>,
	registers: string,
	programChange: number,
	insertIndex: number,
	defaultEntry: Record<string, unknown>,
): void {
	const entryIndex = entries.findIndex(
		(entry) => (entry as { ProgramChange?: number }).ProgramChange === programChange,
	);
	const baseEntry: Record<string, unknown> =
		entryIndex >= 0
			? { ...entries[entryIndex] }
			: { ...defaultEntry };

	// Use the requested insertion position only when the ProgramChange entry is
	// missing, so newly created interpolation pairs still land in a stable order.
	baseEntry.registers = registers;

	if (entryIndex >= 0) {
		entries[entryIndex] = baseEntry;
	} else {
		entries.splice(insertIndex, 0, baseEntry);
	}
}

/** Replace or insert compact tone registers for the selected ProgramChange entry. */
export function upsertAttachmentRegisters(
	rawAttachment: string,
	registers: string,
	programChange = 0,
): string {
	const entries = parseAttachmentEntries(rawAttachment);
	upsertEntryRegisters(entries, registers, programChange, 0, {
		ProgramChange: programChange,
	});

	return JSON.stringify(entries, null, 2);
}

/**
 * Replace or insert compact tone registers for the interpolation pair while
 * preserving the rest of the existing attachment JSON structure.
 */
export function upsertInterpolationAttachmentRegisters(
	rawAttachment: string,
	firstRegisters: string,
	secondRegisters: string,
): string {
	const entries = parseAttachmentEntries(rawAttachment);
	upsertEntryRegisters(entries, firstRegisters, 0, 0, {
		ProgramChange: 0,
		ChangeToNextTone: true,
		ChangeToNextToneTime: 10,
	});
	upsertEntryRegisters(entries, secondRegisters, 1, 1, {
		ProgramChange: 1,
	});
	return JSON.stringify(entries, null, 2);
}

/** Build a two-tone ChangeToNextTone attachment with distinct random tones. */
export async function buildRandomInterpolationAttachment(): Promise<string> {
	const generate = await getToneEditorGenerator();
	const seed = Date.now();
	return JSON.stringify(
		[
			{
				ProgramChange: 0,
				ChangeToNextTone: true,
				ChangeToNextToneTime: 10,
				registers: generate(seed, DEFAULT_MIDI_NOTE_FOR_RANDOM),
			},
			{
				ProgramChange: 1,
				registers: generate(seed + 100000, DEFAULT_MIDI_NOTE_FOR_RANDOM),
			},
		],
		null,
		2,
	);
}
