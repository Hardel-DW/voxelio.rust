use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nbt_core::*;
use std::time::Duration;

fn extract_single_field(nbt_file: &NbtFile, field: &str) -> Option<Vec<String>> {
    if field == "palette" {
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
        
        Some(palette)
    } else {
        None
    }
}

fn bench_batch_processing(c: &mut Criterion) {
    let cube_data = include_bytes!("mock/cube.nbt");
    
    let mut group = c.benchmark_group("Batch Processing");
    group.measurement_time(Duration::from_secs(5));  // Reduced from 15s
    group.sample_size(20);  // Reduced from 50
    
    // Process 5 files instead of 50
    group.bench_function("batch_5_files", |b| {
        b.iter(|| {
            let mut results = Vec::with_capacity(5);
            for _ in 0..5 {
                let file = NbtFile::read(black_box(cube_data)).unwrap();
                let result = extract_single_field(&file, "palette");
                results.push(result);
            }
            black_box(results)
        })
    });
    
    // Single file with reuse
    group.bench_function("single_file_10_ops", |b| {
        b.iter(|| {
            let file = NbtFile::read(black_box(cube_data)).unwrap();
            let mut results = Vec::with_capacity(10);
            
            // 10 extractions on same file
            for _ in 0..10 {
                let result = extract_single_field(&file, "palette");
                results.push(result);
            }
            
            black_box(results)
        })
    });
    
    // Pure extraction (no file reading)
    group.bench_function("pure_extraction", |b| {
        let file = NbtFile::read(cube_data).unwrap();
        b.iter(|| {
            let result = extract_single_field(&file, "palette");
            black_box(result)
        })
    });
    
    group.finish();
}

fn bench_parallel_processing(c: &mut Criterion) {
    let cube_data = include_bytes!("mock/cube.nbt");
    
    let mut group = c.benchmark_group("Parallel Processing");
    group.measurement_time(Duration::from_secs(3));  // Reduced from 10s
    group.sample_size(10);  // Reduced from default 100
    
    // Sequential processing baseline (reduced from 20 to 5 files)
    group.bench_function("sequential_5_files", |b| {
        b.iter(|| {
            let mut results = Vec::with_capacity(5);
            for _ in 0..5 {
                let file = NbtFile::read(black_box(cube_data)).unwrap();
                let result = extract_single_field(&file, "palette");
                results.push(result);
            }
            black_box(results)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches, 
    bench_batch_processing,
    bench_parallel_processing
);
criterion_main!(benches); 