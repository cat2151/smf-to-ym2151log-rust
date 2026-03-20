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

/** Replace or insert compact tone registers for the selected ProgramChange entry. */
export function upsertAttachmentRegisters(
	rawAttachment: string,
	registers: string,
	programChange = 0,
): string {
	const parsed = JSON.parse(rawAttachment);
	if (!Array.isArray(parsed)) {
		throw new Error("JSON が配列ではありません。ランダム音色を適用できません。");
	}

	const entries = parsed as Array<Record<string, unknown>>;
	const entryIndex = entries.findIndex(
		(entry) => (entry as { ProgramChange?: number }).ProgramChange === programChange,
	);
	const baseEntry: Record<string, unknown> =
		entryIndex >= 0
			? { ...entries[entryIndex] }
			: { ProgramChange: programChange };

	delete baseEntry.Tone;
	baseEntry.registers = registers;

	if (entryIndex >= 0) {
		entries[entryIndex] = baseEntry;
	} else {
		entries.unshift(baseEntry);
	}

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
