use rust_decimal::Decimal;
use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject};

pub(crate) fn decimal(decoder: &mut DecoderObject) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let read_bytes: &[u8] = decoder.read_bytes(16)?;

    let decimal_value: Decimal = Decimal::deserialize(
        read_bytes
            .try_into()
            .map_err(|_| SpudError::DecodingError("Invalid Decimal bytes".to_owned()))?,
    );

    Ok(Value::String(decimal_value.to_string()))
}
