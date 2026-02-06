import './style.css';

// Import the WASM module for SMF to YM2151 conversion
import init, { smf_to_ym2151_json } from '../pkg/smf_to_ym2151log.js';

let wasmInitialized = false;
let mmlWasmAvailable = false;

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

// Try to load MML WASM module
async function checkMMLWasm(): Promise<void> {
    try {
        // Try to import mmlabc-to-smf-wasm
        // @ts-expect-error - Optional external WASM module
        const mmlModule = await import('../mmlabc-pkg/mmlabc_to_smf_wasm.js');
        await mmlModule.default();
        mmlWasmAvailable = true;
        console.log('MML WASM module available');
    } catch (error) {
        console.error('MML WASM module not available:', (error as Error).message);
        mmlWasmAvailable = false;
        showError('MML WASM module not available. Please follow the setup instructions above.');
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
        } else {
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
        }
    } catch (e) {
        // If not JSON, display as plain text
        const preElement = document.createElement('pre');
        preElement.textContent = result;
        output.appendChild(preElement);
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
}

// Convert MML function
async function convertMML(): Promise<void> {
    const mmlInput = document.getElementById('mml-input') as HTMLTextAreaElement;
    if (!mmlInput) return;

    const mmlCode = mmlInput.value.trim();
    
    if (!mmlCode) {
        showError('Please enter MML code');
        return;
    }

    if (!mmlWasmAvailable) {
        showError('MML WASM module not available. Please follow the setup instructions above.');
        return;
    }

    const output = document.getElementById('output');
    if (!output) return;

    output.innerHTML = '';
    const processingParagraph = document.createElement('p');
    processingParagraph.textContent = 'Processing MML...';
    output.appendChild(processingParagraph);

    try {
        // Note: This requires mmlabc-to-smf-wasm to be loaded
        // The actual implementation would call the MML WASM module here
        // For now, show a placeholder message
        showError('MML conversion requires mmlabc-to-smf-wasm integration. See setup instructions above.');
        
        // Example of how it would work:
        // 1. Use web-tree-sitter to parse MML
        // 2. Call mmlabc-to-smf-wasm with parse tree
        // 3. Get SMF bytes back
        // 4. Call our smf_to_ym2151_json with those bytes
        // 5. Display result
    } catch (error) {
        showError(`Error processing MML: ${(error as Error).message}`);
        console.error('Error:', error);
    }
}

// Load MML example
function loadMMLExample(mml: string): void {
    const mmlInput = document.getElementById('mml-input') as HTMLTextAreaElement;
    if (mmlInput) {
        mmlInput.value = mml;
    }
}

// Setup event listeners
function setupEventListeners(): void {
    // MML convert button
    const convertButton = document.getElementById('convert-mml-button');
    if (convertButton) {
        convertButton.addEventListener('click', () => convertMML());
    }

    // MML example buttons
    const examples = [
        { selector: '.example-button:nth-of-type(1)', mml: 'cdefgab' },
        { selector: '.example-button:nth-of-type(2)', mml: 'o5 l4 cdefgab' },
        { selector: '.example-button:nth-of-type(3)', mml: 'c;e;g' },
        { selector: '.example-button:nth-of-type(4)', mml: 'o4 c c g g a a g2 f f e e d d c2' }
    ];

    examples.forEach(({ selector, mml }) => {
        const button = document.querySelector(selector);
        if (button) {
            button.addEventListener('click', () => loadMMLExample(mml));
        }
    });
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
    initWasm();
    checkMMLWasm();
});
