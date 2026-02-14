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

const DEFAULT_ATTACHMENT = `{
  "Portamento": true,
  "SoftwareLfo": [
    {
      "BaseRegister": "0x60",
      "Depth": 6,
      "RateHz": 4.0,
      "DelaySeconds": 0.1,
      "AttackSeconds": 0.05,
      "Waveform": "triangle"
    }
  ]
}`;

let wasmReady = false;
let midiBytes: Uint8Array | null = null;
let currentOutput: string | null = null;
let attachmentDebounce: number | null = null;

const attachmentField = document.getElementById('attachment-json') as HTMLTextAreaElement | null;
const conversionOutput = document.getElementById('conversion-output') as HTMLPreElement | null;
const conversionStatus = document.getElementById('conversion-status');
const attachmentStatus = document.getElementById('attachment-status');
const fileStatus = document.getElementById('file-status');
const eventCount = document.getElementById('event-count');
const jsonEditor = document.getElementById('jsonEditor') as HTMLTextAreaElement | null;
const playButton = document.getElementById('play-audio') as HTMLButtonElement | null;
const webYmStatus = document.getElementById('web-ym-status');

function updateOutputWithState(text: string): void {
    currentOutput = text;
    updateOutput(text, conversionOutput, jsonEditor, updatePlayButtonState);
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
        '添付 JSON は空です (ポルタメント/ソフトLFO 無効)',
        '添付 JSON を適用します',
    );
}

async function runConversion(trigger: string): Promise<void> {
    if (!wasmReady) {
        setStatus(conversionStatus, 'WASM 初期化中です。少しお待ちください...');
        return;
    }
    if (!midiBytes) {
        setStatus(conversionStatus, 'MIDI ファイルを先に選択してください。', true);
        return;
    }

    const attachmentBytes = readAttachmentBytes();
    if (attachmentBytes === null) {
        updatePlayButtonState();
        return;
    }

    try {
        setStatus(conversionStatus, `変換中... (${trigger})`);
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

function setupMidiInput(): void {
    const midiInput = document.getElementById('midi-input') as HTMLInputElement | null;
    if (!midiInput) return;

    midiInput.addEventListener('change', async event => {
        const target = event.target as HTMLInputElement;
        const file = target.files?.[0];
        if (!file) {
            midiBytes = null;
            updateOutputWithState('');
            setEventCountDisplay(eventCount, undefined);
            setStatus(fileStatus, 'SMF ファイルを選択してください。');
            updatePlayButtonState();
            return;
        }

        setStatus(fileStatus, `${file.name} を読み込み中...`);
        try {
            const arrayBuffer = await file.arrayBuffer();
            midiBytes = new Uint8Array(arrayBuffer);
            setStatus(fileStatus, `${file.name} を読み込みました (${midiBytes.byteLength} bytes)`);
            void runConversion('MIDI 更新');
        } catch (error) {
            midiBytes = null;
            setStatus(fileStatus, `読み込みに失敗しました: ${(error as Error).message}`, true);
        }
    });
}

function bootstrapWebYm(): void {
    setStatus(webYmStatus, 'web-ym2151 を準備中...');
    ensureWebYm2151()
        .then(() => {
            setStatus(webYmStatus, 'web-ym2151 準備完了');
            updatePlayButtonState();
        })
        .catch(error => {
            setStatus(webYmStatus, `web-ym2151 の準備に失敗しました: ${(error as Error).message}`, true);
        });
}

function main(): void {
    setupAttachmentEditor();
    setupMidiInput();
    updateOutputWithState('');
    updatePlayButtonState();
    bootstrapWebYm();
    void initializeWasm();

    if (playButton) {
        playButton.addEventListener('click', () => {
            void handlePlay();
        });
    }
}

document.addEventListener('DOMContentLoaded', main);
