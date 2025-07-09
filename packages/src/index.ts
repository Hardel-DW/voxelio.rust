// Types core - Mapping direct Rust
export type CompressionFormat = 'none' | 'gzip' | 'zlib';

// Union type ultra-simple - délègue la complexité à Rust
export type NbtValue =
    | number
    | string
    | boolean
    | NbtValue[]
    | { [key: string]: NbtValue }
    | Int8Array
    | Int32Array
    | BigInt64Array;

// API exports - Zero abstraction
export { NbtFile } from "./NbtFile";
export { NbtRegion } from "./NbtRegion";
export { NbtChunk } from "./NbtChunk";

// WASM init
export { default as initNbt } from "./wasm";