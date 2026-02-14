import { setStatus } from './shared-demo';

type TreeSitterNode = {
    type: string;
    childCount: number;
    startIndex: number;
    endIndex: number;
    child: (index: number) => TreeSitterNode;
};

type TreeSitterParser = {
    parse: (source: string) => { rootNode: TreeSitterNode };
    setLanguage: (language: unknown) => void;
};

type SetupMmlInputOptions = {
    mmlInput: HTMLTextAreaElement | null;
    mmlStatus: HTMLElement | null;
    fileStatus?: HTMLElement | null;
    onMidiReady: (bytes: Uint8Array) => void;
    onClear?: () => void;
    onAfterConvert?: (trigger: string) => void;
    debounceMs?: number;
    nextRequestId: () => number;
    isLatestRequest: (id: number) => boolean;
};

const WEB_TREE_SITTER_URL = 'https://cat2151.github.io/mmlabc-to-smf-rust/demo/web-tree-sitter.js';
const MML_WASM_MODULE_URL =
    'https://cat2151.github.io/mmlabc-to-smf-rust/mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js';
const MML_LANGUAGE_URL = 'https://cat2151.github.io/mmlabc-to-smf-rust/tree-sitter-mml/tree-sitter-mml.wasm';

let mmlInitPromise: Promise<boolean> | null = null;
let mmlParser: TreeSitterParser | null = null;
let parseTreeJsonToSmf: ((treeJson: string, source: string) => Uint8Array | number[] | ArrayBuffer) | null = null;

function treeToJson(node: TreeSitterNode, source: string): Record<string, unknown> {
    const result: Record<string, unknown> = { type: node.type };
    if (node.childCount === 0) {
        result.text = source.substring(node.startIndex, node.endIndex);
        return result;
    }

    const children: Record<string, unknown>[] = [];
    for (let i = 0; i < node.childCount; i += 1) {
        children.push(treeToJson(node.child(i), source));
    }
    result.children = children;
    return result;
}

async function ensureMmlRuntime(statusEl: HTMLElement | null): Promise<boolean> {
    if (mmlInitPromise) {
        return mmlInitPromise;
    }

    mmlInitPromise = (async () => {
        setStatus(statusEl, 'MML モジュールを読み込み中...');
        // @ts-ignore -- remote module is resolved at runtime
        const [treeSitterModule, mmlModule] = await Promise.all([
            // @ts-ignore -- remote module is resolved at runtime
            import(/* @vite-ignore */ WEB_TREE_SITTER_URL),
            // @ts-ignore -- remote module is resolved at runtime
            import(/* @vite-ignore */ MML_WASM_MODULE_URL),
        ]);

        const ParserCtor = (treeSitterModule as { Parser: any }).Parser;
        const LanguageApi = (treeSitterModule as { Language: any }).Language;
        await ParserCtor.init();
        const parser: TreeSitterParser = new ParserCtor();
        const language = await LanguageApi.load(MML_LANGUAGE_URL);
        parser.setLanguage(language);
        await mmlModule.default();
        mmlParser = parser;
        parseTreeJsonToSmf = mmlModule.parse_tree_json_to_smf;
        setStatus(statusEl, 'MML モジュールの準備ができました。');
        return true;
    })().catch(error => {
        mmlInitPromise = null;
        setStatus(statusEl, `MML モジュールの読み込みに失敗しました: ${(error as Error).message}`, true);
        return false;
    });

    return mmlInitPromise;
}

export function setupMmlToSmf(options: SetupMmlInputOptions): void {
    const {
        mmlInput,
        mmlStatus,
        fileStatus,
        onMidiReady,
        onClear,
        onAfterConvert,
        debounceMs = 400,
        nextRequestId,
        isLatestRequest,
    } = options;

    if (!mmlInput) return;

    let debounceId: number | null = null;

    const handleConvert = async (): Promise<void> => {
        const mmlText = mmlInput.value.trim();
        if (mmlText.length === 0) {
            if (onClear) {
                onClear();
            }
            setStatus(mmlStatus, 'MML を入力すると SMF を生成します。');
            return;
        }

        const requestId = nextRequestId();
        const initialized = await ensureMmlRuntime(mmlStatus);
        if (!initialized || !mmlParser || !parseTreeJsonToSmf) {
            return;
        }
        if (!isLatestRequest(requestId)) {
            return;
        }

        try {
            const tree = mmlParser.parse(mmlText);
            const treeJson = JSON.stringify(treeToJson(tree.rootNode, mmlText));
            const smfBytes = parseTreeJsonToSmf(treeJson, mmlText);
            const midiArray = smfBytes instanceof Uint8Array ? smfBytes : new Uint8Array(smfBytes);

            if (!isLatestRequest(requestId)) {
                return;
            }

            onMidiReady(midiArray);
            setStatus(fileStatus ?? null, `MML 入力を SMF に変換しました (${midiArray.byteLength} bytes)`);
            setStatus(mmlStatus, 'MML から SMF への変換が完了しました。');
            if (onAfterConvert) {
                void onAfterConvert('MML 更新');
            }
        } catch (error) {
            if (!isLatestRequest(requestId)) {
                return;
            }
            setStatus(mmlStatus, `MML 変換に失敗しました: ${(error as Error).message}`, true);
        }
    };

    mmlInput.addEventListener('input', () => {
        if (debounceId) {
            window.clearTimeout(debounceId);
        }
        debounceId = window.setTimeout(() => {
            void handleConvert();
        }, debounceMs);
    });
}
