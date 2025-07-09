import { NbtFile } from "./NbtFile";
import { NbtValue } from "./index";
import { JsNbtTag, JsNbtFile } from "./nbt_wasm";

/**
 * NBT Chunk - Simple wrapper avec coordonnées, délégation minimale
 */
export class NbtChunk {
    private constructor(
        private nbtFile: NbtFile,
        public readonly x: number,
        public readonly z: number,
        public readonly timestamp: number = Date.now()
    ) { }

    static create(data: Uint8Array, x: number, z: number, timestamp: number = Date.now()): NbtChunk {
        return new NbtChunk(NbtFile.read(data), x, z, timestamp);
    }

    static fromJsTag(jsTag: JsNbtTag, x: number, z: number, timestamp: number = Date.now()): NbtChunk {
        const jsonData = jsTag.toJson();
        const jsonStr = JSON.stringify(jsonData);
        const bytes = new TextEncoder().encode(jsonStr);
        return new NbtChunk(NbtFile.read(bytes), x, z, timestamp);
    }

    getString(path: string): string | null {
        return this.nbtFile.getString(path);
    }

    getStringOrThrow(path: string): string {
        return this.nbtFile.getStringOrThrow(path);
    }

    getNumber(path: string): number | null {
        return this.nbtFile.getNumber(path);
    }

    getNumberOrThrow(path: string): number {
        return this.nbtFile.getNumberOrThrow(path);
    }

    getBool(path: string): boolean | null {
        return this.nbtFile.getBool(path);
    }

    getBoolOrThrow(path: string): boolean {
        return this.nbtFile.getBoolOrThrow(path);
    }

    getArray(path: string): NbtValue[] | null {
        return this.nbtFile.getArray(path);
    }

    getArrayOrThrow(path: string): NbtValue[] {
        return this.nbtFile.getArrayOrThrow(path);
    }

    setString(path: string, value: string): void {
        this.nbtFile.setString(path, value);
    }

    getNbtFile(): NbtFile {
        return this.nbtFile;
    }

    write(): Uint8Array {
        return this.nbtFile.write();
    }
}