# Project

This project is designed to read and create .nbt format files. The proprietary
Minecraft format for storing structure and region data.

il nous faut pouvoir traiter une quantités collossalle de fichiers nbt en peu de
temps, car ont souhaite lire des structures composés de plusieurs centaines de
fichiers nbt.

## Goals

- We want to create a simple and universal system to traverse and/or create .nbt
  files.
- We want only one way to do it in the code.
- We want the maximum of performance with rust and bundled it for typescript and
  keep the performance.

## Global Rules

- No redundancy
- No single-use functions/variables
- Avoid over-engineering
- Be clean and methodical, make things reusable and simple when appropriate
- Structure properly
- Code shouldn't be very long normally
- Think about testing properly
- No "dev" code in production, especially for testing. Avoid mocks in production
  code
- No Legacy/Deprecated support

### Structure

This projects includes rust and typescript.

- libs/ -> Includes all Rust packages codes.
- packages/ -> Includes all typescript packages codes.

# TypeScript

We use vitest for testing benchmarks, and tsdown with a wasm file for bundling.
We can leverage all modern TypeScript 2025 technologies without considering
backward compatibility.

## File Descriptions

### libs/src/ (Rust Core)

- **compression.rs** - Optimized gzip/zlib decompression + NbtFile with
  selective lazy loading
- **error.rs** - Unified error types with thiserror for all features
- **reader.rs** - Zero-copy binary parser with streaming and field selection
- **region.rs** - Minecraft .mca file support with 32x32 chunk management
- **snbt.rs** - NBT text parser/formatter using winnow for readable syntax
- **tag.rs** - Core NBT types (Byte, Int, String, Compound, List...) +
  constructors
- **wasm.rs** - WebAssembly bindings with memory management for TypeScript
  interop
- **lib.rs** - Unified entry point, conditional exports based on enabled
  features

### packages/src/ (TypeScript Interface)

- **NbtFile.ts** - High-level interface for read/write with dispose management
- **NbtTag.ts** - TypeScript wrapper around tags with iterators and processing
- **NbtRegion.ts** - Simplified interface for Minecraft region files
- **index.ts** - Public exports and NbtType enum for compatibility
- **wasm.ts** - WebAssembly initialization and compiled module management

### Development Guidelines

- Follow Rust conventions and use `rustfmt`
- Add tests for new functionality
- Ensure all benchmarks maintain performance
- Update documentation for API changes
- No `unsafe` code

### Code Quality

- Zero-copy parsing
- Type safety leveraging Rust's type system
- Performance-first optimization for real-world usage
- Full compatibility with existing NBT implementations
