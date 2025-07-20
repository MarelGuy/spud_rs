#[cfg(all(not(feature = "async"), not(feature = "sync")))]
compile_error!("You must enable either the 'async' or 'sync' feature (or both)!");

#[cfg(any(feature = "sync", feature = "async"))]
pub const SPUD_VERSION: &str = "SPUD-0.8.1";

#[cfg(any(feature = "sync", feature = "async"))]
pub mod types;

#[cfg(any(feature = "sync", feature = "async"))]
mod functions;

#[cfg(any(feature = "sync", feature = "async"))]
mod spud_builder;
#[cfg(any(feature = "sync", feature = "async"))]
mod spud_decoder;
#[cfg(any(feature = "sync", feature = "async"))]
mod spud_error;
#[cfg(any(feature = "sync", feature = "async"))]
mod spud_types;

#[cfg(any(feature = "sync", feature = "async"))]
pub use spud_builder::*;

#[cfg(any(feature = "sync", feature = "async"))]
pub use spud_decoder::*;

#[cfg(any(feature = "sync", feature = "async"))]
pub use spud_error::SpudError;
