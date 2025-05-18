use spud_add_number::SpudAddNumber;
use std::{collections::HashMap, path::Path, process};

use crate::functions::{check_path::check_path, initialise_header::initialise_header};

#[cfg(feature = "async")]
use tokio::fs::write;

#[cfg(not(feature = "async"))]
use std::fs;

use crate::spud_types::SpudTypes;

pub mod spud_add_number;
pub mod spud_type_ext;

pub struct SpudBuilder {
    pub data: Vec<u8>,
    pub field_names: HashMap<(String, u8), u8>,
    pub field_names_index: u8,
}

impl SpudBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            field_names: HashMap::new(),
            field_names_index: 1,
        }
    }

    fn add_field_name(&mut self, field_name: &str) -> &mut Self {
        let key: (String, u8) = (field_name.into(), u8::try_from(field_name.len()).unwrap());

        let id: u8 = if let Some(value) = self.field_names.get(&key) {
            *value
        } else {
            self.field_names_index += 1;
            self.field_names.insert(key, self.field_names_index);
            self.field_names_index
        };

        self.data.push(SpudTypes::FieldNameId as u8);
        self.data.push(id);

        self
    }

    fn add_value_length(&mut self, value_len: usize) -> &mut Self {
        if u8::try_from(value_len).is_ok() {
            self.data.push(SpudTypes::U8 as u8);
            self.data
                .extend_from_slice(&u8::try_from(value_len).unwrap().to_le_bytes());
        } else if u16::try_from(value_len).is_ok() {
            self.data.push(SpudTypes::U16 as u8);
            self.data
                .extend_from_slice(&u16::try_from(value_len).unwrap().to_le_bytes());
        } else if u32::try_from(value_len).is_ok() {
            self.data.push(SpudTypes::U32 as u8);
            self.data
                .extend_from_slice(&u32::try_from(value_len).unwrap().to_le_bytes());
        } else if u64::try_from(value_len).is_ok() {
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
        self.data.push(u8::from(value));

        self
    }

    #[allow(clippy::needless_pass_by_value)]
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

    #[cfg(feature = "async")]
    /// # Panics
    ///
    /// Will panic if the path is invalid
    pub async fn build_file(&mut self, path_str: &str, file_name: &str) {
        let path_str: String = match check_path(path_str, file_name) {
            Some(path) => path,
            None => process::exit(1),
        };

        let path: &Path = Path::new(&path_str);

        let header: Vec<u8> = initialise_header(&self.field_names, &self.data);

        write(path, header).await.unwrap();
    }

    #[cfg(not(feature = "async"))]
    /// # Panics
    ///
    /// Will panic if the path is invalid
    pub fn build_file(&mut self, path_str: &str, file_name: &str) {
        let path_str: String = match check_path(path_str, file_name) {
            Some(path) => path,
            None => process::exit(1),
        };

        let path: &Path = Path::new(&path_str);

        let header: Vec<u8> = initialise_header(&self.field_names, &self.data);

        fs::write(path, header).unwrap();
    }
}

impl Default for SpudBuilder {
    fn default() -> Self {
        Self::new()
    }
}
