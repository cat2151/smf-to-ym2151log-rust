import { setStatus } from "./shared-demo";
import {
	ensureMmlRuntime,
	getMmlParser,
	getParseTreeJsonToSmf,
	treeToJson,
} from "./tone-json-mml";

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
			setStatus(mmlStatus, "MML を入力すると SMF を生成します。");
			return;
		}

		const requestId = nextRequestId();
		const initialized = await ensureMmlRuntime(mmlStatus);
		if (!initialized || !getMmlParser() || !getParseTreeJsonToSmf()) {
			return;
		}
		if (!isLatestRequest(requestId)) {
			return;
		}

		try {
			const parser = getMmlParser()!;
			const smfConverter = getParseTreeJsonToSmf()!;
			const tree = parser.parse(mmlText);
			const treeJson = JSON.stringify(treeToJson(tree.rootNode, mmlText));
			const smfBytes = smfConverter(treeJson, mmlText);
			const midiArray =
				smfBytes instanceof Uint8Array ? smfBytes : new Uint8Array(smfBytes);

			if (!isLatestRequest(requestId)) {
				return;
			}

			onMidiReady(midiArray);
			setStatus(
				fileStatus ?? null,
				`MML 入力を SMF に変換しました (${midiArray.byteLength} bytes)`,
			);
			setStatus(mmlStatus, "MML から SMF への変換が完了しました。");
			if (onAfterConvert) {
				void onAfterConvert("MML 更新");
			}
		} catch (error) {
			if (!isLatestRequest(requestId)) {
				return;
			}
			setStatus(
				mmlStatus,
				`MML 変換に失敗しました: ${(error as Error).message}`,
				true,
			);
		}
	};

	mmlInput.addEventListener("input", () => {
		if (debounceId) {
			window.clearTimeout(debounceId);
		}
		debounceId = window.setTimeout(() => {
			void handleConvert();
		}, debounceMs);
	});
}
