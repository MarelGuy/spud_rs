use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject, types::Date};

pub(crate) fn date(decoder: &mut DecoderObject) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let read_bytes: &[u8] = decoder.read_bytes(4)?;

    let date: Date = DecoderObject::read_date(read_bytes)?;

    Ok(Value::String(date.to_string()))
}
