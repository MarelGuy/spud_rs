#![cfg(feature = "serde")]

mod spud_ser_errors;
mod spud_serializer;

pub(super) use spud_ser_errors::SpudSerializationError;
pub(crate) use spud_serializer::SpudSerializer;
