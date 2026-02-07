import './style.css';

// Import the WASM module for SMF to YM2151 conversion
// @ts-expect-error - Generated WASM module without type declarations
import initSmfWasm, { smf_to_ym2151_json } from '../pkg/smf_to_ym2151log.js';

// web-tree-sitter for MML parsing
import { Parser, Language, Node } from 'web-tree-sitter';

let mmlModuleReady = false;
let smfWasmReady = false;
let parser: Parser | null = null;
let mmlParseTreeToSmf: ((json: string, source: string) => Uint8Array) | null = null;
let debounceTimer: ReturnType<typeof setTimeout> | null = null;
const DEBOUNCE_DELAY_MS = 500;

// Initialize all WASM modules
async function initAll(): Promise<void> {
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

    if (!userInput) {
        const outputDiv = document.getElementById('output');
        if (outputDiv) {
            outputDiv.innerHTML = '';
            const readyMsg = document.createElement('p');
            readyMsg.className = 'success';
            readyMsg.textContent = '✓ Ready! Enter MML code and it will be converted automatically.';
            outputDiv.appendChild(readyMsg);
        }
        return;
    }

    if (!mmlModuleReady || !smfWasmReady || !parser || !mmlParseTreeToSmf) {
        showError('WASM modules not initialized. Please wait or check setup instructions above.');
        return;
    }

    const outputDiv = document.getElementById('output');
    if (!outputDiv) return;

    try {
        // Step 1: Parse MML using web-tree-sitter
        const tree = parser.parse(userInput);
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
            const successP = document.createElement('p');
            successP.className = 'success';
            successP.textContent = `✓ Converted! Event count: ${json.event_count}`;
            outputDiv.appendChild(successP);

            const pre = document.createElement('pre');
            pre.textContent = JSON.stringify(json, null, 2);
            outputDiv.appendChild(pre);
        }
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
        // Trigger conversion
        mmlTextarea.dispatchEvent(new Event('input'));
    }
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
}

// Initialize on page load
document.addEventListener('DOMContentLoaded', () => {
    setupEventListeners();
    initAll();
});
