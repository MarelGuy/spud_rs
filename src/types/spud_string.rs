use core::{fmt, ops::Deref};

use super::object_id::ObjectId;

/// Represents a string for SPUD encoding.
/// This struct wraps a `Vec<u8>` and provides conversion implementations
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpudString(Vec<u8>);

impl SpudString {
    #[must_use]
    /// Returns the length of the string in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    /// Checks if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    /// Returns a byte slice of the string.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    /// Consumes the `SpudString` and returns the inner `Vec<u8>`.
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl From<&str> for SpudString {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

impl From<String> for SpudString {
    fn from(value: String) -> Self {
        Self(value.into_bytes())
    }
}

impl From<&String> for SpudString {
    fn from(value: &String) -> Self {
        Self(value.as_bytes().to_vec())
    }
}

impl AsRef<[u8]> for SpudString {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for SpudString {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ObjectId> for SpudString {
    fn from(value: ObjectId) -> Self {
        Self(bs58::encode(value.as_bytes()).into_string().into_bytes())
    }
}

impl fmt::Display for SpudString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
