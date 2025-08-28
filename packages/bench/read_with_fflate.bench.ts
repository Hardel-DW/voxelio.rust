
import { bench } from "vitest";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { gunzipSync } from "fflate";
import { NbtFile } from "../src/NbtFile";

const uri = path.resolve(fileURLToPath(import.meta.url), "../../examples/cube.nbt");
const compressedArray = new Uint8Array(fs.readFileSync(uri));

bench("NbtFile", async () => {
    const array = gunzipSync(compressedArray);
    using result = await NbtFile.from(array, ["DataVersion"]);
}, { iterations: 50 });  