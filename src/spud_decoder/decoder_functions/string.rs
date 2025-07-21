use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject};

pub(crate) fn string(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    let string_len: usize = decoder.read_variable_length_data()?;

    *next_steps = string_len;

    Ok(Value::String(String::from_utf8(
        decoder.contents[decoder.index..decoder.index + string_len].to_vec(),
    )?))
}

#[cfg(test)]
mod tests {
    use crate::{types::SpudString, *};

    #[cfg(feature = "sync")]
    #[test]
    fn test_string() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj| {
                obj.add_value("string", SpudString::from("Hello, world!"))?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoderSync = SpudDecoderSync::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_string_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("string", SpudString::from("Hello, world!"))
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
