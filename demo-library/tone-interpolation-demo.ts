import "./style.css";

import { smf_to_ym2151_json_with_attachment } from "smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js";
import {
	ensureWasmInitialized,
	ensureWebYm2151,
	setEventCountDisplay,
	setStatus,
	updateOutput,
} from "./shared-demo";
import { normalizeAttachmentText } from "./tone-json-attachment";
import { setupMmlToSmf } from "./mml-support";
import { createLogVisualizer } from "./log-visualizer";

/** URL of the ym2151-tone-editor WASM library used for random tone generation. */
const YM2151_TONE_EDITOR_WASM_URL =
	"https://cat2151.github.io/ym2151-tone-editor/demo-library/pkg/ym2151_wasm.js";

/** MIDI note number used when generating random tones (A4 = 69). */
const DEFAULT_MIDI_NOTE_FOR_RANDOM = 69;

/**
 * Fallback default attachment in compact nibble format.
 *
 * Program 0: Modulator TL = 0x10 (bright, rich harmonics)
 * Program 1: Modulator TL = 0x7F (dark, pure sine-like)
 * Used when the ym2151-tone-editor WASM cannot be loaded.
 */
const DEFAULT_COMPACT_ATTACHMENT = `[
  {
    "ProgramChange": 0,
    "ChangeToNextTone": true,
    "ChangeToNextToneTime": 10,
    "registers": "20C760006810801F881FE00FE80F"
  },
  {
    "ProgramChange": 1,
    "registers": "20C76000687F801F881FE00FE80F"
  }
]`;

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
				// Reset so the next call can retry (handles transient network errors).
				toneEditorInitPromise = null;
				throw e;
			}
		})();
	}
	return toneEditorInitPromise;
}

/**
 * Generate a compact nibble attachment JSON string with two random tones.
 * Uses the ym2151-tone-editor WASM library for random tone generation.
 */
async function buildRandomAttachment(): Promise<string> {
	const generate = await getToneEditorGenerator();
	const seed = Date.now();
	const registers1 = generate(seed, DEFAULT_MIDI_NOTE_FOR_RANDOM);
	// Use a well-separated seed to ensure the second tone is clearly distinct.
	const registers2 = generate(seed + 100000, DEFAULT_MIDI_NOTE_FOR_RANDOM);
	return JSON.stringify(
		[
			{
				ProgramChange: 0,
				ChangeToNextTone: true,
				ChangeToNextToneTime: 10,
				registers: registers1,
			},
			{
				ProgramChange: 1,
				registers: registers2,
			},
		],
		null,
		2,
	);
}

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
) as HTMLPreElement | null;
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
const webYmStatus = document.getElementById("web-ym-status");
const mmlInput = document.getElementById(
	"mml-input",
) as HTMLTextAreaElement | null;
const logVisualizer = createLogVisualizer(
	document.getElementById("log-visualizer"),
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
	if (!attachmentField) return new Uint8Array();
	const raw = attachmentField.value.trim();
	if (raw.length === 0) {
		setStatus(attachmentStatus, "添付 JSON は空です (音色補間無効)");
		return new Uint8Array();
	}
	// Normalize compact nibble to events without modifying the textarea content.
	const normalized = normalizeAttachmentText(raw, attachmentStatus);
	if (normalized === null) return null;
	return new TextEncoder().encode(normalized);
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
	// Set the fallback compact nibble attachment immediately, then try to
	// replace it with randomly generated tones from the ym2151-tone-editor WASM.
	attachmentField.value = DEFAULT_COMPACT_ATTACHMENT;

	buildRandomAttachment()
		.then((attachment) => {
			if (!attachmentField) return;
			// Only replace if the user has not yet edited the field.
			if (attachmentField.value === DEFAULT_COMPACT_ATTACHMENT) {
				attachmentField.value = attachment;
				// Only trigger conversion if MIDI is already loaded.
				if (midiBytes && wasmReady) {
					void runConversion("デフォルト音色 (ランダム生成)");
				}
			}
		})
		.catch(() => {
			// ym2151-tone-editor WASM failed to load; keep the fallback attachment.
		});

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

function bootstrapWebYm(): void {
	setStatus(webYmStatus, "web-ym2151 を準備中...");
	ensureWebYm2151()
		.then(() => {
			setStatus(webYmStatus, "web-ym2151 準備完了");
			updatePlayButtonState();
		})
		.catch((error) => {
			setStatus(
				webYmStatus,
				`web-ym2151 の準備に失敗しました: ${(error as Error).message}`,
				true,
			);
		});
}

function main(): void {
	setupAttachmentEditor();
	setupMidiInput();
	setupMmlInput();
	updateOutputWithState("");
	updatePlayButtonState();
	bootstrapWebYm();
	void initializeWasm();

	if (playButton) {
		playButton.addEventListener("click", () => {
			void handlePlay();
		});
	}
}

document.addEventListener("DOMContentLoaded", main);
