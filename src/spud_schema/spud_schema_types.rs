#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum SpudSchemaTypes {
    FieldName = 0x01,
    Null = 0x02,
    Bool = 0x03,
    Number = 0x04,
    String = 0x06,
    Array = 0x7,
    Object = 0x8,
    BinaryBlob = 0x9,
}
