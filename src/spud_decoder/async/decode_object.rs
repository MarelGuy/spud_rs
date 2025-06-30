use indexmap::IndexMap;
use rust_decimal::Decimal;
use serde_json::{Map, Number, Value};

use crate::{
    SpudError,
    spud_types::SpudTypes,
    types::{Date, Time},
};

pub(super) struct DecoderObject<'a> {
    contents: &'a [u8],
    index: usize,
    field_names: &'a IndexMap<u8, String>,
    current_byte: u8,
    current_field: String,
}

impl<'a> DecoderObject<'a> {
    pub fn new(contents: &'a [u8], field_names: &'a IndexMap<u8, String>) -> DecoderObject<'a> {
        DecoderObject {
            contents,
            index: 0,
            field_names,
            current_byte: 0,
            current_field: String::new(),
        }
    }

    pub(super) fn decode(&mut self) -> Result<IndexMap<String, Value>, SpudError> {
        let mut object: IndexMap<String, Value> = IndexMap::new();

        self.next(1)?;

        let id: &[u8] = self.read_bytes(10)?;

        let object_id: String = bs58::encode(&id).into_string();
        object.insert("oid".to_string(), Value::String(object_id));

        while self.index < self.contents.len() {
            if self.current_byte == SpudTypes::ObjectEnd as u8 {
                break;
            }

            let field_value: Option<Value> = self.decode_byte(self.current_byte)?;

            if let Some(value) = field_value {
                object.insert(self.current_field.clone(), value);
            }
        }

        Ok(object)
    }

    /// # Panics
    ///
    /// Will panic if the index is out of bounds
    fn next(&mut self, steps: usize) -> Result<(), SpudError> {
        if self.index + steps >= self.contents.len() {
            return Err(SpudError::DecodingError(format!(
                "Index out of bounds, current index: {}, object length: {}, tried to read: {}",
                self.index,
                self.contents.len(),
                self.index + steps
            )));
        }

        self.index += steps;

        self.current_byte = self.contents[self.index];

        Ok(())
    }

    fn read_field_name(&mut self) -> Result<usize, SpudError> {
        self.next(1)?;

        let field_name_id: u8 = self.contents[self.index];

        self.current_field = self
            .field_names
            .get(&field_name_id)
            .cloned()
            .ok_or_else(|| {
                SpudError::DecodingError(format!(
                    "Field name ID {field_name_id} not found in field names map"
                ))
            })?;

        Ok(1)
    }

    /// # Panics
    ///
    /// Will panic on unknown token
    fn read_variable_length_data(&mut self) -> Result<usize, SpudError> {
        self.next(1)?;

        let read_byte_value: u64 = match self.current_byte {
            val if val == SpudTypes::U8 as u8 => 1,
            val if val == SpudTypes::U16 as u8 => 2,
            val if val == SpudTypes::U32 as u8 => 4,
            val if val == SpudTypes::U64 as u8 => 8,
            _ => Err(SpudError::DecodingError(
                "Expected: U8, U16, U32, U64, but got an unknown token".to_string(),
            ))?,
        };

        self.next(1)?;

        let read_bytes: &[u8] = self.read_bytes(usize::try_from(read_byte_value)?)?;

        Ok(match read_byte_value {
            1 => u8::from_le_bytes(
                read_bytes
                    .try_into()
                    .map_err(|_| SpudError::DecodingError("Invalid U8 bytes".to_owned()))?,
            ) as usize,
            2 => u16::from_le_bytes(
                read_bytes
                    .try_into()
                    .map_err(|_| SpudError::DecodingError("Invalid U16 bytes".to_owned()))?,
            ) as usize,
            4 => u32::from_le_bytes(
                read_bytes
                    .try_into()
                    .map_err(|_| SpudError::DecodingError("Invalid U32 bytes".to_owned()))?,
            ) as usize,
            8 => {
                usize::try_from(u64::from_le_bytes(read_bytes.try_into().map_err(|_| {
                    SpudError::DecodingError("Invalid U64 bytes".to_owned())
                })?))?
            }
            _ => unreachable!(),
        })
    }

    fn read_bytes(&mut self, steps: usize) -> Result<&'a [u8], SpudError> {
        let result: &[u8] = &self.contents[self.index..self.index + steps];

        self.next(steps)?;

        Ok(result)
    }

    fn read_date(read_bytes: &[u8]) -> Result<Date, SpudError> {
        let year: u16 = u16::from_le_bytes(
            read_bytes[0..2]
                .try_into()
                .map_err(|_| SpudError::DecodingError("Invalid Date bytes".to_owned()))?,
        );

        let month: u8 = read_bytes[2];
        let day: u8 = read_bytes[3];

        Date::new(year, month, day)
    }

    fn read_time(read_bytes: &[u8]) -> Result<Time, SpudError> {
        let hour: u8 = read_bytes[0];
        let minute: u8 = read_bytes[1];
        let second: u8 = read_bytes[2];
        let nanosecond: u32 = u32::from_le_bytes(
            read_bytes[3..7]
                .try_into()
                .map_err(|_| SpudError::DecodingError("Invalid Time bytes".to_owned()))?,
        );

        Time::new(hour, minute, second, nanosecond)
    }

    /// # Panics
    ///
    /// Will panic on unknown type
    fn decode_byte(&mut self, byte: u8) -> Result<Option<Value>, SpudError> {
        let decode_result: Option<SpudTypes> = SpudTypes::from_u8(byte);

        let mut next_steps: usize = 0;

        if decode_result == Some(SpudTypes::FieldNameId) {
            next_steps = self.read_field_name()?;

            self.next(next_steps)?;

            Ok(None)
        } else {
            let return_value: Value = match decode_result {
                Some(SpudTypes::Null) => {
                    next_steps = 1;

                    Value::Null
                }
                Some(SpudTypes::Bool) => {
                    self.next(1)?;

                    let value: Value = match self.contents.get(self.index) {
                        Some(0) => Value::Bool(false),
                        Some(1) => Value::Bool(true),
                        _ => Err(SpudError::DecodingError(format!(
                            "Unknown bool value: {}",
                            self.contents[self.index]
                        )))?,
                    };

                    next_steps = 1;

                    value
                }
                Some(SpudTypes::U8) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(1)?;

                    Value::Number(Number::from(u8::from_le_bytes(
                        read_bytes
                            .try_into()
                            .map_err(|_| SpudError::DecodingError("Invalid U8 bytes".to_owned()))?,
                    )))
                }
                Some(SpudTypes::U16) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(2)?;

                    Value::Number(Number::from(u16::from_le_bytes(
                        read_bytes.try_into().map_err(|_| {
                            SpudError::DecodingError("Invalid U16 bytes".to_owned())
                        })?,
                    )))
                }
                Some(SpudTypes::U32) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(4)?;

                    Value::Number(Number::from(u32::from_le_bytes(
                        read_bytes.try_into().map_err(|_| {
                            SpudError::DecodingError("Invalid U32 bytes".to_owned())
                        })?,
                    )))
                }
                Some(SpudTypes::U64) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(8)?;

                    Value::Number(Number::from(u64::from_le_bytes(
                        read_bytes.try_into().map_err(|_| {
                            SpudError::DecodingError("Invalid U64 bytes".to_owned())
                        })?,
                    )))
                }
                Some(SpudTypes::I8) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(1)?;

                    Value::Number(Number::from(i8::from_le_bytes(
                        read_bytes
                            .try_into()
                            .map_err(|_| SpudError::DecodingError("Invalid I8 bytes".to_owned()))?,
                    )))
                }
                Some(SpudTypes::I16) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(2)?;

                    Value::Number(Number::from(i16::from_le_bytes(
                        read_bytes.try_into().map_err(|_| {
                            SpudError::DecodingError("Invalid I16 bytes".to_owned())
                        })?,
                    )))
                }
                Some(SpudTypes::I32) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(4)?;

                    Value::Number(Number::from(i32::from_le_bytes(
                        read_bytes.try_into().map_err(|_| {
                            SpudError::DecodingError("Invalid I32 bytes".to_owned())
                        })?,
                    )))
                }
                Some(SpudTypes::I64) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(8)?;

                    Value::Number(Number::from(i64::from_le_bytes(
                        read_bytes.try_into().map_err(|_| {
                            SpudError::DecodingError("Invalid I64 bytes".to_owned())
                        })?,
                    )))
                }
                Some(SpudTypes::F32) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(4)?;

                    Value::Number(
                        Number::from_f64(
                            f32::from_le_bytes(read_bytes.try_into().map_err(|_| {
                                SpudError::DecodingError("Invalid F32 bytes".to_owned())
                            })?)
                            .into(),
                        )
                        .ok_or(SpudError::DecodingError(
                            "Invalid F32 value: cannot be NaN or infinity".to_owned(),
                        ))?,
                    )
                }
                Some(SpudTypes::F64) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(8)?;

                    Value::Number(
                        Number::from_f64(f64::from_le_bytes(read_bytes.try_into().map_err(
                            |_| SpudError::DecodingError("Invalid F64 bytes".to_owned()),
                        )?))
                        .ok_or(SpudError::DecodingError(
                            "Invalid F64 value: cannot be NaN or infinity".to_owned(),
                        ))?,
                    )
                }
                Some(SpudTypes::Decimal) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(16)?;

                    let decimal_value: Decimal =
                        Decimal::deserialize(read_bytes.try_into().map_err(|_| {
                            SpudError::DecodingError("Invalid Decimal bytes".to_owned())
                        })?);

                    Value::String(decimal_value.to_string())
                }
                Some(SpudTypes::String) => {
                    let string_len: usize = self.read_variable_length_data()?;

                    next_steps = string_len;

                    Value::String(String::from_utf8(
                        self.contents[self.index..self.index + string_len].to_vec(),
                    )?)
                }
                Some(SpudTypes::Date) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(4)?;

                    let date: Date = Self::read_date(read_bytes)?;

                    Value::String(date.to_string())
                }
                Some(SpudTypes::Time) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(7)?;

                    let time: Time = Self::read_time(read_bytes)?;

                    Value::String(time.to_string())
                }
                Some(SpudTypes::DateTime) => {
                    self.next(1)?;

                    let read_bytes: &[u8] = self.read_bytes(11)?;

                    let date: Date = Self::read_date(&read_bytes[0..4])?;
                    let time: Time = Self::read_time(&read_bytes[4..])?;

                    Value::String(format!("{date} {time}"))
                }
                Some(SpudTypes::BinaryBlob) => {
                    let blob_len: usize = self.read_variable_length_data()?;

                    let processed: Vec<u8> =
                        self.contents[self.index..self.index + blob_len].to_vec();

                    let mut output_array: Vec<Value> = vec![];

                    for processed_byte in &processed {
                        output_array.push(Value::Number(Number::from(*processed_byte)));
                    }

                    next_steps = blob_len;

                    Value::Array(output_array)
                }
                Some(SpudTypes::ArrayStart) => {
                    self.next(1)?;

                    let mut output_array: Vec<Value> = vec![];

                    loop {
                        let byte: Option<SpudTypes> = SpudTypes::from_u8(self.contents[self.index]);

                        if byte == Some(SpudTypes::ArrayEnd) {
                            break;
                        }

                        let decoded_byte: Option<Value> =
                            self.decode_byte(self.contents[self.index])?;

                        if let Some(value) = decoded_byte {
                            output_array.push(value);
                        }
                    }

                    next_steps = 1;

                    Value::Array(output_array)
                }
                Some(SpudTypes::ObjectStart) => {
                    self.next(1)?;

                    let mut output_object: Map<String, Value> = Map::new();

                    let parent_field: String = self.current_field.clone();

                    loop {
                        let byte: Option<SpudTypes> = SpudTypes::from_u8(self.contents[self.index]);

                        if byte == Some(SpudTypes::ObjectEnd) {
                            break;
                        }

                        let decoded_byte: Option<Value> =
                            self.decode_byte(self.contents[self.index])?;

                        if let Some(value) = decoded_byte {
                            output_object.insert(self.current_field.clone(), value);
                        }
                    }

                    next_steps = 1;

                    self.current_field = parent_field;

                    Value::Object(output_object)
                }
                _ => Err(SpudError::DecodingError(format!(
                    "Unknown type: {byte} at index {}",
                    self.index
                )))?,
            };

            self.next(next_steps)?;

            Ok(Some(return_value))
        }
    }
}
