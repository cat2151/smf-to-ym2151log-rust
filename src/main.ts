import './style.css';

// Import the WASM module for SMF to YM2151 conversion
import init, { smf_to_ym2151_json } from '../pkg/smf_to_ym2151log.js';
import {
    generateAudioFromYm2151Json,
    loadWebYm2151Script,
    OPM_SAMPLE_RATE,
    PrepareAudioResult,
    renderWaveform,
    type WebYm2151Module,
} from './ym2151-audio-utils';

let wasmInitialized = false;
let currentYm2151Json: any = null;

// web-ym2151 WASM module (loaded dynamically)
let webYm2151Module: WebYm2151Module | null = null;
let audioCtx: AudioContext | null = null;
let audioBuffer: AudioBuffer | null = null;
let audioSource: AudioBufferSourceNode | null = null;
let preparedAudioData: Float32Array | null = null;
let isPlaying = false;
let audioModuleReady = false;
let playOverlayVisible = false;

function setRenderingOverlay(isVisible: boolean, message = 'Rendering... UI is temporarily disabled.'): void {
    const overlay = document.getElementById('rendering-overlay');
    if (!overlay) return;
    const text = document.getElementById('rendering-overlay-text');
    if (text) {
        text.textContent = message;
    }
    overlay.style.display = isVisible ? 'flex' : 'none';
}

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
        webYm2151Module = await loadWebYm2151Script(import.meta.env.BASE_URL + 'libs/sine_test.js');
        audioModuleReady = true;
        updatePlayButtonState('▶ Play Audio', false);
    } catch (error) {
        console.error('Failed to initialize web-ym2151:', error);
        throw error;
    }
}

function updatePlayButtonState(text: string, disabled: boolean = false): HTMLButtonElement | null {
    const playBtn = document.getElementById('play-audio-btn') as HTMLButtonElement | null;
    if (playBtn) {
        playBtn.textContent = text;
        playBtn.disabled = disabled;
    }
    return playBtn;
}

function showPlayOverlay(): void {
    const overlay = document.getElementById('play-overlay');
    if (!overlay || playOverlayVisible) return;
    overlay.style.display = 'flex';
    playOverlayVisible = true;
}

function hidePlayOverlay(): void {
    const overlay = document.getElementById('play-overlay');
    if (!overlay) return;
    overlay.style.display = 'none';
    playOverlayVisible = false;
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
    hidePlayOverlay();
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
    const audioData = generateAudioFromYm2151Json(currentYm2151Json, webYm2151Module);
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
    hidePlayOverlay();
    
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
            const prepResult = prepareAudioBuffer();
            if (prepResult === PrepareAudioResult.SUCCESS) {
                showPlayOverlay();
            }
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

        setRenderingOverlay(true, `Rendering ${file.name}... UI is temporarily disabled.`);
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
        } finally {
            setRenderingOverlay(false);
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

function setupPlayOverlay(): void {
    const overlay = document.getElementById('play-overlay');
    const floatingBtn = document.getElementById('floating-play-btn');
    if (!overlay || !floatingBtn) return;

    overlay.addEventListener('click', (event) => {
        if (event.target === overlay) {
            hidePlayOverlay();
        }
    });

    floatingBtn.addEventListener('click', (event) => {
        event.stopPropagation();
        hidePlayOverlay();
        void playAudioAndVisualize();
    });
}

// Setup event listeners
function setupEventListeners(): void {
    // File input
    setupFileInput();
    
    // Play button
    setupPlayButton();

    // Floating play overlay
    setupPlayOverlay();
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
