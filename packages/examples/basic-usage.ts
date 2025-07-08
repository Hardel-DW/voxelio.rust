
import { readFileSync, writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

try {
    const { default: init, JsNbtFile } = await import('../src/nbt_wasm.js');
    const wasmPath = join(__dirname, '../src/nbt_wasm_bg.wasm');
    const wasmBytes = readFileSync(wasmPath);
    await init(wasmBytes);

    const nbtPath = join(__dirname, 'taiga_armorer_2.nbt');
    const nbtBytes = readFileSync(nbtPath);
    const nbtArray = new Uint8Array(nbtBytes);
    const nbtFile = JsNbtFile.read(nbtArray);

    nbtFile.setStringInListItem('palette', 0, 'Name', 'minecraft:diamond_block');
    const bytes = nbtFile.write();
    writeFileSync(join(__dirname, 'taiga_armorer_2_modified.nbt'), bytes);


} catch (error) {
    console.error('❌ Error:', error);
    console.error('Stack:', error.stack);
} 