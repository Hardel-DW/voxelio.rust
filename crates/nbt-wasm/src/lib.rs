use wasm_bindgen::prelude::*;

// Re-export core types
use nbt_core::NbtTag;
use nbt_compression::{NbtFile, CompressionFormat};
use nbt_snbt::{parse_snbt, format_snbt};
use nbt_region::Region;

/// Set panic hook for better debugging
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

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

    /// Get string value by key
    #[wasm_bindgen(js_name = getString)]
    pub fn get_string(&self, key: &str) -> String {
        self.inner.get_string(key).to_string()
    }

    /// Get number value by key
    #[wasm_bindgen(js_name = getNumber)]
    pub fn get_number(&self, key: &str) -> f64 {
        self.inner.get_number(key)
    }

    /// Get boolean value by key
    #[wasm_bindgen(js_name = getBool)]
    pub fn get_bool(&self, key: &str) -> bool {
        self.inner.get_bool(key)
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

    /// Set string value in list item compound by index and key
    #[wasm_bindgen(js_name = setStringInListItem)]
    pub fn set_string_in_list_item(&mut self, index: u32, key: &str, value: &str) -> bool {
        match &mut self.inner {
            NbtTag::List { items, .. } => {
                if let Some(NbtTag::Compound(map)) = items.get_mut(index as usize) {
                    map.insert(key.to_string(), NbtTag::String(value.to_string()));
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Get string value from list item compound by index and key
    #[wasm_bindgen(js_name = getStringFromListItem)]
    pub fn get_string_from_list_item(&self, index: u32, key: &str) -> String {
        match &self.inner {
            NbtTag::List { items, .. } => {
                if let Some(NbtTag::Compound(map)) = items.get(index as usize) {
                    if let Some(NbtTag::String(value)) = map.get(key) {
                        value.clone()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }

    /// Convert to JSON for JavaScript consumption
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.serialize_to_json()).unwrap_or(JsValue::NULL)
    }
}

impl JsNbtTag {
    fn serialize_to_json(&self) -> serde_json::Value {
        match &self.inner {
            NbtTag::End => serde_json::Value::Null,
            NbtTag::Byte(v) => serde_json::Value::Number((*v as i64).into()),
            NbtTag::Short(v) => serde_json::Value::Number((*v as i64).into()),
            NbtTag::Int(v) => serde_json::Value::Number((*v as i64).into()),
            NbtTag::Long(v) => serde_json::Value::Number((*v).into()),
            NbtTag::Float(v) => serde_json::Number::from_f64(*v as f64)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            NbtTag::Double(v) => serde_json::Number::from_f64(*v)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            NbtTag::String(s) => serde_json::Value::String(s.clone()),
            NbtTag::ByteArray(arr) => serde_json::Value::Array(
                arr.iter().map(|&v| serde_json::Value::Number((v as i64).into())).collect()
            ),
            NbtTag::IntArray(arr) => serde_json::Value::Array(
                arr.iter().map(|&v| serde_json::Value::Number((v as i64).into())).collect()
            ),
            NbtTag::LongArray(arr) => serde_json::Value::Array(
                arr.iter().map(|&v| serde_json::Value::Number(v.into())).collect()
            ),
            NbtTag::List { items, .. } => serde_json::Value::Array(
                items.iter().map(|tag| JsNbtTag { inner: tag.clone() }.serialize_to_json()).collect()
            ),
            NbtTag::Compound(map) => serde_json::Value::Object(
                map.iter().map(|(k, v)| {
                    (k.clone(), JsNbtTag { inner: v.clone() }.serialize_to_json())
                }).collect()
            ),
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
    
    /// Get mutable root tag (for editing)
    #[wasm_bindgen(js_name = getRootMut)]
    pub fn root_mut(&mut self) -> JsNbtTag {
        JsNbtTag { inner: self.root.clone() }
    }
    
    /// Update root from modified JsNbtTag
    #[wasm_bindgen(js_name = setRoot)]
    pub fn set_root(&mut self, new_root: JsNbtTag) {
        self.root = new_root.inner;
    }
    
    /// Direct edit methods to avoid copy issues - using same logic as Rust example
    #[wasm_bindgen(js_name = setStringInListItem)]
    pub fn set_string_in_list_item(&mut self, path: &str, index: u32, key: &str, value: &str) -> bool {
        // Same logic as the working Rust example
        if let NbtTag::Compound(root_map) = &mut self.root {
            if let Some(NbtTag::List { items, .. }) = root_map.get_mut(path) {
                if let Some(NbtTag::Compound(item_map)) = items.get_mut(index as usize) {
                    if let Some(NbtTag::String(name)) = item_map.get_mut(key) {
                        *name = value.to_string();
                        return true;
                    }
                }
            }
        }
        false
    }
    
    /// Get string from list item
    #[wasm_bindgen(js_name = getStringFromListItem)]
    pub fn get_string_from_list_item(&self, path: &str, index: u32, key: &str) -> String {
        // Same logic as the working Rust example
        if let NbtTag::Compound(root_map) = &self.root {
            if let Some(NbtTag::List { items, .. }) = root_map.get(path) {
                if let Some(NbtTag::Compound(item_map)) = items.get(index as usize) {
                    if let Some(NbtTag::String(name)) = item_map.get(key) {
                        return name.clone();
                    }
                }
            }
        }
        String::new()
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

    /// Create a new NBT file from SNBT string
    #[wasm_bindgen(js_name = fromSnbt)]
    pub fn from_snbt(snbt: &str, name: &str, compression: &str) -> Result<JsNbtFile, JsError> {
        let root = parse_snbt(snbt)
            .map_err(|e| js_error(&format!("Failed to parse SNBT: {}", e)))?;
            
        let compression_format = match compression {
            "gzip" => CompressionFormat::Gzip,
            "zlib" => CompressionFormat::Zlib,
            "none" => CompressionFormat::None,
            _ => CompressionFormat::Gzip, // default
        };
        
        Ok(JsNbtFile {
            root,
            name: name.to_string(),
            compression: compression_format,
        })
    }
}

// ============================================================================
// SNBT Parser
// ============================================================================

/// Parse SNBT string to NBT tag
#[wasm_bindgen(js_name = parseSnbt)]
pub fn parse_snbt_js(input: &str) -> Result<JsNbtTag, JsError> {
    let tag = parse_snbt(input)
        .map_err(|e| js_error(&format!("SNBT parse error: {}", e)))?;
    Ok(JsNbtTag { inner: tag })
}

/// Format NBT tag to SNBT string
#[wasm_bindgen(js_name = formatSnbt)]
pub fn format_snbt_js(tag: &JsNbtTag) -> String {
    format_snbt(&tag.inner)
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
// Error Handling
// ============================================================================

/// JavaScript-compatible error type alias
pub type JsError = JsValue;

/// Create JavaScript error
pub fn js_error(message: &str) -> JsError {
    JsValue::from_str(&format!("Error: {}", message))
} 