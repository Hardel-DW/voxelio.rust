use nbt_compression::NbtFile;
use nbt_core::NbtTag;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read cube.nbt
    let cube_data = std::fs::read("crates/nbt-bench/bench/mock/cube.nbt")?;
    let mut nbt_file = NbtFile::read(&cube_data)?;
    
    // Go to palette and modify the Name
    if let NbtTag::Compound(root) = &mut nbt_file.root {
        if let Some(NbtTag::List { items, .. }) = root.get_mut("palette") {
            for item in items {
                if let NbtTag::Compound(block) = item {
                    if let Some(NbtTag::String(name)) = block.get_mut("Name") {
                        if name == "minecraft:mangrove_stairs" {
                            *name = "minecraft:cherry_stairs".to_string();
                            println!("✅ Modified: mangrove_stairs → cherry_stairs");
                        }
                    }
                }
            }
        }
    }
    
    // Save the modified file
    let new_data = nbt_file.write()?;
    std::fs::write("cube_modified.nbt", &new_data)?;
    
    println!("✅ File saved: cube_modified.nbt");
    Ok(())
} 