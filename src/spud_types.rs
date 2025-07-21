#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum SpudTypes {
    // Core Data Types
    Null = 0x03,
    Bool = 0x04,
    Number(SpudNumberTypes),
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum SpudNumberTypes {
    I8 = 0x05,
    I16 = 0x06,
    I32 = 0x07,
    I64 = 0x08,
    I128 = 0x19,
    U8 = 0x09,
    U16 = 0x0A,
    U32 = 0x0B,
    U64 = 0x0C,
    U128 = 0x20,
    F32 = 0x0D,
    F64 = 0x0E,
}

impl SpudTypes {
    #[must_use]
    pub fn from_u8(value: u8) -> Option<SpudTypes> {
        match value {
            0x01 => Some(SpudTypes::FieldNameListEnd),
            0x02 => Some(SpudTypes::FieldNameId),
            0x03 => Some(SpudTypes::Null),
            0x04 => Some(SpudTypes::Bool),
            5_u8..=14_u8 => Some(SpudTypes::Number(SpudNumberTypes::from_u8(value).unwrap())),
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

    #[must_use]
    pub fn as_u8(self) -> u8 {
        match self {
            SpudTypes::Null => 0x03,
            SpudTypes::Bool => 0x04,
            SpudTypes::Number(num_type) => num_type.as_u8(),
            SpudTypes::Decimal => 0x15,
            SpudTypes::String => 0x0F,
            SpudTypes::BinaryBlob => 0x14,
            SpudTypes::Date => 0x16,
            SpudTypes::Time => 0x17,
            SpudTypes::DateTime => 0x18,
            SpudTypes::ArrayStart => 0x10,
            SpudTypes::ArrayEnd => 0x11,
            SpudTypes::ObjectStart => 0x12,
            SpudTypes::ObjectEnd => 0x13,
            SpudTypes::FieldNameId => 0x02,
            SpudTypes::FieldNameListEnd => 0x01,
        }
    }
}

impl SpudNumberTypes {
    #[must_use]
    pub fn from_u8(value: u8) -> Option<SpudNumberTypes> {
        match value {
            0x05 => Some(SpudNumberTypes::I8),
            0x06 => Some(SpudNumberTypes::I16),
            0x07 => Some(SpudNumberTypes::I32),
            0x08 => Some(SpudNumberTypes::I64),
            0x19 => Some(SpudNumberTypes::I128),
            0x09 => Some(SpudNumberTypes::U8),
            0x0A => Some(SpudNumberTypes::U16),
            0x0B => Some(SpudNumberTypes::U32),
            0x0C => Some(SpudNumberTypes::U64),
            0x20 => Some(SpudNumberTypes::U128),
            0x0D => Some(SpudNumberTypes::F32),
            0x0E => Some(SpudNumberTypes::F64),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spud_types_from_u8() {
        assert_eq!(SpudTypes::from_u8(0x03), Some(SpudTypes::Null));
        assert_eq!(SpudTypes::from_u8(0x04), Some(SpudTypes::Bool));
        assert_eq!(SpudTypes::from_u8(0x0F), Some(SpudTypes::String));
        assert_eq!(SpudTypes::from_u8(0x10), Some(SpudTypes::ArrayStart));
        assert_eq!(SpudTypes::from_u8(0x11), Some(SpudTypes::ArrayEnd));
        assert_eq!(SpudTypes::from_u8(0x12), Some(SpudTypes::ObjectStart));
        assert_eq!(SpudTypes::from_u8(0x13), Some(SpudTypes::ObjectEnd));
        assert_eq!(SpudTypes::from_u8(0x14), Some(SpudTypes::BinaryBlob));
        assert_eq!(SpudTypes::from_u8(0x15), Some(SpudTypes::Decimal));
        assert_eq!(SpudTypes::from_u8(0x16), Some(SpudTypes::Date));
        assert_eq!(SpudTypes::from_u8(0x17), Some(SpudTypes::Time));
        assert_eq!(SpudTypes::from_u8(0x18), Some(SpudTypes::DateTime));
        assert_eq!(SpudTypes::from_u8(0x02), Some(SpudTypes::FieldNameId));
        assert_eq!(SpudTypes::from_u8(0x01), Some(SpudTypes::FieldNameListEnd));
    }

    #[test]
    fn test_spud_number_types_from_u8() {
        assert_eq!(SpudNumberTypes::from_u8(0x05), Some(SpudNumberTypes::I8));
        assert_eq!(SpudNumberTypes::from_u8(0x06), Some(SpudNumberTypes::I16));
        assert_eq!(SpudNumberTypes::from_u8(0x07), Some(SpudNumberTypes::I32));
        assert_eq!(SpudNumberTypes::from_u8(0x08), Some(SpudNumberTypes::I64));
        assert_eq!(SpudNumberTypes::from_u8(0x19), Some(SpudNumberTypes::I128));
        assert_eq!(SpudNumberTypes::from_u8(0x09), Some(SpudNumberTypes::U8));
        assert_eq!(SpudNumberTypes::from_u8(0x0A), Some(SpudNumberTypes::U16));
        assert_eq!(SpudNumberTypes::from_u8(0x0B), Some(SpudNumberTypes::U32));
        assert_eq!(SpudNumberTypes::from_u8(0x0C), Some(SpudNumberTypes::U64));
        assert_eq!(SpudNumberTypes::from_u8(0x20), Some(SpudNumberTypes::U128));
        assert_eq!(SpudNumberTypes::from_u8(0x0D), Some(SpudNumberTypes::F32));
        assert_eq!(SpudNumberTypes::from_u8(0x0E), Some(SpudNumberTypes::F64));
    }

    #[test]
    fn test_spud_types_as_u8() {
        assert_eq!(SpudTypes::Null.as_u8(), 0x03);
        assert_eq!(SpudTypes::Bool.as_u8(), 0x04);
        assert_eq!(SpudTypes::String.as_u8(), 0x0F);
        assert_eq!(SpudTypes::ArrayStart.as_u8(), 0x10);
        assert_eq!(SpudTypes::ArrayEnd.as_u8(), 0x11);
        assert_eq!(SpudTypes::ObjectStart.as_u8(), 0x12);
        assert_eq!(SpudTypes::ObjectEnd.as_u8(), 0x13);
        assert_eq!(SpudTypes::BinaryBlob.as_u8(), 0x14);
        assert_eq!(SpudTypes::Decimal.as_u8(), 0x15);
        assert_eq!(SpudTypes::Date.as_u8(), 0x16);
        assert_eq!(SpudTypes::Time.as_u8(), 0x17);
        assert_eq!(SpudTypes::DateTime.as_u8(), 0x18);
        assert_eq!(SpudTypes::FieldNameId.as_u8(), 0x02);
        assert_eq!(SpudTypes::FieldNameListEnd.as_u8(), 0x01);
    }

    #[test]
    fn test_spud_number_types_as_u8() {
        assert_eq!(SpudNumberTypes::I8.as_u8(), 0x05);
        assert_eq!(SpudNumberTypes::I16.as_u8(), 0x06);
        assert_eq!(SpudNumberTypes::I32.as_u8(), 0x07);
        assert_eq!(SpudNumberTypes::I64.as_u8(), 0x08);
        assert_eq!(SpudNumberTypes::I128.as_u8(), 0x19);
        assert_eq!(SpudNumberTypes::U8.as_u8(), 0x09);
        assert_eq!(SpudNumberTypes::U16.as_u8(), 0x0A);
        assert_eq!(SpudNumberTypes::U32.as_u8(), 0x0B);
        assert_eq!(SpudNumberTypes::U64.as_u8(), 0x0C);
        assert_eq!(SpudNumberTypes::U128.as_u8(), 0x20);
        assert_eq!(SpudNumberTypes::F32.as_u8(), 0x0D);
        assert_eq!(SpudNumberTypes::F64.as_u8(), 0x0E);
    }
}
