use serde_json::{Number, Value};

use crate::{SpudError, spud_decoder::DecoderObject, spud_types::SpudNumberTypes};

pub(crate) fn number(
    decoder: &mut DecoderObject,
    number_type: SpudNumberTypes,
) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let number: Number = match number_type {
        SpudNumberTypes::U8 => {
            let read_bytes: &[u8] = decoder.read_bytes(1)?;

            Number::from(u8::from_le_bytes(read_bytes.try_into().map_err(|_| {
                SpudError::DecodingError("Invalid U8 bytes".to_owned())
            })?))
        }
        SpudNumberTypes::U16 => {
            let read_bytes: &[u8] = decoder.read_bytes(2)?;

            Number::from(u16::from_le_bytes(read_bytes.try_into().map_err(|_| {
                SpudError::DecodingError("Invalid U16 bytes".to_owned())
            })?))
        }
        SpudNumberTypes::U32 => {
            let read_bytes: &[u8] = decoder.read_bytes(4)?;

            Number::from(u32::from_le_bytes(read_bytes.try_into().map_err(|_| {
                SpudError::DecodingError("Invalid U32 bytes".to_owned())
            })?))
        }
        SpudNumberTypes::U64 => {
            let read_bytes: &[u8] = decoder.read_bytes(8)?;

            Number::from(u64::from_le_bytes(read_bytes.try_into().map_err(|_| {
                SpudError::DecodingError("Invalid U64 bytes".to_owned())
            })?))
        }
        SpudNumberTypes::U128 => {
            let read_bytes: &[u8] = decoder.read_bytes(16)?;

            Number::from(u128::from_le_bytes(read_bytes.try_into().map_err(
                |_| SpudError::DecodingError("Invalid U128 bytes".to_owned()),
            )?))
        }
        SpudNumberTypes::I8 => {
            let read_bytes: &[u8] = decoder.read_bytes(1)?;

            Number::from(i8::from_le_bytes(read_bytes.try_into().map_err(|_| {
                SpudError::DecodingError("Invalid I8 bytes".to_owned())
            })?))
        }
        SpudNumberTypes::I16 => {
            let read_bytes: &[u8] = decoder.read_bytes(2)?;

            Number::from(i16::from_le_bytes(read_bytes.try_into().map_err(|_| {
                SpudError::DecodingError("Invalid I16 bytes".to_owned())
            })?))
        }
        SpudNumberTypes::I32 => {
            let read_bytes: &[u8] = decoder.read_bytes(4)?;

            Number::from(i32::from_le_bytes(read_bytes.try_into().map_err(|_| {
                SpudError::DecodingError("Invalid I32 bytes".to_owned())
            })?))
        }
        SpudNumberTypes::I64 => {
            let read_bytes: &[u8] = decoder.read_bytes(8)?;

            Number::from(i64::from_le_bytes(read_bytes.try_into().map_err(|_| {
                SpudError::DecodingError("Invalid I64 bytes".to_owned())
            })?))
        }
        SpudNumberTypes::I128 => {
            let read_bytes: &[u8] = decoder.read_bytes(16)?;

            Number::from(i128::from_le_bytes(read_bytes.try_into().map_err(
                |_| SpudError::DecodingError("Invalid I128 bytes".to_owned()),
            )?))
        }
        SpudNumberTypes::F32 => {
            let read_bytes: &[u8] = decoder.read_bytes(4)?;

            Number::from_f64(
                f32::from_le_bytes(
                    read_bytes
                        .try_into()
                        .map_err(|_| SpudError::DecodingError("Invalid F32 bytes".to_owned()))?,
                )
                .into(),
            )
            .ok_or(SpudError::DecodingError(
                "Invalid F32 value: cannot be NaN or infinity".to_owned(),
            ))?
        }
        SpudNumberTypes::F64 => {
            let read_bytes: &[u8] = decoder.read_bytes(8)?;

            Number::from_f64(f64::from_le_bytes(
                read_bytes
                    .try_into()
                    .map_err(|_| SpudError::DecodingError("Invalid F64 bytes".to_owned()))?,
            ))
            .ok_or(SpudError::DecodingError(
                "Invalid F64 value: cannot be NaN or infinity".to_owned(),
            ))?
        }
    };

    Ok(Value::Number(number))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[cfg(feature = "sync")]
    #[test]
    fn test_number() {
        let builder = SpudBuilderSync::new();

        builder
            .object(|obj| {
                obj.add_value("i8", 1i8)?;
                obj.add_value("u8", 1u8)?;
                obj.add_value("i16", 1i16)?;
                obj.add_value("u16", 1u16)?;
                obj.add_value("i32", 1i32)?;
                obj.add_value("u32", 1u32)?;
                obj.add_value("f32", 1.0f32)?;
                obj.add_value("i64", 1i64)?;
                obj.add_value("u64", 1u64)?;
                obj.add_value("f64", 1.0f64)?;
                obj.add_value("i128", 1i128)?;
                obj.add_value("u128", 1u128)?;
                Ok(())
            })
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_number_async() {
        use std::sync::Arc;

        use tokio::sync::{Mutex, MutexGuard};

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                obj.add_value("i8", 1i8).await?;
                obj.add_value("u8", 1u8).await?;
                obj.add_value("i16", 1i16).await?;
                obj.add_value("u16", 1u16).await?;
                obj.add_value("i32", 1i32).await?;
                obj.add_value("u32", 1u32).await?;
                obj.add_value("f32", 1.0f32).await?;
                obj.add_value("i64", 1i64).await?;
                obj.add_value("u64", 1u64).await?;
                obj.add_value("f64", 1.0f64).await?;
                obj.add_value("i128", 1i128).await?;
                obj.add_value("u128", 1u128).await?;
                Ok(())
            })
            .await
            .unwrap();

        let encoded_bytes: Vec<u8> = builder.encode().await.unwrap();

        let mut decoder: SpudDecoder = SpudDecoder::new(&encoded_bytes).unwrap();

        decoder.decode(false, false).unwrap();
    }
}
