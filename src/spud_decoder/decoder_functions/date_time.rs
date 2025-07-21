use serde_json::Value;

use crate::{
    SpudError,
    spud_decoder::DecoderObject,
    types::{Date, Time},
};

pub(crate) fn date_time(decoder: &mut DecoderObject) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let read_bytes: &[u8] = decoder.read_bytes(11)?;

    let date: Date = DecoderObject::read_date(&read_bytes[0..4])?;
    let time: Time = DecoderObject::read_time(&read_bytes[4..])?;

    Ok(Value::String(format!("{date} {time}")))
}

#[cfg(test)]
mod tests {
    use crate::{
        types::{Date, DateTime, Time},
        *,
    };

    #[cfg(feature = "sync")]
    #[test]
    fn test_date_time() {
        let builder = SpudBuilderSync::new();

        let date: Date = Date::new(2023, 3, 14).unwrap();
        let time: Time = Time::new(12, 30, 45, 123_456_789).unwrap();

        let date_time: DateTime = DateTime::new(date, time);

        builder
            .object(|obj| {
                obj.add_value("date_time", date_time)?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoderSync = SpudDecoderSync::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_date_time_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        let date: Date = Date::new(2023, 3, 14).unwrap();
        let time: Time = Time::new(12, 30, 45, 123_456_789).unwrap();

        let date_time: DateTime = DateTime::new(date, time);

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("date_time", date_time).await?;
                Ok(())
            })
            .await
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().await.unwrap();

        let mut decoder: SpudDecoderSync = SpudDecoderSync::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }
}
