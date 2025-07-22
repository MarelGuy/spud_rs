use serde_json::{Number, Value};

use crate::{SpudError, spud_decoder::DecoderObject};

pub(crate) fn binary_blob(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    let blob_len: usize = decoder.read_variable_length_data()?;

    let processed: Vec<u8> = decoder.contents[decoder.index..decoder.index + blob_len].to_vec();

    let mut output_array: Vec<Value> = vec![];

    for processed_byte in &processed {
        output_array.push(Value::Number(Number::from(*processed_byte)));
    }

    *next_steps = blob_len;

    Ok(Value::Array(output_array))
}

#[cfg(test)]
mod tests {
    use crate::{types::BinaryBlob, *};

    #[cfg(feature = "sync")]
    #[test]
    fn test_blob() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj| {
                obj.add_value("blob", BinaryBlob::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_blob_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("blob", BinaryBlob::new(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]))
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
