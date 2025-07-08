//! Demo of NBT-Rust functionality
//! 
//! This example shows how to use nbt-core, nbt-compression, and nbt-snbt together

use nbt_compression::NbtFile;
use nbt_core::NbtTag;
use nbt_snbt::{format_snbt, format_snbt_pretty, parse_snbt};
use std::collections::HashMap;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("=== NBT Demo ===");
    
    // Create a simple NBT structure
    let mut player = HashMap::new();
    player.insert("Name".to_string(), NbtTag::String("Steve".to_string()));
    player.insert("Level".to_string(), NbtTag::Int(30));
    player.insert("Health".to_string(), NbtTag::Double(20.0));
    player.insert("Hardcore".to_string(), NbtTag::Byte(0));
    
    // Position as list
    let position = vec![
        NbtTag::Double(128.5),
        NbtTag::Double(64.0),
        NbtTag::Double(-256.3)
    ];
    player.insert("Pos".to_string(), NbtTag::List { tag_type: 6, items: position });
    
    // Inventory
    let mut inventory = Vec::new();
    for i in 0..5 {
        let mut item = HashMap::new();
        item.insert("id".to_string(), NbtTag::String(format!("minecraft:item_{}", i)));
        item.insert("Count".to_string(), NbtTag::Byte(64));
        item.insert("Slot".to_string(), NbtTag::Byte(i as i8));
        inventory.push(NbtTag::Compound(item));
    }
    player.insert("Inventory".to_string(), NbtTag::List { tag_type: 10, items: inventory });
    
    let mut root = HashMap::new();
    root.insert("DataVersion".to_string(), NbtTag::Int(3210));
    root.insert("Player".to_string(), NbtTag::Compound(player));
    
    let nbt_root = NbtTag::Compound(root);
    
    println!("1. Created NBT structure");
    
    // Test SNBT formatting
    let snbt_compact = format_snbt(&nbt_root);
    println!("2. SNBT (compact): {}", &snbt_compact[..100.min(snbt_compact.len())]); // First 100 chars
    
    let snbt_pretty = format_snbt_pretty(&nbt_root);
    println!("3. SNBT (pretty): \n{}", &snbt_pretty[..200.min(snbt_pretty.len())]); // First 200 chars
    
    // Test parsing back
    let _parsed_nbt = parse_snbt(&snbt_compact)?;
    println!("4. Parsed SNBT back to NBT successfully");
    
    // Test NBT file operations
    let nbt_file = NbtFile::new(nbt_root.clone(), "Data".to_string());
    let compressed_data = nbt_file.write()?;
    println!("5. Compressed NBT size: {} bytes", compressed_data.len());
    
    // Read back
    let decompressed_file = NbtFile::read(&compressed_data)?;
    println!("6. Decompressed NBT successfully");
    println!("   Root name: {}", decompressed_file.root_name);
    println!("   Compression: {:?}", decompressed_file.compression);
    
    // Test convenience methods
    let data_version = decompressed_file.get_number("DataVersion");
    println!("7. DataVersion: {}", data_version);
    
    if let Some(NbtTag::Compound(player_data)) = decompressed_file.root.get("Player") {
        if let Some(NbtTag::String(name)) = player_data.get("Name") {
            println!("   Player name: {}", name);
        }
    }
    
    println!("=== Demo completed successfully ===");
    
    Ok(())
} 