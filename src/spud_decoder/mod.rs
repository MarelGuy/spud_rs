mod decode_object;
mod decoder_functions;

pub(crate) use decode_object::DecoderObject;

mod decoder;

pub use decoder::SpudDecoder;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "sync")]
    #[test]
    fn test_sync_encoder_to_sync_decoder() {
        let mut decoder: SpudDecoder =
            SpudDecoder::new_from_path("./.tmp/spud/sync_test.spud").unwrap();
        decoder.decode(true, false).unwrap();

        decoder
            .build_file("./.tmp/json/sync_to_sync_test_output.json")
            .unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_async_encoder_to_async_decoder() {
        let mut decoder: SpudDecoder =
            SpudDecoder::new_from_path_async("./.tmp/spud/async_test.spud")
                .await
                .unwrap();
        decoder.decode(true, false).unwrap();

        decoder
            .build_file_async("./.tmp/json/async_to_async_test_output.json")
            .await
            .unwrap();
    }

    #[cfg(all(feature = "sync", feature = "async"))]
    #[test]
    fn test_async_encoder_to_sync_decoder() {
        let mut decoder: SpudDecoder =
            SpudDecoder::new_from_path("./.tmp/spud/async_test.spud").unwrap();
        decoder.decode(true, false).unwrap();

        decoder
            .build_file("./.tmp/json/async_to_sync_test_output.json")
            .unwrap();
    }

    #[cfg(all(feature = "sync", feature = "async"))]
    #[tokio::test]
    async fn test_sync_encoder_to_async_decoder() {
        let mut decoder: SpudDecoder =
            SpudDecoder::new_from_path_async("./.tmp/spud/sync_test.spud")
                .await
                .unwrap();
        decoder.decode(true, false).unwrap();

        decoder
            .build_file_async("./.tmp/json/sync_to_async_test_output.json")
            .await
            .unwrap();
    }
}
