# @voxelio/nbt-wasm

Fast, modern NBT (Named Binary Tag) library for web browsers using WebAssembly.

🚀 **10x+ faster** than pure JavaScript implementations\
⚡ **Zero-copy parsing** with optimized memory usage\
🧩 **All NBT formats** - Files, SNBT, Regions\
🎯 **Modern TypeScript** API with full type safety\
📦 **Tiny bundle** - Optimized WASM for web

## Installation

```bash
npm install @voxelio/nbt-wasm
```

## Quick Start

```typescript
import { initNbt, loadNbtFile, parseSnbtString } from "@voxelio/nbt-wasm";

// Initialize WASM module once
await initNbt();

// Parse SNBT strings
const tag = parseSnbtString('{name: "Steve", level: 42}');
console.log(tag.getString("name")); // "Steve"
console.log(tag.getNumber("level")); // 42

// Load NBT files
const file = await loadNbtFile(nbtFileBlob);
console.log(file.compression); // "gzip" | "zlib" | "none"
console.log(file.root.toJson()); // Full data as JSON
```

## API Reference

### Core Functions

#### `initNbt(): Promise<void>`

Initialize the WASM module. Call once before using any NBT functions.

#### `parseSnbtString(snbt: string): NbtTag`

Parse SNBT (String NBT) format to NBT tag.

```typescript
const tag = parseSnbtString('{player: {name: "Steve", health: 20.0f}}');
const playerName = tag.get("player")?.getString("name");
```

#### `formatSnbtString(tag: NbtTag): string`

Format NBT tag back to SNBT string.

#### `readNbt(data: Uint8Array): NbtFile`

Read NBT file from bytes with automatic compression detection.

#### `readNbtFields(data: Uint8Array, fields?: string): NbtFile`

Read NBT file with selective field parsing for performance.

```typescript
// Only parse specific fields
const nbt = readNbtFields(data, "Player,Level,Data");
```

### Region Files

#### `readRegion(data: Uint8Array): NbtRegion`

Read Minecraft region file (.mca).

```typescript
const region = readRegion(mcaData);
console.log(region.chunkCount());

const chunks = parseChunkPositions(region.getChunkPositions());
for (const { x, z } of chunks) {
    const chunk = region.getChunk(x, z);
    // Process chunk data...
}
```

### Type System

#### `NbtTag`

Main NBT data type with type-safe access methods:

```typescript
interface NbtTag {
    readonly typeId: number;

    // Value accessors
    asNumber(): number;
    asString(): string;

    // Compound accessors
    get(key: string): NbtTag | undefined;
    getString(key: string): string;
    getNumber(key: string): number;
    getBool(key: string): boolean;
    keys(): string[];

    // Type checking
    isNumber(): boolean;
    isString(): boolean;
    isCompound(): boolean;
    isList(): boolean;

    // JSON conversion
    toJson(): unknown;
}
```

#### `NbtFile`

Represents a complete NBT file with metadata:

```typescript
interface NbtFile {
    readonly root: NbtTag;
    readonly name: string;
    readonly compression: "none" | "gzip" | "zlib";
}
```

## Performance

The WASM implementation provides significant performance benefits:

- **Parsing**: 10-50x faster than pure JS
- **Memory**: Zero-copy reading, minimal allocations
- **Bundle size**: ~50KB WASM vs ~200KB pure JS
- **Selective parsing**: Only parse needed fields

## Browser Support

- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- Node.js 16+ (with WASM support)

## Examples

### File Upload Handler

```typescript
import { initNbt, loadNbtFile } from "@voxelio/nbt-wasm";

await initNbt();

fileInput.addEventListener("change", async (e) => {
    const file = e.target.files[0];
    if (!file) return;

    try {
        const nbt = await loadNbtFile(file);
        displayNbtData(nbt.root.toJson());
    } catch (error) {
        console.error("Invalid NBT file:", error);
    }
});
```

### SNBT Editor

```typescript
import { formatSnbtString, parseSnbtString } from "@voxelio/nbt-wasm";

function updateSnbt(input: string) {
    try {
        const tag = parseSnbtString(input);
        const formatted = formatSnbtString(tag);
        editor.setValue(formatted);
    } catch (error) {
        showError("Invalid SNBT syntax");
    }
}
```

### Region Viewer

```typescript
import { parseChunkPositions, readRegion } from "@voxelio/nbt-wasm";

async function loadRegion(mcaFile: File) {
    const data = new Uint8Array(await mcaFile.arrayBuffer());
    const region = readRegion(data);

    const chunks = parseChunkPositions(region.getChunkPositions());
    console.log(`Region contains ${chunks.length} chunks`);

    // Load chunk data on demand
    chunks.forEach(({ x, z }) => {
        const chunk = region.getChunk(x, z);
        if (chunk) {
            renderChunk(x, z, chunk);
        }
    });
}
```

## License

MIT
