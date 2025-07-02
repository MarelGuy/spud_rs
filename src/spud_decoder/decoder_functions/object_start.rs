use serde_json::{Map, Value};

use crate::{SpudError, spud_decoder::DecoderObject, spud_types::SpudTypes};

pub(crate) fn object_start(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    decoder.next(1)?;

    let mut output_object: Map<String, Value> = Map::new();

    let parent_field: String = decoder.current_field.clone();

    loop {
        let byte: Option<SpudTypes> = SpudTypes::from_u8(decoder.contents[decoder.index]);

        if byte == Some(SpudTypes::ObjectEnd) {
            break;
        }

        let decoded_byte: Option<Value> = decoder.decode_byte(decoder.contents[decoder.index])?;

        if let Some(value) = decoded_byte {
            output_object.insert(decoder.current_field.clone(), value);
        }
    }

    *next_steps = 1;

    decoder.current_field = parent_field;

    Ok(Value::Object(output_object))
}
