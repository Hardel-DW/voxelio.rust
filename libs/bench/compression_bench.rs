use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use nbt::{
    compress_data, decompress_optimized, detect_compression, CompressionFormat,
};

fn bench_different_compression_formats(c: &mut Criterion) {
    let cube_data = include_bytes!("mock/cube.nbt");

    // Créer des versions avec différents formats
    let original_format = detect_compression(cube_data);
    let decompressed = decompress_optimized(cube_data, original_format).unwrap();

    let mut group = c.benchmark_group("Compression Formats");

    // Benchmark pour chaque format
    for format in [
        CompressionFormat::None,
        CompressionFormat::Gzip,
        CompressionFormat::Zlib,
    ] {
        group.bench_with_input(
            BenchmarkId::new("decompress", format.as_u8()),
            &format,
            |b, &format| {
                b.iter(|| {
                    if format == CompressionFormat::None {
                        black_box(decompressed.clone())
                    } else {
                        // Compresser puis décompresser
                        let compressed = compress_data(&decompressed, format).unwrap();
                        let result = decompress_optimized(&compressed, format).unwrap();
                        black_box(result)
                    }
                })
            },
        );
    }

    group.finish();
}

fn bench_compression_ratios(c: &mut Criterion) {
    let test_files = [("cube", include_bytes!("mock/cube.nbt"))];

    let mut group = c.benchmark_group("Compression Ratios");

    for (name, data) in test_files {
        let format = detect_compression(data);
        let decompressed = decompress_optimized(data, format).unwrap();

        group.bench_function(&format!("analyze_{}", name), |b| {
            b.iter(|| {
                let original_size = data.len();
                let decompressed_size = decompressed.len();
                let compression_ratio = decompressed_size as f64 / original_size as f64;

                black_box((original_size, decompressed_size, compression_ratio))
            })
        });
    }

    group.finish();
}

criterion_group!(
    compression_benches,
    bench_different_compression_formats,
    bench_compression_ratios
);
criterion_main!(compression_benches);
