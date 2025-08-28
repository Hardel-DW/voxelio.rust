#[cfg(feature = "region")]
use crate::{CompressionFormat, Endian, NbtError, NbtFile, NbtTag, Result};

#[cfg(feature = "region")]
const REGION_SIZE: i32 = 32;
#[cfg(feature = "region")]
const CHUNK_COUNT: usize = (REGION_SIZE * REGION_SIZE) as usize;
#[cfg(feature = "region")]
const SECTOR_SIZE: usize = 4096;
#[cfg(feature = "region")]
const HEADER_SIZE: usize = 8192; // 2 sectors

#[cfg(feature = "region")]
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

#[cfg(feature = "region")]
impl Chunk {
    /// Create a new chunk
    pub fn new(x: i32, z: i32, compression: u8, timestamp: u32, raw_data: Vec<u8>) -> Result<Self> {
        if !Self::valid_coordinates(x, z) {
            return Err(NbtError::InvalidCoordinates { x, z });
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
            return Err(NbtError::InvalidCoordinates { x, z });
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
            _ => CompressionFormat::Zlib,
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
            let nbt_file =
                NbtFile::read_with_format(&self.raw_data, self.get_compression(), Endian::Big, None)?;
            self.cached_nbt = Some(nbt_file);
        }
        Ok(self.cached_nbt.as_ref().unwrap())
    }

    /// Get the root NBT tag
    pub fn get_root(&mut self) -> Result<&NbtTag> {
        let nbt = self.get_nbt()?;
        Ok(&nbt.root)
    }

    /// Get the root NBT tag (immutable version - parses without caching)
    pub fn get_root_immutable(&self) -> Result<NbtTag> {
        let nbt_file =
            NbtFile::read_with_format(&self.raw_data, self.get_compression(), Endian::Big, None)?;
        Ok(nbt_file.root)
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

#[cfg(feature = "region")]
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

#[cfg(feature = "region")]
impl PartialEq for Chunk {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.z == other.z
            && self.compression == other.compression
            && self.timestamp == other.timestamp
            && self.raw_data == other.raw_data
    }
}

#[cfg(feature = "region")]
#[derive(Debug)]
pub struct Region {
    chunks: [Option<Chunk>; CHUNK_COUNT],
}

#[cfg(feature = "region")]
impl Region {
    /// Create empty region
    pub fn new() -> Self {
        Self {
            chunks: std::array::from_fn(|_| None),
        }
    }

    /// Create region from chunks
    pub fn from_chunks(chunks: Vec<Chunk>) -> Result<Self> {
        let mut region = Self::new();
        for chunk in chunks {
            region.set_chunk(chunk)?;
        }
        Ok(region)
    }

    /// Read region from bytes (classic .mca format)
    pub fn read(data: &[u8]) -> Result<Self> {
        if data.len() < HEADER_SIZE {
            return Err(NbtError::region_error("File too small for region header"));
        }

        let mut region = Self::new();

        // Parse each potential chunk location
        for x in 0..REGION_SIZE {
            for z in 0..REGION_SIZE {
                let index = (x + z * REGION_SIZE) as usize;
                let header_offset = index * 4;

                // Read location header (4 bytes: 3 bytes offset + 1 byte sector count)
                let location_data = &data[header_offset..header_offset + 4];
                let offset = ((location_data[0] as u32) << 16)
                    | ((location_data[1] as u32) << 8)
                    | (location_data[2] as u32);
                let sectors = location_data[3];

                if sectors == 0 {
                    continue; // Empty chunk slot
                }

                // Read timestamp header (4 bytes at offset + SECTOR_SIZE)
                let timestamp_offset = header_offset + SECTOR_SIZE;
                if timestamp_offset + 4 > data.len() {
                    continue;
                }
                let timestamp_data = &data[timestamp_offset..timestamp_offset + 4];
                let timestamp = u32::from_be_bytes([
                    timestamp_data[0],
                    timestamp_data[1],
                    timestamp_data[2],
                    timestamp_data[3],
                ]);

                // Read chunk data
                let chunk_offset = (offset as usize) * SECTOR_SIZE;
                if chunk_offset + 5 > data.len() {
                    continue;
                }

                // Read chunk length and compression
                let length_data = &data[chunk_offset..chunk_offset + 4];
                let length = u32::from_be_bytes([
                    length_data[0],
                    length_data[1],
                    length_data[2],
                    length_data[3],
                ]) as usize;

                if length == 0 || chunk_offset + 4 + length > data.len() {
                    continue;
                }

                let compression = data[chunk_offset + 4];
                let chunk_data = data[chunk_offset + 5..chunk_offset + 4 + length].to_vec();

                let chunk = Chunk::new(x, z, compression, timestamp, chunk_data)?;
                region.chunks[index] = Some(chunk);
            }
        }

        Ok(region)
    }

    /// Write region to bytes
    pub fn write(&self) -> Result<Vec<u8>> {
        // Calculate total sectors needed
        let mut total_sectors = 2; // Header always takes 2 sectors
        for chunk in self.chunks.iter().flatten() {
            let chunk_size = chunk.size() + 5; // +5 for length(4) + compression(1)
            let sectors_needed = chunk_size.div_ceil(SECTOR_SIZE);
            total_sectors += sectors_needed;
        }

        let mut data = vec![0u8; total_sectors * SECTOR_SIZE];
        let mut current_offset = 2; // Start after header

        // Write chunks and build headers
        for (index, chunk) in self
            .chunks
            .iter()
            .enumerate()
            .filter_map(|(i, c)| c.as_ref().map(|chunk| (i, chunk)))
        {
            let chunk_data = chunk.get_raw_data();
            let chunk_size = chunk_data.len() + 5;
            let sectors_needed = chunk_size.div_ceil(SECTOR_SIZE);

            // Write location header (offset in sectors + sector count)
            let header_pos = index * 4;
            data[header_pos] = (current_offset >> 16) as u8;
            data[header_pos + 1] = (current_offset >> 8) as u8;
            data[header_pos + 2] = current_offset as u8;
            data[header_pos + 3] = sectors_needed as u8;

            // Write timestamp header
            let timestamp_pos = header_pos + SECTOR_SIZE;
            let timestamp_bytes = chunk.timestamp.to_be_bytes();
            data[timestamp_pos..timestamp_pos + 4].copy_from_slice(&timestamp_bytes);

            // Write chunk data
            let data_pos = current_offset * SECTOR_SIZE;
            let length_bytes = (chunk_data.len() + 1) as u32; // +1 for compression byte
            data[data_pos..data_pos + 4].copy_from_slice(&length_bytes.to_be_bytes());
            data[data_pos + 4] = chunk.compression;
            data[data_pos + 5..data_pos + 5 + chunk_data.len()].copy_from_slice(chunk_data);

            current_offset += sectors_needed;
        }

        Ok(data)
    }

    /// Get chunk at coordinates
    pub fn get_chunk(&self, x: i32, z: i32) -> Result<Option<&Chunk>> {
        let index = Chunk::coords_to_index(x, z).ok_or(NbtError::InvalidCoordinates { x, z })?;
        Ok(self.chunks[index].as_ref())
    }

    /// Get mutable chunk at coordinates  
    pub fn get_chunk_mut(&mut self, x: i32, z: i32) -> Result<Option<&mut Chunk>> {
        let index = Chunk::coords_to_index(x, z).ok_or(NbtError::InvalidCoordinates { x, z })?;
        Ok(self.chunks[index].as_mut())
    }

    /// Set chunk at coordinates
    pub fn set_chunk(&mut self, chunk: Chunk) -> Result<()> {
        let index =
            Chunk::coords_to_index(chunk.x, chunk.z).ok_or(NbtError::InvalidCoordinates {
                x: chunk.x,
                z: chunk.z,
            })?;
        self.chunks[index] = Some(chunk);
        Ok(())
    }

    /// Remove chunk at coordinates
    pub fn remove_chunk(&mut self, x: i32, z: i32) -> Result<Option<Chunk>> {
        let index = Chunk::coords_to_index(x, z).ok_or(NbtError::InvalidCoordinates { x, z })?;
        Ok(self.chunks[index].take())
    }

    /// Get all chunk positions that exist
    pub fn get_chunk_positions(&self) -> Vec<(i32, i32)> {
        self.chunks
            .iter()
            .filter_map(|chunk| chunk.as_ref().map(|c| (c.x, c.z)))
            .collect()
    }

    /// Get all chunks as iterator
    pub fn chunks(&self) -> impl Iterator<Item = &Chunk> {
        self.chunks.iter().filter_map(|chunk| chunk.as_ref())
    }

    /// Get all chunks as mutable iterator
    pub fn chunks_mut(&mut self) -> impl Iterator<Item = &mut Chunk> {
        self.chunks.iter_mut().filter_map(|chunk| chunk.as_mut())
    }

    /// Get chunk count
    pub fn chunk_count(&self) -> usize {
        self.chunks.iter().filter(|chunk| chunk.is_some()).count()
    }

    /// Check if region is empty
    pub fn is_empty(&self) -> bool {
        self.chunk_count() == 0
    }
}

#[cfg(feature = "region")]
impl Default for Region {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "region")]
impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.chunks == other.chunks
    }
}

/// Read region file from bytes
#[cfg(feature = "region")]
pub fn read_region(data: &[u8]) -> Result<Region> {
    Region::read(data)
}

/// Write region to bytes
#[cfg(feature = "region")]
pub fn write_region(region: &Region) -> Result<Vec<u8>> {
    region.write()
}

#[cfg(not(feature = "region"))]
pub struct Chunk;

#[cfg(not(feature = "region"))]
pub struct Region;

#[cfg(not(feature = "region"))]
pub fn read_region(_data: &[u8]) -> Result<Region> {
    Err(NbtError::Parse("Region feature not enabled".to_string()))
}

#[cfg(not(feature = "region"))]
pub fn write_region(_region: &Region) -> Result<Vec<u8>> {
    Err(NbtError::Parse("Region feature not enabled".to_string()))
}
