import { nbt_file_read, nbt_file_write, nbt_file_dispose, nbt_get_string, nbt_get_number, nbt_get_root, nbt_file_set_list_item_string } from "./nbt";
import { NbtTag } from "./NbtTag";
import { ensureWasmInit } from "./wasm";

export class NbtFile {
    private handle: number;
    private disposed = false;

    private constructor(handle: number) {
        this.handle = handle;
    }

    static async from(data: Uint8Array): Promise<NbtFile> {
        await ensureWasmInit();
        const handle = nbt_file_read(data);
        return new NbtFile(handle);
    }

    static async processBatch<T>(
        files: Uint8Array[],
        processor: (nbt: NbtFile, index: number) => T
    ): Promise<T[]> {
        const results: T[] = [];
        for (let i = 0; i < files.length; i++) {
            using nbt = await NbtFile.from(files[i]);
            results.push(processor(nbt, i));
        }
        return results;
    }

    getString(key: string): string {
        this.ensureNotDisposed();
        return nbt_get_string(this.handle, key);
    }

    getNumber(key: string): number {
        this.ensureNotDisposed();
        return nbt_get_number(this.handle, key);
    }

    getRoot(): NbtTag {
        this.ensureNotDisposed();
        const tagHandle = nbt_get_root(this.handle);
        return new NbtTag(tagHandle);
    }

    process<T>(processor: (root: NbtTag) => T): T {
        this.ensureNotDisposed();
        using root = this.getRoot();
        return processor(root);
    }

    write(): Uint8Array {
        this.ensureNotDisposed();
        return nbt_file_write(this.handle);
    }

    setListItemString(path: string, index: number, key: string, value: string): void {
        this.ensureNotDisposed();
        nbt_file_set_list_item_string(this.handle, path, index, key, value);
    }

    private ensureNotDisposed() {
        if (this.disposed) {
            throw new Error('NbtFile has been disposed');
        }
    }

    dispose(): void {
        if (!this.disposed) {
            nbt_file_dispose(this.handle);
            this.disposed = true;
        }
    }

    [Symbol.dispose](): void {
        this.dispose();
    }
}
