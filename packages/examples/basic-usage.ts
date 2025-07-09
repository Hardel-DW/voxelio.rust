
import { NbtFile, NbtRegion } from '../src/index';
import initNbt from '../src/wasm';
import { readFileSync } from 'fs';

// Initialize WASM
initNbt();

async function basicExample() {
    console.log('=== NBT Basic Usage - Simplified API ===\n');

    // Lecture simple
    const data = readFileSync('./taiga_armorer_2.nbt');
    const nbt = NbtFile.read(data);

    console.log(`File: ${nbt.name}`);
    console.log(`Compression: ${nbt.compression}`);

    // Accès simple aux données
    const playerName = nbt.getString('Data.Player.Name') ?? 'Unknown';
    const level = nbt.getNumber('Data.Player.Level') ?? 0;
    const health = nbt.getNumber('Data.Player.Health') ?? 20;

    console.log(`Player: ${playerName}`);
    console.log(`Level: ${level}`);
    console.log(`Health: ${health}\n`);

    // Édition simple
    nbt.setString('Data.Player.Name', 'SuperSteve');
    nbt.setInt('Data.Player.Level', level + 10);

    console.log('After modification:');
    console.log(`Player: ${nbt.getString('Data.Player.Name')}`);
    console.log(`Level: ${nbt.getNumber('Data.Player.Level')}\n`);

    // Performance - Lecture lazy
    console.log('=== Lazy Loading Demo ===');
    const lazyNbt = NbtFile.readLazy(data, ['Data.Player.Name', 'Data.Player.Level']);
    console.log(`Lazy Name: ${lazyNbt.getString('Data.Player.Name')}`);
    console.log(`Lazy Level: ${lazyNbt.getNumber('Data.Player.Level')}\n`);

    // Batch processing
    console.log('=== Batch Processing Demo ===');
    const files = [data, data, data]; // Simuler plusieurs fichiers
    NbtFile.processBatch(files, (nbt, index) => {
        const name = nbt.getString('Data.Player.Name') ?? 'Unknown';
        console.log(`File ${index}: ${name}`);
    });

    // Test d'écriture
    const outputBytes = nbt.write();
    console.log(`\nOutput size: ${outputBytes.length} bytes`);
}

async function regionExample() {
    console.log('\n=== Region File Demo ===');

    // Créer une région vide
    const region = NbtRegion.new();
    console.log(`Empty region chunks: ${region.getChunkCount()}`);
    console.log(`Is empty: ${region.isEmpty()}`);

    // TODO: Ajouter exemple avec vraie région quand setChunk sera implémenté
}

basicExample().then(() => regionExample()).catch(console.error); 