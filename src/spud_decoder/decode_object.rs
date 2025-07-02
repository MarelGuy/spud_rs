use indexmap::IndexMap;
use serde_json::Value;

use crate::{
    SpudError,
    spud_decoder::decoder_functions::{
        array_start, binary_blob, bool as d_bool, date, date_time, decimal, null, number,
        object_start, string, time,
    },
    spud_types::{SpudNumberTypes, SpudTypes},
    types::{Date, Time},
};

pub(crate) struct DecoderObject<'a> {
    pub(crate) contents: &'a [u8],
    pub(crate) index: usize,
    pub(crate) field_names: &'a IndexMap<u8, String>,
    pub(crate) current_byte: u8,
    pub(crate) current_field: String,
}

impl<'a> DecoderObject<'a> {
    pub(crate) fn new(
        contents: &'a [u8],
        field_names: &'a IndexMap<u8, String>,
    ) -> DecoderObject<'a> {
        DecoderObject {
            contents,
            index: 0,
            field_names,
            current_byte: 0,
            current_field: String::new(),
        }
    }

    pub(crate) fn decode(&mut self) -> Result<IndexMap<String, Value>, SpudError> {
        let mut object: IndexMap<String, Value> = IndexMap::new();

        self.next(1)?;

        let id: &[u8] = self.read_bytes(10)?;

        let object_id: String = bs58::encode(&id).into_string();
        object.insert("oid".to_string(), Value::String(object_id));

        while self.index < self.contents.len() {
            if self.current_byte == SpudTypes::ObjectEnd.as_u8() {
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
    pub(crate) fn next(&mut self, steps: usize) -> Result<(), SpudError> {
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

    pub(crate) fn read_field_name(&mut self) -> Result<usize, SpudError> {
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
    pub(crate) fn read_variable_length_data(&mut self) -> Result<usize, SpudError> {
        self.next(1)?;

        let read_byte_value: u64 = match self.current_byte {
            val if val == SpudTypes::Number(SpudNumberTypes::U8).as_u8() => 1,
            val if val == SpudTypes::Number(SpudNumberTypes::U16).as_u8() => 2,
            val if val == SpudTypes::Number(SpudNumberTypes::U32).as_u8() => 4,
            val if val == SpudTypes::Number(SpudNumberTypes::U64).as_u8() => 8,
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

    pub(crate) fn read_bytes(&mut self, steps: usize) -> Result<&'a [u8], SpudError> {
        let result: &[u8] = &self.contents[self.index..self.index + steps];

        self.next(steps)?;

        Ok(result)
    }

    pub(crate) fn read_date(read_bytes: &[u8]) -> Result<Date, SpudError> {
        let year: u16 = u16::from_le_bytes(
            read_bytes[0..2]
                .try_into()
                .map_err(|_| SpudError::DecodingError("Invalid Date bytes".to_owned()))?,
        );

        let month: u8 = read_bytes[2];
        let day: u8 = read_bytes[3];

        Date::new(year, month, day)
    }

    pub(crate) fn read_time(read_bytes: &[u8]) -> Result<Time, SpudError> {
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
    pub(crate) fn decode_byte(&mut self, byte: u8) -> Result<Option<Value>, SpudError> {
        let decode_result: Option<SpudTypes> = SpudTypes::from_u8(byte);

        let mut next_steps: usize = 0;

        if decode_result == Some(SpudTypes::FieldNameId) {
            next_steps = self.read_field_name()?;

            self.next(next_steps)?;

            Ok(None)
        } else {
            let return_value: Value = match decode_result {
                Some(SpudTypes::Null) => null(&mut next_steps),
                Some(SpudTypes::Bool) => d_bool(self, &mut next_steps)?,
                Some(SpudTypes::Number(number_type)) => number(self, number_type)?,
                Some(SpudTypes::Decimal) => decimal(self)?,
                Some(SpudTypes::String) => string(self, &mut next_steps)?,
                Some(SpudTypes::Date) => date(self)?,
                Some(SpudTypes::Time) => time(self)?,
                Some(SpudTypes::DateTime) => date_time(self)?,
                Some(SpudTypes::BinaryBlob) => binary_blob(self, &mut next_steps)?,
                Some(SpudTypes::ArrayStart) => array_start(self, &mut next_steps)?,
                Some(SpudTypes::ObjectStart) => object_start(self, &mut next_steps)?,
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
