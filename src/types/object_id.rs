use core::sync::atomic::{AtomicU32, Ordering};
use std::{
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

use super::spud_string::SpudString;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ObjectId(pub [u8; 10]);

static INSTANCE_IDENTIFIER: LazyLock<[u8; 3]> = LazyLock::new(|| {
    let mut instance_bytes: [u8; 3] = [0u8; 3];
    match getrandom::fill(&mut instance_bytes) {
        Ok(()) => instance_bytes,
        Err(e) => {
            tracing::error!("Failed to generate instance identifier: {e}");
            panic!("Closing...")
        }
    }
});

static COUNTER_SEED: LazyLock<u32> = LazyLock::new(|| {
    let mut seed_bytes: [u8; 4] = [0u8; 4];
    match getrandom::fill(&mut seed_bytes[0..3]) {
        Ok(()) => u32::from_le_bytes(seed_bytes) & 0x00FF_FFFF,
        Err(e) => {
            tracing::error!("Failed to generate counter seed: {e}");
            panic!("Closing...")
        }
    }
});

static ID_COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(*COUNTER_SEED));

impl ObjectId {
    pub(crate) fn new() -> Self {
        let mut id: [u8; 10] = [0u8; 10];

        let timestamp_secs: u32 = u32::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("System time is before UNIX EPOCH, which should not happen.")
                .as_secs(),
        )
        .unwrap();

        id[0..4].copy_from_slice(&timestamp_secs.to_le_bytes());
        id[4..7].copy_from_slice(&INSTANCE_IDENTIFIER[..]);

        let count_val: u32 = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let counter_24bit: u32 = count_val & 0x00FF_FFFF;
        let counter_bytes: [u8; 4] = counter_24bit.to_le_bytes();

        id[7..10].copy_from_slice(&counter_bytes[0..3]);

        ObjectId(id)
    }
}

impl ToString for ObjectId {
    fn to_string(&self) -> String {
        bs58::encode(&self.0).into_string()
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
