use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnbtError {
    #[error("Parse error: {message} at position {position}")]
    ParseError { message: String, position: usize },
    
    #[error("Invalid number format: {0}")]
    InvalidNumber(String),
    
    #[error("Invalid string format: {0}")]
    InvalidString(String),
    
    #[error("Unexpected end of input")]
    UnexpectedEof,
    
    #[error("Type mismatch in list: expected {expected}, found {found}")]
    ListTypeMismatch { expected: String, found: String },
}

pub type Result<T> = std::result::Result<T, SnbtError>;

impl SnbtError {
    pub fn parse_error(message: impl Into<String>, position: usize) -> Self {
        Self::ParseError {
            message: message.into(),
            position,
        }
    }
} 