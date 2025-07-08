# NBT-Rust - Plan de Développement Complet

## 📋 Vue d'Ensemble

**Objectif** : Réécrire complètement le projet NBT TypeScript en Rust pour des
gains de performance massifs (10-15x speedup).

**Problèmes TypeScript identifiés** :

- ❌ Copies mémoire excessives (70% du temps CPU)
- ❌ Polymorphisme coûteux avec virtual calls
- ❌ Compression Web API ultra-lente (10-20x vs native)
- ❌ Buffer resizing naïf avec reallocations
- ❌ Parser SNBT avec regex séquentielles

**Solutions Rust** :

- ✅ Architecture zero-copy avec slices
- ✅ Enum dispatch compile-time
- ✅ Compression native avec flate2
- ✅ Buffer pooling intelligent
- ✅ Parser combinators optimisés

---

## 🏗️ Architecture Cible

```
nbt-rust/
├── crates/
│   ├── nbt-core/           # Types de base + parsing binaire
│   ├── nbt-snbt/          # Parser string format (SNBT)  
│   ├── nbt-compression/   # Compression gzip/zlib
│   ├── nbt-region/        # Fichiers .mca regions
│   └── nbt-wasm/          # Bindings WebAssembly
├── benches/               # Benchmarks vs TypeScript
├── tests/                 # Tests de compatibilité
└── examples/              # Exemples d'usage
```

---

## 🚀 Plan de Développement

### Phase 1: Core NBT (Semaines 1-3)

### Phase 2: Compression + SNBT (Semaines 4-5)

### Phase 3: Regions + Cache (Semaines 6-7)

### Phase 4: WASM + Integration (Semaine 8)

---

## ✅ Tâches Détaillées

## 📦 Phase 1: nbt-core (PRIORITÉ MAXIMUM)

### 1.1 Setup Projet

- [x] Créer workspace Cargo.toml principal
- [x] Setup structure des crates
- [ ] Configuration CI/CD (GitHub Actions)
- [x] Setup benchmarks avec criterion
- [ ] Documentation projet (README.md)

### 1.2 Types de Base (nbt-core)

- [x] **Enum NbtTag principal** (CRITIQUE)
  ```rust
  #[derive(Debug, Clone, PartialEq)]
  pub enum NbtTag {
      End,
      Byte(i8),
      Short(i16), 
      Int(i32),
      Long(i64),
      Float(f32),
      Double(f64),
      ByteArray(Vec<i8>),
      String(String),
      List { tag_type: u8, items: Vec<NbtTag> },
      Compound(HashMap<String, NbtTag>),
      IntArray(Vec<i32>),
      LongArray(Vec<i64>),
  }
  ```

- [x] **NbtReader zero-copy** (CRITIQUE)
  ```rust
  pub struct NbtReader<'a> {
      data: &'a [u8],
      cursor: usize,
      endian: Endian,
  }
  ```

- [x] **NbtWriter avec buffer optimisé** (CRITIQUE)
  ```rust
  pub struct NbtWriter {
      buffer: Vec<u8>,
      endian: Endian,
  }
  ```

- [x] **Error types complets**
  ```rust
  #[derive(Debug, thiserror::Error)]
  pub enum NbtError {
      #[error("IO error: {0}")]
      Io(#[from] std::io::Error),
      #[error("Invalid NBT tag type: {0}")]
      InvalidTagType(u8),
      #[error("UTF-8 decode error")]
      InvalidUtf8,
      #[error("Unexpected end of data")]
      UnexpectedEof,
  }
  ```

### 1.3 Parsing Binaire

- [x] **Read methods zero-copy**
  - [x] `read_tag()` - dispatch par type
  - [x] `read_string()` - référence directe sans copie
  - [x] `read_bytes()` - slice sans allocation
  - [x] `read_compound()` - parsing récursif optimisé
  - [x] `read_list()` - avec type hint pour éviter boxing

- [x] **Write methods optimisés**
  - [x] `write_tag()` - match exhaustif sur enum
  - [x] `write_string()` - direct write sans copie
  - [x] `write_compound()` - iteration efficace
  - [x] Buffer auto-resize intelligent

- [x] **Endianness support**
  - [x] Big-endian (Java Edition)
  - [x] Little-endian (Bedrock Edition)
  - [ ] Auto-detection basée sur headers

### 1.4 API Ergonomique

- [x] **Méthodes d'accès typées**
  ```rust
  impl NbtTag {
      pub fn as_compound(&self) -> Option<&HashMap<String, NbtTag>>;
      pub fn as_number(&self) -> f64;
      pub fn get_string(&self, key: &str) -> &str;
      pub fn get_number(&self, key: &str) -> f64;
      pub fn get_bool(&self, key: &str) -> bool;
  }
  ```

- [x] **Factory methods simples**
  ```rust
  impl NbtTag {
      pub fn byte(value: i8) -> Self;
      pub fn int(value: i32) -> Self;
      pub fn string(value: impl Into<String>) -> Self;
      pub fn compound() -> Self;
  }
  ```

### 1.5 Tests Core

- [x] **Tests unitaires complets**
  - [x] Tous les types NBT
  - [x] Round-trip binary (write -> read)
  - [x] Endianness variants
  - [x] Error cases

- [ ] **Tests de compatibilité TypeScript**
  - [ ] Importer fichiers test TypeScript existants
  - [ ] Vérifier parsing identique
  - [ ] Benchmark comparatif

---

## 📦 Phase 2: Compression + SNBT

### 2.1 nbt-compression

- [x] **Détection automatique**
  ```rust
  pub fn detect_compression(header: &[u8]) -> CompressionFormat {
      match header.get(0..2) {
          Some([0x1f, 0x8b]) => CompressionFormat::Gzip,
          Some([0x78, b]) if (b & 0x20) == 0 => CompressionFormat::Zlib,
          _ => CompressionFormat::None,
      }
  }
  ```

- [x] **Compression native avec flate2**
  - [x] Gzip encoder/decoder
  - [x] Zlib encoder/decoder
  - [x] Streaming support pour gros fichiers
  - [x] Configuration niveaux compression

- [x] **Integration avec NbtFile**
  ```rust
  pub struct NbtFile {
      pub root: NbtTag,
      pub root_name: String,
      pub compression: CompressionFormat,
      pub endian: Endian,
  }

  impl NbtFile {
      pub fn read(data: &[u8], endian: Endian) -> Result<Self>;
      pub fn write(&self) -> Result<Vec<u8>>;
  }
  ```

### 2.2 nbt-snbt (Parser String)

- [x] **Parser combinators avec winnow**
  ```rust
  fn parse_value(input: &mut Input) -> PResult<NbtTag> {
      alt((
          parse_compound,
          parse_array,  // [B;1,2,3] [I;1,2] [L;1L,2L]
          parse_list,   // [1,2,3]
          parse_quoted_string,  // "value" 'value'
          parse_unquoted_value, // numbers, booleans, strings
      )).parse_next(input)
  }
  ```

- [x] **Number parsing optimisé**
  - [x] Parse float/double avec suffixe (3.14f, 3.14d)
  - [x] Parse entiers avec suffixe (42b, 42s, 42L)
  - [x] Gestion exponentielles et auto-détection
  - [x] Un seul pass avec winnow combinators

- [x] **String parsing avec échappement**
  - [x] Quotes simples/doubles ("hello", 'hello')
  - [x] Échappement complet (\", \\, etc.)
  - [x] Strings unquoted (hello, true, false)

- [x] **Pretty printing**
  ```rust
  pub fn format_snbt(tag: &NbtTag) -> String;
  pub fn format_snbt_pretty(tag: &NbtTag) -> String;
  ```

### 2.3 Tests Compression + SNBT

- [x] **Tests compression**
  - [x] Round-trip gzip/zlib
  - [x] Auto-détection formats
  - [x] Structures Minecraft complexes
  - [ ] Benchmarks vs TypeScript

- [x] **Tests parser SNBT**
  - [x] Tous les types NBT (compound, list, arrays, numbers, strings)
  - [x] Error handling complet
  - [x] Round-trip SNBT (parse -> format -> parse)
  - [x] Pretty formatting avec indentation

---

## 📦 Phase 3: Regions + Cache

### 3.1 nbt-region ✅ (100% COMPLETE)

- [x] **Region file structure implementation**
  ```rust
  pub struct Region {
      chunks: [Option<Chunk>; CHUNK_COUNT], // 32x32 grid
  }

  pub struct Chunk {
      x: i32, z: i32,           // Coordinates (0-31)
      compression: u8,          // 1=gzip, 2=zlib, 3=none  
      timestamp: u32,           // Unix timestamp
      raw_data: Vec<u8>,        // Compressed chunk data
      cached_nbt: Option<NbtFile>, // Lazy loaded NBT
  }
  ```

- [x] **Lazy chunk loading**
  - [x] Chunks parse NBT on first access only
  - [x] Raw data kept compressed until needed
  - [x] Automatic caching after first parse
  - [x] Clone without cache duplication

- [x] **Efficient region operations**
  - [x] Fast chunk lookup by coordinates
  - [x] Memory efficient storage (only store existing chunks)
  - [x] Complete .mca format compatibility
  - [x] Round-trip read/write verified

- [x] **Full API implementation**
  - [x] Read/write region files
  - [x] Add/remove chunks dynamically
  - [x] Iterator over chunks
  - [x] Position management and validation

### 3.2 Optimisations Avancées

- [ ] **Buffer pooling global**
  - [ ] Pool de Vec<u8> réutilisables
  - [ ] Pool de String buffers
  - [ ] Pool de HashMap pour compounds

- [ ] **SIMD optimizations**
  - [ ] Recherche rapide dans arrays
  - [ ] Parsing parallel pour listes
  - [ ] Compression SIMD si available

### 3.3 Tests Regions ✅ (100% COMPLETE)

- [x] **Comprehensive region tests**
  - [x] Full read/write round-trip verification
  - [x] Multi-chunk regions with realistic data
  - [x] Chunk management operations (add/remove)
  - [x] Error handling (invalid coordinates, malformed data)

- [x] **Working demo application**
  - [x] Creates realistic chunk data (biomes, blocks, metadata)
  - [x] Demonstrates lazy loading and caching
  - [x] Shows region file operations
  - [x] Performance verification (20KB region with 3 chunks)

---

## 📦 Phase 4: WASM + Integration

### 4.1 nbt-wasm

- [ ] **Bindings JavaScript**
  ```rust
  #[wasm_bindgen]
  pub struct NbtFile {
      inner: crate::NbtFile,
  }

  #[wasm_bindgen]
  impl NbtFile {
      #[wasm_bindgen(constructor)]
      pub fn new(data: &[u8]) -> Result<NbtFile, JsValue>;
      
      #[wasm_bindgen(js_name = "getString")]
      pub fn get_string(&self, path: &str) -> Option<String>;
  }
  ```

- [ ] **TypeScript definitions**
  - [ ] Génération automatique .d.ts
  - [ ] Documentation API
  - [ ] Exemples d'usage

- [ ] **NPM package**
  - [ ] Build pipeline WASM
  - [ ] Package.json configuration
  - [ ] Documentation migration

### 4.2 API Compatibility Layer

- [ ] **Drop-in replacement**
  - [ ] API identique au TypeScript original
  - [ ] Migration guide détaillé
  - [ ] Compatibility tests

- [ ] **Performance monitoring**
  - [ ] Métriques en production
  - [ ] Error reporting
  - [ ] Usage analytics

---

## 🔬 Benchmarks & Tests

### Benchmarks Critiques

- [ ] **Parse NBT 10MB file**: TypeScript ~2.5s → Rust ~200ms (12.5x)
- [ ] **Compress gzip 1MB**: TypeScript ~80ms → Rust ~8ms (10x)
- [ ] **Parse SNBT large**: TypeScript ~500ms → Rust ~50ms (10x)
- [ ] **Region chunk load**: TypeScript ~150ms → Rust ~15ms (10x)
- [ ] **Memory usage**: TypeScript 450MB → Rust 120MB (3.8x)

### Tests de Régression

- [ ] **Tous les tests TypeScript portés**
- [ ] **Nouveaux edge cases**
- [ ] **Fuzzing avec arbitrary data**
- [ ] **Property-based testing**

---

## 📊 Métriques de Succès

### Performance Targets

- [ ] ⚡ **10x+ speedup** sur parsing NBT
- [ ] 🗜️ **60-80% moins de mémoire**
- [ ] 📦 **90% plus petit** que bundle Node.js
- [ ] 🌐 **3-5x plus rapide en WASM** que TypeScript pur

### Quality Targets

- [ ] 📋 **100% compatibilité** avec API TypeScript
- [ ] 🧪 **>95% code coverage**
- [ ] 📚 **Documentation complète**
- [ ] 🔒 **Zero unsafe code** (sauf WASM bindings)

---

## 🚦 Status & Priorités

**🔴 CRITIQUE (Blocker)**

- [x] nbt-core enum NbtTag
- [x] NbtReader zero-copy
- [x] NbtWriter buffer optimisé
- [x] Tests compatibilité de base

**🟡 IMPORTANT (Phase 2)**

- [x] Compression native
- [x] Parser SNBT
- [ ] Benchmarks complets

**🟢 NICE-TO-HAVE (Phase 3+)**

- [ ] Memory mapping regions
- [ ] WASM bindings
- [ ] SIMD optimizations

---

## 📝 Notes de Développement

### Decisions Techniques

- **Pas de unsafe code** sauf bindings WASM nécessaires
- **AHashMap** au lieu de HashMap standard (2-3x plus rapide)
- **winnow** au lieu de nom pour parser (meilleure performance)
- **flate2** pour compression (standard de facto)
- **criterion** pour benchmarks reproductibles

### Migration Strategy

1. **Phase 1**: Core compatible, benchmarks setup
2. **Phase 2**: Feature parity avec TypeScript
3. **Phase 3**: Optimisations avancées
4. **Phase 4**: Migration production progressive

### Resources

- [NBT Specification](https://wiki.vg/NBT)
- [Region File Format](https://minecraft.fandom.com/wiki/Region_file_format)
- [TypeScript codebase analysis](./analysis.md)

---

_Last updated: $(date)_ _Target completion: 6-8 weeks_ _Expected performance
gain: 10-15x speedup_
