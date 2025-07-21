use serde_json::{Map, Value};

use crate::{SpudError, spud_decoder::DecoderObject, spud_types::SpudTypes};

pub(crate) fn object_start(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let mut output_object: Map<String, Value> = Map::new();

    let parent_field: String = decoder.current_field.clone();

    loop {
        let byte: Option<SpudTypes> = SpudTypes::from_u8(decoder.contents[decoder.index]);

        if byte == Some(SpudTypes::ObjectEnd) {
            break;
        }

        let decoded_byte: Option<Value> = decoder.decode_byte(decoder.contents[decoder.index])?;

        if let Some(value) = decoded_byte {
            output_object.insert(decoder.current_field.clone(), value);
        }
    }

    *next_steps = 1;

    decoder.current_field = parent_field;

    Ok(Value::Object(output_object))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[cfg(feature = "sync")]
    #[test]
    fn test_single_object() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("null", ())?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoderSync = SpudDecoderSync::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_single_object_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("null", ()).await?;
                Ok(())
            })
            .await
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().await.unwrap();

        let mut decoder: SpudDecoderSync = SpudDecoderSync::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_nested_object() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("null", ())?;

                obj.object("object", |nested_obj: &SpudObjectSync| {
                    nested_obj.add_value("null", ())?;
                    Ok(())
                })?;

                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoderSync = SpudDecoderSync::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_nested_object_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("null", ()).await?;

                obj.object("object", async |nested_obj: Arc<Mutex<SpudObjectAsync>>| {
                    let nested_obj: MutexGuard<'_, SpudObjectAsync> = nested_obj.lock().await;

                    nested_obj.add_value("null", ()).await?;
                    Ok(())
                }).await?;

                Ok(())
            })
            .await
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().await.unwrap();

        let mut decoder: SpudDecoderSync = SpudDecoderSync::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }
}
