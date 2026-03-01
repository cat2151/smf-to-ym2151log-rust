import { setStatus } from "./shared-demo";

export type TreeSitterNode = {
	type: string;
	childCount: number;
	startIndex: number;
	endIndex: number;
	child: (index: number) => TreeSitterNode;
};

export type TreeSitterParser = {
	parse: (source: string) => { rootNode: TreeSitterNode };
	setLanguage: (language: unknown) => void;
};

const WEB_TREE_SITTER_URL =
	"https://cat2151.github.io/mmlabc-to-smf-rust/demo/web-tree-sitter.js";
const MML_WASM_MODULE_URL =
	"https://cat2151.github.io/mmlabc-to-smf-rust/mmlabc-to-smf-wasm/pkg/mmlabc_to_smf_wasm.js";
const MML_LANGUAGE_URL =
	"https://cat2151.github.io/mmlabc-to-smf-rust/tree-sitter-mml/tree-sitter-mml.wasm";

let mmlInitPromise: Promise<boolean> | null = null;
let mmlParser: TreeSitterParser | null = null;
let parseTreeJsonToSmf:
	| ((treeJson: string, source: string) => Uint8Array | number[] | ArrayBuffer)
	| null = null;

export function getMmlParser(): TreeSitterParser | null {
	return mmlParser;
}

export function getParseTreeJsonToSmf():
	| ((treeJson: string, source: string) => Uint8Array | number[] | ArrayBuffer)
	| null {
	return parseTreeJsonToSmf;
}

export function treeToJson(
	node: TreeSitterNode,
	source: string,
): Record<string, unknown> {
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

export async function ensureMmlRuntime(
	mmlStatusEl: HTMLElement | null,
): Promise<boolean> {
	if (mmlInitPromise) {
		return mmlInitPromise;
	}

	mmlInitPromise = (async () => {
		setStatus(mmlStatusEl, "MML モジュールを読み込み中...");
		// @ts-ignore -- remote module is resolved at runtime
		const [treeSitterModule, mmlModule] = await Promise.all([
			// @ts-ignore -- remote module is resolved at runtime
			import(/* @vite-ignore */ WEB_TREE_SITTER_URL),
			// @ts-ignore -- remote module is resolved at runtime
			import(/* @vite-ignore */ MML_WASM_MODULE_URL),
		]);

		const ParserCtor = (treeSitterModule as { Parser: unknown }).Parser;
		const LanguageApi = (treeSitterModule as { Language: unknown }).Language;
		await (ParserCtor as { init: () => Promise<void> }).init();
		const parser: TreeSitterParser = new (
			ParserCtor as new () => TreeSitterParser
		)();
		const language = await (
			LanguageApi as { load: (url: string) => Promise<unknown> }
		).load(MML_LANGUAGE_URL);
		parser.setLanguage(language);
		await (mmlModule as { default: () => Promise<void> }).default();
		mmlParser = parser;
		parseTreeJsonToSmf = (
			mmlModule as {
				parse_tree_json_to_smf: (
					treeJson: string,
					source: string,
				) => Uint8Array | number[] | ArrayBuffer;
			}
		).parse_tree_json_to_smf;
		setStatus(mmlStatusEl, "MML モジュールの準備ができました。");
		return true;
	})().catch((error) => {
		mmlInitPromise = null;
		setStatus(
			mmlStatusEl,
			`MML モジュールの読み込みに失敗しました: ${(error as Error).message}`,
			true,
		);
		return false;
	});

	return mmlInitPromise;
}
