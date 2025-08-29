use nbt::NbtFile;
use nbt::NbtTag;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let cube_data = std::fs::read("mock/cube.nbt")?;
    let nbt_file = NbtFile::read(&cube_data, None)?;

    // Go to palette and print block names
    if let NbtTag::Compound(root) = &nbt_file.root {
        if let Some(NbtTag::List { items, .. }) = root.get("palette") {
            for (i, item) in items.iter().enumerate() {
                if let NbtTag::Compound(block) = item {
                    if let Some(NbtTag::String(name)) = block.get("Name") {
                        println!("Block {i}: {name}");
                    }
                }
            }
        }
    }

    Ok(())
}
