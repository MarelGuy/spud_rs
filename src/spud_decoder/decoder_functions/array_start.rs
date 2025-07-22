use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject, spud_types::SpudTypes};

pub(crate) fn array_start(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let mut output_array: Vec<Value> = vec![];

    loop {
        let byte: Option<SpudTypes> = SpudTypes::from_u8(decoder.contents[decoder.index]);

        if byte == Some(SpudTypes::ArrayEnd) {
            break;
        }

        let decoded_byte: Option<Value> = decoder.decode_byte(decoder.contents[decoder.index])?;

        if let Some(value) = decoded_byte {
            output_array.push(value);
        }
    }

    *next_steps = 1;

    Ok(Value::Array(output_array))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[cfg(feature = "sync")]
    #[test]
    fn test_array() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj| {
                obj.add_value("array", vec![1, 2, 3])?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_array_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("array", vec![1, 2, 3]).await?;
                Ok(())
            })
            .await
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().await.unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }
}
