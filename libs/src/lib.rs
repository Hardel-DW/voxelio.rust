mod error;
mod reader;
mod tag;

#[cfg(feature = "compression")]
mod compression;

#[cfg(feature = "snbt")]
mod snbt;

#[cfg(feature = "region")]
mod region;

pub use error::*;
pub use reader::*;
pub use tag::*;
pub mod wasm;

pub use std::collections::HashMap;

#[cfg(feature = "compression")]
pub use compression::*;

#[cfg(feature = "snbt")]
pub use snbt::*;

#[cfg(feature = "region")]
pub use region::*;
