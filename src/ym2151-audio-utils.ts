export interface WebYm2151Module {
    HEAPU8: Uint8Array;
    _generate_sound: (ptr: number, eventCount: number, numFrames: number) => number;
    _get_sample: (index: number) => number;
    _free_buffer: () => void;
    _malloc: (size: number) => number;
    _free: (ptr: number) => void;
}

export enum PrepareAudioResult {
    SUCCESS = 'success',
    MODULE_NOT_READY = 'module_not_ready',
    GENERATION_FAILED = 'generation_failed',
}

export const OPM_CLOCK = 3579545;
export const CLOCK_STEP = 64;
export const OPM_SAMPLE_RATE = OPM_CLOCK / CLOCK_STEP; // ≈ 55930.4Hz

export async function loadWebYm2151Script(scriptSrc: string): Promise<WebYm2151Module> {
    return new Promise((resolve, reject) => {
        let isActive = true;
        const script = document.createElement('script');

        const moduleObj: Partial<WebYm2151Module> & {
            onRuntimeInitialized?: () => void;
            print?: (text: string) => void;
            printErr?: (text: string) => void;
        } = {
            onRuntimeInitialized: () => {
                if (!isActive) return;
                finalizeSuccess();
            },
            print: (text: string) => console.log('[web-ym2151]:', text),
            printErr: (text: string) => console.error('[web-ym2151]:', text),
        };

        const cleanupFailure = (error: Error): void => {
            if (!isActive) return;
            isActive = false;
            clearTimeout(timeoutId);
            clearInterval(checkIntervalId);
            script.remove();
            if ((window as any).Module === moduleObj) {
                delete (window as any).Module;
            }
            reject(error);
        };

        const finalizeSuccess = (): void => {
            if (!isActive) return;
            isActive = false;
            clearTimeout(timeoutId);
            clearInterval(checkIntervalId);
            resolve(moduleObj as WebYm2151Module);
        };

        (window as any).Module = moduleObj;
        script.src = scriptSrc;
        document.head.appendChild(script);

        const checkIntervalId = window.setInterval(() => {
            if (!isActive) return;
            if ((moduleObj as WebYm2151Module)._generate_sound) {
                finalizeSuccess();
            }
        }, 20);

        const timeoutId = window.setTimeout(() => {
            cleanupFailure(new Error('web-ym2151 module initialization timeout'));
        }, 10000);

        script.onerror = () => {
            cleanupFailure(new Error(`Failed to load web-ym2151 script: ${scriptSrc}`));
        };
    });
}

function parseEventField(value: any, isHex: boolean = false): number {
    if (typeof value === 'number') {
        return value;
    }
    if (typeof value === 'string') {
        const parsed = isHex ? parseInt(value, 16) : parseFloat(value);
        return isNaN(parsed) ? 0 : parsed;
    }
    return 0;
}

export function generateAudioFromYm2151Json(json: any, webYm2151Module: WebYm2151Module): Float32Array | null {
    if (!webYm2151Module || !webYm2151Module._generate_sound) {
        console.error('web-ym2151 module not loaded');
        return null;
    }

    try {
        const events = json.events;
        if (!events || events.length === 0) {
            console.error('No events in JSON');
            return null;
        }

        const maxTime = Math.max(...events.map((e: any) => {
            const time = parseFloat(e.time);
            return isNaN(time) ? 0 : time;
        }));
        const durationSeconds = maxTime + 0.5;
        const numFrames = Math.floor(durationSeconds * OPM_SAMPLE_RATE);

        const eventSize = 8; // float(4) + uint8(1) + uint8(1) + padding(2)
        const totalSize = events.length * eventSize;
        const dataPtr = webYm2151Module._malloc(totalSize);
        const view = new DataView(webYm2151Module.HEAPU8.buffer);

        try {
            for (let i = 0; i < events.length; i++) {
                const event = events[i];
                const baseAddr = dataPtr + (i * eventSize);

                const time = parseEventField(event.time, false);
                const addr = parseEventField(event.addr, true);
                const data = parseEventField(event.data, true);

                view.setFloat32(baseAddr, time, true);
                webYm2151Module.HEAPU8[baseAddr + 4] = addr & 0xFF;
                webYm2151Module.HEAPU8[baseAddr + 5] = data & 0xFF;
            }

            const actualFrames = webYm2151Module._generate_sound(dataPtr, events.length, numFrames);

            const audioData = new Float32Array(actualFrames);
            for (let i = 0; i < actualFrames; i++) {
                audioData[i] = webYm2151Module._get_sample(i * 2);
            }

            webYm2151Module._free_buffer();

            return audioData;
        } finally {
            webYm2151Module._free(dataPtr);
        }
    } catch (error) {
        console.error('Error generating audio:', error);
        return null;
    }
}

export function renderWaveform(audioData: Float32Array, canvasId = 'waveform-canvas'): void {
    const canvas = document.getElementById(canvasId) as HTMLCanvasElement;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const width = canvas.width;
    const height = canvas.height;

    ctx.clearRect(0, 0, width, height);
    ctx.fillStyle = '#000000';
    ctx.fillRect(0, 0, width, height);

    ctx.strokeStyle = '#ff8c00';
    ctx.lineWidth = 2;

    const step = Math.max(1, Math.floor(audioData.length / width));
    ctx.beginPath();

    for (let x = 0; x < width; x++) {
        const start = x * step;
        let sum = 0;
        let count = 0;

        for (let i = 0; i < step && (start + i) < audioData.length; i++) {
            sum += audioData[start + i];
            count++;
        }

        const sample = count > 0 ? sum / count : 0;
        const normalized = (sample + 1) / 2;
        const y = (1 - normalized) * height;

        if (x === 0) {
            ctx.moveTo(x, y);
        } else {
            ctx.lineTo(x, y);
        }
    }

    ctx.stroke();
}
