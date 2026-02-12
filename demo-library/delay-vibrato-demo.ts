import './style.css';

import init, {
    smf_to_ym2151_json_with_attachment,
} from 'smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js';

const DEFAULT_ATTACHMENT = `{
  "DelayVibrato": true
}`;

type WebYmApi = {
    playAudioWithOverlay: () => void;
    clearAudioCache: () => void;
};

declare global {
    interface Window {
        Module?: any;
    }
}

let wasmReady = false;
let midiBytes: Uint8Array | null = null;
let currentOutput: string | null = null;
let attachmentDebounce: number | null = null;
let webYmApiPromise: Promise<WebYmApi> | null = null;
let webYmScriptAdded = false;
let webYmScriptEl: HTMLScriptElement | null = null;

const attachmentField = document.getElementById('attachment-json') as HTMLTextAreaElement | null;
const conversionOutput = document.getElementById('conversion-output') as HTMLPreElement | null;
const conversionStatus = document.getElementById('conversion-status');
const attachmentStatus = document.getElementById('attachment-status');
const fileStatus = document.getElementById('file-status');
const eventCount = document.getElementById('event-count');
const jsonEditor = document.getElementById('jsonEditor') as HTMLTextAreaElement | null;
const playButton = document.getElementById('play-audio') as HTMLButtonElement | null;
const webYmStatus = document.getElementById('web-ym-status');

function setStatus(element: HTMLElement | null, message: string, isError = false): void {
    if (!element) return;
    element.textContent = message;
    element.classList.toggle('error', isError);
}

function setEventCountDisplay(count?: number): void {
    if (!eventCount) return;
    if (typeof count === 'number') {
        eventCount.textContent = `Event count: ${count}`;
    } else {
        eventCount.textContent = '';
    }
}

function updateOutput(text: string): void {
    if (conversionOutput) {
        conversionOutput.textContent = text;
    }
    if (jsonEditor) {
        jsonEditor.value = text;
    }
    currentOutput = text;
    if (webYmApiPromise) {
        webYmApiPromise
            .then(api => api.clearAudioCache())
            .catch(() => {
                /* ignore cache clear failures */
            });
    }
    updatePlayButtonState();
}

function updatePlayButtonState(): void {
    if (!playButton) return;
    playButton.disabled = !currentOutput;
}

async function initializeWasm(): Promise<void> {
    try {
        await init();
        wasmReady = true;
        setStatus(conversionStatus, 'WASM 初期化完了。MIDI を読み込んでください。');
    } catch (error) {
        setStatus(conversionStatus, `WASM 初期化に失敗しました: ${(error as Error).message}`, true);
    }
}

function readAttachmentBytes(): Uint8Array | null {
    if (!attachmentField) return new Uint8Array();
    const raw = attachmentField.value.trim();
    if (raw.length === 0) {
        setStatus(attachmentStatus, '添付 JSON は空です (Delay Vibrato 無効)');
        return new Uint8Array();
    }

    try {
        JSON.parse(raw);
        setStatus(attachmentStatus, '添付 JSON を適用します');
        return new TextEncoder().encode(raw);
    } catch (error) {
        setStatus(attachmentStatus, `JSON が不正です: ${(error as Error).message}`, true);
        return null;
    }
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
        setEventCountDisplay(typeof parsed.event_count === 'number' ? parsed.event_count : undefined);
        updateOutput(formatted);
        setStatus(conversionStatus, '変換が完了しました。');
    } catch (error) {
        updateOutput('');
        setEventCountDisplay(undefined);
        setStatus(conversionStatus, `変換に失敗しました: ${(error as Error).message}`, true);
    }
}

function ensureWebYm2151(): Promise<WebYmApi> {
    if (webYmApiPromise) {
        return webYmApiPromise;
    }

    webYmApiPromise = new Promise<WebYmApi>((resolve, reject) => {
        const moduleRef: any = window.Module ?? {};
        window.Module = moduleRef;

        const WEB_YM_SRC = 'https://cat2151.github.io/web-ym2151/sine_test.js';
        const WEB_YM_TIMEOUT_MS = 12000;
        let isActive = true;
        let timeoutId: number | null = null;

        const cleanup = () => {
            if (timeoutId) {
                window.clearTimeout(timeoutId);
                timeoutId = null;
            }
            if (webYmScriptEl && webYmScriptEl.parentNode) {
                webYmScriptEl.parentNode.removeChild(webYmScriptEl);
            }
            webYmScriptEl = null;
            webYmScriptAdded = false;
            delete (window as any).Module;
            webYmApiPromise = null;
        };

        timeoutId = window.setTimeout(() => {
            if (!isActive) return;
            isActive = false;
            cleanup();
            reject(new Error('web-ym2151 のロードがタイムアウトしました'));
        }, WEB_YM_TIMEOUT_MS);

        const runtimeReady = new Promise<void>(runtimeResolve => {
            const previous = moduleRef.onRuntimeInitialized;
            moduleRef.onRuntimeInitialized = () => {
                if (!isActive) {
                    return;
                }
                if (typeof previous === 'function') {
                    previous();
                }
                runtimeResolve();
            };
        });

        if (!webYmScriptAdded) {
            const script = document.createElement('script');
            script.src = WEB_YM_SRC;
            script.defer = true;
            script.onload = () => {
                /* runtimeReady will resolve when initialized */
            };
            script.onerror = () => {
                if (!isActive) return;
                isActive = false;
                cleanup();
                reject(new Error('web-ym2151 のロードに失敗しました'));
            };
            document.body.appendChild(script);
            webYmScriptAdded = true;
            webYmScriptEl = script;
        }

        (async () => {
            try {
                await runtimeReady;
                if (!isActive) return;
                const audioModule = await import(
                    /* @vite-ignore */ 'https://cat2151.github.io/web-ym2151/dist/audio/index.js'
                );
                if (!isActive) return;
                if (timeoutId) {
                    window.clearTimeout(timeoutId);
                    timeoutId = null;
                }
                resolve({
                    playAudioWithOverlay: audioModule.playAudioWithOverlay,
                    clearAudioCache: audioModule.clearAudioCache,
                });
            } catch (error) {
                if (!isActive) return;
                isActive = false;
                cleanup();
                reject(error);
            }
        })();
    });

    return webYmApiPromise;
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
            updateOutput('');
            setEventCountDisplay(undefined);
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
    updateOutput('');
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
