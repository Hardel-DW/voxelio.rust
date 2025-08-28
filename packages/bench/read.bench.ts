import { readFileSync } from "node:fs";
import { NbtFile } from "../src/NbtFile";
import { bench } from "vitest";
import path from "node:path";
import { fileURLToPath } from "node:url";

const uri = path.resolve(fileURLToPath(import.meta.url), "../../examples/cube.nbt");
const data = readFileSync(uri);

bench("Read Cube NBT Data", async () => {
    using nbt = await NbtFile.from(data, ["DataVersion"]);
}, { iterations: 100 }); 