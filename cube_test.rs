use std::time::Instant;
use nbt_compression::*;
use nbt_core::*;

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

fn main() {
    let cube_data = include_bytes!("crates/nbt-compression/tests/cube.nbt");
    println!("Cube file size: {} bytes ({:.1} KB)", cube_data.len(), cube_data.len() as f64 / 1024.0);
    
    // Parse the NBT file once to get the structure
    let nbt_file = NbtFile::read(cube_data, Endian::Big).unwrap();
    let nbt_data = &nbt_file.root;
    
    // Test extraction once to verify it works
    let test_palette = extract_palette_from_nbt(nbt_data);
    println!("Palette entries found: {}", test_palette.len());
    if !test_palette.is_empty() {
        println!("First few palette entries: {:?}", &test_palette[..std::cmp::min(3, test_palette.len())]);
    }
    
    // Test extract palette 10 times
    let start = Instant::now();
    for _ in 0..10 {
        let _palette = extract_palette_from_nbt(nbt_data);
    }
    let extract_duration = start.elapsed();
    println!("Extract 10x: {:?} ({:.2}μs per operation)", extract_duration, extract_duration.as_micros() as f64 / 10.0);
    
    // Test read + extract 10 times
    let start = Instant::now();
    for _ in 0..10 {
        let file = NbtFile::read(cube_data, Endian::Big).unwrap();
        let _palette = extract_palette_from_nbt(&file.root);
    }
    let read_extract_duration = start.elapsed();
    println!("Read+Extract 10x: {:?} ({:.2}ms per operation)", read_extract_duration, read_extract_duration.as_millis() as f64 / 10.0);
    
    // Calculate ops/sec
    let extract_ops_per_sec = 10.0 / extract_duration.as_secs_f64();
    let read_extract_ops_per_sec = 10.0 / read_extract_duration.as_secs_f64();
    
    println!("Performance:");
    println!("  Extract only: {:.0} ops/sec", extract_ops_per_sec);
    println!("  Read+Extract: {:.0} ops/sec", read_extract_ops_per_sec);
} 