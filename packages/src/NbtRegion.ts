import { NbtChunk } from "./NbtChunk";
import { JsNbtRegion } from "./nbt_wasm";
import { ensureInitialized } from "./wasm";

/**
 * NBT Region - Direct WASM mapping, zero conversion overhead
 */
export class NbtRegion {
    private constructor(private jsRegion: JsNbtRegion) { }

    static read(data: Uint8Array): NbtRegion {
        ensureInitialized();
        return new NbtRegion(JsNbtRegion.read(data));
    }

    static new(): NbtRegion {
        ensureInitialized();
        return new NbtRegion(JsNbtRegion.new());
    }

    getChunkCount(): number {
        return this.jsRegion.chunkCount();
    }

    isEmpty(): boolean {
        return this.jsRegion.isEmpty();
    }

    getChunkPositions(): Array<{ x: number, z: number }> {
        const positions = this.jsRegion.getChunkPositions();
        const result: Array<{ x: number, z: number }> = [];
        for (let i = 0; i < positions.length; i += 2) {
            result.push({ x: positions[i], z: positions[i + 1] });
        }

        return result;
    }

    hasChunk(x: number, z: number): boolean {
        try {
            const chunk = this.jsRegion.getChunk(x, z);
            return chunk !== null && chunk !== undefined;
        } catch {
            return false;
        }
    }

    getChunk(x: number, z: number): NbtChunk | null {
        try {
            const jsTag = this.jsRegion.getChunk(x, z);
            return jsTag ? NbtChunk.fromJsTag(jsTag, x, z) : null;
        } catch {
            return null;
        }
    }

    *iterateChunks(): Generator<NbtChunk, void, unknown> {
        for (const coords of this.getChunkPositions()) {
            const chunk = this.getChunk(coords.x, coords.z);
            if (chunk) yield chunk;
        }
    }

    processChunks(processor: (chunk: NbtChunk, x: number, z: number) => void): void {
        for (const coords of this.getChunkPositions()) {
            const chunk = this.getChunk(coords.x, coords.z);
            if (chunk) processor(chunk, coords.x, coords.z);
        }
    }

    processChunkBatch(
        coordinates: Array<{ x: number, z: number }>,
        processor: (chunk: NbtChunk | null, x: number, z: number) => void
    ): void {
        for (const coord of coordinates) {
            const chunk = this.getChunk(coord.x, coord.z);
            processor(chunk, coord.x, coord.z);
        }
    }

    write(): Uint8Array {
        return new Uint8Array(this.jsRegion.write());
    }

    getStats(): {
        totalChunks: number;
        coordinates: Array<{ x: number, z: number }>;
    } {
        const coordinates = this.getChunkPositions();
        return {
            totalChunks: coordinates.length,
            coordinates
        };
    }
}