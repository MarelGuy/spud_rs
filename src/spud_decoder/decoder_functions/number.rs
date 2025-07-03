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
