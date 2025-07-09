import { JsNbtFile, JsNbtTag } from "./nbt_wasm";
import { NbtValue, CompressionFormat } from "./index";
import { ensureInitialized } from "./wasm";

/**
 * NBT File - Direct WASM mapping, zero abstraction
 */
export class NbtFile {
    private constructor(private jsFile: JsNbtFile) { }

    get name(): string {
        return this.jsFile.name;
    }

    get compression(): CompressionFormat {
        return this.jsFile.compression as CompressionFormat;
    }

    static read(data: Uint8Array): NbtFile {
        ensureInitialized();
        return new NbtFile(JsNbtFile.read(data));
    }

    static readLazy(data: Uint8Array, fields: string[]): NbtFile {
        ensureInitialized();
        return new NbtFile(JsNbtFile.readFields(data, fields.join(',')));
    }

    static processBatch(files: Uint8Array[], processor: (nbt: NbtFile, index: number) => void): void {
        ensureInitialized();
        files.forEach((data, index) => {
            processor(new NbtFile(JsNbtFile.read(data)), index);
        });
    }

    getRoot(): JsNbtTag {
        return this.jsFile.root;
    }

    getString(path: string): string | null {
        const tag = this.findTag(path);
        return tag?.isString() ? tag.asString() : null;
    }

    getStringOrThrow(path: string): string {
        const result = this.getString(path);
        if (result === null) throw new Error(`String not found at path: ${path}`);
        return result;
    }

    getNumber(path: string): number | null {
        const tag = this.findTag(path);
        return tag?.isNumber() ? tag.asNumber() : null;
    }

    getNumberOrThrow(path: string): number {
        const result = this.getNumber(path);
        if (result === null) throw new Error(`Number not found at path: ${path}`);
        return result;
    }

    getBool(path: string): boolean | null {
        const num = this.getNumber(path);
        return num !== null ? num !== 0 : null;
    }

    getBoolOrThrow(path: string): boolean {
        const result = this.getBool(path);
        if (result === null) throw new Error(`Boolean not found at path: ${path}`);
        return result;
    }

    getArray(path: string): NbtValue[] | null {
        const tag = this.findTag(path);
        if (!tag?.isList()) return null;

        const result: NbtValue[] = [];
        const length = tag.listLength();
        for (let i = 0; i < length; i++) {
            const item = tag.getListItem(i);
            if (item) result.push(this.tagToValue(item));
        }
        return result;
    }

    getArrayOrThrow(path: string): NbtValue[] {
        const result = this.getArray(path);
        if (result === null) throw new Error(`Array not found at path: ${path}`);
        return result;
    }

    setString(path: string, value: string): void {
        const parent = this.findParentTag(path);
        const key = this.getKeyFromPath(path);
        if (!parent || !parent.setString(key, value)) {
            throw new Error(`Cannot set string at path: ${path}`);
        }
    }

    write(): Uint8Array {
        return this.jsFile.write();
    }

    private findTag(path: string): JsNbtTag | null {
        const parts = path.split('.');
        let current: JsNbtTag | undefined = this.getRoot();

        for (const part of parts) {
            if (!current) return null;

            // Array access: "items[0]"
            const arrayMatch = part.match(/^(.+)\[(\d+)\]$/);
            if (arrayMatch) {
                const [, key, indexStr] = arrayMatch;
                current = current.get(key);
                if (!current) return null;
                current = current.getListItem(parseInt(indexStr));
            } else {
                current = current.get(part);
            }
        }

        return current || null;
    }

    private findParentTag(path: string): JsNbtTag | null {
        const parts = path.split('.');
        if (parts.length === 1) return this.getRoot();

        const parentPath = parts.slice(0, -1).join('.');
        return this.findTag(parentPath);
    }

    private getKeyFromPath(path: string): string {
        const parts = path.split('.');
        return parts[parts.length - 1];
    }

    private tagToValue(tag: JsNbtTag): NbtValue {
        if (tag.isNumber()) return tag.asNumber();
        if (tag.isString()) return tag.asString();
        if (tag.isList()) {
            const result: NbtValue[] = [];
            const length = tag.listLength();
            for (let i = 0; i < length; i++) {
                const item = tag.getListItem(i);
                if (item) result.push(this.tagToValue(item));
            }
            return result;
        }
        if (tag.isCompound()) {
            const result: { [key: string]: NbtValue } = {};
            const keys = tag.keys();
            for (const key of keys) {
                const value = tag.get(key);
                if (value) result[key] = this.tagToValue(value);
            }
            return result;
        }
        return tag.asNumber();
    }
}