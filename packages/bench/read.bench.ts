import { readFileSync } from "node:fs";
import { NbtFile } from "../src/NbtFile";
import { bench } from "vitest";

const data = readFileSync('./examples/cube.nbt');

bench("Read Cube NBT Data", async () => {
    await NbtFile.from(data);
}, { iterations: 100 });