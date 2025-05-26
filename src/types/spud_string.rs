use super::object_id::ObjectId;

/// Represents a string for SPUD encoding.
/// This struct wraps a `String` and provides conversion implementations
pub struct SpudString(pub String);

impl From<&str> for SpudString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for SpudString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&String> for SpudString {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}

impl From<ObjectId> for SpudString {
    fn from(value: ObjectId) -> Self {
        Self(bs58::encode(&value.0).into_string())
    }
}
