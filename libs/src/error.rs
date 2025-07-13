use thiserror::Error;

#[derive(Error, Debug)]
pub enum NbtError {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid tag type: {0}")]
    InvalidTagType(u8),

    #[error("Unexpected end of file")]
    UnexpectedEof,

    #[error("Invalid string length: {0}")]
    InvalidStringLength(usize),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // Compression errors (feature = "compression")
    #[cfg(feature = "compression")]
    #[error("Compression error: {0}")]
    Compression(String),

    #[cfg(feature = "compression")]
    #[error("Invalid compression format")]
    InvalidFormat,

    // SNBT errors (feature = "snbt")
    #[cfg(feature = "snbt")]
    #[error("SNBT parse error: {message} at position {position}")]
    SnbtParse { message: String, position: usize },

    #[cfg(feature = "snbt")]
    #[error("Invalid number format: {0}")]
    InvalidNumber(String),

    #[cfg(feature = "snbt")]
    #[error("Invalid string format: {0}")]
    InvalidString(String),

    #[error("List type mismatch: expected {expected}, found {found}")]
    ListTypeMismatch { expected: u8, found: u8 },

    #[cfg(feature = "snbt")]
    #[error("SNBT type mismatch in list: expected {expected}, found {found}")]
    SnbtListTypeMismatch { expected: String, found: String },

    // Region errors (feature = "region")
    #[cfg(feature = "region")]
    #[error("Invalid region data: {0}")]
    InvalidRegionData(String),

    #[cfg(feature = "region")]
    #[error("Chunk not found at ({x}, {z})")]
    ChunkNotFound { x: i32, z: i32 },

    #[cfg(feature = "region")]
    #[error("Invalid chunk coordinates: ({x}, {z}) - must be 0-31")]
    InvalidCoordinates { x: i32, z: i32 },
}

pub type Result<T> = std::result::Result<T, NbtError>;

impl NbtError {
    #[cfg(feature = "snbt")]
    pub fn snbt_parse_error(message: impl Into<String>, position: usize) -> Self {
        Self::SnbtParse {
            message: message.into(),
            position,
        }
    }

    #[cfg(feature = "compression")]
    pub fn compression_error(message: impl Into<String>) -> Self {
        Self::Compression(message.into())
    }

    #[cfg(feature = "region")]
    pub fn region_error(message: impl Into<String>) -> Self {
        Self::InvalidRegionData(message.into())
    }
}
