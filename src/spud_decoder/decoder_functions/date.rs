use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject, types::Date};

pub(crate) fn date(decoder: &mut DecoderObject) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let read_bytes: &[u8] = decoder.read_bytes(4)?;

    let date: Date = DecoderObject::read_date(read_bytes)?;

    Ok(Value::String(date.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::{types::Date, *};

    #[cfg(feature = "sync")]
    #[test]
    fn test_date() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj| {
                obj.add_value("date", Date::new(2023, 3, 14).unwrap())?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_date_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("date", Date::new(2023, 3, 14).unwrap())
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
