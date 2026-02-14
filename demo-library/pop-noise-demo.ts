import './style.css';

import { smf_to_ym2151_json_with_attachment } from 'smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js';
import {
    ensureWasmInitialized,
    ensureWebYm2151,
    parseAttachmentField,
    setEventCountDisplay,
    setStatus,
    updateOutput,
} from './shared-demo';
import { setupMmlToSmf } from './mml-support';
import { createLogVisualizer } from './log-visualizer';

const DEFAULT_ATTACHMENT = `[
  {
    "ProgramChange": 0,
    "PopNoiseEnvelope": {
      "Enabled": true,
      "OffsetSeconds": 0.001,
      "Registers": [
        { "BaseRegister": "0x80", "Value": "0x0A" },
        { "BaseRegister": "0xA0", "Value": "0x04" },
        { "BaseRegister": "0xA8", "Value": "0x04" }
      ]
    },
    "AttackContinuationFix": {
      "Enabled": true,
      "OffsetSeconds": 0.001,
      "ReleaseRate": 240
    }
  }
]`;

let wasmReady = false;
let midiBytes: Uint8Array | null = null;
let currentOutput: string | null = null;
let attachmentDebounce: number | null = null;
let lastMidiSource: 'file' | 'mml' | null = null;
let latestMidiRequestId = 0;

const attachmentField = document.getElementById('attachment-json') as HTMLTextAreaElement | null;
const conversionOutput = document.getElementById('conversion-output') as HTMLPreElement | null;
const conversionStatus = document.getElementById('conversion-status');
const attachmentStatus = document.getElementById('attachment-status');
const fileStatus = document.getElementById('file-status');
const mmlStatus = document.getElementById('mml-status');
const eventCount = document.getElementById('event-count');
const jsonEditor = document.getElementById('jsonEditor') as HTMLTextAreaElement | null;
const playButton = document.getElementById('play-audio') as HTMLButtonElement | null;
const webYmStatus = document.getElementById('web-ym-status');
const mmlInput = document.getElementById('mml-input') as HTMLTextAreaElement | null;
const logVisualizer = createLogVisualizer(document.getElementById('log-visualizer'));

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
        'WASM 初期化完了。MIDI を読み込んでください。',
    );
}

function readAttachmentBytes(): Uint8Array | null {
    return parseAttachmentField(
        attachmentField,
        attachmentStatus,
        '添付 JSON は空です (ポップノイズ/アタック継続対策なし)',
        '添付 JSON を適用します',
    );
}

async function runConversion(trigger: string): Promise<void> {
    if (!wasmReady) {
        setStatus(conversionStatus, 'WASM 初期化中です。少しお待ちください...');
        return;
    }
    if (!midiBytes) {
        setStatus(conversionStatus, 'MIDI ファイルを選択するか、MML を入力してください。', true);
        return;
    }

    const attachmentBytes = readAttachmentBytes();
    if (attachmentBytes === null) {
        updatePlayButtonState();
        return;
    }

    try {
        const triggerLabel =
            lastMidiSource === 'mml'
                ? `${trigger} (MML 入力)`
                : lastMidiSource === 'file'
                  ? `${trigger} (SMF ファイル)`
                  : trigger;
        setStatus(conversionStatus, `変換中... (${triggerLabel})`);
        const result = smf_to_ym2151_json_with_attachment(midiBytes, attachmentBytes);
        const parsed = JSON.parse(result);
        const formatted = JSON.stringify(parsed, null, 2);
        setEventCountDisplay(eventCount, typeof parsed.event_count === 'number' ? parsed.event_count : undefined);
        updateOutputWithState(formatted);
        setStatus(conversionStatus, '変換が完了しました。');
    } catch (error) {
        updateOutputWithState('');
        setEventCountDisplay(eventCount, undefined);
        setStatus(conversionStatus, `変換に失敗しました: ${(error as Error).message}`, true);
    }
}

async function handlePlay(): Promise<void> {
    if (!currentOutput) {
        setStatus(conversionStatus, '先に SMF を変換してください。', true);
        return;
    }
    setStatus(conversionStatus, 'web-ym2151 で再生します...');
    try {
        const api = await ensureWebYm2151();
        api.playAudioWithOverlay();
        setStatus(conversionStatus, '再生コマンドを送信しました。');
    } catch (error) {
        setStatus(conversionStatus, `再生に失敗しました: ${(error as Error).message}`, true);
    }
}

function setupAttachmentEditor(): void {
    if (!attachmentField) return;
    attachmentField.value = DEFAULT_ATTACHMENT;
    attachmentField.addEventListener('input', () => {
        if (attachmentDebounce) {
            window.clearTimeout(attachmentDebounce);
        }
        attachmentDebounce = window.setTimeout(() => {
            void runConversion('添付 JSON 更新');
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
        onMidiReady: bytes => {
            midiBytes = bytes;
            lastMidiSource = 'mml';
        },
        onClear: () => {
            if (lastMidiSource === 'mml') {
                midiBytes = null;
                lastMidiSource = null;
            }
        },
        onAfterConvert: trigger => {
            void runConversion(trigger);
        },
    });
}

function setupMidiInput(): void {
    const midiInput = document.getElementById('midi-input') as HTMLInputElement | null;
    if (!midiInput) return;

    midiInput.addEventListener('change', async event => {
        const target = event.target as HTMLInputElement;
        const file = target.files?.[0];
        if (!file) {
            nextRequestId();
            midiBytes = null;
            lastMidiSource = null;
            updateOutputWithState('');
            setEventCountDisplay(eventCount, undefined);
            setStatus(fileStatus, 'SMF ファイルを選択するか、MML を入力してください。');
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
            lastMidiSource = 'file';
            setStatus(fileStatus, `${file.name} を読み込みました (${midiBytes.byteLength} bytes)`);
            void runConversion('MIDI 更新');
        } catch (error) {
            midiBytes = null;
            lastMidiSource = null;
            setStatus(fileStatus, `読み込みに失敗しました: ${(error as Error).message}`, true);
        }
    });
}

function setupPlayButton(): void {
    if (!playButton) return;
    playButton.addEventListener('click', () => {
        void handlePlay();
    });
}

function bootstrap(): void {
    void initializeWasm();
    setupAttachmentEditor();
    setupMidiInput();
    setupPlayButton();
    setupMmlInput();
}

bootstrap();
