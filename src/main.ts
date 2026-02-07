import './style.css';

// Import the WASM module for SMF to YM2151 conversion
import init, { smf_to_ym2151_json } from '../pkg/smf_to_ym2151log.js';

let wasmInitialized = false;
let currentYm2151Json: any = null;

// web-ym2151 WASM module (loaded dynamically)
let webYm2151Module: any = null;
let audioCtx: AudioContext | null = null;
let audioBuffer: AudioBuffer | null = null;
let audioSource: AudioBufferSourceNode | null = null;
let preparedAudioData: Float32Array | null = null;
let isPlaying = false;
let audioModuleReady = false;

enum PrepareAudioResult {
    SUCCESS = 'success',
    MODULE_NOT_READY = 'module_not_ready',
    GENERATION_FAILED = 'generation_failed',
}

// YM2151 emulator constants
const OPM_CLOCK = 3579545;
const CLOCK_STEP = 64;
const OPM_SAMPLE_RATE = OPM_CLOCK / CLOCK_STEP; // ≈ 55930.4Hz

// Initialize WASM
async function initWasm(): Promise<void> {
    try {
        await init();
        wasmInitialized = true;
        console.log('SMF to YM2151 WASM module initialized successfully');
    } catch (error) {
        console.error('Failed to initialize WASM:', error);
        const output = document.getElementById('output');
        if (output) {
            output.innerHTML = '';
            const errorParagraph = document.createElement('p');
            errorParagraph.className = 'error';
            errorParagraph.textContent = `Failed to initialize WASM module: ${(error as Error).message}`;
            output.appendChild(errorParagraph);
        }
    }
}

// Initialize web-ym2151 WASM module
async function initWebYm2151(): Promise<void> {
    try {
        // Load the Emscripten-generated WASM module
        const script = document.createElement('script');
        script.src = import.meta.env.BASE_URL + 'libs/sine_test.js';
        
        // Set up Module object before loading the script
        (window as any).Module = {
            onRuntimeInitialized: () => {
                webYm2151Module = (window as any).Module;
                console.log('web-ym2151 WASM module initialized');
            },
            print: (text: string) => console.log('[web-ym2151]:', text),
            printErr: (text: string) => console.error('[web-ym2151]:', text),
        };
        
        document.head.appendChild(script);
        
        // Wait for module to initialize with timeout and error handling
        await new Promise<void>((resolve, reject) => {
            const checkInterval = setInterval(() => {
                if ((window as any).Module && (window as any).Module._generate_sound) {
                    clearTimeout(timeout);
                    clearInterval(checkInterval);
                    webYm2151Module = (window as any).Module;
                    audioModuleReady = true;
                    updatePlayButtonState('▶ Play Audio', false);
                    resolve();
                }
            }, 20); // 20ms polling for responsive module detection
            
            const timeout = setTimeout(() => {
                clearInterval(checkInterval);
                reject(new Error('web-ym2151 module initialization timeout'));
            }, 10000); // 10 second timeout
            
            // Handle script load errors
            script.onerror = () => {
                clearTimeout(timeout);
                clearInterval(checkInterval);
                reject(new Error('Failed to load web-ym2151 script'));
            };
        });
    } catch (error) {
        console.error('Failed to initialize web-ym2151:', error);
        throw error;
    }
}

// Helper function to parse event field values
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

// Generate audio from YM2151 JSON
function generateAudioFromYm2151Json(json: any): Float32Array | null {
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
        
        // Calculate duration
        const maxTime = Math.max(...events.map((e: any) => {
            const time = parseFloat(e.time);
            return isNaN(time) ? 0 : time;
        }));
        const durationSeconds = maxTime + 0.5;
        const numFrames = Math.floor(durationSeconds * OPM_SAMPLE_RATE);
        
        // Allocate memory for events
        const eventSize = 8; // float(4) + uint8(1) + uint8(1) + padding(2)
        const totalSize = events.length * eventSize;
        const dataPtr = webYm2151Module._malloc(totalSize);
        const view = new DataView(webYm2151Module.HEAPU8.buffer);
        
        try {
            // Write events to WASM memory
            for (let i = 0; i < events.length; i++) {
                const event = events[i];
                const baseAddr = dataPtr + (i * eventSize);
                
                // Parse event fields using helper function
                const time = parseEventField(event.time, false);
                const addr = parseEventField(event.addr, true);
                const data = parseEventField(event.data, true);
                
                view.setFloat32(baseAddr, time, true);
                webYm2151Module.HEAPU8[baseAddr + 4] = addr & 0xFF;
                webYm2151Module.HEAPU8[baseAddr + 5] = data & 0xFF;
            }
            
            // Generate audio
            console.log(`Generating ${numFrames} frames (${durationSeconds.toFixed(2)}s)`);
            const actualFrames = webYm2151Module._generate_sound(dataPtr, events.length, numFrames);
            
            // Read audio samples (mono - left channel only for oscilloscope)
            const audioData = new Float32Array(actualFrames);
            for (let i = 0; i < actualFrames; i++) {
                audioData[i] = webYm2151Module._get_sample(i * 2); // Left channel
            }
            
            // Free the audio buffer
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

// Render waveform to canvas (orange line on black background)
function renderWaveform(audioData: Float32Array): void {
    const canvas = document.getElementById('waveform-canvas') as HTMLCanvasElement;
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
        const normalized = (sample + 1) / 2; // map [-1,1] -> [0,1]
        const y = (1 - normalized) * height;
        
        if (x === 0) {
            ctx.moveTo(x, y);
        } else {
            ctx.lineTo(x, y);
        }
    }
    
    ctx.stroke();
}

function updatePlayButtonState(text: string, disabled: boolean = false): HTMLButtonElement | null {
    const playBtn = document.getElementById('play-audio-btn') as HTMLButtonElement | null;
    if (playBtn) {
        playBtn.textContent = text;
        playBtn.disabled = disabled;
    }
    return playBtn;
}

function stopPlayback(): void {
    if (audioSource) {
        try {
            audioSource.stop();
        } catch (e) {
            console.warn('Stopping audio source failed:', e);
        }
        audioSource.disconnect();
        audioSource = null;
    }
    isPlaying = false;
    updatePlayButtonState('▶ Play Audio', !audioModuleReady);
}

function resetAudioState(): void {
    stopPlayback();
    preparedAudioData = null;
    audioBuffer = null;
}

function prepareAudioBuffer(): PrepareAudioResult {
    if (!currentYm2151Json) {
        return PrepareAudioResult.GENERATION_FAILED;
    }
    if (!audioModuleReady || !webYm2151Module || !webYm2151Module._generate_sound) {
        console.warn('Audio module is not ready yet; postponing audio buffer preparation.');
        updatePlayButtonState('Loading audio...', true);
        return PrepareAudioResult.MODULE_NOT_READY;
    }
    const audioData = generateAudioFromYm2151Json(currentYm2151Json);
    if (!audioData) {
        console.error('Failed to generate audio from YM2151 JSON; audio buffer will not be prepared.');
        resetAudioState();
        updatePlayButtonState('▶ Play Audio', false);
        return PrepareAudioResult.GENERATION_FAILED;
    }
    preparedAudioData = audioData;
    renderWaveform(audioData);
    
    if (!audioCtx) {
        audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)();
    }
    
    audioBuffer = audioCtx.createBuffer(2, audioData.length, OPM_SAMPLE_RATE);
    audioBuffer.getChannelData(0).set(audioData);
    audioBuffer.getChannelData(1).set(audioData);
    updatePlayButtonState('▶ Play Audio', false);
    return PrepareAudioResult.SUCCESS;
}

async function startPlayback(): Promise<void> {
    if (!audioBuffer) {
        throw new Error('Audio buffer is not prepared');
    }
    if (!audioCtx) {
        audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)();
    }
    await audioCtx.resume();
    
    const source = audioCtx.createBufferSource();
    source.buffer = audioBuffer;
    source.loop = true; // Enable looping to match oscilloscope visualization
    source.connect(audioCtx.destination);
    source.start();
    
    audioSource = source;
    isPlaying = true;
    updatePlayButtonState('⏹ Stop Audio', false);
    
    console.log('Audio playback started');
}

// Play/stop audio and update waveform
async function playAudioAndVisualize(): Promise<void> {
    if (!currentYm2151Json) {
        console.error('No YM2151 JSON data');
        return;
    }
    
    if (isPlaying) {
        stopPlayback();
        return;
    }
    
    updatePlayButtonState('⏳ Generating...', true);
    
    try {
        let prepResult = PrepareAudioResult.SUCCESS;
        if (!audioBuffer || !preparedAudioData) {
            prepResult = prepareAudioBuffer();
        }

        if (prepResult === PrepareAudioResult.MODULE_NOT_READY) {
            updatePlayButtonState('Loading audio...', true);
            return;
        }
        if (prepResult === PrepareAudioResult.GENERATION_FAILED) {
            updatePlayButtonState('▶ Play Audio', !audioModuleReady);
            return;
        }
        
        await startPlayback();
    } catch (error) {
        console.error('Error in playAudioAndVisualize:', error);
        const message =
            (error as Error)?.message !== undefined
                ? (error as Error).message
                : String(error);
        appendError(message);
        resetAudioState();
        updatePlayButtonState('▶ Play Audio', !audioModuleReady);
    }
}

// Display conversion result
async function displayResult(result: string): Promise<void> {
    const output = document.getElementById('output');
    if (!output) return;

    output.innerHTML = '';

    try {
        const json = JSON.parse(result);
        
        if (json.error) {
            const errorParagraph = document.createElement('p');
            errorParagraph.className = 'error';
            errorParagraph.textContent = `Error: ${json.error}`;
            output.appendChild(errorParagraph);
            
            // Hide waveform section on error
            const waveformSection = document.getElementById('waveform-section');
            if (waveformSection) waveformSection.style.display = 'none';
            currentYm2151Json = null;
            resetAudioState();
        } else {
            // Store the JSON for audio generation
            currentYm2151Json = json;
            resetAudioState();
            
            const successParagraph = document.createElement('p');
            successParagraph.className = 'success';
            successParagraph.textContent = '✓ Successfully converted!';
            output.appendChild(successParagraph);
            
            const eventCountParagraph = document.createElement('p');
            const strongElement = document.createElement('strong');
            strongElement.textContent = 'Event count:';
            eventCountParagraph.appendChild(strongElement);
            eventCountParagraph.appendChild(document.createTextNode(` ${json.event_count}`));
            output.appendChild(eventCountParagraph);
            
            const preElement = document.createElement('pre');
            preElement.textContent = JSON.stringify(json, null, 2);
            output.appendChild(preElement);
            
            // Show waveform section
            const waveformSection = document.getElementById('waveform-section');
            if (waveformSection) {
                waveformSection.style.display = 'block';
            }
            prepareAudioBuffer();
        }
    } catch (e) {
        // If not JSON, display as plain text
        const preElement = document.createElement('pre');
        preElement.textContent = result;
        output.appendChild(preElement);
        
        // Hide waveform section
        const waveformSection = document.getElementById('waveform-section');
        if (waveformSection) waveformSection.style.display = 'none';
        currentYm2151Json = null;
        resetAudioState();
    }
}

function appendError(message: string): void {
    const output = document.getElementById('output');
    if (!output) return;

    const errorParagraph = document.createElement('p');
    errorParagraph.className = 'error';
    errorParagraph.textContent = `Error: ${message}`;
    output.appendChild(errorParagraph);
}

// Show error message
function showError(message: string): void {
    const output = document.getElementById('output');
    if (!output) return;

    output.innerHTML = '';
    const errorParagraph = document.createElement('p');
    errorParagraph.className = 'error';
    errorParagraph.textContent = message;
    output.appendChild(errorParagraph);
    
    // Hide waveform section on error
    const waveformSection = document.getElementById('waveform-section');
    if (waveformSection) waveformSection.style.display = 'none';
    currentYm2151Json = null;
    resetAudioState();
}

// Handle file input
function setupFileInput(): void {
    const fileInput = document.getElementById('file-input') as HTMLInputElement;
    if (!fileInput) return;

    fileInput.addEventListener('change', async (event) => {
        const target = event.target as HTMLInputElement;
        const file = target.files?.[0];
        if (!file) return;

        if (!wasmInitialized) {
            showError('WASM not initialized. Please wait...');
            return;
        }

        const output = document.getElementById('output');
        if (!output) return;

        output.innerHTML = '';
        const processingParagraph = document.createElement('p');
        processingParagraph.textContent = `Processing ${file.name}...`;
        output.appendChild(processingParagraph);

        try {
            // Read file as array buffer
            const arrayBuffer = await file.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);

            // Convert SMF to YM2151 JSON
            const result = smf_to_ym2151_json(uint8Array);

            // Display result
            await displayResult(result);
        } catch (error) {
            showError(`Error processing file: ${(error as Error).message}`);
            console.error('Error:', error);
        }
    });
}

// Setup play button
function setupPlayButton(): void {
    const playBtn = document.getElementById('play-audio-btn');
    if (!playBtn) return;
    
    playBtn.addEventListener('click', () => {
        void playAudioAndVisualize();
    });
}

// Setup event listeners
function setupEventListeners(): void {
    // File input
    setupFileInput();
    
    // Play button
    setupPlayButton();
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', async () => {
    setupEventListeners();
    updatePlayButtonState('Loading audio...', true);
    
    // Initialize main WASM module
    await initWasm();
    
    // Initialize web-ym2151
    try {
        await initWebYm2151();
        console.log('web-ym2151 initialized successfully');
    } catch (error) {
        console.error('Failed to initialize audio/visualization modules:', error);
        console.log('Demo will work but audio playback and waveform visualization will not be available');
        updatePlayButtonState('Audio unavailable', true);
    }
});
