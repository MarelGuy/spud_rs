use rust_decimal::Decimal;
use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject};

pub(crate) fn decimal(decoder: &mut DecoderObject) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let read_bytes: &[u8] = decoder.read_bytes(16)?;

    let decimal_value: Decimal = Decimal::deserialize(
        read_bytes
            .try_into()
            .map_err(|_| SpudError::DecodingError("Invalid Decimal bytes".to_owned()))?,
    );

    Ok(Value::String(decimal_value.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::{types::Decimal, *};

    #[cfg(feature = "sync")]
    #[test]
    fn test_decimal() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj| {
                obj.add_value("decimal", Decimal::from_f32_retain(0.1).unwrap())?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_decimal_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("decimal", Decimal::from_f32_retain(0.1).unwrap())
                    .await?;
                Ok(())
            })
            .await
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().await.unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }
}
