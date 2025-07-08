use crate::{Chunk, Result, RegionError};

/// Constants from Minecraft region file format
const REGION_SIZE: i32 = 32;
const CHUNK_COUNT: usize = (REGION_SIZE * REGION_SIZE) as usize;
const SECTOR_SIZE: usize = 4096;
const HEADER_SIZE: usize = 8192; // 2 sectors

/// A Minecraft region file containing up to 1024 chunks (32x32)
#[derive(Debug)]
pub struct Region {
    chunks: [Option<Chunk>; CHUNK_COUNT],
}

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
            return Err(RegionError::InvalidData("File too small for region header".to_string()));
        }

        let mut region = Self::new();
        
        // Parse each potential chunk location
        for x in 0..REGION_SIZE {
            for z in 0..REGION_SIZE {
                let index = (x + z * REGION_SIZE) as usize;
                let header_offset = index * 4;
                
                // Read location header (4 bytes: 3 bytes offset + 1 byte sector count)
                let location_data = &data[header_offset..header_offset + 4];
                let offset = ((location_data[0] as u32) << 16) | 
                            ((location_data[1] as u32) << 8) | 
                            (location_data[2] as u32);
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
                    timestamp_data[0], timestamp_data[1], 
                    timestamp_data[2], timestamp_data[3]
                ]);
                
                // Read chunk data
                let chunk_offset = (offset as usize) * SECTOR_SIZE;
                if chunk_offset + 5 > data.len() {
                    continue;
                }
                
                // Read chunk length and compression
                let length_data = &data[chunk_offset..chunk_offset + 4];
                let length = u32::from_be_bytes([
                    length_data[0], length_data[1], 
                    length_data[2], length_data[3]
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
        for (index, chunk) in self.chunks.iter().enumerate().filter_map(|(i, c)| c.as_ref().map(|chunk| (i, chunk))) {
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
        let index = Chunk::coords_to_index(x, z)
            .ok_or(RegionError::InvalidCoordinates { x, z })?;
        Ok(self.chunks[index].as_ref())
    }

    /// Get mutable chunk at coordinates  
    pub fn get_chunk_mut(&mut self, x: i32, z: i32) -> Result<Option<&mut Chunk>> {
        let index = Chunk::coords_to_index(x, z)
            .ok_or(RegionError::InvalidCoordinates { x, z })?;
        Ok(self.chunks[index].as_mut())
    }

    /// Set chunk at coordinates
    pub fn set_chunk(&mut self, chunk: Chunk) -> Result<()> {
        let index = Chunk::coords_to_index(chunk.x, chunk.z)
            .ok_or(RegionError::InvalidCoordinates { x: chunk.x, z: chunk.z })?;
        self.chunks[index] = Some(chunk);
        Ok(())
    }

    /// Remove chunk at coordinates
    pub fn remove_chunk(&mut self, x: i32, z: i32) -> Result<Option<Chunk>> {
        let index = Chunk::coords_to_index(x, z)
            .ok_or(RegionError::InvalidCoordinates { x, z })?;
        Ok(self.chunks[index].take())
    }

    /// Get all chunk positions that exist
    pub fn get_chunk_positions(&self) -> Vec<(i32, i32)> {
        self.chunks.iter()
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

impl Default for Region {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.chunks == other.chunks
    }
} 