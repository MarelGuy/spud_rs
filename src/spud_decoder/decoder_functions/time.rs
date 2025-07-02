use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject, types::Time};

pub(crate) fn time(decoder: &mut DecoderObject) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let read_bytes: &[u8] = decoder.read_bytes(7)?;

    let time: Time = DecoderObject::read_time(read_bytes)?;

    Ok(Value::String(time.to_string()))
}
