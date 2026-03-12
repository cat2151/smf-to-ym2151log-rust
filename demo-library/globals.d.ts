declare module "https://cat2151.github.io/web-ym2151/dist/audio/index.js" {
	export interface AudioData {
		left: Float32Array;
		right: Float32Array;
		frames: number;
		duration: number;
		frequencyEstimate?: number;
	}
	export function playAudioWithOverlay(): void;
	export function clearAudioCache(): void;
	export function generateAudioFromJson(jsonString: string): AudioData | null;
}
