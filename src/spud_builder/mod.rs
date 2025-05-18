use spud_add_number::SpudAddNumber;
use std::{collections::HashMap, path::Path, process};

use crate::functions::{check_path::check_path, initialise_header::initialise_header};

#[cfg(feature = "async")]
use tokio::fs::write;

#[cfg(not(feature = "async"))]
use std::fs;

use crate::SpudTypes;

pub mod spud_add_number;
pub mod spud_type_ext;

pub struct SpudBuilder {
    pub data: Vec<u8>,
    pub field_names: HashMap<(String, u8), u8>,
    field_names_index: u8,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SpudTypes;

    #[test]
    fn test_new_spud_builder() {
        let builder = SpudBuilder::new();

        assert!(builder.data.is_empty());
        assert!(builder.field_names.is_empty());
        assert_eq!(builder.field_names_index, 1);
    }

    #[test]
    fn test_add_field_name() {
        let mut builder = SpudBuilder::new();

        builder.add_field_name("test_field");

        assert_eq!(builder.data.len(), 2);
        assert_eq!(builder.data[0], SpudTypes::FieldNameId as u8);
        assert_eq!(builder.data[1], 2);
        assert_eq!(builder.field_names_index, 2);
        assert!(
            builder
                .field_names
                .contains_key(&("test_field".to_string(), 10))
        );

        builder.add_field_name("test_field");

        assert_eq!(builder.data.len(), 4);
        assert_eq!(builder.data[2], SpudTypes::FieldNameId as u8);
        assert_eq!(builder.data[3], 2);
        assert_eq!(builder.field_names_index, 2);
    }

    #[test]
    fn test_add_null() {
        let mut builder = SpudBuilder::new();

        builder.add_null("null_field");

        assert_eq!(builder.data.len(), 3);
        assert_eq!(builder.data[0], SpudTypes::FieldNameId as u8);
        assert_eq!(builder.data[2], SpudTypes::Null as u8);
    }

    #[test]
    fn test_add_bool() {
        let mut builder = SpudBuilder::new();

        builder.add_bool("bool_true_field", true);
        builder.add_bool("bool_false_field", false);

        assert_eq!(builder.data.len(), 8);
        assert_eq!(builder.data[2], SpudTypes::Bool as u8);
        assert_eq!(builder.data[3], 1); // true
        assert_eq!(builder.data[6], SpudTypes::Bool as u8);
        assert_eq!(builder.data[7], 0); // false
    }

    #[test]
    fn test_add_number_u8() {
        let mut builder = SpudBuilder::new();
        let value: u8 = 42;

        builder.add_number("u8_field", value);

        assert_eq!(builder.data.len(), 2 + 1 + 1);
        assert_eq!(builder.data[2], SpudTypes::U8 as u8);
        assert_eq!(builder.data[3], value);
    }

    #[test]
    fn test_add_number_i32() {
        let mut builder = SpudBuilder::new();

        let value: i32 = -1000;

        builder.add_number("i32_field", value);

        assert_eq!(builder.data.len(), 2 + 1 + 4);
        assert_eq!(builder.data[2], SpudTypes::I32 as u8);
        assert_eq!(&builder.data[3..7], &value.to_le_bytes());
    }

    #[test]
    fn test_add_string() {
        let mut builder = SpudBuilder::new();
        let value = "hello";

        builder.add_string("string_field", value);

        assert_eq!(builder.data.len(), 2 + 1 + 1 + 1 + value.len());
        assert_eq!(builder.data[2], SpudTypes::String as u8);
        assert_eq!(builder.data[3], SpudTypes::U8 as u8);
        assert_eq!(builder.data[4], value.len() as u8);
        assert_eq!(&builder.data[5..5 + value.len()], value.as_bytes());
    }

    #[test]
    fn test_add_binary_blob() {
        let mut builder = SpudBuilder::new();

        let value: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05];

        builder.add_binary_blob("blob_field", value);

        assert_eq!(builder.data.len(), 2 + 1 + 1 + 1 + value.len());
        assert_eq!(builder.data[2], SpudTypes::BinaryBlob as u8);
        assert_eq!(builder.data[3], SpudTypes::U8 as u8);
        assert_eq!(builder.data[4], value.len() as u8);
        assert_eq!(&builder.data[5..5 + value.len()], value);
    }

    #[test]
    fn test_add_value_length() {
        let mut builder = SpudBuilder::new();

        builder.data.clear();
        builder.add_value_length(u8::MAX as usize);

        assert_eq!(builder.data[0], SpudTypes::U8 as u8);
        assert_eq!(builder.data[1], u8::MAX);

        builder.data.clear();

        let len_u16 = (u8::MAX as usize) + 1;

        builder.add_value_length(len_u16);

        assert_eq!(builder.data[0], SpudTypes::U16 as u8);
        assert_eq!(&builder.data[1..3], &(len_u16 as u16).to_le_bytes());

        builder.data.clear();

        let len_u32 = (u16::MAX as usize) + 1;

        builder.add_value_length(len_u32);

        assert_eq!(builder.data[0], SpudTypes::U32 as u8);
        assert_eq!(&builder.data[1..5], &(len_u32 as u32).to_le_bytes());

        builder.data.clear();

        let len_u64 = (u32::MAX as usize) + 1;

        builder.add_value_length(len_u64);

        assert_eq!(builder.data[0], SpudTypes::U64 as u8);
        assert_eq!(&builder.data[1..9], &(len_u64 as u64).to_le_bytes());
    }

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_build_file_sync() {
        let mut builder = SpudBuilder::new();

        builder.add_string("greeting", "hello spud");

        let dir = tempfile::tempdir().unwrap();
        let path_str = dir.path().to_str().unwrap();

        builder.build_file(path_str, "test_output_sync");

        let mut expected_file_path = dir.path().to_path_buf();

        expected_file_path.push("test_output_sync.spud");

        assert!(expected_file_path.exists());

        dir.close().unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_build_file_async() {
        let mut builder = SpudBuilder::new();

        builder.add_string("greeting_async", "hello async spud");

        let dir = tempfile::tempdir().unwrap();
        let path_str = dir.path().to_str().unwrap();

        builder.build_file(path_str, "test_output_async").await;

        let mut expected_file_path = dir.path().to_path_buf();

        expected_file_path.push("test_output_async.spud");

        assert!(tokio::fs::try_exists(&expected_file_path).await.unwrap());

        dir.close().unwrap();
    }
}
