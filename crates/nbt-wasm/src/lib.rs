use wasm_bindgen::prelude::*;

// Re-export core types
use nbt_core::NbtTag;
use nbt_compression::{NbtFile, CompressionFormat};
use nbt_region::Region;
use nbt_snbt::{parse_snbt, format_snbt, format_snbt_pretty};
use regex::Regex;
use std::sync::OnceLock;

/// Set panic hook for better debugging
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

// Regex for array access parsing "items[0]"
static ARRAY_REGEX: OnceLock<Regex> = OnceLock::new();

// ============================================================================
// Core NBT Tag System
// ============================================================================

/// NBT tag wrapper for JavaScript - single point of truth
#[wasm_bindgen]
#[derive(Clone)]
pub struct JsNbtTag {
    inner: NbtTag,
}

#[wasm_bindgen]
impl JsNbtTag {
    /// Get tag type ID (matches TypeScript NbtType enum)
    #[wasm_bindgen(getter, js_name = typeId)]
    pub fn type_id(&self) -> u8 {
        self.inner.type_id()
    }

    /// Get as number (0 if not numeric)
    #[wasm_bindgen(js_name = asNumber)]
    pub fn as_number(&self) -> f64 {
        self.inner.as_number()
    }

    /// Get as string (empty if not string)
    #[wasm_bindgen(js_name = asString)]
    pub fn as_string(&self) -> String {
        self.inner.as_string().to_string()
    }

    /// Get nested value by key (compound only)
    #[wasm_bindgen]
    pub fn get(&self, key: &str) -> Option<JsNbtTag> {
        self.inner.get(key).map(|tag| JsNbtTag { inner: tag.clone() })
    }

    /// Type checking
    #[wasm_bindgen(js_name = isNumber)]
    pub fn is_number(&self) -> bool {
        self.inner.is_number()
    }

    #[wasm_bindgen(js_name = isString)]
    pub fn is_string(&self) -> bool {
        self.inner.is_string()
    }

    #[wasm_bindgen(js_name = isCompound)]
    pub fn is_compound(&self) -> bool {
        self.inner.is_compound()
    }

    #[wasm_bindgen(js_name = isList)]
    pub fn is_list(&self) -> bool {
        self.inner.is_list()
    }

    /// Get keys for compound tags
    #[wasm_bindgen]
    pub fn keys(&self) -> Vec<String> {
        match &self.inner {
            NbtTag::Compound(map) => map.keys().cloned().collect(),
            _ => Vec::new(),
        }
    }

    /// Set a string value by key (compound only)
    #[wasm_bindgen(js_name = setString)]
    pub fn set_string(&mut self, key: &str, value: &str) -> bool {
        match &mut self.inner {
            NbtTag::Compound(map) => {
                map.insert(key.to_string(), NbtTag::String(value.to_string()));
                true
            }
            _ => false,
        }
    }

    /// Get list length
    #[wasm_bindgen(js_name = listLength)]
    pub fn list_length(&self) -> u32 {
        match &self.inner {
            NbtTag::List { items, .. } => items.len() as u32,
            _ => 0,
        }
    }

    /// Get item from list by index
    #[wasm_bindgen(js_name = getListItem)]
    pub fn get_list_item(&self, index: u32) -> Option<JsNbtTag> {
        match &self.inner {
            NbtTag::List { items, .. } => {
                items.get(index as usize).map(|tag| JsNbtTag { inner: tag.clone() })
            }
            _ => None,
        }
    }

    // ============================================================================
    // NEW OPTIMIZED PATH OPERATIONS
    // ============================================================================

    /// Get tag by path (e.g., "Data.Player.Name") - OPTIMIZED RUST PARSING
    #[wasm_bindgen(js_name = getByPath)]
    pub fn get_by_path(&self, path: &str) -> Option<JsNbtTag> {
        let regex = ARRAY_REGEX.get_or_init(|| {
            Regex::new(r"^(.+)\[(\d+)\]$").unwrap()
        });

        let parts: Vec<&str> = path.split('.').collect();
        let mut current = self.clone();

        for part in parts {
            // Handle array access: "items[0]"
            if let Some(captures) = regex.captures(part) {
                let key = captures.get(1)?.as_str();
                let index: u32 = captures.get(2)?.as_str().parse().ok()?;
                current = current.get(key)?;
                current = current.get_list_item(index)?;
            } else {
                current = current.get(part)?;
            }
        }

        Some(current)
    }

    /// Set string by path - OPTIMIZED RUST PARSING
    #[wasm_bindgen(js_name = setStringByPath)]
    pub fn set_string_by_path(&mut self, path: &str, value: &str) -> bool {
        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return false;
        }

        // Simple case: direct key
        if parts.len() == 1 {
            return self.set_string(parts[0], value);
        }

        // Find parent path
        let parent_path = parts[..parts.len() - 1].join(".");
        let key = parts[parts.len() - 1];

        // Get parent tag
        if let Some(mut parent) = self.get_by_path(&parent_path) {
            parent.set_string(key, value)
        } else {
            false
        }
    }

    /// Get string value by path - HIGH PERFORMANCE
    #[wasm_bindgen(js_name = getStringPath)]
    pub fn get_string_path(&self, path: &str) -> Option<String> {
        let tag = self.get_by_path(path)?;
        if tag.is_string() {
            Some(tag.as_string())
        } else {
            None
        }
    }

    /// Get number value by path - HIGH PERFORMANCE
    #[wasm_bindgen(js_name = getNumberPath)]
    pub fn get_number_path(&self, path: &str) -> Option<f64> {
        let tag = self.get_by_path(path)?;
        if tag.is_number() {
            Some(tag.as_number())
        } else {
            None
        }
    }
}

// ============================================================================
// NBT File with Compression
// ============================================================================

/// NBT file wrapper - handles all compression formats
#[wasm_bindgen]
pub struct JsNbtFile {
    root: NbtTag,
    name: String,
    compression: CompressionFormat,
}

#[wasm_bindgen]
impl JsNbtFile {
    /// Read NBT file from bytes
    #[wasm_bindgen]
    pub fn read(data: &[u8]) -> Result<JsNbtFile, JsError> {
        let file = NbtFile::read(data)
            .map_err(|e| js_error(&format!("Failed to read NBT: {}", e)))?;
            
        Ok(JsNbtFile {
            root: file.root,
            name: file.root_name,
            compression: file.compression,
        })
    }

    /// Read NBT file with selective field parsing (performance optimization)
    #[wasm_bindgen(js_name = readFields)]
    pub fn read_fields(data: &[u8], fields: &str) -> Result<JsNbtFile, JsError> {
        let field_names: Vec<&str> = if fields.is_empty() {
            Vec::new()
        } else {
            fields.split(',').map(|s| s.trim()).collect()
        };
        
        let file = NbtFile::read_lazy(data, &field_names)
            .map_err(|e| js_error(&format!("Failed to read NBT: {}", e)))?;
            
        Ok(JsNbtFile {
            root: file.root,
            name: file.root_name,
            compression: file.compression,
        })
    }

    /// Get root tag
    #[wasm_bindgen(getter)]
    pub fn root(&self) -> JsNbtTag {
        JsNbtTag { inner: self.root.clone() }
    }

    // ============================================================================
    // NEW BATCH OPERATIONS - HIGH PERFORMANCE
    // ============================================================================

    /// Process multiple paths in one call - avoids WASM round-trips
    #[wasm_bindgen(js_name = getMultiplePaths)]
    pub fn get_multiple_paths(&self, paths: &str) -> JsValue {
        let path_list: Vec<&str> = paths.split(',').collect();
        let mut results = std::collections::HashMap::new();
        let root = JsNbtTag { inner: self.root.clone() };
        
        for path in path_list {
            let trimmed_path = path.trim();
            if let Some(tag) = root.get_by_path(trimmed_path) {
                if tag.is_string() {
                    results.insert(trimmed_path, tag.as_string());
                } else if tag.is_number() {
                    results.insert(trimmed_path, tag.as_number().to_string());
                }
            }
        }
        
        serde_wasm_bindgen::to_value(&results).unwrap_or(JsValue::NULL)
    }

    /// Get file name
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Get compression format as string
    #[wasm_bindgen(getter)]
    pub fn compression(&self) -> String {
        match self.compression {
            CompressionFormat::None => "none".to_string(),
            CompressionFormat::Gzip => "gzip".to_string(),
            CompressionFormat::Zlib => "zlib".to_string(),
        }
    }

    /// Write NBT file to bytes
    #[wasm_bindgen]
    pub fn write(&self) -> Result<Vec<u8>, JsError> {
        let file = NbtFile {
            root: self.root.clone(),
            root_name: self.name.clone(),
            compression: self.compression,
        };
        
        file.write()
            .map_err(|e| js_error(&format!("Failed to write NBT: {}", e)))
    }

    // ============================================================================
    // DIRECT MODIFICATION METHODS - Work on internal root to persist changes
    // ============================================================================

    /// Set string value by path - DIRECTLY modifies the internal root
    #[wasm_bindgen(js_name = setStringByPath)]
    pub fn set_string_by_path(&mut self, path: &str, value: &str) -> bool {
        let regex = ARRAY_REGEX.get_or_init(|| {
            Regex::new(r"^(.+)\[(\d+)\]$").unwrap()
        });

        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return false;
        }

        // Navigate to the parent and modify directly
        let mut current = &mut self.root;
        
        // Navigate through all but the last part
        for part in &parts[..parts.len() - 1] {
            // Handle array access: "items[0]"
            if let Some(captures) = regex.captures(part) {
                if let (Some(key_match), Some(index_match)) = (captures.get(1), captures.get(2)) {
                    let key = key_match.as_str();
                    if let Ok(index) = index_match.as_str().parse::<usize>() {
                        // Get the compound first
                        if let Some(map) = current.as_compound_mut() {
                            if let Some(NbtTag::List { items, .. }) = map.get_mut(&key.to_string()) {
                                if let Some(item) = items.get_mut(index) {
                                    current = item;
                                } else {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                // Regular key access
                if let Some(map) = current.as_compound_mut() {
                    if let Some(item) = map.get_mut(&part.to_string()) {
                        current = item;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        // Handle the final part
        let final_part = parts[parts.len() - 1];
        if let Some(captures) = regex.captures(final_part) {
            if let (Some(key_match), Some(index_match)) = (captures.get(1), captures.get(2)) {
                let key = key_match.as_str();
                if let Ok(index) = index_match.as_str().parse::<usize>() {
                    if let Some(map) = current.as_compound_mut() {
                        if let Some(NbtTag::List { items, .. }) = map.get_mut(&key.to_string()) {
                            if let Some(item) = items.get_mut(index) {
                                *item = NbtTag::String(value.to_string());
                                return true;
                            }
                        }
                    }
                }
            }
        } else {
            // Regular key modification
            if let Some(map) = current.as_compound_mut() {
                map.insert(final_part.to_string(), NbtTag::String(value.to_string()));
                return true;
            }
        }

        false
    }

    /// Set number value by path - DIRECTLY modifies the internal root
    #[wasm_bindgen(js_name = setNumberByPath)]
    pub fn set_number_by_path(&mut self, path: &str, value: f64) -> bool {
        let regex = ARRAY_REGEX.get_or_init(|| {
            Regex::new(r"^(.+)\[(\d+)\]$").unwrap()
        });

        let parts: Vec<&str> = path.split('.').collect();
        if parts.is_empty() {
            return false;
        }

        // Navigate to the parent and modify directly
        let mut current = &mut self.root;
        
        // Navigate through all but the last part
        for part in &parts[..parts.len() - 1] {
            if let Some(captures) = regex.captures(part) {
                if let (Some(key_match), Some(index_match)) = (captures.get(1), captures.get(2)) {
                    let key = key_match.as_str();
                    if let Ok(index) = index_match.as_str().parse::<usize>() {
                        if let Some(map) = current.as_compound_mut() {
                            if let Some(NbtTag::List { items, .. }) = map.get_mut(&key.to_string()) {
                                if let Some(item) = items.get_mut(index) {
                                    current = item;
                                } else {
                                    return false;
                                }
                            } else {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                if let Some(map) = current.as_compound_mut() {
                    if let Some(item) = map.get_mut(&part.to_string()) {
                        current = item;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        // Handle the final part
        let final_part = parts[parts.len() - 1];
        if let Some(captures) = regex.captures(final_part) {
            if let (Some(key_match), Some(index_match)) = (captures.get(1), captures.get(2)) {
                let key = key_match.as_str();
                if let Ok(index) = index_match.as_str().parse::<usize>() {
                    if let Some(map) = current.as_compound_mut() {
                        if let Some(NbtTag::List { items, .. }) = map.get_mut(&key.to_string()) {
                            if let Some(item) = items.get_mut(index) {
                                *item = NbtTag::Double(value);
                                return true;
                            }
                        }
                    }
                }
            }
        } else {
            if let Some(map) = current.as_compound_mut() {
                map.insert(final_part.to_string(), NbtTag::Double(value));
                return true;
            }
        }

        false
    }

    /// Modify list item by path and index - for compound modifications
    #[wasm_bindgen(js_name = modifyListItem)]
    pub fn modify_list_item(&mut self, list_path: &str, index: u32, key: &str, value: &str) -> bool {
        if let Some(root_map) = self.root.as_compound_mut() {
            if let Some(NbtTag::List { items, .. }) = root_map.get_mut(list_path) {
                if let Some(item) = items.get_mut(index as usize) {
                    if let Some(item_map) = item.as_compound_mut() {
                        item_map.insert(key.to_string(), NbtTag::String(value.to_string()));
                        return true;
                    }
                }
            }
        }
        false
    }
}


// ============================================================================
// Region File Support
// ============================================================================

/// NBT region file wrapper
#[wasm_bindgen]
pub struct JsNbtRegion {
    inner: Region,
}

#[wasm_bindgen]
impl JsNbtRegion {
    /// Read region from bytes
    #[wasm_bindgen]
    pub fn read(data: &[u8]) -> Result<JsNbtRegion, JsError> {
        let region = Region::read(data)
            .map_err(|e| js_error(&format!("Failed to read region: {}", e)))?;
        Ok(JsNbtRegion { inner: region })
    }

    /// Create empty region
    #[wasm_bindgen]
    pub fn new() -> JsNbtRegion {
        JsNbtRegion { inner: Region::new() }
    }

    /// Write region to bytes
    #[wasm_bindgen]
    pub fn write(&self) -> Result<Vec<u8>, JsError> {
        self.inner.write()
            .map_err(|e| js_error(&format!("Failed to write region: {}", e)))
    }

    /// Get chunk count
    #[wasm_bindgen(js_name = chunkCount)]
    pub fn chunk_count(&self) -> u32 {
        self.inner.chunk_count() as u32
    }

    /// Check if region is empty
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Get chunk positions as flat array [x1, z1, x2, z2, ...]
    #[wasm_bindgen(js_name = getChunkPositions)]
    pub fn get_chunk_positions(&self) -> Vec<i32> {
        let positions = self.inner.get_chunk_positions();
        let mut result = Vec::with_capacity(positions.len() * 2);
        for (x, z) in positions {
            result.push(x);
            result.push(z);
        }
        result
    }

    /// Get chunk data as NBT tag
    #[wasm_bindgen(js_name = getChunk)]
    pub fn get_chunk(&self, x: i32, z: i32) -> Result<Option<JsNbtTag>, JsError> {
        let chunk = self.inner.get_chunk(x, z)
            .map_err(|e| js_error(&format!("Failed to get chunk: {}", e)))?;
            
        if let Some(chunk) = chunk {
            let root = chunk.get_root_immutable()
                .map_err(|e| js_error(&format!("Failed to get chunk root: {}", e)))?;
            Ok(Some(JsNbtTag { inner: root }))
        } else {
            Ok(None)
        }
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Detect compression format from bytes
#[wasm_bindgen(js_name = detectCompression)]
pub fn detect_compression(data: &[u8]) -> String {
    if data.len() < 2 {
        return "none".to_string();
    }

    if data[0] == 0x1f && data[1] == 0x8b {
        "gzip".to_string()
    } else if data[0] == 0x78 && (data[1] & 0x20) == 0 {
        "zlib".to_string()
    } else {
        "none".to_string()
    }
}

/// Get version info
#[wasm_bindgen(js_name = getVersion)]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ============================================================================
// SNBT Support
// ============================================================================

/// Parse SNBT string to NBT tag
#[wasm_bindgen(js_name = parseSnbt)]
pub fn parse_snbt_wasm(input: &str) -> Result<JsNbtTag, JsError> {
    let tag = parse_snbt(input)
        .map_err(|e| js_error(&format!("SNBT parse error: {}", e)))?;
    Ok(JsNbtTag { inner: tag })
}

/// Format NBT tag to SNBT string
#[wasm_bindgen(js_name = formatSnbt)]
pub fn format_snbt_wasm(tag: &JsNbtTag) -> String {
    format_snbt(&tag.inner)
}

/// Format NBT tag to pretty SNBT string with indentation
#[wasm_bindgen(js_name = formatSnbtPretty)]
pub fn format_snbt_pretty_wasm(tag: &JsNbtTag) -> String {
    format_snbt_pretty(&tag.inner)
}

// ============================================================================
// Error Handling
// ============================================================================

/// JavaScript-compatible error type alias
pub type JsError = JsValue;

/// Create JavaScript error
pub fn js_error(message: &str) -> JsError {
    JsValue::from_str(&format!("Error: {}", message))
} 