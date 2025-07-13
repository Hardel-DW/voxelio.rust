import { nbt_tag_type, nbt_tag_as_string, nbt_tag_as_number, nbt_tag_get_compound_keys, nbt_tag_get_compound_value, nbt_tag_get_string, nbt_tag_get_number, nbt_tag_set_string, nbt_tag_set_number, nbt_tag_get_list_length, nbt_tag_get_list_item, nbt_tag_dispose } from "./nbt_wasm";
import { NbtType } from "./index";

export class NbtTag {
    private handle: number;
    private disposed = false;

    constructor(handle: number) {
        this.handle = handle;
    }

    getType(): NbtType {
        this.ensureNotDisposed();
        return nbt_tag_type(this.handle) as NbtType;
    }

    asString(): string {
        this.ensureNotDisposed();
        return nbt_tag_as_string(this.handle);
    }

    asNumber(): number {
        this.ensureNotDisposed();
        return nbt_tag_as_number(this.handle);
    }

    getCompoundKeys(): string[] {
        this.ensureNotDisposed();
        return nbt_tag_get_compound_keys(this.handle);
    }

    getCompoundValue(key: string): NbtTag {
        this.ensureNotDisposed();
        const tagHandle = nbt_tag_get_compound_value(this.handle, key);
        return new NbtTag(tagHandle);
    }

    getString(key: string): string {
        this.ensureNotDisposed();
        return nbt_tag_get_string(this.handle, key);
    }

    getNumber(key: string): number {
        this.ensureNotDisposed();
        return nbt_tag_get_number(this.handle, key);
    }

    setString(key: string, value: string): void {
        this.ensureNotDisposed();
        nbt_tag_set_string(this.handle, key, value);
    }

    setNumber(key: string, value: number): void {
        this.ensureNotDisposed();
        nbt_tag_set_number(this.handle, key, value);
    }

    getListLength(): number {
        this.ensureNotDisposed();
        return nbt_tag_get_list_length(this.handle);
    }

    getListItem(index: number): NbtTag {
        this.ensureNotDisposed();
        const tagHandle = nbt_tag_get_list_item(this.handle, index);
        return new NbtTag(tagHandle);
    }

    *iterateList(): Generator<NbtTag> {
        const length = this.getListLength();
        for (let i = 0; i < length; i++) {
            yield this.getListItem(i);
        }
    }

    *iterateCompound(): Generator<[string, NbtTag]> {
        const keys = this.getCompoundKeys();
        for (const key of keys) {
            yield [key, this.getCompoundValue(key)];
        }
    }

    processCompound<T>(processor: (key: string, value: NbtTag) => T): T[] {
        const results: T[] = [];
        for (const [key, value] of this.iterateCompound()) {
            using tag = value;
            results.push(processor(key, tag));
        }
        return results;
    }

    processList<T>(processor: (item: NbtTag, index: number) => T): T[] {
        const results: T[] = [];
        const length = this.getListLength();
        for (let i = 0; i < length; i++) {
            using item = this.getListItem(i);
            results.push(processor(item, i));
        }
        return results;
    }

    private ensureNotDisposed() {
        if (this.disposed) {
            throw new Error('NbtTag has been disposed');
        }
    }

    dispose(): void {
        if (!this.disposed) {
            nbt_tag_dispose(this.handle);
            this.disposed = true;
        }
    }

    [Symbol.dispose](): void {
        this.dispose();
    }
}