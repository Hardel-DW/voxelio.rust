use crate::{Result, RegionError};
use nbt_compression::{NbtFile, CompressionFormat};
use nbt_core::{NbtTag, Endian};

/// A Minecraft chunk within a region file
#[derive(Debug)]
pub struct Chunk {
    /// X coordinate (0-31 within region)
    pub x: i32,
    /// Z coordinate (0-31 within region)  
    pub z: i32,
    /// Compression format (1=gzip, 2=zlib, 3=none)
    pub compression: u8,
    /// Unix timestamp
    pub timestamp: u32,
    /// Raw compressed chunk data
    raw_data: Vec<u8>,
    /// Cached parsed NBT (lazy loaded)
    cached_nbt: Option<NbtFile>,
}

impl Chunk {
    /// Create a new chunk
    pub fn new(x: i32, z: i32, compression: u8, timestamp: u32, raw_data: Vec<u8>) -> Result<Self> {
        if !Self::valid_coordinates(x, z) {
            return Err(RegionError::InvalidCoordinates { x, z });
        }
        
        Ok(Self {
            x,
            z,
            compression,
            timestamp,
            raw_data,
            cached_nbt: None,
        })
    }

    /// Create chunk from NBT data
    pub fn from_nbt(x: i32, z: i32, nbt_file: NbtFile, timestamp: u32) -> Result<Self> {
        if !Self::valid_coordinates(x, z) {
            return Err(RegionError::InvalidCoordinates { x, z });
        }

        let compression = Self::compression_to_id(nbt_file.compression);
        let raw_data = nbt_file.write()?;

        Ok(Self {
            x,
            z,
            compression,
            timestamp,
            raw_data,
            cached_nbt: Some(nbt_file),
        })
    }

    /// Get the compression format
    pub fn get_compression(&self) -> CompressionFormat {
        match self.compression {
            1 => CompressionFormat::Gzip,
            2 => CompressionFormat::Zlib,
            3 => CompressionFormat::None,
            _ => CompressionFormat::Zlib, // Default fallback
        }
    }

    /// Get raw compressed data
    pub fn get_raw_data(&self) -> &[u8] {
        &self.raw_data
    }

    /// Get raw data size
    pub fn size(&self) -> usize {
        self.raw_data.len()
    }

    /// Parse NBT data (cached after first call)
    pub fn get_nbt(&mut self) -> Result<&NbtFile> {
        if self.cached_nbt.is_none() {
            let nbt_file = NbtFile::read_with_format(
                &self.raw_data,
                self.get_compression(),
                Endian::Big,
            )?;
            self.cached_nbt = Some(nbt_file);
        }
        Ok(self.cached_nbt.as_ref().unwrap())
    }

    /// Get the root NBT tag
    pub fn get_root(&mut self) -> Result<&NbtTag> {
        let nbt = self.get_nbt()?;
        Ok(&nbt.root)
    }

    /// Update the chunk's NBT data
    pub fn set_nbt(&mut self, nbt_file: NbtFile) -> Result<()> {
        self.compression = Self::compression_to_id(nbt_file.compression);
        self.raw_data = nbt_file.write()?;
        self.cached_nbt = Some(nbt_file);
        Ok(())
    }

    /// Check if coordinates are valid for region (0-31)
    pub fn valid_coordinates(x: i32, z: i32) -> bool {
        (0..32).contains(&x) && (0..32).contains(&z)
    }

    /// Convert coordinate to array index  
    pub fn coords_to_index(x: i32, z: i32) -> Option<usize> {
        if Self::valid_coordinates(x, z) {
            Some((x + z * 32) as usize)
        } else {
            None
        }
    }

    /// Convert compression format to numeric ID
    pub fn compression_to_id(format: CompressionFormat) -> u8 {
        match format {
            CompressionFormat::Gzip => 1,
            CompressionFormat::Zlib => 2,
            CompressionFormat::None => 3,
        }
    }
}

impl Clone for Chunk {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            z: self.z,
            compression: self.compression,
            timestamp: self.timestamp,
            raw_data: self.raw_data.clone(),
            cached_nbt: None, // Don't clone the cache, let it be lazily reloaded
        }
    }
}

impl PartialEq for Chunk {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.z == other.z
            && self.compression == other.compression
            && self.timestamp == other.timestamp
            && self.raw_data == other.raw_data
    }
} 