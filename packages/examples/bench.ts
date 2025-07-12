import { NbtFile, NbtType } from '../src/index.js';
import { readFileSync, writeFileSync } from 'node:fs';
import { initNbt } from '../src/wasm.js';

async function processNbt(data) {
    const start = performance.now();
    try {
        using nbt = await NbtFile.from(data);
        using root = nbt.getRoot();

        if (root.getType() !== NbtType.Compound) return;

        const keys = root.getCompoundKeys();
        if (!keys.includes('palette')) return;

        using palette = root.getCompoundValue('palette');
        if (palette.getType() !== NbtType.List) return;

        const length = palette.getListLength();
        let modifiedCount = 0;

        for (let i = 0; i < length; i++) {
            try {
                using item = palette.getListItem(i);
                if (item.getType() === NbtType.Compound) {
                    const itemKeys = item.getCompoundKeys();
                    if (itemKeys.includes('Name')) {
                        using nameTag = item.getCompoundValue('Name');
                        if (nameTag.asString() === 'minecraft:grass_block') {
                            nbt.setListItemString('palette', i, 'Name', 'minecraft:cherry_stairs');
                            modifiedCount++;
                        }
                    }
                }
            } catch (itemError) {
                break;
            }
        }

        if (modifiedCount > 0) {
            const newData = nbt.write();
            writeFileSync('cube_processed.nbt', newData);
        }
    } catch (error) {
        console.error('Error:', error.message);
    }

    const end = performance.now();
    return end - start;
}

async function main() {
    const memBefore = process.memoryUsage().heapUsed;
    await initNbt();
    const data = readFileSync('./taiga_armorer_2.nbt');

    for (let iteration = 1; iteration <= 20; iteration++) {
        const time = await processNbt(data);
        console.log(`Iteration ${iteration}: ${time?.toFixed(2) ?? 'N/A'}ms`);

        if (global.gc) {
            global.gc();
        }
    }
    const memAfter = process.memoryUsage().heapUsed;
    console.log(`Memory usage: ${((memAfter - memBefore) / 1024 / 1024).toFixed(2)} MB`);
}

main().catch(console.error);