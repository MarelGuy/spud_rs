#[cfg(all(not(feature = "async"), not(feature = "sync")))]
compile_error!("You must enable either the 'async' or 'sync' feature (or both)!");

pub const SPUD_VERSION: &str = "SPUD-0.8.0";

pub mod types;

mod functions;
mod spud_builder;
mod spud_decoder;
mod spud_error;
mod spud_types;

#[cfg(any(feature = "sync", feature = "async"))]
pub use spud_builder::*;

#[cfg(any(feature = "sync", feature = "async"))]
pub use spud_decoder::SpudDecoder;

#[cfg(any(feature = "sync", feature = "async"))]
pub use spud_error::SpudError;
