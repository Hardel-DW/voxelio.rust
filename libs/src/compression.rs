use crate::{Endian, NbtError, NbtReader, NbtTag, NbtWriter, Result};

#[cfg(feature = "compression")]
use flate2::read::{GzDecoder, ZlibDecoder};
#[cfg(feature = "compression")]
use flate2::write::{GzEncoder, ZlibEncoder};
#[cfg(feature = "compression")]
use flate2::Compression;
#[cfg(feature = "compression")]
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionFormat {
    None,
    Gzip,
    Zlib,
}

impl CompressionFormat {
    pub fn as_u8(self) -> u8 {
        match self {
            CompressionFormat::None => 0,
            CompressionFormat::Gzip => 1,
            CompressionFormat::Zlib => 2,
        }
    }
}

pub fn detect_compression(data: &[u8]) -> CompressionFormat {
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

pub fn decompress_optimized(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>> {
    match format {
        CompressionFormat::None => Ok(data.to_vec()),
        #[cfg(feature = "compression")]
        CompressionFormat::Gzip => {
            let mut decoder = GzDecoder::new(data);
            let mut result = Vec::with_capacity(data.len() * 3); // Conservative estimation
            decoder.read_to_end(&mut result).map_err(|e| {
                NbtError::compression_error(format!("Gzip decompression failed: {e}"))
            })?;
            Ok(result)
        }
        #[cfg(feature = "compression")]
        CompressionFormat::Zlib => {
            let mut decoder = ZlibDecoder::new(data);
            let mut result = Vec::with_capacity(data.len() * 3);
            decoder.read_to_end(&mut result).map_err(|e| {
                NbtError::compression_error(format!("Zlib decompression failed: {e}"))
            })?;
            Ok(result)
        }
        #[cfg(not(feature = "compression"))]
        CompressionFormat::Gzip | CompressionFormat::Zlib => Err(NbtError::InvalidFormat),
    }
}

pub fn compress_data(data: &[u8], format: CompressionFormat) -> Result<Vec<u8>> {
    match format {
        CompressionFormat::None => Ok(data.to_vec()),
        #[cfg(feature = "compression")]
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data).map_err(|e| {
                NbtError::compression_error(format!("Gzip compression failed: {e}"))
            })?;
            encoder
                .finish()
                .map_err(|e| NbtError::compression_error(format!("Gzip finish failed: {e}")))
        }
        #[cfg(feature = "compression")]
        CompressionFormat::Zlib => {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data).map_err(|e| {
                NbtError::compression_error(format!("Zlib compression failed: {e}"))
            })?;
            encoder
                .finish()
                .map_err(|e| NbtError::compression_error(format!("Zlib finish failed: {e}")))
        }
        #[cfg(not(feature = "compression"))]
        CompressionFormat::Gzip | CompressionFormat::Zlib => Err(NbtError::InvalidFormat),
    }
}

#[derive(Debug)]
pub struct NbtFile {
    pub root: NbtTag,
    pub root_name: String,
    pub compression: CompressionFormat,
}

impl NbtFile {
    pub fn read(data: &[u8], fields: Option<&[&str]>) -> Result<Self> {
        let fields = fields.unwrap_or(&[]);
        let format = detect_compression(data);

        let decompressed = decompress_optimized(data, format)?;

        let mut reader = NbtReader::new(&decompressed, Endian::Big);
        let tag_type = reader.read_u8()?;

        if tag_type != 10 {
            return Err(NbtError::Parse("Expected compound tag at root".to_string()));
        }

        let root_name = reader.read_string()?;

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

    pub fn new_with_settings(
        root: NbtTag,
        root_name: String,
        compression: CompressionFormat,
        _endian: Endian,
    ) -> Self {
        Self {
            root,
            root_name,
            compression,
        }
    }

    pub fn new(root: NbtTag, root_name: String, compression: CompressionFormat) -> Self {
        Self::new_with_settings(root, root_name, compression, Endian::Big)
    }

    pub fn read_with_format(
        data: &[u8],
        format: CompressionFormat,
        endian: Endian,
        fields: Option<&[&str]>,
    ) -> Result<Self> {
        let fields = fields.unwrap_or(&[]);
        let decompressed = decompress_optimized(data, format)?;

        let mut reader = NbtReader::new(&decompressed, endian);
        let tag_type = reader.read_u8()?;

        if tag_type != 10 {
            return Err(NbtError::Parse("Expected compound tag at root".to_string()));
        }

        let root_name = reader.read_string()?;
        
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

    pub fn write(&self) -> Result<Vec<u8>> {
        let mut writer = NbtWriter::new(Endian::Big);
        writer.write_u8(10); // Compound tag
        writer.write_string(&self.root_name);
        writer.write_tag(&self.root)?;

        let uncompressed = writer.into_bytes();
        compress_data(&uncompressed, self.compression)
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
}

#[cfg(feature = "compression")]
impl NbtFile {
    pub fn compress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        compress_data(data, CompressionFormat::Gzip)
    }

    pub fn compress_zlib(data: &[u8]) -> Result<Vec<u8>> {
        compress_data(data, CompressionFormat::Zlib)
    }

    pub fn decompress_gzip(data: &[u8]) -> Result<Vec<u8>> {
        decompress_optimized(data, CompressionFormat::Gzip)
    }

    pub fn decompress_zlib(data: &[u8]) -> Result<Vec<u8>> {
        decompress_optimized(data, CompressionFormat::Zlib)
    }
}
