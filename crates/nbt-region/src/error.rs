use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegionError {
    #[error("NBT error: {0}")]
    Nbt(#[from] nbt_core::NbtError),
    
    #[error("Compression error: {0}")]
    Compression(#[from] nbt_compression::CompressionError),
    
    #[error("Invalid region data: {0}")]
    InvalidData(String),
    
    #[error("Chunk not found at ({x}, {z})")]
    ChunkNotFound { x: i32, z: i32 },
    
    #[error("Invalid chunk coordinates: ({x}, {z}) - must be 0-31")]
    InvalidCoordinates { x: i32, z: i32 },
    
    #[error("Unexpected end of file")]
    UnexpectedEof,
}

pub type Result<T> = std::result::Result<T, RegionError>; 