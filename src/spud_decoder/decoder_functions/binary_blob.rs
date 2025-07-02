use serde_json::{Number, Value};

use crate::{SpudError, spud_decoder::DecoderObject};

pub(crate) fn binary_blob(
    decoder: &mut DecoderObject,
    next_steps: &mut usize,
) -> Result<Value, SpudError> {
    let blob_len: usize = decoder.read_variable_length_data()?;

    let processed: Vec<u8> = decoder.contents[decoder.index..decoder.index + blob_len].to_vec();

    let mut output_array: Vec<Value> = vec![];

    for processed_byte in &processed {
        output_array.push(Value::Number(Number::from(*processed_byte)));
    }

    *next_steps = blob_len;

    Ok(Value::Array(output_array))
}
