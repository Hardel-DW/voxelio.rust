//! NBT Core - Fast zero-copy NBT parsing for Minecraft
//! 
//! Simple, efficient replacement for the TypeScript NBT implementation.

mod error;
mod reader;
mod tag;

pub use error::*;
pub use reader::*;
pub use tag::*;

// Re-export commonly used types
pub use std::collections::HashMap;

#[cfg(test)]
mod tests; 