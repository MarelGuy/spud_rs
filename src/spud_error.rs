use std::{
    array::TryFromSliceError, error::Error, fmt, num::TryFromIntError, string::FromUtf8Error,
};

#[derive(Debug)]
pub enum SpudError {
    Io(std::io::Error),
    FromUtf8(FromUtf8Error),
    SerdeJson(serde_json::Error),
    GetRandom(getrandom::Error),
    Bs58(bs58::decode::Error),
    TryFromInt(TryFromIntError),
    TryFromSlice(TryFromSliceError),
    InvalidPath(String),
    InvalidSpudFile(String),
    DecodingError(String),
    EncodingError(String),
    ValidationError(String),
    DateError(String),
    TimeError(String),
}

impl fmt::Display for SpudError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpudError::Io(err) => write!(f, "IO error: {err}"),
            SpudError::FromUtf8(err) => write!(f, "UTF-8 conversion error: {err}"),
            SpudError::SerdeJson(err) => write!(f, "JSON serialization error: {err}"),
            SpudError::GetRandom(err) => write!(f, "getrandom error: {err}"),
            SpudError::Bs58(err) => write!(f, "Base58 decoding error: {err}"),
            SpudError::TryFromInt(err) => write!(f, "Integer conversion error: {err}"),
            SpudError::TryFromSlice(err) => write!(f, "Slice conversion error: {err}"),
            SpudError::InvalidPath(s) => write!(f, "Invalid path: {s}"),
            SpudError::InvalidSpudFile(s) => write!(f, "Invalid SPUD file: {s}"),
            SpudError::DecodingError(s) => write!(f, "Decoding error: {s}"),
            SpudError::EncodingError(s) => write!(f, "Encoding error: {s}"),
            SpudError::ValidationError(s) => write!(f, "Validation error: {s}"),
            SpudError::DateError(s) => write!(f, "Date error: {s}"),
            SpudError::TimeError(s) => write!(f, "Time error: {s}"),
        }
    }
}

impl Error for SpudError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SpudError::Io(err) => Some(err),
            SpudError::FromUtf8(err) => Some(err),
            SpudError::SerdeJson(err) => Some(err),
            SpudError::GetRandom(err) => Some(err),
            SpudError::Bs58(err) => Some(err),
            SpudError::TryFromInt(err) => Some(err),
            SpudError::TryFromSlice(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SpudError {
    fn from(err: std::io::Error) -> SpudError {
        SpudError::Io(err)
    }
}

impl From<FromUtf8Error> for SpudError {
    fn from(err: FromUtf8Error) -> SpudError {
        SpudError::FromUtf8(err)
    }
}

impl From<serde_json::Error> for SpudError {
    fn from(err: serde_json::Error) -> SpudError {
        SpudError::SerdeJson(err)
    }
}

impl From<getrandom::Error> for SpudError {
    fn from(err: getrandom::Error) -> SpudError {
        SpudError::GetRandom(err)
    }
}

impl From<bs58::decode::Error> for SpudError {
    fn from(err: bs58::decode::Error) -> SpudError {
        SpudError::Bs58(err)
    }
}

impl From<TryFromIntError> for SpudError {
    fn from(err: TryFromIntError) -> SpudError {
        SpudError::TryFromInt(err)
    }
}

impl From<TryFromSliceError> for SpudError {
    fn from(err: TryFromSliceError) -> SpudError {
        SpudError::TryFromSlice(err)
    }
}
