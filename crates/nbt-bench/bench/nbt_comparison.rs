use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use nbt_compression::*;
use nbt_core::*;
use nbt_snbt::*;
use std::time::Duration;

fn create_large_nbt() -> NbtTag {
    let mut root = HashMap::new();
    
    // Metadata
    root.insert("DataVersion".to_string(), NbtTag::Int(3210));
    root.insert("Version".to_string(), NbtTag::String("1.19.3".to_string()));
    root.insert("LastPlayed".to_string(), NbtTag::Long(1673890234));
    
    // Player data
    let mut player = HashMap::new();
    player.insert("Name".to_string(), NbtTag::String("TestPlayer".to_string()));
    player.insert("Level".to_string(), NbtTag::Int(30));
    player.insert("Experience".to_string(), NbtTag::Float(0.75));
    player.insert("Health".to_string(), NbtTag::Double(20.0));
    player.insert("FoodLevel".to_string(), NbtTag::Int(20));
    
    // Position
    let position = vec![
        NbtTag::Double(128.5),
        NbtTag::Double(64.0), 
        NbtTag::Double(-256.3)
    ];
    player.insert("Pos".to_string(), NbtTag::List { tag_type: 6, items: position });
    
    // Inventory (large array)
    let mut inventory = Vec::new();
    for i in 0..36 {
        let mut slot = HashMap::new();
        slot.insert("Slot".to_string(), NbtTag::Byte(i as i8));
        slot.insert("id".to_string(), NbtTag::String(format!("minecraft:item_{}", i)));
        slot.insert("Count".to_string(), NbtTag::Byte(64));
        slot.insert("Damage".to_string(), NbtTag::Short(0));
        inventory.push(NbtTag::Compound(slot));
    }
    player.insert("Inventory".to_string(), NbtTag::List { tag_type: 10, items: inventory });
    
    root.insert("Player".to_string(), NbtTag::Compound(player));
    
    // World data with large arrays
    let mut world = HashMap::new();
    let blocks = (0..4096).map(|i| (i % 256) as i32).collect(); // 16x16x16 chunk
    world.insert("Blocks".to_string(), NbtTag::IntArray(blocks));
    
    let biomes = vec![1i32; 1024]; // 32x32 biomes
    world.insert("Biomes".to_string(), NbtTag::IntArray(biomes));
    
    let heights = (0..256).map(|i| (64 + i % 128) as i8).collect(); // Height map
    world.insert("HeightMap".to_string(), NbtTag::ByteArray(heights));
    
    root.insert("World".to_string(), NbtTag::Compound(world));
    
    NbtTag::Compound(root)
}

fn bench_nbt_binary_operations(c: &mut Criterion) {
    let large_nbt = create_large_nbt();
    
    let mut group = c.benchmark_group("NBT Binary Operations");
    group.measurement_time(Duration::from_secs(10));
    
    // Write benchmark
    group.bench_function("write_uncompressed", |b| {
        b.iter(|| {
            let mut writer = NbtWriter::new(Endian::Big);
            writer.write_u8(10);
            writer.write_string("Data");
            writer.write_tag(black_box(&large_nbt)).unwrap();
            black_box(writer.into_bytes())
        })
    });
    
    // Create compressed data for read benchmark
    let nbt_file = NbtFile::new(large_nbt.clone(), "Data".to_string(), CompressionFormat::Gzip, Endian::Big);
    let compressed_data = nbt_file.write().unwrap();
    
    group.bench_function("read_compressed", |b| {
        b.iter(|| {
            black_box(NbtFile::read(black_box(&compressed_data), Endian::Big).unwrap())
        })
    });
    
    group.bench_function("write_compressed", |b| {
        b.iter(|| {
            let file = NbtFile::new(black_box(large_nbt.clone()), "Data".to_string(), CompressionFormat::Gzip, Endian::Big);
            black_box(file.write().unwrap())
        })
    });
    
    println!("Compressed size: {} bytes ({:.1} KB)", compressed_data.len(), compressed_data.len() as f64 / 1024.0);
    
    group.finish();
}

fn bench_snbt_operations(c: &mut Criterion) {
    let large_nbt = create_large_nbt();
    
    let mut group = c.benchmark_group("SNBT Operations");
    group.measurement_time(Duration::from_secs(5));
    
    // Format benchmark
    group.bench_function("format_compact", |b| {
        b.iter(|| {
            black_box(format_snbt(black_box(&large_nbt)))
        })
    });
    
    group.bench_function("format_pretty", |b| {
        b.iter(|| {
            black_box(format_snbt_pretty(black_box(&large_nbt)))
        })
    });
    
    // Create SNBT string for parse benchmark
    let snbt_string = format_snbt(&large_nbt);
    
    group.bench_function("parse", |b| {
        b.iter(|| {
            black_box(parse_snbt(black_box(&snbt_string)).unwrap())
        })
    });
    
    println!("SNBT size: {} characters ({:.1} KB)", snbt_string.len(), snbt_string.len() as f64 / 1024.0);
    
    group.finish();
}

fn bench_compression_formats(c: &mut Criterion) {
    let large_nbt = create_large_nbt();
    
    let mut group = c.benchmark_group("Compression Formats");
    group.measurement_time(Duration::from_secs(5));
    
    for &format in &[CompressionFormat::None, CompressionFormat::Gzip, CompressionFormat::Zlib] {
        let format_name = match format {
            CompressionFormat::None => "none",
            CompressionFormat::Gzip => "gzip", 
            CompressionFormat::Zlib => "zlib",
        };
        
        group.bench_with_input(BenchmarkId::new("compress", format_name), &format, |b, &format| {
            b.iter(|| {
                let file = NbtFile::new(black_box(large_nbt.clone()), "Data".to_string(), format, Endian::Big);
                black_box(file.write().unwrap())
            })
        });
        
        // Pre-compress for decompress benchmark
        let file = NbtFile::new(large_nbt.clone(), "Data".to_string(), format, Endian::Big);
        let compressed = file.write().unwrap();
        
        group.bench_with_input(BenchmarkId::new("decompress", format_name), &compressed, |b, data| {
            b.iter(|| {
                black_box(NbtFile::read(black_box(data), Endian::Big).unwrap())
            })
        });
        
        println!("{} compressed size: {} bytes", format_name, compressed.len());
    }
    
    group.finish();
}

fn bench_real_minecraft_file(c: &mut Criterion) {
    let data = include_bytes!("mock/taiga_armorer_2.nbt");
    
    let mut group = c.benchmark_group("Real Minecraft File");
    group.measurement_time(Duration::from_secs(5));
    
    group.bench_function("parse_real_file", |b| {
        b.iter(|| {
            black_box(NbtFile::read(black_box(data), Endian::Big).unwrap())
        })
    });
    
    // Parse once to get the data for write benchmark
    let nbt_file = NbtFile::read(data, Endian::Big).unwrap();
    
    group.bench_function("write_real_file", |b| {
        b.iter(|| {
            black_box(nbt_file.write().unwrap())
        })
    });
    
    println!("Real file size: {} bytes", data.len());
    
    group.finish();
}

criterion_group!(
    benches, 
    bench_nbt_binary_operations,
    bench_snbt_operations, 
    bench_compression_formats,
    bench_real_minecraft_file
);
criterion_main!(benches); 