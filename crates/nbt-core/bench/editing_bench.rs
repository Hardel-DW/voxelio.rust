use nbt_core::{NbtFile, NbtTag};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn edit_nbt_file(nbt_file: &mut NbtFile) {
    if let NbtTag::Compound(root) = &mut nbt_file.root {
        if let Some(NbtTag::List { items, .. }) = root.get_mut("palette") {
            for item in items {
                if let NbtTag::Compound(block) = item {
                    if let Some(NbtTag::String(name)) = block.get_mut("Name") {
                        if name == "minecraft:mangrove_stairs" {
                            *name = "minecraft:cherry_stairs".to_string();
                        }
                    }
                }
            }
        }
    }
}

fn bench_editing(c: &mut Criterion) {
    let cube_data = include_bytes!("mock/cube.nbt");
    
    let mut group = c.benchmark_group("Editing");
    group.sample_size(20);  // Reduced from default 100
    
    group.bench_function("edit_10x", |b| {
        b.iter(|| {
            let mut nbt_file = NbtFile::read(cube_data).unwrap();
            for _ in 0..10 {
                edit_nbt_file(&mut nbt_file);
            }
            black_box(())
        })
    });
    
    group.finish();
}

criterion_group!(benches, bench_editing);
criterion_main!(benches);