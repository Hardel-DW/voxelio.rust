/**
 * Simple NBT Reading Example
 * 
 * Shows basic file reading and data access
 */

import { NbtFile } from '../src/index.js';
import { readFileSync } from 'fs';

async function main() {
    const nbtData = readFileSync('examples/cube.nbt');
    using nbt = await NbtFile.from(nbtData);

    // Direct access to simple values
    console.log('Data version:', nbt.getNumber('DataVersion'));
    
    // Process with automatic memory management
    const result = nbt.process(root => {
        using palette = root.getCompoundValue('palette');
        const blockCount = palette.getListLength();
        
        console.log(`Found ${blockCount} block types`);
        return blockCount;
    });
    
    console.log('Total blocks:', result);
}

main().catch(console.error);