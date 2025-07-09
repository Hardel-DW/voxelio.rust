export type CompressionFormat = 'none' | 'gzip' | 'zlib';
export { NbtFile } from "./NbtFile";
export { NbtRegion } from "./NbtRegion";
export type { JsNbtTag, JsNbtFile, JsNbtRegion } from "./nbt_wasm";
export { parseSnbt, formatSnbt, formatSnbtPretty } from "./snbt";
export { detectCompression, getVersion } from "./utils";
export { default as initNbt } from "./wasm";