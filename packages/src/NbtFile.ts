import { JsNbtFile, JsNbtTag } from "./nbt_wasm";
import { CompressionFormat } from "./index";
import { ensureInitialized } from "./wasm";

/**
 * NBT File - Direct WASM bridge with zero abstraction
 */
export class NbtFile {
    private constructor(private jsFile: JsNbtFile) { }

    // === STATIC FACTORIES - Direct WASM ===
    static read(data: Uint8Array): NbtFile {
        ensureInitialized();
        return new NbtFile(JsNbtFile.read(data));
    }

    static readLazy(data: Uint8Array, fields: string[]): NbtFile {
        ensureInitialized();
        return new NbtFile(JsNbtFile.readFields(data, fields.join(',')));
    }

    // === PROPERTIES - Direct WASM ===
    get name(): string {
        return this.jsFile.name;
    }

    get compression(): CompressionFormat {
        return this.jsFile.compression as CompressionFormat;
    }

    // === ROOT TAG ACCESS - Direct WASM ===
    getRoot(): JsNbtTag {
        return this.jsFile.root;
    }

    // === BATCH OPERATIONS - High Performance ===
    getMultiplePaths(paths: string[]): Record<string, string> {
        return this.jsFile.getMultiplePaths(paths.join(','));
    }

    // === DIRECT MODIFICATION METHODS - Work on internal root ===
    setStringByPath(path: string, value: string): boolean {
        return this.jsFile.setStringByPath(path, value);
    }

    setNumberByPath(path: string, value: number): boolean {
        return this.jsFile.setNumberByPath(path, value);
    }

    modifyListItem(listPath: string, index: number, key: string, value: string): boolean {
        return this.jsFile.modifyListItem(listPath, index, key, value);
    }

    // === I/O - Direct WASM ===
    write(): Uint8Array {
        return this.jsFile.write();
    }

    // === BATCH PROCESSING - Zero TypeScript Logic ===
    static processBatch(files: Uint8Array[], processor: (nbt: NbtFile, index: number) => void): void {
        ensureInitialized();
        files.forEach((data, index) => {
            processor(new NbtFile(JsNbtFile.read(data)), index);
        });
    }
}