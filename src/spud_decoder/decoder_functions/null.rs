use serde_json::Value;

pub(crate) fn null(next_steps: &mut usize) -> Value {
    *next_steps = 1;

    Value::Null
}
