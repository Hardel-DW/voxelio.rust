# NBT TypeScript - API Simplifiée & Performante

Librairie TypeScript ultra-performante pour les fichiers NBT/MCA de Minecraft,
basée sur Rust/WASM.

## 🎯 **Design Philosophy**

- **Performance native** : Mapping direct Rust/WASM, zero abstraction inutile
- **API simple** : Pas de over-engineering, méthodes directes
- **Zero-copy** : Réutilisation des buffers Rust autant que possible
- **Type-safe** : Types TypeScript stricts avec gestion d'erreur explicite

## 🚀 **API Core**

### **NbtFile** - Performance-First

```typescript
import { NbtFile } from "./index";
import initNbt from "./wasm";

// Initialize WASM once
initNbt();

// === LECTURE ===
const nbt = NbtFile.read(data); // Lecture complète
const lazy = NbtFile.readLazy(data, ["DataVersion", "LastPlayed"]); // Lazy loading

// === ACCÈS SIMPLE ===
// Safe (retourne null si absent)
const playerName = nbt.getString("Data.Player.Name") ?? "Steve";
const level = nbt.getNumber("Data.Player.Level") ?? 1;
const health = nbt.getNumber("Data.Player.Health") ?? 20;

// Strict (throw si absent/mauvais type)
const dataVersion = nbt.getNumberOrThrow("DataVersion");
const worldName = nbt.getStringOrThrow("Data.LevelName");

// === ÉDITION ===
nbt.setString("Data.Player.Name", "SuperSteve");
nbt.setInt("Data.Player.Level", 50);
nbt.setByte("Data.Player.NewFlag", 1);

// === I/O ===
const bytes = nbt.write();
```

### **Batch Processing** - Haute Performance

```typescript
// Traitement de nombreux fichiers
const files = [file1Data, file2Data, file3Data];

NbtFile.processBatch(files, (nbt, index) => {
    const level = nbt.getNumber("Data.Player.Level") ?? 0;
    nbt.setInt("Data.Player.Level", level + 10);

    console.log(`File ${index}: Level updated to ${level + 10}`);
});

// Lazy batch pour économiser mémoire
const lazyFiles = files.map((data) =>
    NbtFile.readLazy(data, ["Data.Player.Level", "Data.Player.Name"])
);
```

### **Region Files** (.mca)

```typescript
import { NbtRegion } from "./index";

// === LECTURE RÉGION ===
const region = NbtRegion.read(regionData);

// === INFORMATIONS ===
console.log(`Chunks: ${region.getChunkCount()}`);
console.log(`Empty: ${region.isEmpty()}`);
console.log(`Positions:`, region.getChunkPositions());

// === ACCÈS CHUNKS ===
const chunk = region.getChunk(0, 0);
if (chunk) {
    const biome = chunk.getString("sections[0].biomes.palette[0]");
    chunk.setString("sections[0].biomes.palette[0]", "minecraft:plains");
}

// === ITERATION ===
region.processChunks((chunk, x, z) => {
    const status = chunk.getString("Level.Status");
    if (status === "postprocessed") {
        chunk.setString("Level.Status", "full");
    }
});

// Batch de chunks spécifiques
const targetChunks = [{ x: 0, z: 0 }, { x: 1, z: 0 }];
region.processChunkBatch(targetChunks, (chunk, x, z) => {
    if (chunk) {
        console.log(`Processing chunk ${x},${z}`);
    }
});
```

## 🔥 **Performances**

### **Optimisations Clés**

1. **Direct WASM mapping** : Zero abstraction TypeScript
2. **Lazy loading** : Parse seulement les champs nécessaires
3. **Batch processing** : Partage du contexte WASM
4. **Path resolution** : Optimisée côté Rust
5. **Type preservation** : Évite les conversions inutiles

### **Exemple Performance**

```typescript
// ❌ ANCIEN - Over-engineered
const accessor = nbt.createAccessor("Data.Player.Level");
for (let i = 0; i < 1000; i++) {
    const level = accessor.getNumberOrThrow();
}

// ✅ NOUVEAU - Simple et rapide
const level = nbt.getNumber("Data.Player.Level");
// Direct WASM call, path parsé une seule fois
```

## 📋 **Types**

```typescript
// Path navigation
type NbtPath = string | string[];

// Union type simple
type NbtValue =
    | number // Byte, Short, Int, Long, Float, Double
    | string // String
    | boolean // Converti en Byte (0/1)
    | NbtValue[] // List
    | { [key: string]: NbtValue } // Compound
    | Int8Array // ByteArray
    | Int32Array // IntArray
    | BigInt64Array; // LongArray

type CompressionFormat = "none" | "gzip" | "zlib";
```

## 🎮 **Use Cases Minecraft**

### **Player Data Edition**

```typescript
const nbt = NbtFile.read(playerData);

// Boost player
nbt.setInt("Data.Player.Level", 100);
nbt.setFloat("Data.Player.Health", 20.0);
nbt.setString("Data.Player.Name", "SuperPlayer");

// Inventory modification
const inventory = nbt.getArray("Data.Player.Inventory");
// Process inventory items...
```

### **World Processing**

```typescript
// Batch process multiple worlds
const worldFiles = [...]; // Array de Uint8Array

NbtFile.processBatch(worldFiles, (world, index) => {
    const spawnX = world.getNumber('Data.SpawnX') ?? 0;
    const spawnZ = world.getNumber('Data.SpawnZ') ?? 0;
    
    // Recentrer spawn
    world.setInt('Data.SpawnX', 0);
    world.setInt('Data.SpawnZ', 0);
    
    console.log(`World ${index}: Spawn moved from (${spawnX}, ${spawnZ}) to (0, 0)`);
});
```

### **Chunk Processing**

```typescript
const region = NbtRegion.read(mcaFile);

// Convertir tous les chunks ocean en plains
region.processChunks((chunk, x, z) => {
    const sections = chunk.getArray("Level.Sections");
    if (sections) {
        // Process biome palettes...
        chunk.setString("Level.Biomes.palette[0]", "minecraft:plains");
    }
});

const modifiedBytes = region.write();
```

## 🔧 **Setup**

```bash
npm install
npm run build
```

```typescript
import initNbt, { NbtFile, NbtRegion } from "./index";

// Initialize once at app start
initNbt();

// Use anywhere
const nbt = NbtFile.read(data);
```

---

## 🧠 **Architecture**

- **Rust Core** : Parsing, compression, région handling
- **WASM Bridge** : Types JsNbtFile, JsNbtTag, JsNbtRegion
- **TypeScript Layer** : API simple wrappant WASM directement

**Philosophy** : Laisser Rust faire le travail lourd, TypeScript just expose
l'API proprement.

Performance maximale, complexité minimale. 🚀
