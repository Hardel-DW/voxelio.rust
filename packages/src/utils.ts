import { detectCompression as detectCompressionWasm, getVersion as getVersionWasm } from "./nbt_wasm";
import { ensureInitialized } from "./wasm";
import type { CompressionFormat } from "./index";

/**
 * Detect compression format from bytes - Direct WASM call
 */
export function detectCompression(data: Uint8Array): CompressionFormat {
    ensureInitialized();
    return detectCompressionWasm(data) as CompressionFormat;
}

/**
 * Get library version - Direct WASM call
 */
export function getVersion(): string {
    ensureInitialized();
    return getVersionWasm();
} 