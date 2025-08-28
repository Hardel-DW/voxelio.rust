/**
 * Lazy Loading Example
 * 
 * Shows performance optimization with selective field loading
 */

import { NbtFile } from '../src/index.js';
import { readFileSync } from 'fs';

async function main() {
    const nbtData = readFileSync('examples/cube.nbt');
    
    // Load only specific fields for better performance
    using nbt = await NbtFile.from(nbtData, ['DataVersion', 'palette']);
    
    // Access only the loaded fields
    const version = nbt.getNumber('DataVersion');
    console.log('Data version:', version);
    
    // Process loaded data
    nbt.process(root => {
        using palette = root.getCompoundValue('palette');
        console.log('Palette loaded with', palette.getListLength(), 'entries');
    });
}

main().catch(console.error);