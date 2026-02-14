import './style.css';

import { smf_to_ym2151_json_with_attachment } from 'smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js';
import {
    ensureWasmInitialized,
    ensureWebYm2151,
    setEventCountDisplay,
    setStatus,
    updateOutput,
} from './shared-demo';

type AttachmentPreset = {
    id: string;
    label: string;
    value: string;
};

type Ym2151Event = {
    time: number;
    addr: string;
    data: string;
};

type TreeSitterNode = {
    type: string;
    childCount: number;
    startIndex: number;
    endIndex: number;
    child: (index: number) => TreeSitterNode;
};

type TreeSitterParser = {
    parse: (source: string) => { rootNode: TreeSitterNode };
    setLanguage: (language: unknown) => void;
};

const YM_LOG_STYLE_PRESET = `{
  "event_count": 4,
  "events": [
    { "time": 0, "addr": "0x20", "data": "0xC7" },
    { "time": 0, "addr": "0x60", "data": "0x10" },
    { "time": 0, "addr": "0x80", "data": "0x1F" },
    { "time": 0, "addr": "0xE0", "data": "0x0F" }
  ]
}`;

const COMPACT_NIBBLE_PRESET = `{
  "CompactTones": {
    "0": "20C76010801FE00F"
  }
}`;

const ATTACHMENT_PRESETS: AttachmentPreset[] = [
    {
        id: 'ym-log',
        label: 'YM2151 log 形式 (time + addr + data)',
        value: YM_LOG_STYLE_PRESET,
    },
    {
        id: 'compact-nibbles',
        label: 'コンパクト nibble 連結形式',
        value: COMPACT_NIBBLE_PRESET,
    },
];

const WEB_TREE_SITTER_URL = 'https://cat2151.github.io/mmlabc-to-smf-rust/demo/web-tree-sitter.js';
const MML_WASM_MODULE_URL =
    'https://cat2151.github.io/mmlabc-to-smf-rust/mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js';
const MML_LANGUAGE_URL = 'https://cat2151.github.io/mmlabc-to-smf-rust/tree-sitter-mml/tree-sitter-mml.wasm';

let wasmReady = false;
let midiBytes: Uint8Array | null = null;
let currentOutput: string | null = null;
let attachmentDebounce: number | null = null;
let mmlDebounce: number | null = null;
let latestMidiRequestId = 0;
let lastMidiSource: 'file' | 'mml' | null = null;
let mmlInitPromise: Promise<boolean> | null = null;
let mmlParser: TreeSitterParser | null = null;
let parseTreeJsonToSmf: ((treeJson: string, source: string) => Uint8Array | number[] | ArrayBuffer) | null = null;

const toneJsonField = document.getElementById('tone-json') as HTMLTextAreaElement | null;
const conversionOutput = document.getElementById('conversion-output') as HTMLPreElement | null;
const conversionStatus = document.getElementById('conversion-status');
const attachmentStatus = document.getElementById('attachment-status');
const fileStatus = document.getElementById('file-status');
const mmlStatus = document.getElementById('mml-status');
const eventCount = document.getElementById('event-count');
const jsonEditor = document.getElementById('jsonEditor') as HTMLTextAreaElement | null;
const playButton = document.getElementById('play-audio') as HTMLButtonElement | null;
const attachmentPresetSelect = document.getElementById('attachment-preset') as HTMLSelectElement | null;
const webYmStatus = document.getElementById('web-ym-status');
const mmlInput = document.getElementById('mml-input') as HTMLTextAreaElement | null;

function updateOutputWithState(text: string): void {
    currentOutput = text;
    updateOutput(text, conversionOutput, jsonEditor, updatePlayButtonState);
}

function updatePlayButtonState(): void {
    if (!playButton) return;
    playButton.disabled = !currentOutput;
}

function treeToJson(node: TreeSitterNode, source: string): Record<string, unknown> {
    const result: Record<string, unknown> = { type: node.type };
    if (node.childCount === 0) {
        result.text = source.substring(node.startIndex, node.endIndex);
        return result;
    }

    const children: Record<string, unknown>[] = [];
    for (let i = 0; i < node.childCount; i += 1) {
        children.push(treeToJson(node.child(i), source));
    }
    result.children = children;
    return result;
}

function buildEventsFromCompact(compact: string): Ym2151Event[] {
    const cleaned = compact.replace(/\s+/g, '');
    if (cleaned.length === 0) {
        return [];
    }
    if (cleaned.length % 4 !== 0) {
        throw new Error('CompactTones の長さは4の倍数である必要があります');
    }
    const events: Ym2151Event[] = [];
    for (let i = 0; i < cleaned.length; i += 4) {
        const addr = cleaned.slice(i, i + 2);
        const data = cleaned.slice(i + 2, i + 4);
        if (!/^[0-9a-fA-F]{4}$/.test(`${addr}${data}`)) {
            throw new Error('CompactTones に16進以外の文字が含まれています');
        }
        events.push({
            time: 0,
            addr: `0x${addr.toUpperCase()}`,
            data: `0x${data.toUpperCase()}`,
        });
    }
    return events;
}

function normalizeAttachmentText(raw: string, statusEl: HTMLElement | null): string | null {
    const trimmed = raw.trim();
    if (trimmed.length === 0) {
        setStatus(statusEl, '音色 JSON は空です (デフォルト音色を使用)');
        return '';
    }

    try {
        const parsed = JSON.parse(trimmed);
        const normalized = { ...parsed } as Record<string, any>;
        let mutated = false;

        if (Array.isArray(parsed.events)) {
            normalized.Tones = normalized.Tones ?? {};
            normalized.Tones['0'] = { events: parsed.events };
            delete normalized.events;
            delete normalized.event_count;
            mutated = true;
        }

        if (parsed.CompactTones && typeof parsed.CompactTones === 'object') {
            const compactTones = parsed.CompactTones as Record<string, unknown>;
            const toneMap = normalized.Tones ?? {};
            Object.entries(compactTones).forEach(([program, value]) => {
                if (typeof value !== 'string') {
                    throw new Error('CompactTones の値は16進文字列である必要があります');
                }
                const events = buildEventsFromCompact(value);
                toneMap[program] = { events };
            });
            normalized.Tones = toneMap;
            delete normalized.CompactTones;
            mutated = true;
        }

        const output = JSON.stringify(normalized, null, 2);
        setStatus(statusEl, mutated ? 'プリセットを YM2151 音色 JSON に正規化しました' : '音色 JSON を適用します');
        return output;
    } catch (error) {
        setStatus(statusEl, `JSON が不正です: ${(error as Error).message}`, true);
        return null;
    }
}

async function initializeWasm(): Promise<void> {
    wasmReady = await ensureWasmInitialized(
        (message, isError) => setStatus(conversionStatus, message, isError),
        'WASM 初期化完了。MIDI を読み込んでください。',
    );
}

async function ensureMmlRuntime(): Promise<boolean> {
    if (mmlInitPromise) {
        return mmlInitPromise;
    }

    mmlInitPromise = (async () => {
        setStatus(mmlStatus, 'MML モジュールを読み込み中...');
        // @ts-ignore -- remote module is resolved at runtime
        const [treeSitterModule, mmlModule] = await Promise.all([
            // @ts-ignore -- remote module is resolved at runtime
            import(/* @vite-ignore */ WEB_TREE_SITTER_URL),
            // @ts-ignore -- remote module is resolved at runtime
            import(/* @vite-ignore */ MML_WASM_MODULE_URL),
        ]);

        const ParserCtor = (treeSitterModule as { Parser: any }).Parser;
        const LanguageApi = (treeSitterModule as { Language: any }).Language;
        await ParserCtor.init();
        const parser: TreeSitterParser = new ParserCtor();
        const language = await LanguageApi.load(MML_LANGUAGE_URL);
        parser.setLanguage(language);
        await mmlModule.default();
        mmlParser = parser;
        parseTreeJsonToSmf = mmlModule.parse_tree_json_to_smf;
        setStatus(mmlStatus, 'MML モジュールの準備ができました。');
        return true;
    })().catch(error => {
        mmlInitPromise = null;
        setStatus(mmlStatus, `MML モジュールの読み込みに失敗しました: ${(error as Error).message}`, true);
        return false;
    });

    return mmlInitPromise;
}

async function convertMmlToSmf(trigger: string): Promise<void> {
    if (!mmlInput) return;
    const mmlText = mmlInput.value.trim();
    if (mmlText.length === 0) {
        setStatus(mmlStatus, 'MML を入力すると SMF を生成します。');
        return;
    }

    const requestId = ++latestMidiRequestId;
    const initialized = await ensureMmlRuntime();
    if (!initialized || !mmlParser || !parseTreeJsonToSmf) {
        return;
    }

    try {
        const tree = mmlParser.parse(mmlText);
        const treeJson = JSON.stringify(treeToJson(tree.rootNode, mmlText));
        const smfBytes = parseTreeJsonToSmf(treeJson, mmlText);
        const midiArray = smfBytes instanceof Uint8Array ? smfBytes : new Uint8Array(smfBytes);

        if (requestId !== latestMidiRequestId) {
            return;
        }

        midiBytes = midiArray;
        lastMidiSource = 'mml';
        setStatus(fileStatus, `MML 入力を SMF に変換しました (${midiArray.byteLength} bytes)`);
        setStatus(mmlStatus, 'MML から SMF への変換が完了しました。');
        void runConversion(trigger);
    } catch (error) {
        if (requestId !== latestMidiRequestId) {
            return;
        }
        setStatus(mmlStatus, `MML 変換に失敗しました: ${(error as Error).message}`, true);
    }
}

function readAttachmentBytes(): Uint8Array | null {
    if (!toneJsonField) {
        return new Uint8Array();
    }

    const normalized = normalizeAttachmentText(toneJsonField.value, attachmentStatus);
    if (normalized === null) {
        return null;
    }
    if (normalized.length === 0) {
        return new Uint8Array();
    }

    toneJsonField.value = normalized;
    return new TextEncoder().encode(normalized);
}

async function runConversion(trigger: string): Promise<void> {
    if (!wasmReady) {
        setStatus(conversionStatus, 'WASM 初期化中です。少しお待ちください...');
        return;
    }
    if (!midiBytes) {
        setStatus(conversionStatus, 'SMF ファイルを選択するか、MML を入力してください。', true);
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
    if (!toneJsonField) return;
    toneJsonField.value = YM_LOG_STYLE_PRESET;

    if (attachmentPresetSelect) {
        attachmentPresetSelect.innerHTML = '';
        const manualOption = document.createElement('option');
        manualOption.value = '';
        manualOption.textContent = '手動入力';
        attachmentPresetSelect.appendChild(manualOption);
        ATTACHMENT_PRESETS.forEach(preset => {
            const option = document.createElement('option');
            option.value = preset.id;
            option.textContent = preset.label;
            attachmentPresetSelect.appendChild(option);
        });
        attachmentPresetSelect.value = ATTACHMENT_PRESETS[0]?.id ?? '';
        attachmentPresetSelect.addEventListener('change', () => {
            const preset = ATTACHMENT_PRESETS.find(p => p.id === attachmentPresetSelect.value);
            if (!preset || !toneJsonField) return;
            toneJsonField.value = preset.value;
            void runConversion(`プリセット選択: ${preset.label}`);
        });
    }

    toneJsonField.addEventListener('input', () => {
        if (attachmentDebounce) {
            window.clearTimeout(attachmentDebounce);
        }
        attachmentDebounce = window.setTimeout(() => {
            void runConversion('音色 JSON 更新');
        }, 400);
    });
}

function setupMmlInput(): void {
    if (!mmlInput) return;
    mmlInput.addEventListener('input', () => {
        if (mmlDebounce) {
            window.clearTimeout(mmlDebounce);
        }
        mmlDebounce = window.setTimeout(() => {
            void convertMmlToSmf('MML 更新');
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
            lastMidiSource = null;
            latestMidiRequestId += 1;
            updateOutputWithState('');
            setEventCountDisplay(eventCount, undefined);
            setStatus(fileStatus, 'SMF ファイルを選択してください。');
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
    setupMmlInput();
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
