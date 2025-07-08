use thiserror::Error;

/// NBT parsing and manipulation errors
#[derive(Debug, Error)]
pub enum NbtError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Invalid NBT tag type: {0}")]
    InvalidTagType(u8),
    
    #[error("Invalid UTF-8 string")]
    InvalidUtf8,
    
    #[error("Unexpected end of data")]
    UnexpectedEof,
    
    #[error("List type mismatch: expected {expected}, found {found}")]
    ListTypeMismatch { expected: u8, found: u8 },
}

pub type Result<T> = std::result::Result<T, NbtError>; 