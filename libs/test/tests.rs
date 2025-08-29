use nbt::{Endian, NbtReader, NbtTag, NbtWriter, HashMap};

#[test]
fn test_basic_types() {
    // Test simple byte
    let byte_tag = NbtTag::Byte(42);
    assert_eq!(byte_tag.type_id(), 1);
    assert_eq!(byte_tag.as_number(), 42.0);
    assert!(byte_tag.is_number());

    // Test string
    let string_tag = NbtTag::String("Hello".to_string());
    assert_eq!(string_tag.type_id(), 8);
    assert_eq!(string_tag.as_string(), "Hello");
    assert!(string_tag.is_string());

    // Test compound
    let compound_tag = NbtTag::Compound(HashMap::new());
    assert_eq!(compound_tag.type_id(), 10);
    assert!(compound_tag.is_compound());
}

#[test]
fn test_compound_access() {
    let mut map = HashMap::new();
    map.insert("name".to_string(), NbtTag::String("Steve".to_string()));
    map.insert("level".to_string(), NbtTag::Int(42));
    
    let compound = NbtTag::Compound(map);
    
    // Test access methods
    assert_eq!(compound.get_string("name"), "Steve");
    assert_eq!(compound.get_number("level"), 42.0);
    assert_eq!(compound.get_bool("level"), true); // 42 != 0
    assert_eq!(compound.get_string("missing"), ""); // Default for missing
}

#[test]
fn test_write_read_roundtrip() {
    // Create a compound like in TypeScript test: {foo: "Hello!"}
    let mut map = HashMap::new();
    map.insert("foo".to_string(), NbtTag::String("Hello!".to_string()));
    let original = NbtTag::Compound(map);

    // Write to bytes
    let mut writer = NbtWriter::new(Endian::Big);
    writer.write_u8(original.type_id()); // Compound type
    writer.write_string(""); // Root name (empty)
    writer.write_tag(&original).unwrap();
    let bytes = writer.into_bytes();

    // Expected bytes from TypeScript test
    let expected: &[u8] = &[10, 0, 0, 8, 0, 3, 102, 111, 111, 0, 6, 72, 101, 108, 108, 111, 33, 0];
    assert_eq!(bytes, expected);

    // Read back
    let mut reader = NbtReader::new(&bytes, Endian::Big);
    let tag_type = reader.read_u8().unwrap();
    let _name = reader.read_string().unwrap(); // Root name
    let parsed = reader.read_tag(tag_type).unwrap();

    // Verify round-trip
    assert_eq!(parsed, original);
    assert_eq!(parsed.get_string("foo"), "Hello!");
}

#[test]
fn test_list() {
    // Test list creation and access
    let items = vec![
        NbtTag::Int(1),
        NbtTag::Int(2),
        NbtTag::Int(3),
    ];
    let list = NbtTag::List { tag_type: 3, items }; // Int list

    if let Some((tag_type, items)) = list.as_list() {
        assert_eq!(*tag_type, 3);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].as_number(), 1.0);
    } else {
        panic!("Should be a list");
    }
}

#[test]
fn test_endianness() {
    let tag = NbtTag::Int(0x12345678);

    // Big endian (Java)
    let mut writer_be = NbtWriter::new(Endian::Big);
    writer_be.write_tag(&tag).unwrap();
    let bytes_be = writer_be.into_bytes();

    // Little endian (Bedrock)  
    let mut writer_le = NbtWriter::new(Endian::Little);
    writer_le.write_tag(&tag).unwrap();
    let bytes_le = writer_le.into_bytes();

    // Should be different
    assert_ne!(bytes_be, bytes_le);

    // But both should read back correctly
    let mut reader_be = NbtReader::new(&bytes_be, Endian::Big);
    let parsed_be = reader_be.read_tag(3).unwrap();
    assert_eq!(parsed_be, tag);

    let mut reader_le = NbtReader::new(&bytes_le, Endian::Little);
    let parsed_le = reader_le.read_tag(3).unwrap();
    assert_eq!(parsed_le, tag);
}

// =============== NOUVEAUX TESTS DE COMPATIBILITÉ ===============

#[test]
fn test_all_numeric_types() {
    // Test tous les types numériques avec valeurs limites
    let byte_max = NbtTag::Byte(127);
    let byte_min = NbtTag::Byte(-128);
    let short_max = NbtTag::Short(32767);
    let int_max = NbtTag::Int(2147483647);
    let long_max = NbtTag::Long(9223372036854775807);
    let float_val = NbtTag::Float(3.14159);
    let double_val = NbtTag::Double(2.718281828459045);

    // Test type IDs (comme TypeScript)
    assert_eq!(byte_max.type_id(), 1);
    assert_eq!(short_max.type_id(), 2);
    assert_eq!(int_max.type_id(), 3);
    assert_eq!(long_max.type_id(), 4);
    assert_eq!(float_val.type_id(), 5);
    assert_eq!(double_val.type_id(), 6);

    // Test conversions
    assert_eq!(byte_max.as_number(), 127.0);
    assert_eq!(byte_min.as_number(), -128.0);
    assert_eq!(float_val.as_number() as f32, 3.14159_f32);
}

#[test]
fn test_arrays() {
    // ByteArray
    let byte_array = NbtTag::ByteArray(vec![-1, 0, 1, 127, -128]);
    assert_eq!(byte_array.type_id(), 7);

    // IntArray
    let int_array = NbtTag::IntArray(vec![1000000, -1000000, 0]);
    assert_eq!(int_array.type_id(), 11);

    // LongArray  
    let long_array = NbtTag::LongArray(vec![1000000000000, -1000000000000]);
    assert_eq!(long_array.type_id(), 12);

    // Test round-trip arrays
    let mut writer = NbtWriter::new(Endian::Big);
    writer.write_tag(&byte_array).unwrap();
    let bytes = writer.into_bytes();

    let mut reader = NbtReader::new(&bytes, Endian::Big);
    let parsed = reader.read_tag(7).unwrap();
    assert_eq!(parsed, byte_array);
}

#[test]
fn test_nested_compounds() {
    // Test compound imbriqué comme dans Minecraft
    let mut inner = HashMap::new();
    inner.insert("x".to_string(), NbtTag::Int(100));
    inner.insert("y".to_string(), NbtTag::Int(64));
    inner.insert("z".to_string(), NbtTag::Int(-200));

    let mut outer = HashMap::new();
    outer.insert("position".to_string(), NbtTag::Compound(inner));
    outer.insert("name".to_string(), NbtTag::String("Player".to_string()));
    outer.insert("health".to_string(), NbtTag::Float(20.0));

    let root = NbtTag::Compound(outer);

    // Test accès imbriqué
    if let Some(pos) = root.get_compound("position") {
        let compound_pos = NbtTag::Compound(pos.clone());
        assert_eq!(compound_pos.get_number("x"), 100.0);
        assert_eq!(compound_pos.get_number("y"), 64.0);
        assert_eq!(compound_pos.get_number("z"), -200.0);
    }

    assert_eq!(root.get_string("name"), "Player");
    assert_eq!(root.get_number("health"), 20.0);
}

#[test]
fn test_mixed_list() {
    // Test liste avec types cohérents (comme TypeScript exige)
    let string_list = NbtTag::List {
        tag_type: 8, // String
        items: vec![
            NbtTag::String("first".to_string()),
            NbtTag::String("second".to_string()),
            NbtTag::String("third".to_string()),
        ],
    };

    // Test round-trip
    let mut writer = NbtWriter::new(Endian::Big);
    writer.write_tag(&string_list).unwrap();
    let bytes = writer.into_bytes();

    let mut reader = NbtReader::new(&bytes, Endian::Big);
    let parsed = reader.read_tag(9).unwrap();
    assert_eq!(parsed, string_list);

    // Test accès
    if let Some((tag_type, items)) = parsed.as_list() {
        assert_eq!(*tag_type, 8);
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].as_string(), "first");
        assert_eq!(items[2].as_string(), "third");
    }
}

#[test]
fn test_complex_minecraft_structure() {
    // Simule une structure NBT typique de Minecraft
    let mut entities = Vec::new();
    
    // Zombie
    let mut zombie = HashMap::new();
    zombie.insert("id".to_string(), NbtTag::String("minecraft:zombie".to_string()));
    zombie.insert("Health".to_string(), NbtTag::Float(20.0));
    zombie.insert("Age".to_string(), NbtTag::Int(0));
    entities.push(NbtTag::Compound(zombie));

    // Player  
    let mut player = HashMap::new();
    player.insert("id".to_string(), NbtTag::String("minecraft:player".to_string()));
    player.insert("Health".to_string(), NbtTag::Float(20.0));
    player.insert("foodLevel".to_string(), NbtTag::Int(20));
    entities.push(NbtTag::Compound(player));

    let mut root = HashMap::new();
    root.insert("DataVersion".to_string(), NbtTag::Int(3210));
    root.insert("entities".to_string(), NbtTag::List {
        tag_type: 10, // Compound
        items: entities,
    });

    let nbt = NbtTag::Compound(root);

    // Tests comme dans TypeScript
    assert_eq!(nbt.get_number("DataVersion"), 3210.0);
    
    if let Some((_, entities)) = nbt.get("entities").unwrap().as_list() {
        assert_eq!(entities.len(), 2);
        
        // Test premier entity (zombie)
        let zombie_nbt = &entities[0];
        assert_eq!(zombie_nbt.get_string("id"), "minecraft:zombie");
        assert_eq!(zombie_nbt.get_number("Health"), 20.0);
    }

    // Test round-trip complet
    let mut writer = NbtWriter::new(Endian::Big);
    writer.write_u8(nbt.type_id());
    writer.write_string(""); // Root name
    writer.write_tag(&nbt).unwrap();
    let bytes = writer.into_bytes();

    let mut reader = NbtReader::new(&bytes, Endian::Big);
    let _tag_type = reader.read_u8().unwrap();
    let _name = reader.read_string().unwrap();
    let parsed = reader.read_tag(10).unwrap();

    assert_eq!(parsed, nbt);
} 