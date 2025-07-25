use core::{
    fmt::Display,
    sync::atomic::{AtomicU32, Ordering},
};
use std::{
    fmt,
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::SpudError;

use super::spud_string::SpudString;

/// Represents a unique identifier for an object in SPUD format.
///  The `ObjectId` is a 10-byte identifier that includes:
/// - 4 bytes for the timestamp (seconds since UNIX epoch)
/// - 3 bytes for a unique instance identifier
/// - 3 bytes for a counter that increments with each new `ObjectId` generated.
///   The `ObjectId` is designed to be unique across different instances and time, ensuring that each object can be distinctly identified.
///   The default display format is a base58-encoded string representation of the identifier.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId([u8; 10]);

static INSTANCE_IDENTIFIER: LazyLock<[u8; 3]> = LazyLock::new(|| {
    let mut instance_bytes: [u8; 3] = [0u8; 3];

    getrandom::fill(&mut instance_bytes).expect("Failed to generate instance identifier");

    instance_bytes
});

static COUNTER_SEED: LazyLock<u32> = LazyLock::new(|| {
    let mut seed_bytes: [u8; 4] = [0u8; 4];

    getrandom::fill(&mut seed_bytes[0..3]).expect("Failed to generate counter seed");

    u32::from_le_bytes(seed_bytes) & 0x00FF_FFFF
});

static ID_COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(*COUNTER_SEED));

impl ObjectId {
    pub(crate) fn new() -> Result<Self, SpudError> {
        let mut id: [u8; 10] = [0u8; 10];

        let timestamp_secs: u32 = if let Ok(value) = u32::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| {
                    SpudError::ValidationError("System time is before UNIX epoch".to_string())
                })?
                .as_secs(),
        ) {
            value
        } else {
            return Err(SpudError::ValidationError(
                "Failed to get current timestamp".to_string(),
            ));
        };

        id[0..4].copy_from_slice(&timestamp_secs.to_le_bytes());
        id[4..7].copy_from_slice(&INSTANCE_IDENTIFIER[..]);

        let count_val: u32 = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let counter_24bit: u32 = count_val & 0x00FF_FFFF;
        let counter_bytes: [u8; 4] = counter_24bit.to_le_bytes();

        id[7..10].copy_from_slice(&counter_bytes[0..3]);

        Ok(ObjectId(id))
    }

    #[must_use]
    /// Returns the 10-byte representation of the `ObjectId`.
    pub fn as_bytes(&self) -> &[u8; 10] {
        &self.0
    }
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", bs58::encode(&self.0).into_string())
    }
}

impl TryFrom<&str> for ObjectId {
    type Error = SpudError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let decoded: Vec<u8> = bs58::decode(s)
            .into_vec()
            .map_err(|e| SpudError::ValidationError(format!("Failed to decode base58: {e}")))?;

        let bytes: [u8; 10] = decoded
            .try_into()
            .map_err(|_| SpudError::ValidationError("Invalid ObjectId length".to_string()))?;

        Ok(ObjectId(bytes))
    }
}

impl TryFrom<String> for ObjectId {
    type Error = SpudError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let decoded: Vec<u8> = bs58::decode(s)
            .into_vec()
            .map_err(|e| SpudError::ValidationError(format!("Failed to decode base58: {e}")))?;

        Ok(ObjectId(decoded.try_into().map_err(|_| {
            SpudError::ValidationError("Invalid ObjectId length".to_string())
        })?))
    }
}

impl From<[u8; 10]> for ObjectId {
    fn from(bytes: [u8; 10]) -> Self {
        ObjectId(bytes)
    }
}

impl TryFrom<SpudString> for ObjectId {
    type Error = SpudError;

    fn try_from(value: SpudString) -> Result<Self, Self::Error> {
        let decoded: Vec<u8> = bs58::decode(value.as_bytes())
            .into_vec()
            .map_err(|e| SpudError::ValidationError(format!("Failed to decode base58: {e}")))?;

        let mut id: [u8; 10] = [0u8; 10];

        id.copy_from_slice(&decoded);

        Ok(ObjectId(id))
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectId({self})")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_id_creation() {
        let id: ObjectId = ObjectId::new().expect("Failed to create ObjectId");

        assert_eq!(id.as_bytes().len(), 10);
        assert!(!id.to_string().is_empty());
    }

    #[test]
    fn test_from_str() {
        let id: ObjectId = ObjectId::new().expect("Failed to create ObjectId");
        let id_str: String = id.to_string();
        let parsed_id: ObjectId =
            ObjectId::try_from(id_str.as_str()).expect("Failed to parse ObjectId");

        assert_eq!(id, parsed_id);
    }

    #[test]
    fn test_from_string() {
        let id: ObjectId = ObjectId::new().expect("Failed to create ObjectId");
        let id_str: String = id.to_string();
        let parsed_id: ObjectId = ObjectId::try_from(id_str).expect("Failed to parse ObjectId");

        assert_eq!(id, parsed_id);
    }

    #[test]
    fn test_from_str_err() {
        let invalid_str: &'static str = "invalid_base58_string";
        let parsed_id: Result<ObjectId, SpudError> = ObjectId::try_from(invalid_str);

        assert!(parsed_id.is_err());
    }

    #[test]
    fn test_from_string_err() {
        let invalid_str: String = "invalid_base58_string".into();
        let parsed_id: Result<ObjectId, SpudError> = ObjectId::try_from(invalid_str);

        assert!(parsed_id.is_err());
    }

    #[test]
    fn test_from_bytes() {
        let id: ObjectId = ObjectId::new().expect("Failed to create ObjectId");
        let bytes: [u8; 10] = *id.as_bytes();
        let from_bytes: ObjectId = ObjectId::from(bytes);

        assert_eq!(id, from_bytes);
    }

    #[test]
    fn test_try_from_spud_string() {
        let id: ObjectId = ObjectId::new().expect("Failed to create ObjectId");
        let spud_string: SpudString = SpudString::from(id.to_string());
        let parsed_id: ObjectId =
            ObjectId::try_from(spud_string).expect("Failed to parse ObjectId from SpudString");

        assert_eq!(id, parsed_id);
    }

    #[test]
    fn test_try_from_spud_string_err() {
        let invalid_spud_string: SpudString = SpudString::from("invalid_base58_string");
        let parsed_id: Result<ObjectId, SpudError> = ObjectId::try_from(invalid_spud_string);

        assert!(parsed_id.is_err());
    }

    #[test]
    fn test_debug_impl() {
        let id: ObjectId = ObjectId::new().expect("Failed to create ObjectId");
        let debug_str: String = format!("{id:?}");

        assert!(debug_str.contains("ObjectId"));
        assert!(debug_str.contains(&id.to_string()));
    }
}
