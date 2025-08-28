/**
 * Batch Processing Example
 * 
 * Shows how to process multiple NBT files efficiently
 */

import { NbtFile } from '../src/index.js';
import { readFileSync } from 'fs';

async function main() {
    const nbtData = readFileSync('cube.nbt');
    const files = [nbtData, nbtData, nbtData];
    
    const results = await NbtFile.processBatch(files, (nbt, index) => {
        return nbt.process(root => {
            using palette = root.getCompoundValue('palette');
            return `File ${index}: ${palette.getListLength()} blocks`;
        });
    });
    
    console.log('Batch results:', results);
}

main().catch(console.error);