mod decode_object;
mod decoder_functions;

pub(crate) use decode_object::DecoderObject;

#[cfg(feature = "async")]
mod async_decoder;
#[cfg(feature = "async")]
pub use async_decoder::SpudDecoder;

#[cfg(not(feature = "async"))]
mod sync_decoder;
#[cfg(not(feature = "async"))]
pub use sync_decoder::SpudDecoder;
