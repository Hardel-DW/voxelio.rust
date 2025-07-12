import { ensureWasmInit } from "./wasm";
import { nbt_region_read, nbt_region_write, nbt_region_dispose } from "./nbt_wasm";

export class NbtRegion {
    private handle: number;
    private disposed = false;

    private constructor(handle: number) {
        this.handle = handle;
    }

    static async from(data: Uint8Array): Promise<NbtRegion> {
        await ensureWasmInit();
        const handle = nbt_region_read(data);
        return new NbtRegion(handle);
    }

    write(): Uint8Array {
        this.ensureNotDisposed();
        return nbt_region_write(this.handle);
    }

    private ensureNotDisposed() {
        if (this.disposed) {
            throw new Error('NbtRegion has been disposed');
        }
    }

    dispose() {
        if (!this.disposed) {
            nbt_region_dispose(this.handle);
            this.disposed = true;
        }
    }

    [Symbol.dispose]() {
        this.dispose();
    }
}