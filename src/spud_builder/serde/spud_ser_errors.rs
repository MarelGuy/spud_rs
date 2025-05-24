#![cfg(feature = "serde")]

use serde::ser;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum SpudSerializationError {
    Custom(String),
    Io(std::io::Error),
}

impl ser::Error for SpudSerializationError {
    fn custom<T: Display>(msg: T) -> Self {
        SpudSerializationError::Custom(msg.to_string())
    }
}

impl Display for SpudSerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpudSerializationError::Custom(s) => write!(f, "SPUD Serialization Error: {s}"),
            SpudSerializationError::Io(e) => write!(f, "SPUD IO Error: {e}"),
        }
    }
}

impl std::error::Error for SpudSerializationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpudSerializationError::Io(e) => Some(e),
            SpudSerializationError::Custom(_) => None,
        }
    }
}

impl From<std::io::Error> for SpudSerializationError {
    fn from(err: std::io::Error) -> Self {
        SpudSerializationError::Io(err)
    }
}
