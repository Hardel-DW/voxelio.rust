# NBT Core

Core NBT tag types and binary parsing/writing functionality for Minecraft NBT
files. Provides zero-copy parsing with optimized enum dispatch and efficient
memory usage for maximum performance.

## Features

- **Zero-copy parsing** - Memory allocations through direct slice access
- **NBT support** - All 13 NBT tag types with type-safe enum
- **Dual endianness** - Support for Java Edition (big-endian) and Bedrock
  Edition (little-endian)
- **Ergonomic API** - Typed accessors and builder methods
- **Performance optimized** - Optimized read/write operations

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
