import './style.css';

// Import the WASM module for SMF to YM2151 conversion from the GitHub-installed package
// Installed via: npm install github:cat2151/smf-to-ym2151log-rust
import init, {
    smf_to_ym2151_json_with_attachment,
} from 'smf-to-ym2151log-rust/pkg/smf_to_ym2151log.js';

let wasmInitialized = false;

// Initialize WASM
async function initWasm(): Promise<void> {
    try {
        await init();
        wasmInitialized = true;
        console.log('SMF to YM2151 WASM library initialized successfully');
    } catch (error) {
        console.error('Failed to initialize WASM:', error);
        showError(`Failed to initialize WASM module: ${(error as Error).message}`);
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

async function readAttachmentBytes(): Promise<Uint8Array> {
    const attachmentInput = document.getElementById('attachment-input') as HTMLInputElement | null;
    const attachmentFile = attachmentInput?.files?.[0];
    if (!attachmentFile) {
        return new Uint8Array();
    }

    const text = await attachmentFile.text();
    return new TextEncoder().encode(text);
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
            const attachmentBytes = await readAttachmentBytes();

            // Convert SMF to YM2151 JSON using the library with optional attachment JSON
            const result = smf_to_ym2151_json_with_attachment(uint8Array, attachmentBytes);

            // Display result
            displayResult(result);
        } catch (error) {
            showError(`Error processing file: ${(error as Error).message}`);
            console.error('Error:', error);
        }
    });
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
    setupFileInput();
    initWasm();
});
