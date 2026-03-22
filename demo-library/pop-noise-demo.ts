import "./style.css";

import { smf_to_ym2151_json_with_attachment } from "smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js";
import {
	ensureWasmInitialized,
	ensureWebYm2151,
	setEventCountDisplay,
	setStatus,
	updateOutput,
} from "./shared-demo";
import { setupMmlToSmf } from "./mml-support";
import { createLogVisualizer } from "./log-visualizer";
import { createWaveformViewer } from "./waveform-viewer";
import {
	generateRandomToneRegisters,
	upsertAttachmentRegisters,
	validateRandomToneAttachment,
} from "./random-tone";
import { normalizeAttachmentText } from "./tone-json-attachment";

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
const registerValidationStatus = document.getElementById(
	"register-validation-status",
);
const fileStatus = document.getElementById("file-status");
const mmlStatus = document.getElementById("mml-status");
const eventCount = document.getElementById("event-count");
const registerReflectionStatus = document.getElementById(
	"register-reflection-status",
);
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
		updateRegisterReflectionStatus(text);
		updatePlayButtonState();
	});
}

function updatePlayButtonState(): void {
	if (!playButton) return;
	playButton.disabled = !currentOutput;
}

function updateRegisterReflectionStatus(outputJson: string): void {
	if (!registerReflectionStatus) return;
	if (outputJson.trim().length === 0) {
		setStatus(registerReflectionStatus, "最終 JSON 反映: 未確認");
		return;
	}
	try {
		const parsed = JSON.parse(outputJson) as {
			events?: unknown;
		};
		const events = Array.isArray(parsed.events)
			? (parsed.events as Array<{ addr?: string }>)
			: [];
		// YM2151 tone-related register groups:
		// - 0x20..0x27: RL/FB/CONNECT (channel)
		// - 0x40..0x5f, 0x60..0x7f, 0x80..0x9f, 0xe0..0xff: operator tone params
		const hasToneLikeRegister = events.some((event) => {
			if (typeof event.addr !== "string") return false;
			const addr = Number.parseInt(event.addr, 16);
			return (
				Number.isFinite(addr) &&
				((addr >= 0x20 && addr <= 0x27) ||
					(addr >= 0x40 && addr <= 0x5f) ||
					(addr >= 0x60 && addr <= 0x7f) ||
					(addr >= 0x80 && addr <= 0x9f) ||
					(addr >= 0xe0 && addr <= 0xff))
			);
		});
		setStatus(
			registerReflectionStatus,
			hasToneLikeRegister
				? "最終 JSON 反映: OK（音色レジスタ書き込みを検出）"
				: "最終 JSON 反映: NG（音色レジスタ書き込みを検出できません）",
			!hasToneLikeRegister,
		);
	} catch (error) {
		setStatus(
			registerReflectionStatus,
			`最終 JSON 反映: 判定失敗 (${(error as Error).message})`,
			true,
		);
	}
}

function countRegisterNormalizationTargets(rawJson: string): number {
	const parsed = JSON.parse(rawJson) as unknown;
	if (Array.isArray(parsed)) {
		return parsed.reduce((count, item) => {
			if (!item || typeof item !== "object" || Array.isArray(item))
				return count;
			const entry = item as Record<string, unknown>;
			const hasRegisters =
				typeof entry.registers === "string" && entry.registers.length > 0;
			const hasCompactTone =
				typeof entry.CompactTone === "string" && entry.CompactTone.length > 0;
			const tone = entry.Tone;
			const hasToneRegisters =
				tone !== null &&
				typeof tone === "object" &&
				!Array.isArray(tone) &&
				typeof (tone as Record<string, unknown>).registers === "string" &&
				((tone as Record<string, unknown>).registers as string).length > 0;
			return hasRegisters || hasCompactTone || hasToneRegisters
				? count + 1
				: count;
		}, 0);
	}
	if (!parsed || typeof parsed !== "object") {
		return 0;
	}
	const obj = parsed as Record<string, unknown>;
	const compactTones = obj.CompactTones;
	if (
		compactTones !== null &&
		typeof compactTones === "object" &&
		!Array.isArray(compactTones)
	) {
		return Object.values(compactTones).filter(
			(value) => typeof value === "string" && value.length > 0,
		).length;
	}
	return 0;
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
		setStatus(attachmentStatus, "添付 JSON は空です (ポップノイズ対策なし)");
		setStatus(registerValidationStatus, "registers 検証: 未実行");
		return new Uint8Array();
	}
	let normalizationTargetCount = 0;
	try {
		normalizationTargetCount = countRegisterNormalizationTargets(raw);
	} catch {
		normalizationTargetCount = 0;
	}
	const normalized = normalizeAttachmentText(raw, attachmentStatus);
	if (normalized === null) {
		const detail = attachmentStatus?.textContent?.trim();
		setStatus(
			registerValidationStatus,
			detail ? `registers 検証: NG (${detail})` : "registers 検証: NG",
			true,
		);
		return null;
	}
	const normalizedTrimmed = normalized.trim();
	if (normalizedTrimmed.length > 0) {
		try {
			JSON.parse(normalizedTrimmed);
			if (normalizationTargetCount > 0) {
				setStatus(
					registerValidationStatus,
					`registers 検証: OK（${normalizationTargetCount}件を正規化）`,
				);
			} else {
				setStatus(registerValidationStatus, "registers 検証: 対象なし");
			}
		} catch (error) {
			setStatus(
				registerValidationStatus,
				`registers 検証: NG (${(error as Error).message})`,
				true,
			);
			return null;
		}
	} else {
		setStatus(registerValidationStatus, "registers 検証: 未実行");
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

async function applyRandomToneToAttachment(): Promise<void> {
	if (!attachmentField) return;

	try {
		validateRandomToneAttachment(attachmentField.value);
	} catch (error) {
		setStatus(attachmentStatus, (error as Error).message, true);
		return;
	}

	let registers: string;
	try {
		registers = await generateRandomToneRegisters();
	} catch {
		setStatus(
			attachmentStatus,
			"ym2151-tone-editor の読み込みに失敗しました。ランダム音色を適用できません。",
			true,
		);
		return;
	}

	try {
		attachmentField.value = upsertAttachmentRegisters(
			attachmentField.value,
			registers,
		);
	} catch (error) {
		setStatus(attachmentStatus, (error as Error).message, true);
		return;
	}

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
