import {encode_audiodom as wasmEncode, decode_audiodom as wasmDecode} from 'audio-dom-core';

/**
 * Types for metadata structure.
 * You can extend this based on your actual schema.
 */
export interface AudioDomMetadata {
  events: Array<{
	type: string;
	timestamp: number;
	selector?: string;
	value?: string;
	[key: string]: any;
  }>;
  [key: string]: any;
}

/**
 * Encodes the AudioDom metadata into a binary format using WebAssembly.
 *
 * @param metadata - The metadata to encode, structured as per AudioDom specifications.
 * The metadata should include an array of events, each with a type, timestamp, and optional
 * selector and value properties. Additional properties can be included as needed.
 *
 * @returns A Promise that resolves to a Uint8Array containing the encoded binary data.
 */
export async function encodeAudioDom( metadata: AudioDomMetadata, audio: Uint8Array ): Promise<Uint8Array> {
	const metadataJson = JSON.stringify(metadata);
	return wasmEncode(metadataJson, audio);
}

export async function decodeAudioDom( binary: Uint8Array ): Promise<{ metadata: AudioDomMetadata; audio: Uint8Array }> {
	const result = wasmDecode(binary);
	const parsed: AudioDomMetadata = JSON.parse(result.metadata);
	return {
		metadata: parsed,
		audio: result.audio,
	};
}
