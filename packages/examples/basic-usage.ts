// /**
//  * Basic Usage Examples - New NBT API
//  *
//  * Demonstrates:
//  * - Zero memory leaks with 'using' syntax
//  * - High-performance path-based access
//  * - Efficient iteration and editing
//  * - Batch operations
//  */

// import { NbtFile, NbtRegion, initNbt } from '../src/index.js';
// import { readFileSync } from 'fs';

// // Initialize WASM once
// await initNbt();


// const nbtData = readFileSync('cube.nbt');
// using nbt = await NbtFile.from(nbtData);

// // Direct access
// console.log('Player name:', nbt.getString('Data.Player.Name'));
// console.log('Player level:', nbt.getNumber('Data.Player.Level'));

// // Root tag processing
// const result = nbt.process(root => {
//     const playerName = root.getString('Data.Player.Name');
//     const level = root.getNumber('Data.Player.Level');

//     // Modify data
//     root.setString('Data.Player.Name', 'SuperSteve');
//     root.setNumber('Data.Player.Level', level + 1);

//     return { playerName, level };
// });

// console.log('Original data:', result);

// // === BATCH PROCESSING ===
// console.log('\n=== BATCH PROCESSING ===');

// const files = [nbtData, nbtData, nbtData];
// const results = await NbtFile.processBatch(files, (nbt, index) => {
//     return nbt.process(root => {
//         const name = root.getString('Data.Player.Name');
//         root.setString('Data.Player.Name', `Player_${index}`);
//         return name;
//     });
// });

// console.log('Batch results:', results);

// // === REGION EXAMPLE ===
// console.log('\n=== REGION PROCESSING ===');

// try {
//     const regionData = readFileSync('region.mca');
//     using region = await NbtRegion.from(regionData);

//     // Process all chunks
//     const chunkCount = region.processChunks((chunk, x, z) => {
//         const status = chunk.getString('Level.Status');
//         if (status === 'full') {
//             console.log(`Chunk ${x},${z} is fully generated`);
//             return 1;
//         }
//         return 0;
//     }).reduce((sum, count) => sum + count, 0);

//     console.log(`Found ${chunkCount} fully generated chunks`);
// } catch (error) {
//     console.log('No region file found, skipping region example');
// }

// console.log('\n=== DONE ==='); 