use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject};

pub(crate) fn bool(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let value: Value = match decoder.contents.get(decoder.index) {
        Some(0) => Value::Bool(false),
        Some(1) => Value::Bool(true),
        _ => Err(SpudError::DecodingError(format!(
            "Unknown bool value: {}",
            decoder.contents[decoder.index]
        )))?,
    };

    *next_steps = 1;

    Ok(value)
}
