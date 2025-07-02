use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject};

pub(crate) fn string(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    let string_len: usize = decoder.read_variable_length_data()?;

    *next_steps = string_len;

    Ok(Value::String(String::from_utf8(
        decoder.contents[decoder.index..decoder.index + string_len].to_vec(),
    )?))
}
