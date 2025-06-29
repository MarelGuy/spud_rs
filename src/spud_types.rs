#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum SpudTypes {
    // Core Data Types
    Null = 0x03,
    Bool = 0x04,
    I8 = 0x05,
    I16 = 0x06,
    I32 = 0x07,
    I64 = 0x08,
    U8 = 0x09,
    U16 = 0x0A,
    U32 = 0x0B,
    U64 = 0x0C,
    F32 = 0x0D,
    F64 = 0x0E,
    Decimal = 0x15,

    // Variable-Length Types
    String = 0x0F,
    BinaryBlob = 0x14,

    // Date and Time Types
    Date = 0x16,
    Time = 0x17,
    DateTime = 0x18,

    // Composite Type Delimiters
    ArrayStart = 0x10,
    ArrayEnd = 0x11,
    ObjectStart = 0x12,
    ObjectEnd = 0x13,

    // Identifiers and Metadata
    FieldNameId = 0x02,
    FieldNameListEnd = 0x01,
}

impl SpudTypes {
    #[must_use]
    pub fn from_u8(value: u8) -> Option<SpudTypes> {
        match value {
            0x01 => Some(SpudTypes::FieldNameListEnd),
            0x02 => Some(SpudTypes::FieldNameId),
            0x03 => Some(SpudTypes::Null),
            0x04 => Some(SpudTypes::Bool),
            0x05 => Some(SpudTypes::I8),
            0x06 => Some(SpudTypes::I16),
            0x07 => Some(SpudTypes::I32),
            0x08 => Some(SpudTypes::I64),
            0x09 => Some(SpudTypes::U8),
            0x0A => Some(SpudTypes::U16),
            0x0B => Some(SpudTypes::U32),
            0x0C => Some(SpudTypes::U64),
            0x0D => Some(SpudTypes::F32),
            0x0E => Some(SpudTypes::F64),
            0x0F => Some(SpudTypes::String),
            0x10 => Some(SpudTypes::ArrayStart),
            0x11 => Some(SpudTypes::ArrayEnd),
            0x12 => Some(SpudTypes::ObjectStart),
            0x13 => Some(SpudTypes::ObjectEnd),
            0x14 => Some(SpudTypes::BinaryBlob),
            0x15 => Some(SpudTypes::Decimal),
            0x16 => Some(SpudTypes::Date),
            0x17 => Some(SpudTypes::Time),
            0x18 => Some(SpudTypes::DateTime),
            _ => None,
        }
    }
}
