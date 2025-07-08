# NBT Region

Minecraft region file (`.mca`) support with efficient chunk management. Provides
lazy loading and caching for optimal memory usage when working with large
Minecraft region files.

## Features

- **Full .mca format** - Complete implementation of Minecraft's region file
  format
- **Lazy loading** - Chunks are parsed only when accessed
- **Efficient caching** - Automatic caching of parsed NBT data
- **32x32 grid** - Proper handling of region coordinate system
- **Round-trip support** - Read and write region files with perfect fidelity

## Usage

### Reading Region Files

```rust
use nbt_region::*;

// Read a region file
let data = std::fs::read("r.0.0.mca")?;
let mut region = Region::read(&data)?;

// Check if a chunk exists
if let Some(chunk) = region.get_chunk(0, 0)? {
    println!("Chunk at (0,0) exists");
    println!("Compression: {}", chunk.get_compression() as u8);
    println!("Size: {} bytes", chunk.size());
}

// Access chunk NBT data (lazy loaded)
if let Some(chunk) = region.get_chunk_mut(0, 0)? {
    let nbt = chunk.get_nbt()?;
    println!("Root NBT: {}", nbt.root_name);
    
    // Access specific data
    let level = nbt.root.get_compound("Level").unwrap();
    let x_pos = level.get_number("xPos") as i32;
    let z_pos = level.get_number("zPos") as i32;
    println!("Chunk position: ({}, {})", x_pos, z_pos);
}
```

### Creating Region Files

```rust
use nbt_region::*;
use nbt_compression::*;
use nbt_core::*;

// Create a new region
let mut region = Region::new();

// Create chunk data
let mut level = std::collections::HashMap::new();
level.insert("xPos".to_string(), NbtTag::int(0));
level.insert("zPos".to_string(), NbtTag::int(0));
level.insert("TerrainPopulated".to_string(), NbtTag::byte(1));

let mut root = std::collections::HashMap::new();
root.insert("Level".to_string(), NbtTag::Compound(level));

let nbt_file = NbtFile::new(
    NbtTag::Compound(root),
    "".to_string(),
    CompressionFormat::Zlib,
    Endian::Big
);

// Create and add chunk
let chunk = Chunk::from_nbt(0, 0, nbt_file, 1234567890)?;
region.set_chunk(chunk)?;

// Write region file
let region_data = region.write()?;
std::fs::write("new_region.mca", region_data)?;
```

### Iterating Chunks

```rust
use nbt_region::*;

let data = std::fs::read("r.0.0.mca")?;
let region = Region::read(&data)?;

// Get all chunk positions
let positions = region.get_chunk_positions();
println!("Region contains {} chunks", positions.len());

for (x, z) in positions {
    println!("Chunk at ({}, {})", x, z);
}

// Iterate over all chunks
for chunk in region.chunks() {
    println!("Chunk at ({}, {}) - {} bytes", 
             chunk.x, chunk.z, chunk.size());
}
```

### Chunk Management

```rust
use nbt_region::*;

let mut region = Region::read(&data)?;

// Add new chunk
let chunk = Chunk::new(1, 1, 2, 1234567890, chunk_data)?;
region.set_chunk(chunk)?;

// Remove chunk
if let Some(removed) = region.remove_chunk(1, 1)? {
    println!("Removed chunk: {} bytes", removed.size());
}

// Check if region is empty
if region.is_empty() {
    println!("Region contains no chunks");
}

println!("Total chunks: {}", region.chunk_count());
```

## Region File Format

Minecraft region files (`.mca`) use a specific format:

- **Header**: 8KB containing chunk locations and timestamps
- **Chunks**: Variable-size compressed NBT data
- **Coordinates**: 32x32 grid (0-31 in both X and Z)
- **Compression**: Usually Zlib, but Gzip is also supported

## Coordinate System

```
Region coordinates: 0-31 in both X and Z
World coordinates: region_x * 32 + chunk_x, region_z * 32 + chunk_z

Example:
- Chunk (0,0) in region r.0.0.mca = world chunk (0,0)
- Chunk (31,31) in region r.0.0.mca = world chunk (31,31)
- Chunk (0,0) in region r.1.0.mca = world chunk (32,0)
```

## Performance

- **Lazy loading** - Chunks parsed only when accessed
- **Memory efficient** - Raw compressed data kept until needed
- **Fast lookup** - O(1) chunk access by coordinates
- **Caching** - Parsed NBT automatically cached for reuse

## License

MIT License
