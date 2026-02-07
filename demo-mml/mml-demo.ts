import './style.css';

// Import the WASM module for SMF to YM2151 conversion
import initSmfWasm, { smf_to_ym2151_json } from '../pkg/smf_to_ym2151log.js';

// web-tree-sitter for MML parsing
import { Parser, Language, Node } from 'web-tree-sitter';
import {
    generateAudioFromYm2151Json,
    loadWebYm2151Script,
    OPM_SAMPLE_RATE,
    PrepareAudioResult,
    renderWaveform,
    type WebYm2151Module,
} from '../src/ym2151-audio-utils';

let currentYm2151Json: any = null;
let mmlModuleReady = false;
let smfWasmReady = false;
let webYm2151Module: WebYm2151Module | null = null;
let parser: Parser | null = null;
let mmlParseTreeToSmf: ((json: string, source: string) => Uint8Array) | null = null;
let debounceTimer: ReturnType<typeof setTimeout> | null = null;
const DEBOUNCE_DELAY_MS = 500;

let audioCtx: AudioContext | null = null;
let audioBuffer: AudioBuffer | null = null;
let audioSource: AudioBufferSourceNode | null = null;
let preparedAudioData: Float32Array | null = null;
let isPlaying = false;
let audioModuleReady = false;
let playOverlayVisible = false;

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
    source.loop = true;
    source.connect(audioCtx.destination);
    source.start();

    audioSource = source;
    isPlaying = true;
    updatePlayButtonState('⏹ Stop Audio', false);

    console.log('Audio playback started');
}

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
        showError(message);
        resetAudioState();
        updatePlayButtonState('▶ Play Audio', !audioModuleReady);
    }
}

function hideWaveformSection(): void {
    const waveformSection = document.getElementById('waveform-section');
    if (waveformSection) waveformSection.style.display = 'none';
    currentYm2151Json = null;
    resetAudioState();
}

function showWaveformSection(): void {
    const waveformSection = document.getElementById('waveform-section');
    if (waveformSection) {
        waveformSection.style.display = 'block';
    }
}

async function initWebYm2151(): Promise<void> {
    const candidates = ['../libs/sine_test.js', './libs/sine_test.js'];
    let lastError: Error | null = null;

    for (const src of candidates) {
        try {
            webYm2151Module = await loadWebYm2151Script(src);
            audioModuleReady = true;
            updatePlayButtonState('▶ Play Audio', false);
            if (currentYm2151Json) {
                const prepResult = prepareAudioBuffer();
                if (prepResult === PrepareAudioResult.SUCCESS) {
                    showPlayOverlay();
                }
            }
            return;
        } catch (error) {
            console.warn(`web-ym2151 load failed for ${src}:`, error);
            lastError = error as Error;
        }
    }

    throw lastError ?? new Error('Failed to initialize web-ym2151');
}

// Initialize all WASM modules
async function initAll(): Promise<void> {
    updatePlayButtonState('Loading audio...', true);

    const outputDiv = document.getElementById('output');
    if (outputDiv) {
        outputDiv.innerHTML = '';
        const statusMsg = document.createElement('p');
        statusMsg.textContent = 'Initializing WASM modules...';
        outputDiv.appendChild(statusMsg);
    }

    // Initialize SMF to YM2151 WASM
    try {
        await initSmfWasm();
        smfWasmReady = true;
        console.log('SMF to YM2151 WASM module initialized successfully');
    } catch (error) {
        console.error('Failed to initialize SMF WASM:', error);
        showError(`Failed to initialize SMF WASM module: ${(error as Error).message}`);
        return;
    }

    // Initialize web-tree-sitter and MML WASM
    try {
        await Parser.init();
        parser = new Parser();

        // Load MML grammar
        const lang = await Language.load('./tree-sitter-mml.wasm');
        parser.setLanguage(lang);
        console.log('web-tree-sitter initialized with MML grammar');
    } catch (error) {
        console.error('Failed to initialize tree-sitter:', error);
        showError(`Failed to initialize MML parser: ${(error as Error).message}. See setup instructions above.`);
        return;
    }

    // Initialize mmlabc-to-smf WASM
    try {
        // @ts-expect-error - External WASM module built during deployment
        const mmlModule = await import('./mmlabc-pkg/mmlabc_to_smf_wasm.js');
        await mmlModule.default();
        mmlParseTreeToSmf = mmlModule.parse_tree_json_to_smf;
        mmlModuleReady = true;
        console.log('MML WASM module initialized successfully');
    } catch (error) {
        console.error('MML WASM module not available:', (error as Error).message);
        mmlModuleReady = false;
        showError('MML WASM module not available. Please follow the setup instructions above.');
        return;
    }

    try {
        await initWebYm2151();
        console.log('web-ym2151 initialized successfully');
    } catch (error) {
        console.error('Failed to initialize audio/visualization modules:', error);
        console.log('Demo will work but audio playback and waveform visualization will not be available');
        updatePlayButtonState('Audio unavailable', true);
    }

    // All modules ready - hide setup instructions and show ready state
    const infoBox = document.getElementById('setup-info');
    if (infoBox) {
        infoBox.style.display = 'none';
    }

    if (outputDiv) {
        outputDiv.innerHTML = '';
        const readyMsg = document.createElement('p');
        readyMsg.className = 'success';
        readyMsg.textContent = '✓ Ready! Enter MML code and it will be converted automatically.';
        outputDiv.appendChild(readyMsg);
    }
}

// Convert parse tree node to JSON for the WASM module
function treeToJSON(node: Node, source: string): Record<string, unknown> {
    const result: Record<string, unknown> = {
        type: node.type,
    };

    if (node.childCount === 0) {
        result.text = source.substring(node.startIndex, node.endIndex);
    }

    if (node.childCount > 0) {
        const children: Record<string, unknown>[] = [];
        for (let i = 0; i < node.childCount; i++) {
            const child = node.child(i);
            if (child) {
                children.push(treeToJSON(child, source));
            }
        }
        result.children = children;
    }

    return result;
}

// Show error message
function showError(message: string): void {
    hideWaveformSection();

    const outputDiv = document.getElementById('output');
    if (!outputDiv) return;

    outputDiv.innerHTML = '';
    const errMsg = document.createElement('p');
    errMsg.className = 'error';
    errMsg.textContent = message;
    outputDiv.appendChild(errMsg);
}

// Convert MML to YM2151 register log
async function convertMML(): Promise<void> {
    const mmlTextarea = document.getElementById('mml-input') as HTMLTextAreaElement;
    if (!mmlTextarea) return;

    const userInput = mmlTextarea.value.trim();
    hideWaveformSection();

    if (!userInput) {
        if (mmlModuleReady && smfWasmReady && parser) {
            const outputDiv = document.getElementById('output');
            if (outputDiv) {
                outputDiv.innerHTML = '';
                const readyMsg = document.createElement('p');
                readyMsg.className = 'success';
                readyMsg.textContent = '✓ Ready! Enter MML code and it will be converted automatically.';
                outputDiv.appendChild(readyMsg);
            }
            updatePlayButtonState('▶ Play Audio', !audioModuleReady);
        }
        return;
    }

    if (!mmlModuleReady || !smfWasmReady || !parser || !mmlParseTreeToSmf) {
        showError('WASM modules not initialized. Please wait or check setup instructions above.');
        return;
    }

    const outputDiv = document.getElementById('output');
    if (!outputDiv) return;

    let tree: ReturnType<Parser['parse']> | null = null;
    try {
        // Step 1: Parse MML using web-tree-sitter
        tree = parser.parse(userInput);
        if (!tree) {
            showError('Failed to parse MML input');
            return;
        }
        const parseTreeJSON = treeToJSON(tree.rootNode, userInput);
        const parseTreeStr = JSON.stringify(parseTreeJSON);

        // Step 2: Convert parse tree to SMF using mmlabc-to-smf-wasm
        const smfData = mmlParseTreeToSmf(parseTreeStr, userInput);

        // Step 3: Convert SMF to YM2151 register log
        const ym2151Json = smf_to_ym2151_json(smfData);

        // Step 4: Display result
        outputDiv.innerHTML = '';

        const json = JSON.parse(ym2151Json);

        if (json.error) {
            const errorP = document.createElement('p');
            errorP.className = 'error';
            errorP.textContent = `Error: ${json.error}`;
            outputDiv.appendChild(errorP);
        } else {
            currentYm2151Json = json;
            resetAudioState();
            showWaveformSection();

            const successP = document.createElement('p');
            successP.className = 'success';
            successP.textContent = `✓ Converted! Event count: ${json.event_count}`;
            outputDiv.appendChild(successP);

            const pre = document.createElement('pre');
            pre.textContent = JSON.stringify(json, null, 2);
            outputDiv.appendChild(pre);

            const prepResult = prepareAudioBuffer();
            if (prepResult === PrepareAudioResult.SUCCESS) {
                showPlayOverlay();
            } else if (prepResult === PrepareAudioResult.MODULE_NOT_READY) {
                updatePlayButtonState('Loading audio...', true);
            }
        }
    } catch (error) {
        showError(`Error processing MML: ${(error as Error).message}`);
        console.error('Error:', error);
    } finally {
        // Free WASM memory held by the tree-sitter Tree object
        if (tree) {
            tree.delete();
        }
    }
}

// Load MML example
function loadMMLExample(exampleText: string): void {
    const mmlTextarea = document.getElementById('mml-input') as HTMLTextAreaElement;
    if (mmlTextarea) {
        mmlTextarea.value = exampleText;
        // Trigger conversion
        mmlTextarea.dispatchEvent(new Event('input'));
    }
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
    // MML convert button
    const convertBtn = document.getElementById('convert-mml-button');
    if (convertBtn) {
        convertBtn.addEventListener('click', () => convertMML());
    }

    // Auto-convert on input (with debounce)
    const mmlInput = document.getElementById('mml-input') as HTMLTextAreaElement;
    if (mmlInput) {
        mmlInput.addEventListener('input', () => {
            if (debounceTimer) {
                clearTimeout(debounceTimer);
            }
            debounceTimer = setTimeout(() => {
                convertMML();
            }, DEBOUNCE_DELAY_MS);
        });
    }

    // MML example buttons
    const exampleConfigs = [
        { selector: '.example-button:nth-of-type(1)', text: 'cdefgab' },
        { selector: '.example-button:nth-of-type(2)', text: 'o5 l4 cdefgab' },
        { selector: '.example-button:nth-of-type(3)', text: 'c;e;g' },
        { selector: '.example-button:nth-of-type(4)', text: 'o4 c c g g a a g2 f f e e d d c2' }
    ];

    exampleConfigs.forEach(({ selector, text }) => {
        const btn = document.querySelector(selector);
        if (btn) {
            btn.addEventListener('click', () => loadMMLExample(text));
        }
    });

    setupPlayButton();
    setupPlayOverlay();
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
    void initAll();
});
