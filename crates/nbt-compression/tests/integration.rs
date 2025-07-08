use nbt_compression::*;
use nbt_core::{NbtTag, Endian};

#[test]
fn test_real_minecraft_file() {
    // Load the real Minecraft NBT file from TypeScript tests
    let data = include_bytes!("taiga_armorer_2.nbt");
    
    // Parse the file - should be gzip compressed
    let nbt_file = NbtFile::read(data, Endian::Big).expect("Failed to read real NBT file");
    
    // Verify basic structure like TypeScript test does
    let root = &nbt_file.root;
    if let NbtTag::Compound(map) = root {
        // Check DataVersion exists and is a number (like TypeScript test)
        assert!(map.contains_key("DataVersion"));
        let data_version = map.get("DataVersion").unwrap().as_number() as i32;
        assert_eq!(data_version, 3210, "DataVersion should match TypeScript test");
        
        // Check entities list exists and has 2 items (like TypeScript test)
        assert!(map.contains_key("entities"));
        if let NbtTag::List { items, .. } = map.get("entities").unwrap() {
            assert_eq!(items.len(), 2, "Should have 2 entities like TypeScript test");
        } else {
            panic!("entities should be a list");
        }
        
        println!("✅ Real Minecraft file parsed successfully!");
        println!("   - DataVersion: {}", data_version);
        println!("   - Entities: {} items", 
                if let NbtTag::List { items, .. } = map.get("entities").unwrap() { 
                    items.len() 
                } else { 
                    0 
                });
    } else {
        panic!("Root should be a compound");
    }
    
    // Test round-trip to ensure we can write it back correctly
    let written_data = nbt_file.write().expect("Failed to write NBT file");
    let nbt_file2 = NbtFile::read(&written_data, Endian::Big).expect("Failed to read written file");
    
    // Verify the round-trip maintains the same data
    assert_eq!(nbt_file.root, nbt_file2.root, "Round-trip should preserve data");
    println!("✅ Round-trip test passed!");
}

#[test] 
fn test_minimal_nbt_compatibility() {
    // Test the exact bytes from TypeScript NbtFile.test.ts
    let test_bytes = [10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0];
    
    let nbt_file = NbtFile::read(&test_bytes, Endian::Big).expect("Failed to read test bytes");
    
    // Verify it matches TypeScript expectations
    assert_eq!(nbt_file.root_name, "", "Name should be empty like TypeScript test");
    
    if let NbtTag::Compound(map) = &nbt_file.root {
        assert_eq!(map.len(), 1, "Should have 1 item like TypeScript test");
        assert!(map.contains_key("foo"), "Should contain 'foo' key");
        
        if let NbtTag::String(value) = map.get("foo").unwrap() {
            assert_eq!(value, "Hello!", "Value should be 'Hello!' like TypeScript test");
        } else {
            panic!("foo should be a string");
        }
    } else {
        panic!("Root should be a compound");
    }
    
    // Test round-trip matches original bytes
    let written_bytes = nbt_file.write().expect("Failed to write NBT");
    assert_eq!(written_bytes, test_bytes, "Round-trip should match original bytes");
    
    println!("✅ Minimal NBT compatibility test passed!");
} 