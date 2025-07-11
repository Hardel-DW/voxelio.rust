//! NBT Core - Unified NBT library for Minecraft
//!
//! High-performance, zero-copy NBT parsing with optional compression, SNBT, and region support.

// Core modules (always available)
mod error;
mod reader;
mod tag;

// Optional modules (feature-gated)
#[cfg(feature = "compression")]
mod compression;

#[cfg(feature = "snbt")]
mod snbt;

#[cfg(feature = "region")]
mod region;

// Core exports (always available)
pub use error::*;
pub use reader::*;
pub use tag::*;

// Re-export commonly used types
pub use std::collections::HashMap;

// Conditional exports based on features
#[cfg(feature = "compression")]
pub use compression::*;

#[cfg(feature = "snbt")]
pub use snbt::*;

#[cfg(feature = "region")]
pub use region::*;
