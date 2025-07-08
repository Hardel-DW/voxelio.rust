# Voxelio Rust

Voxelio Rust is a high-performance Rust implementation for Minecraft oriented
packages. This project includes a lot of packages that are used for webapp
development.

## Packages

### [nbt-core](./crates/nbt-core)

Core NBT tag types and binary parsing/writing functionality. Zero-copy parsing
with optimized enum dispatch for maximum performance.

### [nbt-compression](./crates/nbt-compression)

NBT file compression and decompression with automatic format detection. Native
gzip/zlib support that's 10x faster than web APIs.

### [nbt-snbt](./crates/nbt-snbt)

String NBT (SNBT) parsing and formatting using optimized parser combinators.
Complete syntax support with pretty-printing and error handling.

### [nbt-region](./crates/nbt-region)

Minecraft region file (`.mca`) support with efficient chunk management. Lazy
loading and caching for optimal memory usage.

### [nbt-bench](./crates/nbt-bench)

Performance benchmarks comparing against TypeScript implementations.
Comprehensive benchmark suite with criterion for accurate measurements.

## How to start

### Prerequisites

- Rust 1.70 or higher
- Cargo (included with Rust)

### Installation

This command will install all dependencies for all packages.

```bash
git clone <repository-url>
cd voxelio.rust
cargo build --release
```

### Building

```bash
cargo build --release
```

### Testing

The project uses built-in Rust testing. You can run tests for a specific package
with `cargo test --package <package-name>` or just `cargo test` for all
packages.

```bash
cargo test --all
```

### Benchmarking

Run performance benchmarks to see the speed improvements:

```bash
cargo bench
```

### Type checking

The project uses Rust's built-in type checking:

```bash
cargo check --all
```

## Performance Benchmarks

Comprehensive performance measurements on a real Minecraft structure (cube.nbt
with ~60 palette blocks):

### Core Operations

| Operation                  | Time     | Description                                        |
| -------------------------- | -------- | -------------------------------------------------- |
| **Pure NBT Extraction**    | `5.0µs`  | Extract palette from loaded NBT (cube file)        |
| **Pure NBT Extraction**    | `1.15µs` | Extract palette from loaded NBT (taiga file)       |
| **Multiple Field Access**  | `5.35µs` | Extract palette + version + data_version           |
| **NBT Editing (in-place)** | `~5.4ms` | Modify 10 palette entries (mangrove→cherry stairs) |
| **Pure Extraction**        | `5.0µs`  | Single palette extraction (batch baseline)         |

### File Operations

| Operation                  | Time     | Description                                    |
| -------------------------- | -------- | ---------------------------------------------- |
| **NBT File Loading**       | `50.4ms` | Read + decompress + parse NBT file (cube.nbt)  |
| **NBT File Loading**       | `173µs`  | Read + decompress + parse NBT file (taiga.nbt) |
| **Lazy Loading (palette)** | `10.6ms` | Parse only palette field (cube.nbt)            |
| **Lazy Loading (multi)**   | `11.7ms` | Parse palette + version + data fields          |
| **Single File + 10 Ops**   | `50.4ms` | Load file + perform 10 operations              |
| **Batch 5 Files**          | `252ms`  | Process 5 NBT files sequentially               |

### Memory Efficiency

| Operation                | Time     | Description                             |
| ------------------------ | -------- | --------------------------------------- |
| **Multiple Ops (cube)**  | `10.9µs` | Palette + fields + version on same file |
| **Multiple Ops (taiga)** | `2.77µs` | Multiple operations on larger file      |

### Per-Unit Performance

| Metric                   | Time      | Calculation                              |
| ------------------------ | --------- | ---------------------------------------- |
| **Per NBT file (large)** | `~50ms`   | Average file processing time (cube.nbt)  |
| **Per NBT file (small)** | `~173µs`  | Average file processing time (taiga.nbt) |
| **Per palette edit**     | `~0.54ms` | Single block type modification           |
| **Per field extraction** | `~5µs`    | Individual data access                   |

### Key Insights

- **Ultra-fast in-memory operations**: Microsecond-level access to NBT data
- **Optimized decompression**: 300x speed improvement for small files (173µs vs
  54ms)
- **Lazy parsing available**: 5x faster loading when only specific fields needed
  (10.6ms vs 50ms)
- **File size matters**: Performance scales with file size (173µs for 2.5KB vs
  50ms for 254KB)
- **Memory efficient**: Reusing loaded files for multiple operations
- **Scalable**: Linear performance with file count and operation count
- **Consistent**: Low variance, stable performance across runs

_Benchmarks run on Windows 10 with optimized release builds (`cargo bench`)_

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes and add tests
4. Run the test suite: `cargo test --all`
5. Run benchmarks: `cargo bench`
6. Run clippy: `cargo clippy --all -- -D warnings`
7. Commit your changes: `git commit -m 'Add your feature'`
8. Push to the branch: `git push origin feature/your-feature`
9. Create a Pull Request

## Guidelines

### Development Guidelines

- Follow Rust conventions and use `rustfmt`
- Add tests for new functionality
- Ensure all benchmarks maintain performance
- Update documentation for API changes
- No `unsafe` code except for FFI bindings

### Code Quality

- Zero-copy parsing where possible
- Type safety leveraging Rust's type system
- Performance-first optimization for real-world usage
- Full compatibility with existing NBT implementations

## License

MIT License - see individual package licenses for specific terms.
