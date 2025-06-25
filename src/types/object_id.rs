use core::{
    fmt::Display,
    sync::atomic::{AtomicU32, Ordering},
};
use std::{
    error::Error,
    fmt,
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

use super::spud_string::SpudString;

/// Represents a unique identifier for an object in SPUD format.
///  The `ObjectId` is a 10-byte identifier that includes:
/// - 4 bytes for the timestamp (seconds since UNIX epoch)
/// - 3 bytes for a unique instance identifier
/// - 3 bytes for a counter that increments with each new `ObjectId` generated.
///   The `ObjectId` is designed to be unique across different instances and time, ensuring that each object can be distinctly identified.
///   The default display format is a base58-encoded string representation of the identifier.
#[derive(Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ObjectId(pub(crate) [u8; 10]);

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
    pub(crate) fn new() -> Result<Self, Box<dyn Error>> {
        let mut id: [u8; 10] = [0u8; 10];

        let timestamp_secs: u32 = if let Ok(value) = u32::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("System time is before UNIX EPOCH, which should not happen.")
                .as_secs(),
        ) {
            value
        } else {
            return Err("Failed to get current timestamp".to_string())?;
        };

        id[0..4].copy_from_slice(&timestamp_secs.to_le_bytes());
        id[4..7].copy_from_slice(&INSTANCE_IDENTIFIER[..]);

        let count_val: u32 = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let counter_24bit: u32 = count_val & 0x00FF_FFFF;
        let counter_bytes: [u8; 4] = counter_24bit.to_le_bytes();

        id[7..10].copy_from_slice(&counter_bytes[0..3]);

        Ok(ObjectId(id))
    }
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", bs58::encode(&self.0).into_string())
    }
}

impl From<&str> for ObjectId {
    fn from(s: &str) -> Self {
        let decoded: Vec<u8> = bs58::decode(s).into_vec().expect("Failed to decode base58");

        ObjectId(decoded.try_into().expect("Invalid ObjectId length"))
    }
}

impl From<String> for ObjectId {
    fn from(s: String) -> Self {
        let decoded: Vec<u8> = bs58::decode(s).into_vec().expect("Failed to decode base58");

        ObjectId(decoded.try_into().expect("Invalid ObjectId length"))
    }
}

impl From<[u8; 10]> for ObjectId {
    fn from(bytes: [u8; 10]) -> Self {
        ObjectId(bytes)
    }
}

impl From<SpudString> for ObjectId {
    fn from(value: SpudString) -> Self {
        let decoded: Vec<u8> = bs58::decode(&value.0)
            .into_vec()
            .expect("Failed to decode base58");

        let mut id: [u8; 10] = [0u8; 10];

        id.copy_from_slice(&decoded);

        ObjectId(id)
    }
}

impl fmt::Debug for ObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ObjectId({self})")
    }
}
