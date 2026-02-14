import init from 'smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js';

export type WebYmApi = {
    playAudioWithOverlay: () => void;
    clearAudioCache: () => void;
};

let wasmInitialized = false;

export async function ensureWasmInitialized(
    setStatus: (message: string, isError?: boolean) => void,
    successMessage: string,
): Promise<boolean> {
    if (wasmInitialized) {
        return true;
    }
    try {
        await init();
        wasmInitialized = true;
        setStatus(successMessage);
        return true;
    } catch (error) {
        setStatus(`WASM 初期化に失敗しました: ${(error as Error).message}`, true);
        return false;
    }
}

export function setStatus(element: HTMLElement | null, message: string, isError = false): void {
    if (!element) return;
    element.textContent = message;
    element.classList.toggle('error', isError);
}

export function setEventCountDisplay(element: HTMLElement | null, count?: number): void {
    if (!element) return;
    if (typeof count === 'number') {
        element.textContent = `Event count: ${count}`;
    } else {
        element.textContent = '';
    }
}

let webYmApiPromise: Promise<WebYmApi> | null = null;
let webYmScriptAdded = false;
let webYmScriptEl: HTMLScriptElement | null = null;

export function ensureWebYm2151(): Promise<WebYmApi> {
    if (webYmApiPromise) {
        return webYmApiPromise;
    }

    webYmApiPromise = new Promise<WebYmApi>((resolve, reject) => {
        const moduleRef: any = (window as any).Module ?? {};
        (window as any).Module = moduleRef;

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

export function clearWebYmAudioCache(): void {
    if (!webYmApiPromise) return;
    webYmApiPromise
        .then(api => api.clearAudioCache())
        .catch(() => {
            /* ignore cache clear failures */
        });
}

export function updateOutput(
    text: string,
    conversionOutput: HTMLPreElement | null,
    jsonEditor: HTMLTextAreaElement | null,
    onAfter?: () => void,
): void {
    if (conversionOutput) {
        conversionOutput.textContent = text;
    }
    if (jsonEditor) {
        jsonEditor.value = text;
    }
    clearWebYmAudioCache();
    if (onAfter) {
        onAfter();
    }
}

export function parseAttachmentField(
    field: HTMLTextAreaElement | null,
    statusEl: HTMLElement | null,
    emptyMessage: string,
    applyMessage: string,
): Uint8Array | null {
    if (!field) return new Uint8Array();
    const raw = field.value.trim();
    if (raw.length === 0) {
        setStatus(statusEl, emptyMessage);
        return new Uint8Array();
    }

    try {
        JSON.parse(raw);
        setStatus(statusEl, applyMessage);
        return new TextEncoder().encode(raw);
    } catch (error) {
        setStatus(statusEl, `JSON が不正です: ${(error as Error).message}`, true);
        return null;
    }
}
