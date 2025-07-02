use serde_json::Value;

use crate::{SpudError, spud_decoder::DecoderObject, spud_types::SpudTypes};

pub(crate) fn array_start(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let mut output_array: Vec<Value> = vec![];

    loop {
        let byte: Option<SpudTypes> = SpudTypes::from_u8(decoder.contents[decoder.index]);

        if byte == Some(SpudTypes::ArrayEnd) {
            break;
        }

        let decoded_byte: Option<Value> = decoder.decode_byte(decoder.contents[decoder.index])?;

        if let Some(value) = decoded_byte {
            output_array.push(value);
        }
    }

    *next_steps = 1;

    Ok(Value::Array(output_array))
}
