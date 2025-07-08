use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nbt_compression::*;
use nbt_core::*;
use std::time::Duration;

fn extract_palette(nbt_file: &NbtFile) -> Vec<String> {
    let mut palette = Vec::new();
    
    if let Some(NbtTag::List { items, .. }) = nbt_file.root.get("palette") {
        for item in items {
            if let NbtTag::Compound(block) = item {
                if let Some(NbtTag::String(name)) = block.get("Name") {
                    palette.push(name.clone());
                }
            }
        }
    }
    
    palette
}

fn extract_multiple_fields(nbt_file: &NbtFile) -> (Vec<String>, String, f64) {
    let palette = extract_palette(nbt_file);
    let version = nbt_file.get_string("Version").to_string();
    let data_version = nbt_file.get_number("DataVersion");
    
    (palette, version, data_version)
}

fn bench_optimized_loading(c: &mut Criterion) {
    let cube_data = include_bytes!("mock/cube.nbt");
    let taiga_data = include_bytes!("mock/taiga_armorer_2.nbt");
    
    let mut group = c.benchmark_group("Optimized Loading");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);
    group.warm_up_time(Duration::from_secs(3));
    
    // Test standard read optimisé
    group.bench_function("optimized_read_cube", |b| {
        b.iter(|| {
            let file = NbtFile::read(black_box(cube_data)).unwrap();
            black_box(file)
        })
    });
    
    group.bench_function("optimized_read_taiga", |b| {
        b.iter(|| {
            let file = NbtFile::read(black_box(taiga_data)).unwrap();
            black_box(file)
        })
    });
    
    // Test lazy read - palette uniquement
    group.bench_function("lazy_read_palette_only", |b| {
        b.iter(|| {
            let file = NbtFile::read_lazy(black_box(cube_data), &["palette"]).unwrap();
            black_box(file)
        })
    });
    
    // Test lazy read - champs multiples
    group.bench_function("lazy_read_multiple_fields", |b| {
        b.iter(|| {
            let file = NbtFile::read_lazy(black_box(cube_data), &["palette", "Version", "DataVersion"]).unwrap();
            black_box(file)
        })
    });
    
    group.finish();
}

fn bench_streaming_operations(c: &mut Criterion) {
    let cube_data = include_bytes!("mock/cube.nbt");
    let taiga_data = include_bytes!("mock/taiga_armorer_2.nbt");
    
    // Pre-load files outside benchmarks
    let cube_file = NbtFile::read(cube_data).unwrap();
    let taiga_file = NbtFile::read(taiga_data).unwrap();
    
    let mut group = c.benchmark_group("Streaming Operations");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(30);
    group.warm_up_time(Duration::from_secs(2));
    
    // Pure extraction - no file reading
    group.bench_function("extract_palette_cube", |b| {
        b.iter(|| {
            let palette = extract_palette(black_box(&cube_file));
            black_box(palette)
        })
    });
    
    group.bench_function("extract_palette_taiga", |b| {
        b.iter(|| {
            let palette = extract_palette(black_box(&taiga_file));
            black_box(palette)
        })
    });
    
    // Multiple field extraction
    group.bench_function("extract_multiple_fields", |b| {
        b.iter(|| {
            let result = extract_multiple_fields(black_box(&cube_file));
            black_box(result)
        })
    });
    
    // File loading benchmark (separate)
    group.bench_function("file_loading_only", |b| {
        b.iter(|| {
            let file = NbtFile::read(black_box(cube_data)).unwrap();
            black_box(file)
        })
    });
    
    group.finish();
}

fn bench_lazy_vs_full_parsing(c: &mut Criterion) {
    let cube_data = include_bytes!("mock/cube.nbt");
    
    let mut group = c.benchmark_group("Lazy vs Full Parsing");
    group.measurement_time(Duration::from_secs(8));
    group.sample_size(50);
    
    // Full parsing
    group.bench_function("full_parse_then_extract", |b| {
        b.iter(|| {
            let file = NbtFile::read(black_box(cube_data)).unwrap();
            let palette = extract_palette(&file);
            black_box(palette)
        })
    });
    
    // Lazy parsing - palette only
    group.bench_function("lazy_parse_palette_direct", |b| {
        b.iter(|| {
            let file = NbtFile::read_lazy(black_box(cube_data), &["palette"]).unwrap();
            let palette = extract_palette(&file);
            black_box(palette)
        })
    });
    
    // Compare load + extract vs lazy load
    group.bench_function("load_extract_multiple", |b| {
        b.iter(|| {
            let file = NbtFile::read(black_box(cube_data)).unwrap();
            let (palette, version, data_version) = extract_multiple_fields(&file);
            black_box((palette, version, data_version))
        })
    });
    
    group.bench_function("lazy_load_multiple", |b| {
        b.iter(|| {
            let file = NbtFile::read_lazy(black_box(cube_data), &["palette", "Version", "DataVersion"]).unwrap();
            let (palette, version, data_version) = extract_multiple_fields(&file);
            black_box((palette, version, data_version))
        })
    });
    
    group.finish();
}

fn bench_memory_efficiency(c: &mut Criterion) {
    let cube_data = include_bytes!("mock/cube.nbt");
    let taiga_data = include_bytes!("mock/taiga_armorer_2.nbt");
    
    let cube_file = NbtFile::read(cube_data).unwrap();
    let taiga_file = NbtFile::read(taiga_data).unwrap();
    
    let mut group = c.benchmark_group("Memory Efficiency");
    group.measurement_time(Duration::from_secs(5));
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(2));
    
    // Pure operations on pre-loaded files
    group.bench_function("multiple_ops_cube", |b| {
        b.iter(|| {
            let _palette = extract_palette(black_box(&cube_file));
            let _fields = extract_multiple_fields(black_box(&cube_file));
            let _version = cube_file.get_string("Version");
            black_box(&cube_file)
        })
    });
    
    group.bench_function("multiple_ops_taiga", |b| {
        b.iter(|| {
            let _palette = extract_palette(black_box(&taiga_file));
            let _fields = extract_multiple_fields(black_box(&taiga_file));
            black_box(&taiga_file)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches, 
    bench_optimized_loading,
    bench_streaming_operations,
    bench_lazy_vs_full_parsing,
    bench_memory_efficiency
);
criterion_main!(benches); 