import { readFileSync } from "node:fs";
import { NbtFile } from "../dist/index";
import { bench } from "vitest";

const data = readFileSync('./examples/cube.nbt');

bench("Edit Cube NBT Data", async () => {
    using nbt = await NbtFile.from(data);
    using root = nbt.getRoot();
    using palette = root.getCompoundValue('palette');

    for (let i = 0; i < palette.getListLength(); i++) {
        using item = palette.getListItem(i);
        if (item.getCompoundValue('Name').asString() === 'minecraft:mangrove_stairs') {
            nbt.setListItemString('palette', i, 'Name', 'minecraft:cherry_stairs');
        }
    }

    nbt.write();
}, { iterations: 100 });