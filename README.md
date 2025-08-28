# Voxelio Rust

This project is designed to read and create .nbt format files. The proprietary
Minecraft format for storing structure and region data.

## How to start

### Prerequisites

- Rust 1.70 or higher
- Cargo (included with Rust)

### Commands

- Building - `cargo build --release`
- Testing - `cargo test --all`
- Benchmarking - `cargo bench`
- Type checking - `cargo check --all`
- This command will install all dependencies for all packages.

```bash
git clone <repository-url>
cd voxelio.rust
cargo build --release
```

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

## License

MIT License - see individual package licenses for specific terms.
