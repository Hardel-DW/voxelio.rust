use nbt_core::{NbtReader, NbtTag, Endian};
#[cfg(feature = "compression")]
use flate2::read::GzDecoder;
#[cfg(feature = "compression")]
use std::io::Read;
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



/// Décompression optimisée avec buffer pooling
fn decompress_optimized(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>> {
    match format {
        CompressionFormat::None => Ok(data.to_vec()),
        #[cfg(feature = "compression")]
        CompressionFormat::Gzip => {
            let mut decoder = GzDecoder::new(data);
            let mut result = Vec::with_capacity(data.len() * 3); // Estimation conservative
            decoder.read_to_end(&mut result)?;
            Ok(result)
        },
        #[cfg(feature = "compression")]
        CompressionFormat::Zlib => {
            use flate2::read::ZlibDecoder;
            let mut decoder = ZlibDecoder::new(data);
            let mut result = Vec::with_capacity(data.len() * 3);
            decoder.read_to_end(&mut result)?;
            Ok(result)
        }
        #[cfg(not(feature = "compression"))]
        CompressionFormat::Gzip | CompressionFormat::Zlib => {
            Err(CompressionError::InvalidFormat)
        }
    }
}

/// NBT File optimisé pour le streaming et lazy parsing
#[derive(Debug)]
pub struct NbtFile {
    pub root: NbtTag,
    pub root_name: String,
    pub compression: CompressionFormat,
}

impl NbtFile {
    /// Lecture optimisée avec décompression efficace
    pub fn read(data: &[u8]) -> Result<Self> {
        let format = detect_compression(data);
        
        // Décompression optimisée
        let decompressed = decompress_optimized(data, format)?;
        
        // Parse avec le buffer décompressé
        let mut reader = NbtReader::new(&decompressed, Endian::Big);
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
        })
    }
    
    /// Lecture lazy - ne parse que les champs spécifiés
    pub fn read_lazy(data: &[u8], fields: &[&str]) -> Result<Self> {
        let format = detect_compression(data);
        
        // Décompression optimisée
        let decompressed = decompress_optimized(data, format)?;
        
        // Parse lazy avec NbtReader
        let mut reader = NbtReader::new(&decompressed, Endian::Big);
        let tag_type = reader.read_u8()?;
        
        if tag_type != 10 {
            return Err(CompressionError::InvalidFormat);
        }
        
        let root_name = reader.read_string()?;
        
        // Parse seulement les champs demandés
        let root = if fields.is_empty() {
            reader.read_tag(tag_type)?
        } else {
            reader.read_compound_selective(fields)?
        };
        
        Ok(Self {
            root,
            root_name,
            compression: format,
        })
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

    /// Create NBT file with specific settings
    pub fn new_with_settings(root: NbtTag, root_name: String, compression: CompressionFormat, _endian: Endian) -> Self {
        Self {
            root,
            root_name,
            compression,
        }
    }

    /// Read with specific format and endianness
    pub fn read_with_format(data: &[u8], format: CompressionFormat, endian: Endian) -> Result<Self> {
        // Décompression selon le format spécifié
        let decompressed = decompress_optimized(data, format)?;
        
        // Parse avec l'endianness spécifié
        let mut reader = NbtReader::new(&decompressed, endian);
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
        })
    }

    /// Write NBT file to bytes with compression
    pub fn write(&self) -> Result<Vec<u8>> {
        use nbt_core::NbtWriter;
        
        // Écrire les données NBT
        let mut writer = NbtWriter::new(Endian::Big);
        writer.write_u8(10); // Compound tag
        writer.write_string(&self.root_name);
        writer.write_tag(&self.root)?;
        
        let uncompressed = writer.into_bytes();
        
        // Compresser selon le format
        match self.compression {
            CompressionFormat::None => Ok(uncompressed),
            CompressionFormat::Gzip => compress_data(&uncompressed, CompressionFormat::Gzip),
            CompressionFormat::Zlib => compress_data(&uncompressed, CompressionFormat::Zlib),
        }
    }
}

/// Compression optimisée avec buffer pooling
fn compress_data(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>> {
    #[cfg(feature = "compression")]
    use flate2::write::{GzEncoder, ZlibEncoder};
    #[cfg(feature = "compression")]
    use flate2::Compression;
    #[cfg(feature = "compression")]
    use std::io::Write;
    
    match format {
        CompressionFormat::None => Ok(data.to_vec()),
        #[cfg(feature = "compression")]
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        },
        #[cfg(feature = "compression")]
        CompressionFormat::Zlib => {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data)?;
            Ok(encoder.finish()?)
        }
        #[cfg(not(feature = "compression"))]
        CompressionFormat::Gzip | CompressionFormat::Zlib => {
            Err(CompressionError::InvalidFormat)
        }
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