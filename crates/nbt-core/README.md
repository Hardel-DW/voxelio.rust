# NBT Core

Core NBT tag types and binary parsing/writing functionality for Minecraft NBT
files. Provides zero-copy parsing with optimized enum dispatch and efficient
memory usage for maximum performance.

## Features

- **Zero-copy parsing** - Minimal memory allocations through direct slice access
- **Complete NBT support** - All 13 NBT tag types with type-safe enum
- **Dual endianness** - Support for Java Edition (big-endian) and Bedrock
  Edition (little-endian)
- **Ergonomic API** - Typed accessors and builder methods for easy usage
- **Performance optimized** - Optimized read/write operations for speed

## Usage

### Basic Reading

```rust
use nbt_core::*;

let data = include_bytes!("test.nbt");
let mut reader = NbtReader::new(data, Endian::Big);
let tag = reader.read_tag(10)?; // Read compound tag

if let NbtTag::Compound(root) = tag {
    let name = root.get("Name").unwrap().as_string();
    println!("Name: {}", name);
}
```

### Building NBT Data

```rust
use nbt_core::*;

let mut root = std::collections::HashMap::new();
root.insert("Name".to_string(), NbtTag::string("Test"));
root.insert("Value".to_string(), NbtTag::int(42));

let nbt = NbtTag::Compound(root);

let mut writer = NbtWriter::new(Endian::Big);
writer.write_tag(&nbt)?;
let bytes = writer.into_bytes();
```

### Typed Access

```rust
use nbt_core::*;

// Safe typed access with defaults
let name = tag.get_string("Name");        // Returns "" if not found
let value = tag.get_number("Value");      // Returns 0.0 if not found
let enabled = tag.get_bool("Enabled");    // Returns false if not found
```

## NBT Tag Types

All Minecraft NBT tag types are supported:

| Tag Type    | Rust Type                 | Description              |
| ----------- | ------------------------- | ------------------------ |
| `End`       | `()`                      | End of compound marker   |
| `Byte`      | `i8`                      | 8-bit signed integer     |
| `Short`     | `i16`                     | 16-bit signed integer    |
| `Int`       | `i32`                     | 32-bit signed integer    |
| `Long`      | `i64`                     | 64-bit signed integer    |
| `Float`     | `f32`                     | 32-bit floating point    |
| `Double`    | `f64`                     | 64-bit floating point    |
| `ByteArray` | `Vec<i8>`                 | Array of bytes           |
| `String`    | `String`                  | UTF-8 string             |
| `List`      | `Vec<NbtTag>`             | List of same-type tags   |
| `Compound`  | `HashMap<String, NbtTag>` | Map of named tags        |
| `IntArray`  | `Vec<i32>`                | Array of 32-bit integers |
| `LongArray` | `Vec<i64>`                | Array of 64-bit integers |

## Performance

- **Zero-copy reading** - Direct access to input data without copying
- **Optimized writing** - Efficient buffer management with pre-allocation
- **Type dispatch** - Compile-time enum dispatch for maximum speed
- **Memory efficient** - Minimal allocations and smart buffer reuse

## License

MIT License
