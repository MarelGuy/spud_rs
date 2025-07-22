use serde_json::{Map, Value};

use crate::{SpudError, spud_decoder::DecoderObject, spud_types::SpudTypes};

pub(crate) fn object_start(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    decoder.next(2)?;

    let mut output_object: Map<String, Value> = Map::new();

    let id_bytes: &[u8] = decoder.read_bytes(10)?;
    let object_id: String = bs58::encode(id_bytes).into_string();
    output_object.insert("oid".to_string(), Value::String(object_id));

    let parent_field: String = decoder.current_field.clone();

    loop {
        if decoder.contents.get(decoder.index) == Some(&SpudTypes::ObjectEnd.as_u8())
            && decoder.contents.get(decoder.index + 1) == Some(&SpudTypes::ObjectEnd.as_u8())
        {
            break;
        }

        let decoded_byte: Option<Value> = decoder.decode_byte(decoder.contents[decoder.index])?;

        if let Some(value) = decoded_byte {
            output_object.insert(decoder.current_field.clone(), value);
        }
    }

    *next_steps = 2;

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

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

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

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_multiple_object() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("null", ())?;
                Ok(())
            })
            .unwrap();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("null", ())?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_multiple_object_async() {
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

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("null", ()).await?;
                Ok(())
            })
            .await
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().await.unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

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

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

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
                })
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
