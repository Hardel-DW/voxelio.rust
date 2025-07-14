use crate::{NbtFile, NbtTag};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use wasm_bindgen::prelude::*;

#[cfg(feature = "region")]
use crate::region::Region;

struct FileStore {
    files: HashMap<u32, NbtFile>,
    next_id: u32,
}

struct TagStore {
    tags: HashMap<u32, NbtTag>,
    next_id: u32,
}

#[cfg(feature = "region")]
struct RegionStore {
    regions: HashMap<u32, Region>,
    next_id: u32,
}

static FILE_STORE: LazyLock<Mutex<FileStore>> = LazyLock::new(|| {
    Mutex::new(FileStore {
        files: HashMap::new(),
        next_id: 1,
    })
});

static TAG_STORE: LazyLock<Mutex<TagStore>> = LazyLock::new(|| {
    Mutex::new(TagStore {
        tags: HashMap::new(),
        next_id: 1,
    })
});

#[cfg(feature = "region")]
static REGION_STORE: LazyLock<Mutex<RegionStore>> = LazyLock::new(|| {
    Mutex::new(RegionStore {
        regions: HashMap::new(),
        next_id: 1,
    })
});

#[wasm_bindgen]
pub fn nbt_file_read(data: &[u8]) -> std::result::Result<u32, JsValue> {
    match NbtFile::read(data) {
        Ok(file) => {
            let mut store = FILE_STORE.lock().unwrap();
            let id = store.next_id;
            store.next_id += 1;
            store.files.insert(id, file);
            Ok(id)
        }
        Err(e) => Err(JsValue::from_str(&format!("Failed to read NBT: {}", e))),
    }
}

#[wasm_bindgen]
pub fn nbt_file_read_lazy(data: &[u8], fields: js_sys::Array) -> std::result::Result<u32, JsValue> {
    let fields_vec: Vec<String> = fields
        .iter()
        .map(|val| val.as_string().unwrap_or_default())
        .collect();

    let fields_str: Vec<&str> = fields_vec.iter().map(|s| s.as_str()).collect();
    match NbtFile::read_lazy(data, &fields_str) {
        Ok(file) => {
            let mut store = FILE_STORE.lock().unwrap();
            let id = store.next_id;
            store.next_id += 1;
            store.files.insert(id, file);
            Ok(id)
        }
        Err(e) => Err(JsValue::from_str(&format!(
            "Failed to read NBT lazily: {}",
            e
        ))),
    }
}

#[wasm_bindgen]
pub fn nbt_file_write(handle: u32) -> std::result::Result<Vec<u8>, JsValue> {
    let store = FILE_STORE.lock().unwrap();
    match store.files.get(&handle) {
        Some(file) => match file.write() {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("Write error: {}", e))),
        },
        None => Err(JsValue::from_str("Invalid file handle")),
    }
}

#[wasm_bindgen]
pub fn nbt_file_dispose(handle: u32) {
    let mut store = FILE_STORE.lock().unwrap();
    store.files.remove(&handle);
}

#[wasm_bindgen]
pub fn nbt_get_string(handle: u32, key: &str) -> std::result::Result<String, JsValue> {
    let store = FILE_STORE.lock().unwrap();
    match store.files.get(&handle) {
        Some(file) => Ok(file.get_string(key).to_string()),
        None => Err(JsValue::from_str("Invalid file handle")),
    }
}

#[wasm_bindgen]
pub fn nbt_get_number(handle: u32, key: &str) -> std::result::Result<f64, JsValue> {
    let store = FILE_STORE.lock().unwrap();
    match store.files.get(&handle) {
        Some(file) => Ok(file.get_number(key)),
        None => Err(JsValue::from_str("Invalid file handle")),
    }
}

#[wasm_bindgen]
pub fn nbt_get_root(handle: u32) -> std::result::Result<u32, JsValue> {
    let file_store = FILE_STORE.lock().unwrap();
    match file_store.files.get(&handle) {
        Some(file) => {
            let root_tag = file.root.clone();
            drop(file_store);

            let mut tag_store = TAG_STORE.lock().unwrap();
            let tag_id = tag_store.next_id;
            tag_store.next_id += 1;
            tag_store.tags.insert(tag_id, root_tag);
            Ok(tag_id)
        }
        None => Err(JsValue::from_str("Invalid file handle")),
    }
}

#[wasm_bindgen]
pub fn nbt_tag_type(handle: u32) -> std::result::Result<u8, JsValue> {
    let store = TAG_STORE.lock().unwrap();
    match store.tags.get(&handle) {
        Some(tag) => Ok(tag.type_id()),
        None => Err(JsValue::from_str("Invalid tag handle")),
    }
}

#[wasm_bindgen]
pub fn nbt_tag_as_string(handle: u32) -> std::result::Result<String, JsValue> {
    let store = TAG_STORE.lock().unwrap();
    match store.tags.get(&handle) {
        Some(tag) => Ok(tag.as_string().to_string()),
        None => Err(JsValue::from_str("Invalid tag handle")),
    }
}

#[wasm_bindgen]
pub fn nbt_tag_as_number(handle: u32) -> std::result::Result<f64, JsValue> {
    let store = TAG_STORE.lock().unwrap();
    match store.tags.get(&handle) {
        Some(tag) => Ok(tag.as_number()),
        None => Err(JsValue::from_str("Invalid tag handle")),
    }
}

#[wasm_bindgen]
pub fn nbt_tag_get_compound_keys(handle: u32) -> std::result::Result<Vec<String>, JsValue> {
    let store = TAG_STORE.lock().unwrap();
    let tag = store
        .tags
        .get(&handle)
        .ok_or_else(|| JsValue::from_str("Invalid tag handle"))?;

    match tag.as_compound() {
        Some(compound) => Ok(compound.keys().cloned().collect()),
        None => Ok(vec![]),
    }
}

#[wasm_bindgen]
pub fn nbt_tag_get_compound_value(handle: u32, key: &str) -> std::result::Result<u32, JsValue> {
    let tag_value = {
        let store = TAG_STORE.lock().unwrap();
        let tag = store
            .tags
            .get(&handle)
            .ok_or_else(|| JsValue::from_str("Invalid tag handle"))?;

        let compound = tag
            .as_compound()
            .ok_or_else(|| JsValue::from_str("Not a compound tag"))?;

        compound
            .get(key)
            .ok_or_else(|| JsValue::from_str("Key not found"))?
            .clone()
    };

    let mut tag_store = TAG_STORE.lock().unwrap();
    let tag_id = tag_store.next_id;
    tag_store.next_id += 1;
    tag_store.tags.insert(tag_id, tag_value);
    Ok(tag_id)
}

#[wasm_bindgen]
pub fn nbt_tag_get_list_length(handle: u32) -> std::result::Result<u32, JsValue> {
    let store = TAG_STORE.lock().unwrap();
    let tag = store
        .tags
        .get(&handle)
        .ok_or_else(|| JsValue::from_str("Invalid tag handle"))?;

    match tag.as_list() {
        Some((_, items)) => Ok(items.len() as u32),
        None => Ok(0),
    }
}

#[wasm_bindgen]
pub fn nbt_tag_get_list_item(handle: u32, index: u32) -> std::result::Result<u32, JsValue> {
    let list_item = {
        let store = TAG_STORE.lock().unwrap();
        let tag = store
            .tags
            .get(&handle)
            .ok_or_else(|| JsValue::from_str("Invalid tag handle"))?;

        let (_, items) = tag
            .as_list()
            .ok_or_else(|| JsValue::from_str("Not a list tag"))?;

        items
            .get(index as usize)
            .ok_or_else(|| JsValue::from_str("Index out of bounds"))?
            .clone()
    };

    let mut tag_store = TAG_STORE.lock().unwrap();
    let tag_id = tag_store.next_id;
    tag_store.next_id += 1;
    tag_store.tags.insert(tag_id, list_item);
    Ok(tag_id)
}

#[wasm_bindgen]
pub fn nbt_file_set_list_item_string(
    file_handle: u32,
    path: &str,
    index: u32,
    key: &str,
    value: &str,
) -> std::result::Result<(), JsValue> {
    let mut file_store = FILE_STORE.lock().unwrap();
    let file = file_store
        .files
        .get_mut(&file_handle)
        .ok_or_else(|| JsValue::from_str("Invalid file handle"))?;

    let compound = file
        .root
        .as_compound_mut()
        .ok_or_else(|| JsValue::from_str("Root is not a compound tag"))?;

    let list_tag = compound
        .get_mut(path)
        .ok_or_else(|| JsValue::from_str("Path not found"))?;

    let (_, items) = list_tag
        .as_list_mut()
        .ok_or_else(|| JsValue::from_str("Not a list tag"))?;

    let item = items
        .get_mut(index as usize)
        .ok_or_else(|| JsValue::from_str("Index out of bounds"))?;

    let item_compound = item
        .as_compound_mut()
        .ok_or_else(|| JsValue::from_str("List item is not a compound tag"))?;

    item_compound.insert(key.to_string(), NbtTag::String(value.to_string()));
    Ok(())
}

#[wasm_bindgen]
pub fn nbt_tag_dispose(handle: u32) {
    let mut store = TAG_STORE.lock().unwrap();
    store.tags.remove(&handle);
}

#[wasm_bindgen]
pub fn nbt_tag_get_string(handle: u32, key: &str) -> std::result::Result<String, JsValue> {
    let store = TAG_STORE.lock().unwrap();
    let tag = store
        .tags
        .get(&handle)
        .ok_or_else(|| JsValue::from_str("Invalid tag handle"))?;

    let compound = tag
        .as_compound()
        .ok_or_else(|| JsValue::from_str("Not a compound tag"))?;

    let value = compound
        .get(key)
        .ok_or_else(|| JsValue::from_str("Key not found"))?;

    Ok(value.as_string().to_string())
}

#[wasm_bindgen]
pub fn nbt_tag_get_number(handle: u32, key: &str) -> std::result::Result<f64, JsValue> {
    let store = TAG_STORE.lock().unwrap();
    let tag = store
        .tags
        .get(&handle)
        .ok_or_else(|| JsValue::from_str("Invalid tag handle"))?;

    let compound = tag
        .as_compound()
        .ok_or_else(|| JsValue::from_str("Not a compound tag"))?;

    let value = compound
        .get(key)
        .ok_or_else(|| JsValue::from_str("Key not found"))?;

    Ok(value.as_number())
}

#[wasm_bindgen]
pub fn nbt_tag_set_string(handle: u32, key: &str, value: &str) -> std::result::Result<(), JsValue> {
    let mut store = TAG_STORE.lock().unwrap();
    let tag = store
        .tags
        .get_mut(&handle)
        .ok_or_else(|| JsValue::from_str("Invalid tag handle"))?;

    let compound = tag
        .as_compound_mut()
        .ok_or_else(|| JsValue::from_str("Not a compound tag"))?;

    compound.insert(key.to_string(), NbtTag::String(value.to_string()));
    Ok(())
}

#[wasm_bindgen]
pub fn nbt_tag_set_number(handle: u32, key: &str, value: f64) -> std::result::Result<(), JsValue> {
    let mut store = TAG_STORE.lock().unwrap();
    let tag = store
        .tags
        .get_mut(&handle)
        .ok_or_else(|| JsValue::from_str("Invalid tag handle"))?;

    let compound = tag
        .as_compound_mut()
        .ok_or_else(|| JsValue::from_str("Not a compound tag"))?;

    compound.insert(key.to_string(), NbtTag::Double(value));
    Ok(())
}

#[cfg(feature = "region")]
#[wasm_bindgen]
pub fn nbt_region_read(data: &[u8]) -> std::result::Result<u32, JsValue> {
    match Region::read(data) {
        Ok(region) => {
            let mut store = REGION_STORE.lock().unwrap();
            let id = store.next_id;
            store.next_id += 1;
            store.regions.insert(id, region);
            Ok(id)
        }
        Err(e) => Err(JsValue::from_str(&format!("Failed to read region: {}", e))),
    }
}

#[cfg(feature = "region")]
#[wasm_bindgen]
pub fn nbt_region_write(handle: u32) -> std::result::Result<Vec<u8>, JsValue> {
    let store = REGION_STORE.lock().unwrap();
    match store.regions.get(&handle) {
        Some(region) => match region.write() {
            Ok(data) => Ok(data),
            Err(e) => Err(JsValue::from_str(&format!("Write error: {}", e))),
        },
        None => Err(JsValue::from_str("Invalid region handle")),
    }
}

#[cfg(feature = "region")]
#[wasm_bindgen]
pub fn nbt_region_dispose(handle: u32) {
    let mut store = REGION_STORE.lock().unwrap();
    store.regions.remove(&handle);
}
