use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nbt_compression::*;
use nbt_core::*;
use std::time::Duration;

fn extract_palette_from_nbt(nbt_data: &NbtTag) -> Vec<String> {
    let mut palette = Vec::new();
    
    if let NbtTag::Compound(root) = nbt_data {
        if let Some(NbtTag::List { items, .. }) = root.get("palette") {
            for item in items {
                if let NbtTag::Compound(block) = item {
                    if let Some(NbtTag::String(name)) = block.get("Name") {
                        palette.push(name.clone());
                    }
                }
            }
        }
    }
    
    palette
}

fn bench_cube_palette_extraction(c: &mut Criterion) {
    let cube_data = include_bytes!("mock/cube.nbt");
    println!("Cube file size: {} bytes ({:.1} KB)", cube_data.len(), cube_data.len() as f64 / 1024.0);
    
    let nbt_file = NbtFile::read(cube_data, Endian::Big).unwrap();
    let nbt_data = &nbt_file.root;
    
    let test_palette = extract_palette_from_nbt(nbt_data);
    println!("Palette entries found: {}", test_palette.len());
    if !test_palette.is_empty() {
        println!("First few palette entries: {:?}", &test_palette[..std::cmp::min(3, test_palette.len())]);
    }
    
    let mut group = c.benchmark_group("Cube Palette Extraction");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);
    
    group.bench_function("extract_palette_10x", |b| {
        b.iter(|| {
            for _ in 0..10 {
                black_box(extract_palette_from_nbt(black_box(nbt_data)));
            }
        })
    });
    
    group.bench_function("read_and_extract_10x", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let file = NbtFile::read(black_box(cube_data), Endian::Big).unwrap();
                black_box(extract_palette_from_nbt(&file.root));
            }
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_cube_palette_extraction);
criterion_main!(benches); 