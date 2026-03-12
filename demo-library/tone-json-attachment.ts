import { setStatus } from "./shared-demo";

export type Ym2151Event = {
	time: number;
	addr: string;
	data: string;
};

export type AttachmentPreset = {
	id: string;
	label: string;
	value: string;
};

export const YM_LOG_STYLE_PRESET = `[
  {
    "ProgramChange": 0,
    "Tone": {
      "type": "YM2151 tone",
      "events": [
        { "time": 0, "addr": "0x20", "data": "0xC7" },
        { "time": 0, "addr": "0x60", "data": "0x10" },
        { "time": 0, "addr": "0x80", "data": "0x1F" },
        { "time": 0, "addr": "0xE0", "data": "0x0F" }
      ]
    }
  }
]`;

export const COMPACT_NIBBLE_PRESET = `[
  {
    "ProgramChange": 0,
    "registers": "20C76010801FE00F"
  }
]`;

export const ATTACHMENT_PRESETS: AttachmentPreset[] = [
	{
		id: "ym-log",
		label: "YM2151 log 形式 (time + addr + data)",
		value: YM_LOG_STYLE_PRESET,
	},
	{
		id: "compact-nibbles",
		label: "コンパクト nibble 連結形式",
		value: COMPACT_NIBBLE_PRESET,
	},
];

export function buildEventsFromCompact(compact: string): Ym2151Event[] {
	const cleaned = compact.replace(/\s+/g, "");
	if (cleaned.length === 0) {
		return [];
	}
	if (cleaned.length % 4 !== 0) {
		throw new Error(
			"コンパクト nibble 文字列の長さは4の倍数である必要があります",
		);
	}
	const events: Ym2151Event[] = [];
	for (let i = 0; i < cleaned.length; i += 4) {
		const addr = cleaned.slice(i, i + 2);
		const data = cleaned.slice(i + 2, i + 4);
		if (!/^[0-9a-fA-F]{4}$/.test(`${addr}${data}`)) {
			throw new Error(
				"コンパクト nibble 文字列に16進以外の文字が含まれています",
			);
		}
		events.push({
			time: 0,
			addr: `0x${addr.toUpperCase()}`,
			data: `0x${data.toUpperCase()}`,
		});
	}
	return events;
}

function serializeWithStatus(
	value: unknown,
	statusEl: HTMLElement | null,
	mutated: boolean,
	mutatedMessage: string,
): string {
	const output = JSON.stringify(value, null, 2);
	setStatus(statusEl, mutated ? mutatedMessage : "音色 JSON を適用します");
	return output;
}

export function normalizeAttachmentText(
	raw: string,
	statusEl: HTMLElement | null,
): string | null {
	const trimmed = raw.trim();
	if (trimmed.length === 0) {
		setStatus(statusEl, "音色 JSON は空です (デフォルト音色を使用)");
		return "";
	}

	try {
		const parsed = JSON.parse(trimmed);
		let mutated = false;

		// New array format: normalize per-entry registers (or legacy CompactTone) fields
		if (Array.isArray(parsed)) {
			const normalized = (parsed as Array<Record<string, unknown>>).map(
				(entry) => {
					const compactValue =
						typeof entry.registers === "string" && entry.registers.length > 0
							? entry.registers
							: typeof entry.CompactTone === "string" &&
									entry.CompactTone.length > 0
								? entry.CompactTone
								: null;
					if (compactValue !== null) {
						const events = buildEventsFromCompact(compactValue);
						const {
							registers: _registers,
							CompactTone: _compactTone,
							...rest
						} = entry;
						mutated = true;
						return { ...rest, Tone: { type: "YM2151 tone", events } };
					}

					// Handle Tone.registers: expand compact nibble string inside Tone object
					const toneValue = entry.Tone;
					if (
						toneValue !== null &&
						typeof toneValue === "object" &&
						!Array.isArray(toneValue)
					) {
						const toneObj = toneValue as Record<string, unknown>;
						const toneRegisters =
							typeof toneObj.registers === "string" &&
							toneObj.registers.length > 0
								? toneObj.registers
								: null;
						if (toneRegisters !== null) {
							const events = buildEventsFromCompact(toneRegisters);
							const {
								registers: _registers,
								type: _type,
								...restTone
							} = toneObj;
							mutated = true;
							return {
								...entry,
								Tone: { type: "YM2151 tone", ...restTone, events },
							};
						}
					}

					return entry;
				},
			);
			return serializeWithStatus(
				normalized,
				statusEl,
				mutated,
				"コンパクト nibble を YM2151 音色 JSON に正規化しました",
			);
		}

		// Legacy flat object format
		const normalized = { ...parsed } as Record<string, unknown>;

		if (Array.isArray(parsed.events)) {
			normalized.Tones = normalized.Tones ?? {};
			(normalized.Tones as Record<string, unknown>)["0"] = {
				events: parsed.events,
			};
			delete normalized.events;
			delete normalized.event_count;
			mutated = true;
		}

		if (parsed.CompactTones && typeof parsed.CompactTones === "object") {
			const compactTones = parsed.CompactTones as Record<string, unknown>;
			const toneMap = (normalized.Tones ?? {}) as Record<string, unknown>;
			Object.entries(compactTones).forEach(([program, value]) => {
				if (typeof value !== "string") {
					throw new Error("CompactTones の値は16進文字列である必要があります");
				}
				const events = buildEventsFromCompact(value);
				toneMap[program] = { events };
			});
			normalized.Tones = toneMap;
			delete normalized.CompactTones;
			mutated = true;
		}

		return serializeWithStatus(
			normalized,
			statusEl,
			mutated,
			"プリセットを YM2151 音色 JSON に正規化しました",
		);
	} catch (error) {
		setStatus(statusEl, `JSON が不正です: ${(error as Error).message}`, true);
		return null;
	}
}
