use nbt_core::{NbtReader, NbtTag, NbtWriter, Endian};
use flate2::{read::{GzDecoder, ZlibDecoder}, write::{GzEncoder, ZlibEncoder}, Compression as FlateCompression};
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("NBT error: {0}")]
    Nbt(#[from] nbt_core::NbtError),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid format")]
    InvalidFormat,
}

pub type Result<T> = std::result::Result<T, CompressionError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionFormat {
    None,
    Gzip,
    Zlib,
}

/// Simple NBT file with compression support
#[derive(Debug, Clone)]
pub struct NbtFile {
    pub root: NbtTag,
    pub root_name: String,
    pub compression: CompressionFormat,
    pub endian: Endian,
}

impl NbtFile {
    pub fn new(root: NbtTag, root_name: String) -> Self {
        Self {
            root,
            root_name,
            compression: CompressionFormat::Gzip,
            endian: Endian::Big,
        }
    }

    // Legacy constructor with all parameters
    pub fn new_with_settings(root: NbtTag, root_name: String, compression: CompressionFormat, endian: Endian) -> Self {
        Self {
            root,
            root_name,
            compression,
            endian,
        }
    }

    pub fn read(data: &[u8]) -> Result<Self> {
        let compression = detect_compression(data);
        let uncompressed = decompress(data, compression)?;
        
        let mut reader = NbtReader::new(&uncompressed, Endian::Big);
        let tag_type = reader.read_u8()?;
        
        if tag_type != 10 {
            return Err(CompressionError::InvalidFormat);
        }
        
        let root_name = reader.read_string()?;
        let root = reader.read_tag(tag_type)?;
        
        Ok(Self {
            root,
            root_name,
            compression,
            endian: Endian::Big,
        })
    }

    pub fn read_with_format(data: &[u8], format: CompressionFormat, endian: Endian) -> Result<Self> {
        let uncompressed = decompress(data, format)?;

        let mut reader = NbtReader::new(&uncompressed, endian);
        let tag_type = reader.read_u8()?;
        
        if tag_type != 10 {
            return Err(CompressionError::InvalidFormat);
        }
        
        let root_name = reader.read_string()?;
        let root = reader.read_tag(tag_type)?;

        Ok(Self {
            root,
            root_name,
            compression: format,
            endian,
        })
    }

    pub fn write(&self) -> Result<Vec<u8>> {
        let mut writer = NbtWriter::new(self.endian);
        writer.write_u8(self.root.type_id());
        writer.write_string(&self.root_name);
        writer.write_tag(&self.root)?;
        
        let data = writer.into_bytes();
        compress(&data, self.compression)
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let data = std::fs::read(path)?;
        Self::read(&data)
    }

    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<()> {
        let data = self.write()?;
        std::fs::write(path, data)?;
        Ok(())
    }

    // Legacy API methods
    pub fn parse_full(&self) -> Result<NbtTag> {
        Ok(self.root.clone())
    }

    pub fn get(&self, key: &str) -> Option<&NbtTag> {
        self.root.get(key)
    }

    pub fn get_string(&self, key: &str) -> &str {
        self.root.get_string(key)
    }

    pub fn get_number(&self, key: &str) -> f64 {
        self.root.get_number(key)
    }

    // Compatibility methods (simplified versions)
    pub fn get_tag_by_path(&self, path: &str) -> Result<Option<NbtTag>> {
        Ok(self.root.get(path).cloned())
    }

    pub fn extract_paths(&self, paths: &[&str]) -> Result<std::collections::HashMap<String, NbtTag>> {
        let mut result = std::collections::HashMap::new();
        for path in paths {
            if let Some(tag) = self.root.get(path) {
                result.insert(path.to_string(), tag.clone());
            }
        }
        Ok(result)
    }

    pub fn detect_compression(data: &[u8]) -> CompressionFormat {
        detect_compression(data)
    }
}

fn detect_compression(data: &[u8]) -> CompressionFormat {
    if data.len() < 2 {
        return CompressionFormat::None;
    }

    if data[0] == 0x1f && data[1] == 0x8b {
        return CompressionFormat::Gzip;
    }

    if data[0] == 0x78 && (data[1] & 0x20) == 0 {
        return CompressionFormat::Zlib;
    }

    CompressionFormat::None
}

fn decompress(data: &[u8], compression: CompressionFormat) -> Result<Vec<u8>> {
    match compression {
        CompressionFormat::None => Ok(data.to_vec()),
        CompressionFormat::Gzip => {
        let mut decoder = GzDecoder::new(data);
        let mut result = Vec::new();
        decoder.read_to_end(&mut result)?;
        Ok(result)
        },
        CompressionFormat::Zlib => {
        let mut decoder = ZlibDecoder::new(data);
        let mut result = Vec::new();
        decoder.read_to_end(&mut result)?;
        Ok(result)
        },
    }
}

fn compress(data: &[u8], compression: CompressionFormat) -> Result<Vec<u8>> {
    match compression {
        CompressionFormat::None => Ok(data.to_vec()),
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), FlateCompression::default());
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        },
        CompressionFormat::Zlib => {
            let mut encoder = ZlibEncoder::new(Vec::new(), FlateCompression::default());
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        },
    }
}

// Simple convenience functions
pub fn read_nbt(data: &[u8]) -> Result<(NbtTag, String)> {
    let file = NbtFile::read(data)?;
    Ok((file.root, file.root_name))
}

pub fn write_nbt(root: &NbtTag, root_name: &str, compression: CompressionFormat) -> Result<Vec<u8>> {
    let file = NbtFile {
        root: root.clone(),
        root_name: root_name.to_string(),
        compression,
        endian: Endian::Big,
    };
    file.write()
}

// Legacy compatibility functions
pub fn write_nbt_gzip(root: &NbtTag, root_name: &str, _endian: Endian) -> Result<Vec<u8>> {
    write_nbt(root, root_name, CompressionFormat::Gzip)
}

pub fn write_nbt_zlib(root: &NbtTag, root_name: &str, _endian: Endian) -> Result<Vec<u8>> {
    write_nbt(root, root_name, CompressionFormat::Zlib)
}

pub fn write_nbt_uncompressed(root: &NbtTag, root_name: &str, endian: Endian) -> Result<Vec<u8>> {
    let mut writer = NbtWriter::new(endian);
    writer.write_u8(root.type_id());
    writer.write_string(root_name);
    writer.write_tag(root)?;
    Ok(writer.into_bytes())
} 