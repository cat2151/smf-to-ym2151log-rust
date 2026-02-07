import './style.css';

// Import the WASM module for SMF to YM2151 conversion
import init, { smf_to_ym2151_json } from '../pkg/smf_to_ym2151log.js';

let wasmInitialized = false;
let currentYm2151Json: any = null;

// web-ym2151 WASM module (loaded dynamically)
let webYm2151Module: any = null;

// cat-oscilloscope library (loaded dynamically)
let OscilloscopeClass: any = null;
let BufferSourceClass: any = null;
let oscilloscopeInstance: any = null;

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
        script.src = '/libs/sine_test.js';
        
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
        
        // Wait for module to initialize (with reduced polling interval for faster detection)
        await new Promise<void>((resolve) => {
            const checkInterval = setInterval(() => {
                if ((window as any).Module && (window as any).Module._generate_sound) {
                    webYm2151Module = (window as any).Module;
                    clearInterval(checkInterval);
                    resolve();
                }
            }, 20); // 20ms polling for responsive module detection
        });
    } catch (error) {
        console.error('Failed to initialize web-ym2151:', error);
        throw error;
    }
}

// Initialize cat-oscilloscope library
async function initCatOscilloscope(): Promise<void> {
    try {
        const module = await import('/libs/cat-oscilloscope.mjs' as any);
        OscilloscopeClass = module.Oscilloscope;
        BufferSourceClass = module.BufferSource;
        console.log('cat-oscilloscope library loaded');
    } catch (error) {
        console.error('Failed to load cat-oscilloscope:', error);
        throw error;
    }
}

// Initialize oscilloscope with canvas element
function setupOscilloscope(): void {
    const canvas = document.getElementById('waveform-canvas') as HTMLCanvasElement;
    if (!canvas || !OscilloscopeClass) return;
    
    try {
        // Create hidden canvases for oscilloscope internal use
        // Note: The oscilloscope library requires multiple canvas references for its internal operations
        // (previous waveform, current waveform, similarity plot, and frame buffer)
        // Using the same hidden canvas for all internal canvases to minimize DOM overhead
        const hiddenCanvas = document.createElement('canvas');
        hiddenCanvas.width = 250;
        hiddenCanvas.height = 120;
        
        oscilloscopeInstance = new OscilloscopeClass(
            canvas,
            hiddenCanvas,
            hiddenCanvas,
            hiddenCanvas,
            hiddenCanvas
        );
        
        // Configure oscilloscope for orange waveform on black background
        oscilloscopeInstance.setAutoGain(true);
        oscilloscopeInstance.setNoiseGate(false);
        oscilloscopeInstance.setFrequencyEstimationMethod('autocorrelation');
        oscilloscopeInstance.setDebugOverlaysEnabled(false);
        
        // Set custom colors - orange waveform on black background
        if (oscilloscopeInstance.setColors) {
            oscilloscopeInstance.setColors({
                background: '#000000',
                waveform: '#ff8c00',
                grid: '#333333'
            });
        }
        
        console.log('Oscilloscope initialized with orange/black theme');
    } catch (error) {
        console.error('Failed to initialize oscilloscope:', error);
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

// Start oscilloscope visualization
async function startWaveformVisualization(audioData: Float32Array): Promise<void> {
    if (!oscilloscopeInstance || !BufferSourceClass) {
        console.error('Oscilloscope not initialized');
        return;
    }
    
    try {
        // Stop previous visualization
        await oscilloscopeInstance.stop();
        
        // Create buffer source and start visualization
        const bufferSource = new BufferSourceClass(audioData, OPM_SAMPLE_RATE, {
            loop: true,
            chunkSize: 4096
        });
        
        await oscilloscopeInstance.startFromBuffer(bufferSource);
        console.log('Waveform visualization started');
    } catch (error) {
        console.error('Failed to start waveform visualization:', error);
    }
}

// Play audio and update waveform
async function playAudioAndVisualize(): Promise<void> {
    if (!currentYm2151Json) {
        console.error('No YM2151 JSON data');
        return;
    }
    
    // Show loading state
    const playBtn = document.getElementById('play-audio-btn') as HTMLButtonElement;
    if (playBtn) {
        playBtn.disabled = true;
        playBtn.textContent = '⏳ Generating...';
    }
    
    try {
        // Generate audio
        const audioData = generateAudioFromYm2151Json(currentYm2151Json);
        if (!audioData) {
            throw new Error('Failed to generate audio');
        }
        
        // Create stereo audio buffer for playback (duplicate mono to stereo)
        const audioCtx = new (window.AudioContext || (window as any).webkitAudioContext)();
        const audioBuffer = audioCtx.createBuffer(2, audioData.length, OPM_SAMPLE_RATE);
        audioBuffer.getChannelData(0).set(audioData);
        audioBuffer.getChannelData(1).set(audioData);
        
        const source = audioCtx.createBufferSource();
        source.buffer = audioBuffer;
        source.connect(audioCtx.destination);
        source.start();
        
        // Start waveform visualization
        await startWaveformVisualization(audioData);
        
        // Update button state
        if (playBtn) {
            playBtn.disabled = false;
            playBtn.textContent = '▶ Play Audio';
        }
        
        console.log('Audio playback started');
    } catch (error) {
        console.error('Error in playAudioAndVisualize:', error);
        alert(`Error: ${(error as Error).message}`);
        
        if (playBtn) {
            playBtn.disabled = false;
            playBtn.textContent = '▶ Play Audio';
        }
    }
}

// Display conversion result
function displayResult(result: string): void {
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
        } else {
            // Store the JSON for audio generation
            currentYm2151Json = json;
            
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
        }
    } catch (e) {
        // If not JSON, display as plain text
        const preElement = document.createElement('pre');
        preElement.textContent = result;
        output.appendChild(preElement);
        
        // Hide waveform section
        const waveformSection = document.getElementById('waveform-section');
        if (waveformSection) waveformSection.style.display = 'none';
    }
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
            displayResult(result);
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
        playAudioAndVisualize();
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
    
    // Initialize main WASM module
    await initWasm();
    
    // Initialize web-ym2151 and cat-oscilloscope
    try {
        await Promise.all([
            initWebYm2151(),
            initCatOscilloscope()
        ]);
        
        // Setup oscilloscope after both are loaded
        setupOscilloscope();
        
        console.log('All modules initialized successfully');
    } catch (error) {
        console.error('Failed to initialize audio/visualization modules:', error);
        console.log('Demo will work but audio playback and waveform visualization will not be available');
    }
});
