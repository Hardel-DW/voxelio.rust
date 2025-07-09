import { NbtFile, initNbt } from "../src/index";
import { readFileSync, writeFileSync } from "fs";

await initNbt();

async function modifyCube() {
    try {
        const cubeData = readFileSync("cube.nbt");
        const nbtFile = NbtFile.read(cubeData);
        const root = nbtFile.getRoot();

        const palette = root.get("palette");
        if (!palette || !palette.isList()) {
            throw new Error("No palette found or palette is not a list");
        }

        const paletteLength = palette.listLength();
        let modified = false;

        // Use direct modification on the NBT file instead of working on copies
        for (let i = 0; i < paletteLength; i++) {
            const item = palette.getListItem(i);
            if (!item || !item.isCompound()) continue;

            const name = item.get("Name");
            if (name && name.isString() && name.asString() === "minecraft:mangrove_stairs") {
                // Use the new direct modification method - this will persist!
                const success = nbtFile.modifyListItem("palette", i, "Name", "minecraft:cherry_stairs");
                if (success) {
                    console.log(`✅ Modified palette[${i}]: mangrove_stairs → cherry_stairs`);
                    modified = true;
                }
            }
        }

        if (!modified) {
            console.log("❌ No mangrove_stairs found in palette");
            return;
        }

        const newData = nbtFile.write();
        writeFileSync("cube_modified.nbt", newData);

        console.log("✅ File saved: cube_modified.nbt");

    } catch (error) {
        console.error("❌ Error:", error);
    }
}

modifyCube(); 