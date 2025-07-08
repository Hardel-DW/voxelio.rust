//! NBT Region - Fast Minecraft region file (.mca) reader and writer
//! 
//! Simple, efficient handling of Minecraft region files with lazy chunk loading.

mod error;
mod chunk;
mod region;

pub use error::*;
pub use chunk::*;
pub use region::*;

// Re-export commonly used types
pub use nbt_core::{NbtTag, HashMap, Endian};
pub use nbt_compression::{NbtFile, CompressionFormat};

/// Read region file from bytes
pub fn read_region(data: &[u8]) -> Result<Region> {
    Region::read(data)
}

/// Write region to bytes
pub fn write_region(region: &Region) -> Result<Vec<u8>> {
    region.write()
}

#[cfg(test)]
mod tests {
    use super::*;
    use nbt_core::NbtTag;
    use nbt_compression::{NbtFile, CompressionFormat};

    fn create_test_nbt() -> NbtFile {
        let mut level_data = HashMap::new();
        level_data.insert("xPos".to_string(), NbtTag::Int(2));
        level_data.insert("zPos".to_string(), NbtTag::Int(5));
        level_data.insert("LastUpdate".to_string(), NbtTag::Long(12345));
        
        let root = NbtTag::Compound(level_data);
        NbtFile::new(root, "Level".to_string(), CompressionFormat::Zlib, Endian::Big)
    }

    #[test]
    fn test_chunk_creation() {
        let nbt = create_test_nbt();
        let chunk = Chunk::from_nbt(2, 5, nbt, 1000).unwrap();
        
        assert_eq!(chunk.x, 2);
        assert_eq!(chunk.z, 5);
        assert_eq!(chunk.timestamp, 1000);
        assert_eq!(chunk.get_compression(), CompressionFormat::Zlib);
    }

    #[test]
    fn test_chunk_invalid_coordinates() {
        let nbt = create_test_nbt();
        let result = Chunk::from_nbt(32, 5, nbt, 1000);
        assert!(result.is_err());
    }

    #[test]
    fn test_chunk_nbt_access() {
        let nbt = create_test_nbt();
        let mut chunk = Chunk::from_nbt(2, 5, nbt, 1000).unwrap();
        
        let root = chunk.get_root().unwrap();
        if let NbtTag::Compound(map) = root {
            assert_eq!(map.get("xPos").unwrap().as_number() as i32, 2);
            assert_eq!(map.get("zPos").unwrap().as_number() as i32, 5);
        } else {
            panic!("Expected compound root");
        }
    }

    #[test]
    fn test_empty_region() {
        let region = Region::new();
        assert!(region.is_empty());
        assert_eq!(region.chunk_count(), 0);
        assert_eq!(region.get_chunk_positions().len(), 0);
    }

    #[test]
    fn test_region_chunk_management() {
        let mut region = Region::new();
        let nbt = create_test_nbt();
        let chunk = Chunk::from_nbt(2, 5, nbt, 1000).unwrap();
        
        // Add chunk
        region.set_chunk(chunk).unwrap();
        assert_eq!(region.chunk_count(), 1);
        assert_eq!(region.get_chunk_positions(), vec![(2, 5)]);
        
        // Get chunk
        let retrieved = region.get_chunk(2, 5).unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().x, 2);
        assert_eq!(retrieved.unwrap().z, 5);
        
        // Remove chunk
        let removed = region.remove_chunk(2, 5).unwrap();
        assert!(removed.is_some());
        assert!(region.is_empty());
    }

    #[test]
    fn test_region_write_read_roundtrip() {
        // Create region with chunks
        let mut region = Region::new();
        let nbt1 = create_test_nbt();
        let nbt2 = create_test_nbt();
        
        let chunk1 = Chunk::from_nbt(0, 0, nbt1, 1000).unwrap();
        let chunk2 = Chunk::from_nbt(1, 0, nbt2, 2000).unwrap();
        
        region.set_chunk(chunk1).unwrap();
        region.set_chunk(chunk2).unwrap();
        
        // Write to bytes
        let data = region.write().unwrap();
        assert!(!data.is_empty());
        
        // Read back
        let region2 = Region::read(&data).unwrap();
        assert_eq!(region2.chunk_count(), 2);
        assert_eq!(region2.get_chunk_positions().len(), 2);
        
        // Verify chunks
        let chunk1_back = region2.get_chunk(0, 0).unwrap().unwrap();
        let chunk2_back = region2.get_chunk(1, 0).unwrap().unwrap();
        
        assert_eq!(chunk1_back.x, 0);
        assert_eq!(chunk1_back.z, 0);
        assert_eq!(chunk1_back.timestamp, 1000);
        
        assert_eq!(chunk2_back.x, 1);
        assert_eq!(chunk2_back.z, 0);
        assert_eq!(chunk2_back.timestamp, 2000);
    }

    #[test]
    fn test_convenience_functions() {
        let mut region = Region::new();
        let nbt = create_test_nbt();
        let chunk = Chunk::from_nbt(5, 10, nbt, 3000).unwrap();
        region.set_chunk(chunk).unwrap();
        
        // Test write_region function
        let data = write_region(&region).unwrap();
        
        // Test read_region function
        let region2 = read_region(&data).unwrap();
        assert_eq!(region2.chunk_count(), 1);
        
        let chunk_back = region2.get_chunk(5, 10).unwrap().unwrap();
        assert_eq!(chunk_back.x, 5);
        assert_eq!(chunk_back.z, 10);
        assert_eq!(chunk_back.timestamp, 3000);
    }
} 