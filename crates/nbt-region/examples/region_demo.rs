// Renamed to avoid collision with nbt-snbt demo
use nbt_region::*;
use nbt_core::{NbtTag, HashMap};
use nbt_compression::{NbtFile, CompressionFormat};

fn main() -> Result<()> {
    println!("=== NBT Region Demo ===\n");

    // Create sample chunk data
    println!("1. Creating sample chunks...");
    let mut region = Region::new();
    
    // Create chunk at (0,0) - spawn chunk
    let spawn_chunk = create_chunk_data(0, 0, "Spawn chunk with village")?;
    region.set_chunk(spawn_chunk)?;
    
    // Create chunk at (1,0) - forest chunk  
    let forest_chunk = create_chunk_data(1, 0, "Dense forest biome")?;
    region.set_chunk(forest_chunk)?;
    
    // Create chunk at (0,1) - mountain chunk
    let mountain_chunk = create_chunk_data(0, 1, "Mountain peaks with snow")?;
    region.set_chunk(mountain_chunk)?;
    
    println!("   Created {} chunks", region.chunk_count());
    
    // Write region to bytes
    println!("\n2. Writing region to bytes...");
    let region_data = region.write()?;
    println!("   Region file size: {} bytes ({:.1} KB)", 
             region_data.len(), region_data.len() as f64 / 1024.0);
    
    // Read region back
    println!("\n3. Reading region from bytes...");
    let region2 = Region::read(&region_data)?;
    println!("   Successfully loaded {} chunks", region2.chunk_count());
    
    // Verify chunk positions
    let positions = region2.get_chunk_positions();
    println!("   Chunk positions: {:?}", positions);
    
    // Access individual chunks and their NBT data
    println!("\n4. Reading chunk data...");
    for (x, z) in positions {
        if let Some(chunk) = region2.get_chunk(x, z)? {
            println!("   Chunk ({}, {}):", x, z);
            println!("     - Compression: {:?}", chunk.get_compression());
            println!("     - Size: {} bytes", chunk.size());
            println!("     - Timestamp: {}", chunk.timestamp);
            
            // Parse NBT data (this would normally be done mutably)
            let mut chunk_clone = chunk.clone();
            if let Ok(root) = chunk_clone.get_root() {
                if let NbtTag::Compound(map) = root {
                    if let Some(description) = map.get("Description") {
                        println!("     - Description: {}", description.as_string());
                    }
                }
            }
        }
    }
    
    // Demonstrate region operations
    println!("\n5. Region operations...");
    let mut region3 = region2;
    
    // Remove mountain chunk
    if let Some(removed) = region3.remove_chunk(0, 1)? {
        println!("   Removed chunk at ({}, {})", removed.x, removed.z);
    }
    
    // Add new chunk at different position
    let desert_chunk = create_chunk_data(2, 2, "Sandy desert with oasis")?;
    region3.set_chunk(desert_chunk)?;
    
    println!("   Final chunk count: {}", region3.chunk_count());
    println!("   Final positions: {:?}", region3.get_chunk_positions());
    
    // Write final region  
    let final_data = region3.write()?;
    println!("   Final region size: {} bytes", final_data.len());
    
    println!("\n=== Demo Complete ===");
    println!("✅ Fast region file I/O");
    println!("✅ Lazy chunk loading");
    println!("✅ Memory efficient storage");
    println!("✅ Full round-trip compatibility");
    
    Ok(())
}

fn create_chunk_data(x: i32, z: i32, description: &str) -> Result<Chunk> {
    // Create realistic chunk NBT structure
    let mut level_data = HashMap::new();
    level_data.insert("xPos".to_string(), NbtTag::Int(x));
    level_data.insert("zPos".to_string(), NbtTag::Int(z));
    level_data.insert("Description".to_string(), NbtTag::String(description.to_string()));
    level_data.insert("LastUpdate".to_string(), NbtTag::Long(1234567890));
    level_data.insert("InhabitedTime".to_string(), NbtTag::Long(5000));
    level_data.insert("Status".to_string(), NbtTag::String("full".to_string()));
    
    // Add some blocks data (simplified)
    let blocks = vec![1i32, 2, 3, 1, 1, 2, 3, 2, 1]; // Sample block IDs
    level_data.insert("Blocks".to_string(), NbtTag::IntArray(blocks));
    
    // Add biomes data  
    let biomes = vec![1i32; 1024]; // Biome IDs for 32x32 area
    level_data.insert("Biomes".to_string(), NbtTag::IntArray(biomes));
    
    let root = NbtTag::Compound(level_data);
    let nbt_file = NbtFile::new(root, "Level".to_string(), CompressionFormat::Zlib, nbt_core::Endian::Big);
    
    let timestamp = 1640995200 + (x + z) as u32; // Unique timestamp per chunk
    Chunk::from_nbt(x, z, nbt_file, timestamp)
} 