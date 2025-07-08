/* tslint:disable */
/* eslint-disable */
/**
 * Set panic hook for better debugging
 */
export function main(): void;
/**
 * Parse SNBT string to NBT tag
 */
export function parseSnbt(input: string): JsNbtTag;
/**
 * Format NBT tag to SNBT string
 */
export function formatSnbt(tag: JsNbtTag): string;
/**
 * Detect compression format from bytes
 */
export function detectCompression(data: Uint8Array): string;
/**
 * Get version info
 */
export function getVersion(): string;
/**
 * NBT file wrapper - handles all compression formats
 */
export class JsNbtFile {
  private constructor();
  free(): void;
  /**
   * Read NBT file from bytes
   */
  static read(data: Uint8Array): JsNbtFile;
  /**
   * Read NBT file with selective field parsing (performance optimization)
   */
  static readFields(data: Uint8Array, fields: string): JsNbtFile;
  /**
   * Get mutable root tag (for editing)
   */
  getRootMut(): JsNbtTag;
  /**
   * Update root from modified JsNbtTag
   */
  setRoot(new_root: JsNbtTag): void;
  /**
   * Direct edit methods to avoid copy issues - using same logic as Rust example
   */
  setStringInListItem(path: string, index: number, key: string, value: string): boolean;
  /**
   * Get string from list item
   */
  getStringFromListItem(path: string, index: number, key: string): string;
  /**
   * Write NBT file to bytes
   */
  write(): Uint8Array;
  /**
   * Create a new NBT file from SNBT string
   */
  static fromSnbt(snbt: string, name: string, compression: string): JsNbtFile;
  /**
   * Get root tag
   */
  readonly root: JsNbtTag;
  /**
   * Get file name
   */
  readonly name: string;
  /**
   * Get compression format as string
   */
  readonly compression: string;
}
/**
 * NBT region file wrapper
 */
export class JsNbtRegion {
  private constructor();
  free(): void;
  /**
   * Read region from bytes
   */
  static read(data: Uint8Array): JsNbtRegion;
  /**
   * Create empty region
   */
  static new(): JsNbtRegion;
  /**
   * Write region to bytes
   */
  write(): Uint8Array;
  /**
   * Get chunk count
   */
  chunkCount(): number;
  /**
   * Check if region is empty
   */
  isEmpty(): boolean;
  /**
   * Get chunk positions as flat array [x1, z1, x2, z2, ...]
   */
  getChunkPositions(): Int32Array;
  /**
   * Get chunk data as NBT tag
   */
  getChunk(x: number, z: number): JsNbtTag | undefined;
}
/**
 * NBT tag wrapper for JavaScript - single point of truth
 */
export class JsNbtTag {
  private constructor();
  free(): void;
  /**
   * Get as number (0 if not numeric)
   */
  asNumber(): number;
  /**
   * Get as string (empty if not string)
   */
  asString(): string;
  /**
   * Get nested value by key (compound only)
   */
  get(key: string): JsNbtTag | undefined;
  /**
   * Get string value by key
   */
  getString(key: string): string;
  /**
   * Get number value by key
   */
  getNumber(key: string): number;
  /**
   * Get boolean value by key
   */
  getBool(key: string): boolean;
  /**
   * Type checking
   */
  isNumber(): boolean;
  isString(): boolean;
  isCompound(): boolean;
  isList(): boolean;
  /**
   * Get keys for compound tags
   */
  keys(): string[];
  /**
   * Set a string value by key (compound only)
   */
  setString(key: string, value: string): boolean;
  /**
   * Get list length
   */
  listLength(): number;
  /**
   * Get item from list by index
   */
  getListItem(index: number): JsNbtTag | undefined;
  /**
   * Set string value in list item compound by index and key
   */
  setStringInListItem(index: number, key: string, value: string): boolean;
  /**
   * Get string value from list item compound by index and key
   */
  getStringFromListItem(index: number, key: string): string;
  /**
   * Convert to JSON for JavaScript consumption
   */
  toJson(): any;
  /**
   * Get tag type ID (matches TypeScript NbtType enum)
   */
  readonly typeId: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: () => void;
  readonly __wbg_jsnbttag_free: (a: number, b: number) => void;
  readonly jsnbttag_typeId: (a: number) => number;
  readonly jsnbttag_asNumber: (a: number) => number;
  readonly jsnbttag_asString: (a: number) => [number, number];
  readonly jsnbttag_get: (a: number, b: number, c: number) => number;
  readonly jsnbttag_getString: (a: number, b: number, c: number) => [number, number];
  readonly jsnbttag_getNumber: (a: number, b: number, c: number) => number;
  readonly jsnbttag_getBool: (a: number, b: number, c: number) => number;
  readonly jsnbttag_isNumber: (a: number) => number;
  readonly jsnbttag_isString: (a: number) => number;
  readonly jsnbttag_isCompound: (a: number) => number;
  readonly jsnbttag_isList: (a: number) => number;
  readonly jsnbttag_keys: (a: number) => [number, number];
  readonly jsnbttag_setString: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly jsnbttag_listLength: (a: number) => number;
  readonly jsnbttag_getListItem: (a: number, b: number) => number;
  readonly jsnbttag_setStringInListItem: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly jsnbttag_getStringFromListItem: (a: number, b: number, c: number, d: number) => [number, number];
  readonly jsnbttag_toJson: (a: number) => any;
  readonly __wbg_jsnbtfile_free: (a: number, b: number) => void;
  readonly jsnbtfile_read: (a: number, b: number) => [number, number, number];
  readonly jsnbtfile_readFields: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly jsnbtfile_root: (a: number) => number;
  readonly jsnbtfile_getRootMut: (a: number) => number;
  readonly jsnbtfile_setRoot: (a: number, b: number) => void;
  readonly jsnbtfile_setStringInListItem: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly jsnbtfile_getStringFromListItem: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly jsnbtfile_name: (a: number) => [number, number];
  readonly jsnbtfile_compression: (a: number) => [number, number];
  readonly jsnbtfile_write: (a: number) => [number, number, number, number];
  readonly jsnbtfile_fromSnbt: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number];
  readonly parseSnbt: (a: number, b: number) => [number, number, number];
  readonly formatSnbt: (a: number) => [number, number];
  readonly __wbg_jsnbtregion_free: (a: number, b: number) => void;
  readonly jsnbtregion_read: (a: number, b: number) => [number, number, number];
  readonly jsnbtregion_new: () => number;
  readonly jsnbtregion_write: (a: number) => [number, number, number, number];
  readonly jsnbtregion_chunkCount: (a: number) => number;
  readonly jsnbtregion_isEmpty: (a: number) => number;
  readonly jsnbtregion_getChunkPositions: (a: number) => [number, number];
  readonly jsnbtregion_getChunk: (a: number, b: number, c: number) => [number, number, number];
  readonly detectCompression: (a: number, b: number) => [number, number];
  readonly getVersion: () => [number, number];
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
