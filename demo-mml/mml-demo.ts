import './style.css';

// Import the WASM module for SMF to YM2151 conversion
import init from '../pkg/smf_to_ym2151log.js';

let mmlModuleReady = false;

// Initialize WASM
async function initWasm(): Promise<void> {
    try {
        await init();
        console.log('SMF to YM2151 WASM module initialized successfully');
    } catch (error) {
        console.error('Failed to initialize WASM:', error);
        const outputDiv = document.getElementById('output');
        if (outputDiv) {
            outputDiv.innerHTML = '';
            const errMsg = document.createElement('p');
            errMsg.className = 'error';
            errMsg.textContent = `Failed to initialize WASM module: ${(error as Error).message}`;
            outputDiv.appendChild(errMsg);
        }
    }
}

// Try to load MML WASM module
async function checkMMLWasm(): Promise<void> {
    try {
        // Try to import mmlabc-to-smf-wasm
        // @ts-expect-error - Optional external WASM module
        const mmlModule = await import('../mmlabc-pkg/mmlabc_to_smf_wasm.js');
        await mmlModule.default();
        mmlModuleReady = true;
        console.log('MML WASM module available');
    } catch (error) {
        console.error('MML WASM module not available:', (error as Error).message);
        mmlModuleReady = false;
        showError('MML WASM module not available. Please follow the setup instructions above.');
    }
}

// Show error message
function showError(message: string): void {
    const outputDiv = document.getElementById('output');
    if (!outputDiv) return;

    outputDiv.innerHTML = '';
    const errMsg = document.createElement('p');
    errMsg.className = 'error';
    errMsg.textContent = message;
    outputDiv.appendChild(errMsg);
}

// Convert MML function
async function convertMML(): Promise<void> {
    const mmlTextarea = document.getElementById('mml-input') as HTMLTextAreaElement;
    if (!mmlTextarea) return;

    const userInput = mmlTextarea.value.trim();
    
    if (!userInput) {
        showError('Please enter MML code');
        return;
    }

    if (!mmlModuleReady) {
        showError('MML WASM module not available. Please follow the setup instructions above.');
        return;
    }

    const outputDiv = document.getElementById('output');
    if (!outputDiv) return;

    outputDiv.innerHTML = '';
    const statusMsg = document.createElement('p');
    statusMsg.textContent = 'Processing MML...';
    outputDiv.appendChild(statusMsg);

    try {
        // Note: This requires mmlabc-to-smf-wasm to be loaded
        // The actual implementation would call the MML WASM module here
        showError('MML conversion requires mmlabc-to-smf-wasm integration. See setup instructions above.');
        
        // Implementation outline:
        // 1. Use web-tree-sitter to parse MML
        // 2. Call mmlabc-to-smf-wasm with parse tree
        // 3. Get SMF bytes back
        // 4. Call smf_to_ym2151_json with those bytes
        // 5. Display result
    } catch (error) {
        showError(`Error processing MML: ${(error as Error).message}`);
        console.error('Error:', error);
    }
}

// Load MML example
function loadMMLExample(exampleText: string): void {
    const mmlTextarea = document.getElementById('mml-input') as HTMLTextAreaElement;
    if (mmlTextarea) {
        mmlTextarea.value = exampleText;
    }
}

// Setup event listeners
function setupEventListeners(): void {
    // MML convert button
    const convertBtn = document.getElementById('convert-mml-button');
    if (convertBtn) {
        convertBtn.addEventListener('click', () => convertMML());
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
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
    initWasm();
    checkMMLWasm();
});
