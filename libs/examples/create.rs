fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Create large NBT: root compound with big list of compounds
    let mut root = nbt::NbtTag::Compound(std::collections::HashMap::new());
    let mut large_list = Vec::with_capacity(10000); // Aim for ~1MB
    for i in 0..10000 {
        let mut inner_comp = std::collections::HashMap::new();
        inner_comp.insert("id".to_string(), nbt::NbtTag::Int(i));
        inner_comp.insert(
            "name".to_string(),
            nbt::NbtTag::String(format!("block_{i}")),
        );
        inner_comp.insert("data".to_string(), nbt::NbtTag::ByteArray(vec![0i8; 100])); // Padding
        large_list.push(nbt::NbtTag::Compound(inner_comp));
    }
    if let nbt::NbtTag::Compound(ref mut map) = root {
        map.insert(
            "large_list".to_string(),
            nbt::NbtTag::List {
                tag_type: 10,
                items: large_list,
            },
        );
    }
    let nbt_file = nbt::NbtFile::new(root, "large".to_string(), nbt::CompressionFormat::Zlib);

    let new_data = nbt_file.write()?;
    std::fs::write("large.nbt", &new_data)?;

    println!("✅ Generated large.nbt");
    Ok(())
}
