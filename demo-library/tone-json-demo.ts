import "./style.css";

import { smf_to_ym2151_json_with_attachment } from "smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js";
import {
	ensureWasmInitialized,
	ensureWebYm2151,
	setEventCountDisplay,
	setStatus,
	updateOutput,
} from "./shared-demo";
import { createLogVisualizer } from "./log-visualizer";
import {
	ATTACHMENT_PRESETS,
	YM_LOG_STYLE_PRESET,
	normalizeAttachmentText,
} from "./tone-json-attachment";
import {
	ensureMmlRuntime,
	getMmlParser,
	getParseTreeJsonToSmf,
	treeToJson,
} from "./tone-json-mml";

let wasmReady = false;
let midiBytes: Uint8Array | null = null;
let currentOutput: string | null = null;
let attachmentDebounce: number | null = null;
let mmlDebounce: number | null = null;
let latestMidiRequestId = 0;
let latestAutoPlayId = 0;
let lastMidiSource: "file" | "mml" | null = null;

const toneJsonField = document.getElementById(
	"tone-json",
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
const attachmentPresetSelect = document.getElementById(
	"attachment-preset",
) as HTMLSelectElement | null;
const webYmStatus = document.getElementById("web-ym-status");
const mmlInput = document.getElementById(
	"mml-input",
) as HTMLTextAreaElement | null;
const logVisualizer = createLogVisualizer(
	document.getElementById("log-visualizer"),
);

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

async function convertMmlToSmf(trigger: string): Promise<void> {
	if (!mmlInput) return;
	const mmlText = mmlInput.value.trim();
	if (mmlText.length === 0) {
		if (lastMidiSource === "mml") {
			midiBytes = null;
			lastMidiSource = null;
		}
		setStatus(mmlStatus, "MML を入力すると SMF を生成します。");
		return;
	}

	const requestId = ++latestMidiRequestId;
	const initialized = await ensureMmlRuntime(mmlStatus);
	if (!initialized || !getMmlParser() || !getParseTreeJsonToSmf()) {
		return;
	}
	if (requestId !== latestMidiRequestId) {
		return;
	}

	try {
		const parser = getMmlParser()!;
		const smfConverter = getParseTreeJsonToSmf()!;
		const tree = parser.parse(mmlText);
		const treeJson = JSON.stringify(treeToJson(tree.rootNode, mmlText));
		const smfBytes = smfConverter(treeJson, mmlText);
		const midiArray =
			smfBytes instanceof Uint8Array ? smfBytes : new Uint8Array(smfBytes);

		if (requestId !== latestMidiRequestId) {
			return;
		}

		midiBytes = midiArray;
		lastMidiSource = "mml";
		setStatus(
			fileStatus,
			`MML 入力を SMF に変換しました (${midiArray.byteLength} bytes)`,
		);
		setStatus(mmlStatus, "MML から SMF への変換が完了しました。");
		void runConversion(trigger);
	} catch (error) {
		if (requestId !== latestMidiRequestId) {
			return;
		}
		setStatus(
			mmlStatus,
			`MML 変換に失敗しました: ${(error as Error).message}`,
			true,
		);
	}
}

function readAttachmentBytes(): Uint8Array | null {
	if (!toneJsonField) {
		return new Uint8Array();
	}

	const original = toneJsonField.value;
	const normalized = normalizeAttachmentText(original, attachmentStatus);
	if (normalized === null) {
		return null;
	}
	if (normalized.length === 0) {
		return new Uint8Array();
	}

	const isPresetActive =
		attachmentPresetSelect != null && attachmentPresetSelect.value !== "";
	if (!isPresetActive) {
		toneJsonField.value = normalized;
	}
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
			"SMF ファイルを選択するか、MML を入力してください。",
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
	if (!toneJsonField) return;
	toneJsonField.value = YM_LOG_STYLE_PRESET;

	if (attachmentPresetSelect) {
		attachmentPresetSelect.innerHTML = "";
		const manualOption = document.createElement("option");
		manualOption.value = "";
		manualOption.textContent = "手動入力";
		attachmentPresetSelect.appendChild(manualOption);
		ATTACHMENT_PRESETS.forEach((preset) => {
			const option = document.createElement("option");
			option.value = preset.id;
			option.textContent = preset.label;
			attachmentPresetSelect.appendChild(option);
		});
		attachmentPresetSelect.value = ATTACHMENT_PRESETS[0]?.id ?? "";
		attachmentPresetSelect.addEventListener("change", () => {
			const preset = ATTACHMENT_PRESETS.find(
				(p) => p.id === attachmentPresetSelect.value,
			);
			if (!preset || !toneJsonField) return;
			toneJsonField.value = preset.value;
			void runConversion(`プリセット選択: ${preset.label}`);
		});
	}

	toneJsonField.addEventListener("input", () => {
		if (attachmentPresetSelect && attachmentPresetSelect.value !== "") {
			attachmentPresetSelect.value = "";
		}
		if (attachmentDebounce) {
			window.clearTimeout(attachmentDebounce);
		}
		attachmentDebounce = window.setTimeout(() => {
			void runConversion("音色 JSON 更新");
		}, 400);
	});
}

function setupMmlInput(): void {
	if (!mmlInput) return;
	mmlInput.addEventListener("input", () => {
		if (mmlDebounce) {
			window.clearTimeout(mmlDebounce);
		}
		mmlDebounce = window.setTimeout(() => {
			void convertMmlToSmf("MML 更新");
		}, 400);
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
			midiBytes = null;
			lastMidiSource = null;
			latestMidiRequestId += 1;
			updateOutputWithState("");
			setEventCountDisplay(eventCount, undefined);
			setStatus(fileStatus, "SMF ファイルを選択してください。");
			updatePlayButtonState();
			return;
		}

		setStatus(fileStatus, `${file.name} を読み込み中...`);
		try {
			const requestId = ++latestMidiRequestId;
			const arrayBuffer = await file.arrayBuffer();
			if (requestId !== latestMidiRequestId) {
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

async function initializeWasm(): Promise<void> {
	wasmReady = await ensureWasmInitialized(
		(message, isError) => setStatus(conversionStatus, message, isError),
		"WASM 初期化完了。MIDI を読み込んでください。",
	);
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
