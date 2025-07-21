use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject, types::Time};

pub(crate) fn time(decoder: &mut DecoderObject) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let read_bytes: &[u8] = decoder.read_bytes(7)?;

    let time: Time = DecoderObject::read_time(read_bytes)?;

    Ok(Value::String(time.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[cfg(feature = "sync")]
    #[test]
    fn test_time() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj| {
                obj.add_value("time", Time::new(12, 30, 45, 0).unwrap())?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoderSync = SpudDecoderSync::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_time_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("time", Time::new(12, 30, 45, 0).unwrap())
                    .await?;
                Ok(())
            })
            .await
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().await.unwrap();

        let mut decoder: SpudDecoderSync = SpudDecoderSync::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }
}
