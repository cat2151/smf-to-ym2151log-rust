import "./style.css";

import { smf_to_ym2151_json_with_attachment } from "smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js";
import {
	ensureWasmInitialized,
	ensureWebYm2151,
	parseAttachmentField,
	setEventCountDisplay,
	setStatus,
	updateOutput,
} from "./shared-demo";
import { setupMmlToSmf } from "./mml-support";
import { createLogVisualizer } from "./log-visualizer";
import { createWaveformViewer } from "./waveform-viewer";

const DEFAULT_ATTACHMENT = `[
  {
    "ProgramChange": 0,
    "PopNoiseEnvelope": {
      "Enabled": true,
      "OffsetSeconds": 0.002,
      "Registers": [
        { "BaseRegister": "0xE0", "Value": "0x0F" },
        { "BaseRegister": "0xE8", "Value": "0x0F" },
        { "BaseRegister": "0xF0", "Value": "0x0F" },
        { "BaseRegister": "0xF8", "Value": "0x0F" }
      ]
    }
  }
]`;

let wasmReady = false;
let midiBytes: Uint8Array | null = null;
let currentOutput: string | null = null;
let attachmentDebounce: number | null = null;
let lastMidiSource: "file" | "mml" | null = null;
let latestMidiRequestId = 0;
let latestAutoPlayId = 0;

const attachmentField = document.getElementById(
	"attachment-json",
) as HTMLTextAreaElement | null;
const conversionOutput = document.getElementById(
	"conversion-output",
) as HTMLTextAreaElement | null;
const conversionStatus = document.getElementById("conversion-status");
const attachmentStatus = document.getElementById("attachment-status");
const fileStatus = document.getElementById("file-status");
const mmlStatus = document.getElementById("mml-status");
const eventCount = document.getElementById("event-count");
const jsonEditor = document.getElementById(
	"jsonEditor",
) as HTMLTextAreaElement | null;
const playButton = document.getElementById(
	"play-audio",
) as HTMLButtonElement | null;
const mmlInput = document.getElementById(
	"mml-input",
) as HTMLTextAreaElement | null;
const logVisualizer = createLogVisualizer(
	document.getElementById("log-visualizer"),
);
const waveformViewer = createWaveformViewer(
	document.getElementById("waveform-canvas") as HTMLCanvasElement | null,
	{
		zoomSlider: document.getElementById("wv-zoom") as HTMLInputElement | null,
		zoomLabel: document.getElementById("wv-zoom-label"),
		prevNoteBtn: document.getElementById(
			"wv-prev-note",
		) as HTMLButtonElement | null,
		nextNoteBtn: document.getElementById(
			"wv-next-note",
		) as HTMLButtonElement | null,
		channelSelect: document.getElementById(
			"wv-channel",
		) as HTMLSelectElement | null,
		positionLabel: document.getElementById("wv-position"),
	},
	ensureWebYm2151(),
);

function nextRequestId(): number {
	latestMidiRequestId += 1;
	return latestMidiRequestId;
}

function isLatestRequest(id: number): boolean {
	return id === latestMidiRequestId;
}

function updateOutputWithState(text: string): void {
	currentOutput = text;
	updateOutput(text, conversionOutput, jsonEditor, () => {
		logVisualizer.renderFromJson(text);
		waveformViewer.renderFromJson(text);
		updatePlayButtonState();
	});
}

function updatePlayButtonState(): void {
	if (!playButton) return;
	playButton.disabled = !currentOutput;
}

async function initializeWasm(): Promise<void> {
	wasmReady = await ensureWasmInitialized(
		(message, isError) => setStatus(conversionStatus, message, isError),
		"WASM 初期化完了。MIDI を読み込んでください。",
	);
}

function readAttachmentBytes(): Uint8Array | null {
	return parseAttachmentField(
		attachmentField,
		attachmentStatus,
		"添付 JSON は空です (ポップノイズ対策なし)",
		"添付 JSON を適用します",
	);
}

async function runConversion(trigger: string): Promise<void> {
	if (!wasmReady) {
		setStatus(conversionStatus, "WASM 初期化中です。少しお待ちください...");
		return;
	}
	if (!midiBytes) {
		setStatus(
			conversionStatus,
			"MIDI ファイルを選択するか、MML を入力してください。",
			true,
		);
		return;
	}

	const attachmentBytes = readAttachmentBytes();
	if (attachmentBytes === null) {
		updatePlayButtonState();
		return;
	}

	try {
		const triggerLabel =
			lastMidiSource === "mml"
				? `${trigger} (MML 入力)`
				: lastMidiSource === "file"
					? `${trigger} (SMF ファイル)`
					: trigger;
		setStatus(conversionStatus, `変換中... (${triggerLabel})`);
		const result = smf_to_ym2151_json_with_attachment(
			midiBytes,
			attachmentBytes,
		);
		const parsed = JSON.parse(result);
		const formatted = JSON.stringify(parsed, null, 2);
		setEventCountDisplay(
			eventCount,
			typeof parsed.event_count === "number" ? parsed.event_count : undefined,
		);
		updateOutputWithState(formatted);
		setStatus(conversionStatus, "変換が完了しました。");
		void handlePlay(++latestAutoPlayId);
	} catch (error) {
		updateOutputWithState("");
		setEventCountDisplay(eventCount, undefined);
		setStatus(
			conversionStatus,
			`変換に失敗しました: ${(error as Error).message}`,
			true,
		);
	}
}

async function handlePlay(autoPlayId?: number): Promise<void> {
	if (!currentOutput) {
		setStatus(conversionStatus, "先に SMF を変換してください。", true);
		return;
	}
	setStatus(conversionStatus, "web-ym2151 で再生します...");
	try {
		const api = await ensureWebYm2151();
		if (autoPlayId !== undefined && autoPlayId !== latestAutoPlayId) {
			return;
		}
		api.playAudioWithOverlay();
		setStatus(conversionStatus, "再生コマンドを送信しました。");
	} catch (error) {
		setStatus(
			conversionStatus,
			`再生に失敗しました: ${(error as Error).message}`,
			true,
		);
	}
}

function setupAttachmentEditor(): void {
	if (!attachmentField) return;
	attachmentField.value = DEFAULT_ATTACHMENT;
	attachmentField.addEventListener("input", () => {
		if (attachmentDebounce) {
			window.clearTimeout(attachmentDebounce);
		}
		attachmentDebounce = window.setTimeout(() => {
			void runConversion("添付 JSON 更新");
		}, 400);
	});
}

function setupMmlInput(): void {
	setupMmlToSmf({
		mmlInput,
		mmlStatus,
		fileStatus,
		nextRequestId,
		isLatestRequest,
		onMidiReady: (bytes) => {
			midiBytes = bytes;
			lastMidiSource = "mml";
		},
		onClear: () => {
			if (lastMidiSource === "mml") {
				midiBytes = null;
				lastMidiSource = null;
			}
		},
		onAfterConvert: (trigger) => {
			void runConversion(trigger);
		},
	});
}

function setupMidiInput(): void {
	const midiInput = document.getElementById(
		"midi-input",
	) as HTMLInputElement | null;
	if (!midiInput) return;

	midiInput.addEventListener("change", async (event) => {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (!file) {
			nextRequestId();
			midiBytes = null;
			lastMidiSource = null;
			updateOutputWithState("");
			setEventCountDisplay(eventCount, undefined);
			setStatus(
				fileStatus,
				"SMF ファイルを選択するか、MML を入力してください。",
			);
			updatePlayButtonState();
			return;
		}

		const requestId = nextRequestId();
		setStatus(fileStatus, `${file.name} を読み込み中...`);
		try {
			const arrayBuffer = await file.arrayBuffer();
			if (!isLatestRequest(requestId)) {
				return;
			}
			midiBytes = new Uint8Array(arrayBuffer);
			lastMidiSource = "file";
			setStatus(
				fileStatus,
				`${file.name} を読み込みました (${midiBytes.byteLength} bytes)`,
			);
			void runConversion("MIDI 更新");
		} catch (error) {
			midiBytes = null;
			lastMidiSource = null;
			setStatus(
				fileStatus,
				`読み込みに失敗しました: ${(error as Error).message}`,
				true,
			);
		}
	});
}

function setupPlayButton(): void {
	if (!playButton) return;
	playButton.addEventListener("click", () => {
		void handlePlay();
	});
}

function setupWavExportButton(): void {
	const wavExportBtn = document.getElementById(
		"wv-export-wav",
	) as HTMLButtonElement | null;
	if (!wavExportBtn) return;
	wavExportBtn.addEventListener("click", () => {
		waveformViewer.exportWav("waveform.wav");
	});
}

/** URL of the ym2151-tone-editor WASM library used for random tone generation. */
const YM2151_TONE_EDITOR_WASM_URL =
	"https://cat2151.github.io/ym2151-tone-editor/demo-library/pkg/ym2151_wasm.js";

/** MIDI note number used when generating random tones (A4 = 69). */
const DEFAULT_MIDI_NOTE_FOR_RANDOM = 69;

/** Cached promise that resolves to the generate_random_tone_registers function. */
let toneEditorInitPromise: Promise<
	(seed: number, midiNote: number) => string
> | null = null;

/** Load the ym2151-tone-editor WASM once and return the generation function. */
function getToneEditorGenerator(): Promise<
	(seed: number, midiNote: number) => string
> {
	if (!toneEditorInitPromise) {
		toneEditorInitPromise = (async () => {
			try {
				const mod = await import(
					/* @vite-ignore */ YM2151_TONE_EDITOR_WASM_URL
				);
				await mod.default();
				return mod.generate_random_tone_registers as (
					seed: number,
					midiNote: number,
				) => string;
			} catch (e) {
				toneEditorInitPromise = null;
				throw e;
			}
		})();
	}
	return toneEditorInitPromise;
}

async function applyRandomToneToAttachment(): Promise<void> {
	if (!attachmentField) return;

	let entries: Array<Record<string, unknown>>;
	try {
		const parsed = JSON.parse(attachmentField.value);
		if (!Array.isArray(parsed)) {
			setStatus(
				attachmentStatus,
				"JSON が配列ではありません。ランダム音色を適用できません。",
				true,
			);
			return;
		}
		entries = parsed;
	} catch {
		setStatus(
			attachmentStatus,
			"JSON が不正なためランダム音色を適用できません。",
			true,
		);
		return;
	}

	let registers: string;
	try {
		const generate = await getToneEditorGenerator();
		registers = generate(Date.now(), DEFAULT_MIDI_NOTE_FOR_RANDOM);
	} catch {
		setStatus(
			attachmentStatus,
			"ym2151-tone-editor の読み込みに失敗しました。ランダム音色を適用できません。",
			true,
		);
		return;
	}

	const entryIndex = entries.findIndex(
		(e) => (e as { ProgramChange?: number }).ProgramChange === 0,
	);
	const baseEntry: Record<string, unknown> =
		entryIndex >= 0 ? { ...entries[entryIndex] } : { ProgramChange: 0 };

	delete baseEntry.Tone;
	baseEntry.registers = registers;

	if (entryIndex >= 0) {
		entries[entryIndex] = baseEntry;
	} else {
		entries.unshift(baseEntry);
	}

	attachmentField.value = JSON.stringify(entries, null, 2);
	void runConversion("ランダム音色");
}

function setupRandomToneButton(): void {
	const randomToneBtn = document.getElementById(
		"random-tone",
	) as HTMLButtonElement | null;
	if (!randomToneBtn) return;
	randomToneBtn.addEventListener("click", () => {
		void applyRandomToneToAttachment();
	});
}

function bootstrap(): void {
	void initializeWasm();
	setupAttachmentEditor();
	setupMidiInput();
	setupPlayButton();
	setupMmlInput();
	setupWavExportButton();
	setupRandomToneButton();
}

bootstrap();
