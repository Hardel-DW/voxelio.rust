import { JsNbtRegion, JsNbtTag } from "./nbt_wasm";
import { ensureInitialized } from "./wasm";

/**
 * NBT Region - Direct WASM bridge with zero abstraction
 */
export class NbtRegion {
    private constructor(private jsRegion: JsNbtRegion) { }

    // === STATIC FACTORIES - Direct WASM ===
    static read(data: Uint8Array): NbtRegion {
        ensureInitialized();
        return new NbtRegion(JsNbtRegion.read(data));
    }

    static new(): NbtRegion {
        ensureInitialized();
        return new NbtRegion(JsNbtRegion.new());
    }

    // === PROPERTIES - Direct WASM ===
    get chunkCount(): number {
        return this.jsRegion.chunkCount();
    }

    get isEmpty(): boolean {
        return this.jsRegion.isEmpty();
    }

    // === CHUNK POSITIONS - Direct WASM ===
    getChunkPositions(): Array<{ x: number, z: number }> {
        const positions = this.jsRegion.getChunkPositions();
        const result: Array<{ x: number, z: number }> = [];
        for (let i = 0; i < positions.length; i += 2) {
            result.push({ x: positions[i], z: positions[i + 1] });
        }
        return result;
    }

    // === CHUNK ACCESS - Direct WASM ===
    getChunk(x: number, z: number): JsNbtTag | null {
        return this.jsRegion.getChunk(x, z) || null;
    }

    // === ITERATION - Simple and Direct ===
    processChunks(processor: (chunk: JsNbtTag, x: number, z: number) => void): void {
        for (const { x, z } of this.getChunkPositions()) {
            const chunk = this.getChunk(x, z);
            if (chunk) processor(chunk, x, z);
        }
    }

    // === I/O - Direct WASM ===
    write(): Uint8Array {
        return this.jsRegion.write();
    }
}