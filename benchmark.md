# Benchmarks Voxelio.rust

## TypeScript

- **Read Cube NBT Data:** 32ms (30.83 hz)
- **Edit Cube NBT Data:** 181ms (5.53 hz)

Les benchmarks utilisent la syntaxe `using` pour la gestion mémoire automatique.
La lecture avec lazy loading charge seulement les champs DataVersion. L'édition
modifie les entrées de palette mangrove vers cherry.

---

## Rust

- **Traitement lot 5 fichiers:** 334ms
- **Fichier unique + 10 ops:** 59ms
- **Extraction pure palette:** 6.3µs
- **Décompression gzip:** 53ms
- **Décompression zlib:** 52ms
- **Décompression none:** 502µs
- **Édition 10x modifications:** N/A

Les benchmarks utilisent les fichiers `cube.nbt` et `taiga_armorer_2.nbt`.
L'extraction pure mesure l'accès aux données déjà chargées en mémoire. Le
traitement par lot charge et traite 5 fichiers séquentiellement.
