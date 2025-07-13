import { NbtFile, NbtType } from '../src/index.js';
import { readFileSync, writeFileSync } from 'node:fs';

async function main() {
    try {
        console.log('📁 Loading cube.nbt...');
        const data = readFileSync('./cube.nbt');
        console.log(`📊 File size: ${data.length} bytes`);

        console.log('🔍 Parsing NBT file...');
        using nbt = await NbtFile.from(data);

        console.log('🌳 Getting root tag...');
        using root = nbt.getRoot();
        console.log(`🏷️  Root type: ${root.getType()}`);

        if (root.getType() !== NbtType.Compound) {
            console.error('❌ Root is not a compound tag');
            return;
        }

        console.log('🎨 Looking for palette...');
        const keys = root.getCompoundKeys();
        console.log(`🔑 Available keys: ${keys.join(', ')}`);

        if (!keys.includes('palette')) {
            console.error('❌ No palette key found');
            return;
        }

        using palette = root.getCompoundValue('palette');
        console.log(`🎨 Palette type: ${palette.getType()}`);

        if (palette.getType() !== NbtType.List) {
            console.error('❌ Palette is not a list');
            return;
        }

        const length = palette.getListLength();
        console.log(`📝 Palette has ${length} items`);

        let modifiedCount = 0;

        for (let i = 0; i < length; i++) {
            try {
                using item = palette.getListItem(i);
                if (item.getType() === NbtType.Compound) {
                    const itemKeys = item.getCompoundKeys();
                    if (itemKeys.includes('Name')) {
                        using nameTag = item.getCompoundValue('Name');
                        const blockName = nameTag.asString();
                        if (blockName === 'minecraft:mangrove_stairs') {
                            console.log(`✅ Found mangrove_stairs at index ${i}`);
                            nbt.setListItemString('palette', i, 'Name', 'minecraft:cherry_stairs');
                            modifiedCount++;
                        }
                    }
                }
            } catch (itemError) {
                console.error(`❌ Error processing item ${i}:`, itemError.message);
                break;
            }
        }

        console.log(`📈 Processing complete. Modified ${modifiedCount} blocks`);

        if (modifiedCount > 0) {
            console.log(`💾 Saving modified NBT file...`);
            const newData = nbt.write();
            writeFileSync('cube_processed.nbt', newData);
            console.log('✅ Saved: cube_processed.nbt');
        } else {
            console.log('ℹ️  No mangrove_stairs blocks found to replace');
        }

    } catch (error) {
        console.error('❌ Error:', error.message);
        console.error('Stack:', error.stack);
    }

    console.log('✅ Cube editor completed');
}

main().catch(console.error);