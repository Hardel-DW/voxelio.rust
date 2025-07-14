/* tslint:disable */
/* eslint-disable */
export function nbt_file_read(data: Uint8Array): number;
export function nbt_file_read_lazy(data: Uint8Array, fields: Array<any>): number;
export function nbt_file_write(handle: number): Uint8Array;
export function nbt_file_dispose(handle: number): void;
export function nbt_get_string(handle: number, key: string): string;
export function nbt_get_number(handle: number, key: string): number;
export function nbt_get_root(handle: number): number;
export function nbt_tag_type(handle: number): number;
export function nbt_tag_as_string(handle: number): string;
export function nbt_tag_as_number(handle: number): number;
export function nbt_tag_get_compound_keys(handle: number): string[];
export function nbt_tag_get_compound_value(handle: number, key: string): number;
export function nbt_tag_get_list_length(handle: number): number;
export function nbt_tag_get_list_item(handle: number, index: number): number;
export function nbt_file_set_list_item_string(file_handle: number, path: string, index: number, key: string, value: string): void;
export function nbt_tag_dispose(handle: number): void;
export function nbt_tag_get_string(handle: number, key: string): string;
export function nbt_tag_get_number(handle: number, key: string): number;
export function nbt_tag_set_string(handle: number, key: string, value: string): void;
export function nbt_tag_set_number(handle: number, key: string, value: number): void;
export function nbt_region_read(data: Uint8Array): number;
export function nbt_region_write(handle: number): Uint8Array;
export function nbt_region_dispose(handle: number): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly nbt_file_read: (a: number, b: number, c: number) => void;
  readonly nbt_file_read_lazy: (a: number, b: number, c: number, d: number) => void;
  readonly nbt_file_write: (a: number, b: number) => void;
  readonly nbt_file_dispose: (a: number) => void;
  readonly nbt_get_string: (a: number, b: number, c: number, d: number) => void;
  readonly nbt_get_number: (a: number, b: number, c: number, d: number) => void;
  readonly nbt_get_root: (a: number, b: number) => void;
  readonly nbt_tag_type: (a: number, b: number) => void;
  readonly nbt_tag_as_string: (a: number, b: number) => void;
  readonly nbt_tag_as_number: (a: number, b: number) => void;
  readonly nbt_tag_get_compound_keys: (a: number, b: number) => void;
  readonly nbt_tag_get_compound_value: (a: number, b: number, c: number, d: number) => void;
  readonly nbt_tag_get_list_length: (a: number, b: number) => void;
  readonly nbt_tag_get_list_item: (a: number, b: number, c: number) => void;
  readonly nbt_file_set_list_item_string: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly nbt_tag_dispose: (a: number) => void;
  readonly nbt_tag_get_string: (a: number, b: number, c: number, d: number) => void;
  readonly nbt_tag_get_number: (a: number, b: number, c: number, d: number) => void;
  readonly nbt_tag_set_string: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly nbt_tag_set_number: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly nbt_region_read: (a: number, b: number, c: number) => void;
  readonly nbt_region_write: (a: number, b: number) => void;
  readonly nbt_region_dispose: (a: number) => void;
  readonly __wbindgen_export_0: (a: number, b: number) => number;
  readonly __wbindgen_export_1: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_2: (a: number, b: number, c: number) => void;
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
