use serde_json::Value;

use crate::{
    SpudError,
    spud_decoder::DecoderObject,
    types::{Date, Time},
};

pub(crate) fn date_time(decoder: &mut DecoderObject) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let read_bytes: &[u8] = decoder.read_bytes(11)?;

    let date: Date = DecoderObject::read_date(&read_bytes[0..4])?;
    let time: Time = DecoderObject::read_time(&read_bytes[4..])?;

    Ok(Value::String(format!("{date} {time}")))
}
