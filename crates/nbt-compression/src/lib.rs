use nbt_core::{NbtReader, NbtTag, NbtWriter, Endian, Result as NbtResult};
use flate2::{read::{GzDecoder, ZlibDecoder}, write::{GzEncoder, ZlibEncoder}, Compression};
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("NBT error: {0}")]
    Nbt(#[from] nbt_core::NbtError),
    
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Unknown compression format")]
    UnknownFormat,
    
    #[error("Invalid header")]
    InvalidHeader,
}

pub type Result<T> = std::result::Result<T, CompressionError>;

/// Compression format for NBT files
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionFormat {
    /// No compression (raw NBT)
    None,
    /// Gzip compression (most common in Minecraft)
    Gzip,
    /// Zlib compression (used in region files)
    Zlib,
}

/// NBT file with compression support
#[derive(Debug)]
pub struct NbtFile {
    pub root: NbtTag,
    pub root_name: String,
    pub compression: CompressionFormat,
    pub endian: Endian,
}

impl NbtFile {
    /// Create a new NBT file
    pub fn new(root: NbtTag, root_name: String, compression: CompressionFormat, endian: Endian) -> Self {
        Self { root, root_name, compression, endian }
    }

    /// Auto-detect compression format from file header
    pub fn detect_compression(data: &[u8]) -> CompressionFormat {
        if data.len() < 2 {
            return CompressionFormat::None;
        }

        // Gzip magic bytes: 0x1f 0x8b
        if data[0] == 0x1f && data[1] == 0x8b {
            return CompressionFormat::Gzip;
        }

        // Zlib magic bytes: 0x78 followed by 0x01, 0x9c, 0xda, etc.
        if data[0] == 0x78 && (data[1] & 0x20) == 0 {
            return CompressionFormat::Zlib;
        }

        CompressionFormat::None
    }

    /// Read NBT from compressed bytes with auto-detection
    pub fn read(data: &[u8], endian: Endian) -> Result<Self> {
        let compression = Self::detect_compression(data);
        Self::read_with_format(data, compression, endian)
    }

    /// Read NBT with specific compression format
    pub fn read_with_format(data: &[u8], format: CompressionFormat, endian: Endian) -> Result<Self> {
        let uncompressed = match format {
            CompressionFormat::None => data.to_vec(),
            CompressionFormat::Gzip => Self::decompress_gzip(data)?,
            CompressionFormat::Zlib => Self::decompress_zlib(data)?,
        };

        let mut reader = NbtReader::new(&uncompressed, endian);
        
        // Read root tag (should be compound)
        let tag_type = reader.read_u8()?;
        if tag_type != 10 {
            return Err(CompressionError::InvalidHeader);
        }
        
        let root_name = reader.read_string()?;
        let root = reader.read_tag(tag_type)?;

        Ok(Self::new(root, root_name, format, endian))
    }

    /// Write NBT to compressed bytes
    pub fn write(&self) -> Result<Vec<u8>> {
        // Write uncompressed NBT first
        let mut writer = NbtWriter::new(self.endian);
        writer.write_u8(self.root.type_id());
        writer.write_string(&self.root_name);
        writer.write_tag(&self.root)?;
        let uncompressed = writer.into_bytes();

        // Apply compression
        match self.compression {
            CompressionFormat::None => Ok(uncompressed),
            CompressionFormat::Gzip => Self::compress_gzip(&uncompressed),
            CompressionFormat::Zlib => Self::compress_zlib(&uncompressed),
        }
    }

    /// Compress data with gzip (fast native compression)
    pub fn compress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }

    /// Decompress gzip data (fast native decompression)
    pub fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = GzDecoder::new(data);
        let mut result = Vec::new();
        decoder.read_to_end(&mut result)?;
        Ok(result)
    }

    /// Compress data with zlib (fast native compression)
    pub fn compress_zlib(data: &[u8]) -> Result<Vec<u8>> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }

    /// Decompress zlib data (fast native decompression)  
    pub fn decompress_zlib(data: &[u8]) -> Result<Vec<u8>> {
        let mut decoder = ZlibDecoder::new(data);
        let mut result = Vec::new();
        decoder.read_to_end(&mut result)?;
        Ok(result)
    }
}

// =============== CONVENIENCE FUNCTIONS ===============

/// Read NBT file from bytes with auto-detection
pub fn read_nbt(data: &[u8], endian: Endian) -> Result<(NbtTag, String)> {
    let file = NbtFile::read(data, endian)?;
    Ok((file.root, file.root_name))
}

/// Write NBT to gzip compressed bytes (most common format)
pub fn write_nbt_gzip(root: &NbtTag, root_name: &str, endian: Endian) -> Result<Vec<u8>> {
    let file = NbtFile::new(root.clone(), root_name.to_string(), CompressionFormat::Gzip, endian);
    file.write()
}

/// Write NBT to zlib compressed bytes (region files)
pub fn write_nbt_zlib(root: &NbtTag, root_name: &str, endian: Endian) -> Result<Vec<u8>> {
    let file = NbtFile::new(root.clone(), root_name.to_string(), CompressionFormat::Zlib, endian);
    file.write()
}

/// Write NBT to uncompressed bytes
pub fn write_nbt_uncompressed(root: &NbtTag, root_name: &str, endian: Endian) -> NbtResult<Vec<u8>> {
    let mut writer = NbtWriter::new(endian);
    writer.write_u8(root.type_id());
    writer.write_string(root_name);
    writer.write_tag(root)?;
    Ok(writer.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nbt_core::HashMap;

    fn create_test_nbt() -> (NbtTag, String) {
        let mut map = HashMap::new();
        map.insert("name".to_string(), NbtTag::String("TestWorld".to_string()));
        map.insert("version".to_string(), NbtTag::Int(19133));
        map.insert("spawn_x".to_string(), NbtTag::Int(0));
        map.insert("spawn_y".to_string(), NbtTag::Int(64));
        map.insert("spawn_z".to_string(), NbtTag::Int(0));
        
        (NbtTag::Compound(map), "Data".to_string())
    }

    #[test]
    fn test_compression_detection() {
        // Gzip header
        let gzip_data = &[0x1f, 0x8b, 0x08, 0x00];
        assert_eq!(NbtFile::detect_compression(gzip_data), CompressionFormat::Gzip);

        // Zlib header (0x78 + valid second byte)
        let zlib_data = &[0x78, 0x9c];
        assert_eq!(NbtFile::detect_compression(zlib_data), CompressionFormat::Zlib);

        // Raw NBT (compound tag)
        let raw_data = &[0x0a, 0x00, 0x04];
        assert_eq!(NbtFile::detect_compression(raw_data), CompressionFormat::None);
    }

    #[test]
    fn test_gzip_roundtrip() {
        let (root, root_name) = create_test_nbt();
        
        // Write compressed
        let compressed = write_nbt_gzip(&root, &root_name, Endian::Big).unwrap();
        
        // Note: For very small NBT files, compression might be larger due to overhead
        // This is expected and normal behavior
        let _uncompressed = write_nbt_uncompressed(&root, &root_name, Endian::Big).unwrap();
        
        // Read back and verify round-trip works
        let (parsed_root, parsed_name) = read_nbt(&compressed, Endian::Big).unwrap();
        assert_eq!(parsed_root, root);
        assert_eq!(parsed_name, root_name);
    }

    #[test]
    fn test_zlib_roundtrip() {
        let (root, root_name) = create_test_nbt();
        
        // Write compressed
        let compressed = write_nbt_zlib(&root, &root_name, Endian::Big).unwrap();
        
        // Note: For very small NBT files, compression might be larger due to overhead
        let _uncompressed = write_nbt_uncompressed(&root, &root_name, Endian::Big).unwrap();
        
        // Read back and verify round-trip works
        let (parsed_root, parsed_name) = read_nbt(&compressed, Endian::Big).unwrap();
        assert_eq!(parsed_root, root);
        assert_eq!(parsed_name, root_name);
    }

    #[test]
    fn test_auto_detection() {
        let (root, root_name) = create_test_nbt();
        
        // Test gzip auto-detection
        let gzip_data = write_nbt_gzip(&root, &root_name, Endian::Big).unwrap();
        let file = NbtFile::read(&gzip_data, Endian::Big).unwrap();
        assert_eq!(file.compression, CompressionFormat::Gzip);
        assert_eq!(file.root, root);
        
        // Test zlib auto-detection  
        let zlib_data = write_nbt_zlib(&root, &root_name, Endian::Big).unwrap();
        let file = NbtFile::read(&zlib_data, Endian::Big).unwrap();
        assert_eq!(file.compression, CompressionFormat::Zlib);
        assert_eq!(file.root, root);
    }

    #[test]
    fn test_minecraft_like_structure() {
        // Simule une structure level.dat typique
        let mut data = HashMap::new();
        data.insert("LevelName".to_string(), NbtTag::String("New World".to_string()));
        data.insert("version".to_string(), NbtTag::Int(19133));
        data.insert("SpawnX".to_string(), NbtTag::Int(256));
        data.insert("SpawnY".to_string(), NbtTag::Int(64));
        data.insert("SpawnZ".to_string(), NbtTag::Int(-128));
        data.insert("Time".to_string(), NbtTag::Long(123456789));
        data.insert("raining".to_string(), NbtTag::Byte(0));
        data.insert("thundering".to_string(), NbtTag::Byte(1));
        
        let root = NbtTag::Compound(data);
        
        // Test avec gzip (format level.dat standard)
        let file = NbtFile::new(root.clone(), "".to_string(), CompressionFormat::Gzip, Endian::Big);
        let compressed = file.write().unwrap();
        
        // Vérifie que c'est du gzip valide
        assert_eq!(compressed[0], 0x1f);
        assert_eq!(compressed[1], 0x8b);
        
        // Round-trip
        let parsed = NbtFile::read(&compressed, Endian::Big).unwrap();
        assert_eq!(parsed.root, root);
        assert_eq!(parsed.compression, CompressionFormat::Gzip);
    }
} 