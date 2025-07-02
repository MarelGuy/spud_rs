use serde_json::Value;

use crate::SpudError;

pub(crate) fn null(next_steps: &mut usize) -> Result<Value, SpudError> {
    *next_steps = 1;
    Ok(Value::Null)
}
