pub const SPUD_VERSION: &str = "SPUD-0.3.0";

pub mod types;

mod functions;
mod spud_builder;
mod spud_decoder;
mod spud_types;

pub use spud_builder::{SpudBuilder, SpudObject};
pub use spud_decoder::SpudDecoder;
