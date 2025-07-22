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
            .object(|obj: &SpudObjectSync| {
                obj.add_value("string_u8_len", SpudString::from("Hello, world!"))?;
                obj.add_value(
                    "string_u16_len",
                    SpudString::from("x".repeat(u8::MAX as usize + 1)),
                )?;
                obj.add_value(
                    "string_u32_len",
                    SpudString::from("y".repeat(u16::MAX as usize + 1)),
                )?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

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

                obj.add_value("string_u8_len", SpudString::from("Hello, world!"))
                    .await?;
                obj.add_value(
                    "string_u16_len",
                    SpudString::from("x".repeat(u8::MAX as usize + 1)),
                )
                .await?;
                obj.add_value(
                    "string_u32_len",
                    SpudString::from("y".repeat(u16::MAX as usize + 1)),
                )
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
