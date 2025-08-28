# NBT TypeScript - Zero Abstraction WASM

Performant NBT/MCA library for Minecraft files, direct Rust/WASM mapping.

## 🎯 **Design Philosophy**

- **Direct WASM mapping**: No unnecessary wrappers, direct Rust access
- **Native performance**: Delegates EVERYTHING to Rust, TypeScript = bridge only
- **API**: Direct methods, no over-engineering
- **Type-safe**: Strict types without "as" or conversions

## 🚀 **API Core**

### **NbtFile** - Zero Layer

```typescript
import { NbtFile } from "./index";
import initNbt from "./wasm";

// === READ ===
const nbt = NbtFile.read(data);
const lazy = NbtFile.readLazy(data, ["DataVersion", "LastPlayed"]);

// === DIRECT WASM ACCESS ===
const root = nbt.getRoot(); // JsNbtTag direct du Rust
const playerName = nbt.getString("Data.Player.Name");
const level = nbt.getNumber("Data.Player.Level");

// === EDIT ===
nbt.setString("Data.Player.Name", "SuperSteve");

// === I/O ===
const bytes = nbt.write();
```

### **NbtRegion** - Chunk Access Direct

```typescript
import { NbtRegion } from "./index";

// === READ ===
const region = NbtRegion.read(regionData);

// === CHUNK ACCESS - Direct JsNbtTag ===
const chunk = region.getChunk(0, 0); // Returns JsNbtTag | null
if (chunk) {
    const biome = chunk.getString("sections[0].biomes.palette[0]");
    chunk.setString("sections[0].biomes.palette[0]", "minecraft:plains");
}

// === ITERATION ===
region.processChunks((chunk, x, z) => {
    // chunk is JsNbtTag - direct access
    const status = chunk.getString("Level.Status");
    if (status === "postprocessed") {
        chunk.setString("Level.Status", "full");
    }
});
```

### **JsNbtTag** - Rust NBT Tag Direct

```typescript
// Type checking (no conversion)
tag.isNumber();
tag.isString();
tag.isCompound();
tag.isList();

// Direct access (zero copy when possible)
tag.asNumber();
tag.asString();
tag.get(key);
tag.setString(key, value);

// List operations
tag.listLength();
tag.getListItem(index);
```

## 📋 **Types - Zero Conversion**

```typescript
type CompressionFormat = "none" | "gzip" | "zlib";

type NbtValue =
    | number
    | string
    | boolean
    | NbtValue[]
    | { [key: string]: NbtValue }
    | Int8Array
    | Int32Array
    | BigInt64Array;
```

## 🎮 **Use Cases**

### **Player Data**

```typescript
const nbt = NbtFile.read(playerData);
const root = nbt.getRoot();

// Direct access - no conversion
const level = root.getNumber("Data.Player.Level");
const name = root.getString("Data.Player.Name");

// Direct edit
root.setString("Data.Player.Name", "NewName");
```

### **Region Processing**

```typescript
const region = NbtRegion.read(mcaFile);

// Process all chunks directly
region.processChunks((chunk, x, z) => {
    if (chunk.getString("Level.Status") === "empty") {
        chunk.setString("Level.Status", "populated");
    }
});
```

### **Batch Processing**

```typescript
// Process many files efficiently
NbtFile.processBatch(worldFiles, (nbt, index) => {
    const spawn = nbt.getRoot();
    spawn.setString("Data.SpawnX", "0");
    spawn.setString("Data.SpawnZ", "0");
});
```

## 🔧 **Setup**

```bash
npm install
npm run build
```

```typescript
import { NbtFile, NbtRegion } from "./index";
const nbt = NbtFile.read(data);
const root = nbt.getRoot();
```
