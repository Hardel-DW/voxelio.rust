import init from './nbt_wasm';

let wasmInitialized = false;
let initPromise: Promise<void> | null = null;

/**
 * Initialize NBT WASM module
 * Safe to call multiple times
 */
export default async function initNbt(): Promise<void> {
    if (wasmInitialized) return;

    if (initPromise) {
        await initPromise;
        return;
    }

    initPromise = (async () => {
        try {
            await init();
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