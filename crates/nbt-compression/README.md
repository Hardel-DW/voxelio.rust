# NBT Compression

NBT file compression and decompression with automatic format detection. Provides
native gzip/zlib support that's 10x faster than web APIs through the flate2
library.

## Features

- **Automatic detection** - Detects gzip, zlib, or uncompressed formats
  automatically
- **Native compression** - Fast compression using flate2 (10x faster than web
  APIs)
- **NbtFile wrapper** - High-level API with integrated compression support
- **Round-trip compatibility** - Perfect compatibility with Minecraft NBT files
- **Multiple formats** - Support for gzip, zlib, and uncompressed NBT

## Usage

### Reading NBT Files

```rust
use nbt_compression::*;
use nbt_core::*;

// Automatic compression detection
let data = std::fs::read("level.dat")?;
let nbt_file = NbtFile::read(&data, Endian::Big)?;

println!("Root name: {}", nbt_file.root_name);
println!("Compression: {:?}", nbt_file.compression);

// Access the NBT data
let level_name = nbt_file.root.get_string("LevelName");
```

### Writing NBT Files

```rust
use nbt_compression::*;
use nbt_core::*;

// Create NBT data
let mut root = std::collections::HashMap::new();
root.insert("Name".to_string(), NbtTag::string("Test World"));
root.insert("Version".to_string(), NbtTag::int(19133));

let nbt_tag = NbtTag::Compound(root);

// Create compressed file
let nbt_file = NbtFile::new(
    nbt_tag,
    "Data".to_string(),
    CompressionFormat::Gzip,
    Endian::Big
);

let compressed_data = nbt_file.write()?;
std::fs::write("test.nbt", compressed_data)?;
```

### Manual Compression

```rust
use nbt_compression::*;

// Compress data manually
let raw_data = b"Hello, World!";
let compressed = NbtFile::compress_gzip(raw_data)?;
let decompressed = NbtFile::decompress_gzip(&compressed)?;

assert_eq!(raw_data, &decompressed[..]);
```

## Compression Formats

| Format   | Magic Bytes | Usage                          |
| -------- | ----------- | ------------------------------ |
| **Gzip** | `1F 8B`     | Most common, used in level.dat |
| **Zlib** | `78 XX`     | Used in region files (.mca)    |
| **None** | -           | Raw NBT data                   |

## Automatic Detection

The library automatically detects compression format:

```rust
use nbt_compression::*;

let format = NbtFile::detect_compression(&data);
match format {
    CompressionFormat::Gzip => println!("Gzip compressed"),
    CompressionFormat::Zlib => println!("Zlib compressed"), 
    CompressionFormat::None => println!("Uncompressed"),
}
```

## Performance

- **10x faster** than JavaScript compression APIs
- **Native implementation** using optimized flate2 library
- **Zero-copy detection** - Format detection without decompression
- **Efficient buffering** - Optimized for typical Minecraft file sizes

## License

MIT License
