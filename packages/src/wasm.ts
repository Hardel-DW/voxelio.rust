import init, { initSync } from './nbt_wasm';
import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

let wasmInitialized = false;
let initPromise: Promise<void> | null = null;

/**
 * Initialize NBT WASM module
 * Safe to call multiple times - detects Node.js vs Browser
 */
export default async function initNbt(): Promise<void> {
    if (wasmInitialized) return;

    if (initPromise) {
        await initPromise;
        return;
    }

    initPromise = (async () => {
        try {
            // Detect Node.js environment
            if (typeof window === 'undefined' && typeof global !== 'undefined') {
                // Node.js - use synchronous init with readFileSync
                const currentFile = import.meta.url;
                const currentDir = dirname(fileURLToPath(currentFile));
                const wasmPath = join(currentDir, 'nbt_wasm_bg.wasm');
                const wasmBytes = readFileSync(wasmPath);
                initSync(wasmBytes);
            } else {
                // Browser - use async init with fetch
                await init();
            }
            wasmInitialized = true;
        } catch (error) {
            initPromise = null; // Reset pour retry
            throw new Error(`Failed to initialize NBT WASM: ${error}`);
        }
    })();

    await initPromise;
}

/**
 * Synchronous init check - throws if not initialized
 */
export function ensureInitialized(): void {
    if (!wasmInitialized) {
        throw new Error('NBT WASM not initialized. Call initNbt() first.');
    }
}

/**
 * Check if WASM is ready without throwing
 */
export function isInitialized(): boolean {
    return wasmInitialized;
}

/**
 * Reset initialization state (for testing)
 */
export function resetInitialization(): void {
    wasmInitialized = false;
    initPromise = null;
}