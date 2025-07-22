use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject};

pub(crate) fn bool(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let value: Value = match decoder.contents.get(decoder.index) {
        Some(0) => Value::Bool(false),
        Some(1) => Value::Bool(true),
        _ => Err(SpudError::DecodingError(format!(
            "Unknown bool value: {}",
            decoder.contents[decoder.index]
        )))?,
    };

    *next_steps = 1;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[cfg(feature = "sync")]
    #[test]
    fn test_bool() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj| {
                obj.add_value("bool", false)?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_bool_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("bool", true).await?;
                Ok(())
            })
            .await
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().await.unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }
}
