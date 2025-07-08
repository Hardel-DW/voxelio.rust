import init, { JsNbtTag, JsNbtFile, JsNbtRegion, parseSnbt, formatSnbt, detectCompression, getVersion } from './nbt_wasm.js';

export type CompressionFormat = 'none' | 'gzip' | 'zlib';
export type NbtTag = JsNbtTag;
export type NbtFile = JsNbtFile;
export type NbtRegion = JsNbtRegion;

let wasmInitialized = false;
export default function initNbt(): void {
    if (wasmInitialized) return;
    init();
    wasmInitialized = true;
}

function ensureInitialized() {
    if (!wasmInitialized) {
        throw new Error('NBT WASM not initialized. Call initNbt() first.');
    }
}

/**
 * Read NBT file from bytes with automatic compression detection
 * @param data NBT file bytes
 * @returns NbtFile
 */
export function readNbt(data: Uint8Array): NbtFile {
    ensureInitialized();
    return JsNbtFile.read(data);
}

/**
 * Read NBT file with selective field parsing for performance
 * @param data NBT file bytes
 * @param fields Comma-separated field names to parse (empty = all fields)
 */
export function readNbtFields(data: Uint8Array, fields: string = ''): NbtFile {
    ensureInitialized();
    return JsNbtFile.readFields(data, fields);
}

/**
 * Parse SNBT (String NBT) to tag
 * @param snbt SNBT string
 * @returns NbtTag
 */
export function parseSnbtString(snbt: string): NbtTag {
    ensureInitialized();
    return parseSnbt(snbt);
}

/**
 * Format NBT tag to SNBT string
 * @param tag NbtTag
 * @returns SNBT string
 */
export function formatSnbtString(tag: NbtTag): string {
    ensureInitialized();
    return formatSnbt(tag);
}

/**
 * Read Minecraft region file (.mca)
 * @param data NBT file bytes
 * @returns NbtRegion
 */
export function readRegion(data: Uint8Array): NbtRegion {
    ensureInitialized();
    return JsNbtRegion.read(data);
}

/**
 * Create new empty region
 * @returns NbtRegion
 */
export function createRegion(): NbtRegion {
    ensureInitialized();
    return JsNbtRegion.new();
}

/**
 * Detect compression format from file header
 * @param data NBT file bytes
 * @returns CompressionFormat
 */
export function detectCompressionFormat(data: Uint8Array): CompressionFormat {
    ensureInitialized();
    return detectCompression(data) as CompressionFormat;
}

/**
 * Get NBT WASM version
 * @returns NBT WASM version
 */
export function getNbtVersion(): string {
    ensureInitialized();
    return getVersion();
}