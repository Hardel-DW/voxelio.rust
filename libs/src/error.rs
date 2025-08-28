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

    #[error("Compression error: {0}")]
    Compression(String),

    #[error("Invalid compression format")]
    InvalidFormat,

    #[error("SNBT parse error: {message} at position {position}")]
    SnbtParse { message: String, position: usize },

    #[error("Invalid number format: {0}")]
    InvalidNumber(String),

    #[error("Invalid string format: {0}")]
    InvalidString(String),

    #[error("List type mismatch: expected {expected}, found {found}")]
    ListTypeMismatch { expected: u8, found: u8 },

    #[error("SNBT type mismatch in list: expected {expected}, found {found}")]
    SnbtListTypeMismatch { expected: String, found: String },

    #[error("Invalid region data: {0}")]
    InvalidRegionData(String),

    #[error("Chunk not found at ({x}, {z})")]
    ChunkNotFound { x: i32, z: i32 },

    #[error("Invalid chunk coordinates: ({x}, {z}) - must be 0-31")]
    InvalidCoordinates { x: i32, z: i32 },
}

pub type Result<T> = std::result::Result<T, NbtError>;

impl NbtError {
    pub fn snbt_parse_error(message: impl Into<String>, position: usize) -> Self {
        Self::SnbtParse {
            message: message.into(),
            position,
        }
    }

    pub fn compression_error(message: impl Into<String>) -> Self {
        Self::Compression(message.into())
    }

    pub fn region_error(message: impl Into<String>) -> Self {
        Self::InvalidRegionData(message.into())
    }
}
