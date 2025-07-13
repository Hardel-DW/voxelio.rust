import { fileURLToPath } from "node:url";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { initSync } from "./nbt";

let wasmInitialized = false;

export async function ensureWasmInit(): Promise<void> {
    if (!wasmInitialized) {
        try {
            const __filename = fileURLToPath(import.meta.url);
            const __dirname = dirname(__filename);
            const wasmPath = join(__dirname, 'nbt_wasm_bg.wasm');
            const wasmBuffer = readFileSync(wasmPath);
            initSync({ module: wasmBuffer });
            wasmInitialized = true;
        } catch (error) {
            console.error('Failed to initialize WASM:', error);
            throw error;
        }
    }
}