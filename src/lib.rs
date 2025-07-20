#[cfg(all(not(feature = "async"), not(feature = "sync")))]
compile_error!("You must enable either the 'async' or 'sync' feature (or both)!");

pub const SPUD_VERSION: &str = "SPUD-0.8.0";

pub mod types;

mod functions;
mod spud_builder;
mod spud_decoder;
mod spud_error;
mod spud_types;

pub use spud_builder::*;
pub use spud_decoder::SpudDecoder;
pub use spud_error::SpudError;
