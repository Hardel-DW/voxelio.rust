import { parseSnbt as parseSnbtWasm, formatSnbt as formatSnbtWasm, formatSnbtPretty as formatSnbtPrettyWasm, JsNbtTag } from "./nbt_wasm";
import { ensureInitialized } from "./wasm";

/**
 * Parse SNBT string to NBT tag - Direct WASM call
 */
export function parseSnbt(input: string): JsNbtTag {
    ensureInitialized();
    return parseSnbtWasm(input);
}

/**
 * Format NBT tag to SNBT string - Direct WASM call
 */
export function formatSnbt(tag: JsNbtTag): string {
    ensureInitialized();
    return formatSnbtWasm(tag);
}

/**
 * Format NBT tag to pretty SNBT string - Direct WASM call
 */
export function formatSnbtPretty(tag: JsNbtTag): string {
    ensureInitialized();
    return formatSnbtPrettyWasm(tag);
} 