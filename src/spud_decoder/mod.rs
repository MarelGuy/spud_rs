mod decode_object;
mod decoder_functions;

pub(crate) use decode_object::DecoderObject;

#[cfg(feature = "async")]
mod async_decoder;
#[cfg(feature = "async")]
pub use async_decoder::SpudDecoderAsync;

#[cfg(feature = "sync")]
mod sync_decoder;
#[cfg(feature = "sync")]
pub use sync_decoder::SpudDecoderSync;
