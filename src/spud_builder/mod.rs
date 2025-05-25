mod builder;
mod object;
mod spud_type_ext;

pub(crate) use spud_type_ext::SpudTypesExt;

pub use builder::SpudBuilder;
pub use object::SpudObject;

mod serde;

#[cfg(feature = "serde")]
pub(crate) use serde::SpudSerializer;
