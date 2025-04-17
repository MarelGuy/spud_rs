use std::{collections::HashMap, fs};

use spud_add_number::SpudAddNumber;

use crate::SpudTypes;

pub mod spud_add_number;

pub struct SpudBuilder {
    pub data: Vec<u8>,
    pub field_names: HashMap<(String, u8), u8>,
    field_names_index: u8,
}

impl SpudBuilder {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            field_names: HashMap::new(),
            field_names_index: 1,
        }
    }

    fn add_field_name(&mut self, field_name: &str) -> &mut Self {
        let key: (String, u8) = (field_name.into(), field_name.len() as u8);

        let id: u8 = match self.field_names.get(&key) {
            Some(value) => *value,
            None => {
                self.field_names_index += 1;
                self.field_names.insert(key, self.field_names_index);
                self.field_names_index
            }
        };

        self.data.push(SpudTypes::FieldNameId as u8);
        self.data.push(id);

        self
    }

    fn add_value_length(&mut self, value_len: usize) -> &mut Self {
        if value_len <= u8::MAX as usize {
            self.data.push(SpudTypes::U8 as u8);
            self.data
                .extend_from_slice(&(value_len as u8).to_le_bytes());
        } else if value_len <= u16::MAX as usize {
            self.data.push(SpudTypes::U16 as u8);
            self.data
                .extend_from_slice(&(value_len as u16).to_le_bytes());
        } else if value_len <= u32::MAX as usize {
            self.data.push(SpudTypes::U32 as u8);
            self.data
                .extend_from_slice(&(value_len as u32).to_le_bytes());
        } else if value_len <= u64::MAX as usize {
            self.data.push(SpudTypes::U64 as u8);
            self.data
                .extend_from_slice(&(value_len as u64).to_le_bytes());
        }

        self
    }

    pub fn add_null(&mut self, field_name: &str) -> &mut Self {
        self.add_field_name(field_name);

        self.data.push(SpudTypes::Null as u8);

        self
    }

    pub fn add_bool(&mut self, field_name: &str, value: bool) -> &mut Self {
        self.add_field_name(field_name);

        self.data.push(SpudTypes::Bool as u8);
        self.data.push(value as u8);

        self
    }

    pub fn add_number<T: SpudAddNumber>(&mut self, field_name: &str, value: T) -> &mut Self {
        self.add_field_name(field_name);

        self.data.push(value.get_spud_type_tag() as u8);
        value.write_spud_bytes(&mut self.data);

        self
    }

    pub fn add_string(&mut self, field_name: &str, value: &str) -> &mut Self {
        self.add_field_name(field_name);

        self.data.push(SpudTypes::String as u8);

        self.add_value_length(value.len());

        self.data.extend_from_slice(value.as_bytes());

        self
    }

    pub fn add_binary_blob(&mut self, field_name: &str, value: &[u8]) -> &mut Self {
        self.add_field_name(field_name);

        self.data.push(SpudTypes::BinaryBlob as u8);

        self.add_value_length(value.len());

        self.data.extend_from_slice(value);

        self
    }

    pub fn build_file(&mut self, path: &str, file_name: &str) {
        let path_str: String = format!("{}/{}.spud", path, file_name);

        let mut header: Vec<u8> = env!("SPUD_VERSION").as_bytes().to_vec();
        const EOF_MARKER: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];

        for (name, id) in self.field_names.iter() {
            header.push(name.1);

            header.extend_from_slice(name.0.as_bytes());

            header.push(*id);
        }

        header.push(SpudTypes::FieldNameListEnd as u8);

        header.extend_from_slice(&self.data);
        header.extend_from_slice(EOF_MARKER);

        fs::write(path_str, header).unwrap();
    }
}
