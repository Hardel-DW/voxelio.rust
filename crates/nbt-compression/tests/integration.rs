use nbt_compression::*;
use nbt_core::*;

#[test]
fn test_basic_compression() {
    let data = include_bytes!("taiga_armorer_2.nbt");
    let file = NbtFile::read(data).unwrap();
    
    println!("Root name: {}", file.root_name);
    println!("Compression: {:?}", file.compression);
    
    let written = file.write().unwrap();
    let file2 = NbtFile::read(&written).unwrap();
    
    assert_eq!(file.root_name, file2.root_name);
}

#[test]
fn test_convenience_methods() {
    let data = include_bytes!("taiga_armorer_2.nbt");
    let file = NbtFile::read(data).unwrap();
    
    // Test convenience methods with fields that actually exist
    let data_version = file.get_number("DataVersion");
    assert!(data_version > 0.0);
    
    // Test string access (even if empty, should not panic)
    let version = file.get_string("Version");
    println!("Version field: '{}'", version);
}

#[test]
fn test_multiple_compression_formats() {
    // Create test data
    let mut root = std::collections::HashMap::new();
    root.insert("test".to_string(), NbtTag::String("value".to_string()));
    root.insert("number".to_string(), NbtTag::Int(42));
    let root = NbtTag::Compound(root);
    
    for format in [CompressionFormat::Gzip, CompressionFormat::Zlib, CompressionFormat::None] {
        let file = NbtFile::new_with_settings(root.clone(), "Data".to_string(), format, Endian::Big);
        let compressed = file.write().unwrap();
        let loaded = NbtFile::read(&compressed).unwrap();
        
        assert_eq!(file.root_name, loaded.root_name);
        assert_eq!(format, loaded.compression);
    }
}

#[test]
fn test_global_functions() {
    // Create test data
    let mut root = std::collections::HashMap::new();
    root.insert("test".to_string(), NbtTag::String("hello".to_string()));
    let root = NbtTag::Compound(root);
    
    // Test write functions
    let gzip_data = write_nbt(&root, "Test", CompressionFormat::Gzip).unwrap();
    let zlib_data = write_nbt(&root, "Test", CompressionFormat::Zlib).unwrap();
    let raw_data = write_nbt(&root, "Test", CompressionFormat::None).unwrap();
    
    // Test read function
    let (parsed_gzip, name_gzip) = read_nbt(&gzip_data).unwrap();
    let (parsed_zlib, name_zlib) = read_nbt(&zlib_data).unwrap();
    let (parsed_raw, name_raw) = read_nbt(&raw_data).unwrap();
    
    assert_eq!(name_gzip, "Test");
    assert_eq!(name_zlib, "Test");
    assert_eq!(name_raw, "Test");
    
    // All should parse to same data
    assert_eq!(parsed_gzip, parsed_zlib);
    assert_eq!(parsed_zlib, parsed_raw);
}

#[test]
fn test_format_auto_detection() {
    let mut root = std::collections::HashMap::new();
    root.insert("test".to_string(), NbtTag::String("value".to_string()));
    let root = NbtTag::Compound(root);
    
    let gzip_file = NbtFile::new_with_settings(root.clone(), "Data".to_string(), CompressionFormat::Gzip, Endian::Big);
    let zlib_file = NbtFile::new_with_settings(root.clone(), "Data".to_string(), CompressionFormat::Zlib, Endian::Big);
    let raw_file = NbtFile::new_with_settings(root.clone(), "Data".to_string(), CompressionFormat::None, Endian::Big);
    
    let gzip_data = gzip_file.write().unwrap();
    let zlib_data = zlib_file.write().unwrap();
    let raw_data = raw_file.write().unwrap();
    
    // Test auto-detection
    let detected_gzip = NbtFile::read(&gzip_data).unwrap();
    let detected_zlib = NbtFile::read(&zlib_data).unwrap();
    let detected_raw = NbtFile::read(&raw_data).unwrap();
    
    assert_eq!(detected_gzip.compression, CompressionFormat::Gzip);
    assert_eq!(detected_zlib.compression, CompressionFormat::Zlib);
    assert_eq!(detected_raw.compression, CompressionFormat::None);
}

#[test]
fn test_real_file_access() {
    let data = include_bytes!("taiga_armorer_2.nbt");
    let file = NbtFile::read(data).unwrap();
    
    // Test various field access with fields that exist
    let data_version = file.get_number("DataVersion");
    let spawn_x = file.get_number("SpawnX");
    
    println!("DataVersion: {}", data_version);
    println!("SpawnX: {}", spawn_x);
    
    assert!(data_version > 0.0);
} 