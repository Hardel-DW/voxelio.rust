import { readFileSync } from "node:fs";
import { NbtFile } from "../dist/index";
import { bench } from "vitest";

const data = readFileSync('./examples/cube.nbt');

bench("Read Cube NBT Data", async () => {
    using nbt = await NbtFile.from(data, ["DataVersion"]);
}, { iterations: 100 }); 