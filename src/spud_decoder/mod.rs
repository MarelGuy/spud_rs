#![allow(clippy::too_many_lines)]
use std::{
    collections::HashMap,
    io::{self},
    path::Path,
};

use serde_json::{Number, Value};

#[cfg(feature = "async")]
use tokio::{
    fs::{File as TokioFile, read as tokio_read},
    io::AsyncWriteExt,
};

#[cfg(not(feature = "async"))]
use std::{
    fs::{File as StdFile, read as std_read},
    io::Write,
};

use crate::spud_types::SpudTypes;

pub struct SpudDecoder {
    file_contents: Vec<u8>,
    index: usize,
    field_names: HashMap<String, u8>,
    output: HashMap<String, Value>,
    current_byte: u8,
    current_field: String,
    output_json: String,
}

impl SpudDecoder {
    #[must_use]
    /// # Panics
    ///
    /// Panics if the file is not a valid spud file
    pub fn new(file: &[u8]) -> Self {
        let spud_version: &str = env!("SPUD_VERSION");

        let spud_version_bytes: Vec<u8> = spud_version.as_bytes().to_vec();
        let spud_version_len: usize = spud_version_bytes.len();

        let (file_version, file_contents): (&[u8], &[u8]) = file.split_at(spud_version_len);

        assert!(file_version == spud_version_bytes, "Invalid spud file");

        let mut file_contents: Vec<u8> = file_contents.to_vec();

        let mut field_names: HashMap<String, u8> = HashMap::new();

        let field_name_list_end_byte_index: Option<usize> = file_contents
            .iter()
            .position(|&x| x == SpudTypes::FieldNameListEnd as u8);

        match field_name_list_end_byte_index {
            Some(index) => {
                let (field_names_bytes, file_content) = file_contents.split_at(index + 1);

                let mut cursor: usize = 0;

                loop {
                    let field_name_length: u8 = field_names_bytes[cursor];

                    cursor += 1;

                    let mut field_name: Vec<u8> = vec![];

                    for i in 0..field_name_length {
                        field_name.push(field_names_bytes[cursor + i as usize]);
                    }

                    cursor += field_name_length as usize;

                    let field_id: u8 = field_names_bytes[cursor];

                    cursor += 1;

                    field_names.insert(String::from_utf8(field_name).unwrap(), field_id);

                    if field_names_bytes[cursor] == SpudTypes::FieldNameListEnd as u8 {
                        break;
                    }
                }

                file_contents = file_content.to_vec();
            }
            None => panic!("Invalid spud file, missing FieldNameListEnd byte."),
        }

        Self {
            file_contents,
            index: 0,
            field_names,
            output: HashMap::new(),
            current_byte: 0,
            current_field: String::new(),
            output_json: String::new(),
        }
    }

    #[cfg(feature = "async")]
    #[must_use]
    /// # Panics
    ///
    /// Will panic if the path is invalid
    pub async fn new_from_path(path: &str) -> Self {
        let file: Vec<u8> = tokio_read(path).await.unwrap();

        Self::new(&file)
    }

    #[cfg(not(feature = "async"))]
    #[must_use]
    /// # Panics
    ///
    /// Will panic if the path is invalid
    pub fn new_from_path(path: &str) -> Self {
        let file: Vec<u8> = std_read(path).unwrap();

        Self::new(&file)
    }

    /// # Panics
    ///
    /// Will panic if the index is out of bounds
    fn next(&mut self, steps: usize) -> Result<(), ()> {
        if self.index + steps >= self.file_contents.len() {
            println!("Index out of bounds, decoding failed.");

            return Err(());
        }

        self.index += steps;

        self.current_byte = self.file_contents[self.index];

        Ok(())
    }

    fn peek(&self, steps: usize) -> Option<u8> {
        if self.index + steps >= self.file_contents.len() {
            None
        } else {
            Some(self.file_contents[self.index + steps])
        }
    }

    fn read_bytes(&mut self, steps: usize) -> Vec<u8> {
        let result: Vec<u8> = self.file_contents[self.index..self.index + steps].to_vec();

        self.next(steps).unwrap();

        result
    }

    fn check_end(&self, buffer: usize) -> bool {
        self.peek(buffer) == Some(0xDE)
            && self.peek(1 + buffer) == Some(0xAD)
            && self.peek(2 + buffer) == Some(0xBE)
            && self.peek(3 + buffer) == Some(0xEF)
    }

    fn read_field_name(&mut self) -> usize {
        self.next(1).unwrap();

        let field_name_id: u8 = self.file_contents[self.index];

        let field_name: String = self
            .field_names
            .iter()
            .find(|x| x.1 == &field_name_id)
            .unwrap()
            .0
            .clone();

        self.current_field = field_name;

        1
    }

    /// # Panics
    ///
    /// Will panic on unknown token
    fn read_variable_length_data(&mut self) -> usize {
        self.next(1).unwrap();

        let read_byte_value: usize = match self.current_byte {
            val if val == SpudTypes::U8 as u8 => 1,
            val if val == SpudTypes::U16 as u8 => 2,
            val if val == SpudTypes::U32 as u8 => 4,
            val if val == SpudTypes::U64 as u8 => 8,
            _ => panic!("Expected: U8, U16, U32, U64, but got an unknown token"),
        };

        self.next(1).unwrap();

        let read_bytes: Vec<u8> = self.read_bytes(read_byte_value);

        match read_byte_value {
            1 => u8::from_le_bytes(read_bytes.try_into().unwrap()) as usize,
            2 => u16::from_le_bytes(read_bytes.try_into().unwrap()) as usize,
            4 => u32::from_le_bytes(read_bytes.try_into().unwrap()) as usize,
            8 => usize::try_from(u64::from_le_bytes(read_bytes.try_into().unwrap())).unwrap(),
            _ => unreachable!(),
        }
    }

    fn insert_value(&mut self, value: Value) {
        self.output.insert(self.current_field.clone(), value);
    }

    /// # Panics
    ///
    /// Panics if serde fails to serialize the file
    pub fn decode(&mut self, pretty: bool) -> &str {
        let bit: Option<Value> = self.decode_bit(self.file_contents[self.index]);

        if let Some(value) = bit {
            self.insert_value(value);
        }

        if self.check_end(0) {
            let output_json: Result<String, serde_json::Error> = if pretty {
                serde_json::to_string_pretty(&self.output)
            } else {
                serde_json::to_string(&self.output)
            };

            let output: String = match output_json {
                Ok(output) => output,

                Err(err) => {
                    tracing::error!("Error decoding output: {}", err);

                    panic!("Closing...");
                }
            };

            self.output_json = output;

            return &self.output_json;
        }

        self.decode(pretty);

        &self.output_json
    }

    /// # Panics
    ///
    /// Will panic on unknown type
    fn decode_bit(&mut self, bit: u8) -> Option<Value> {
        let decode_result: Option<SpudTypes> = SpudTypes::from_u8(bit);

        let mut next_steps: usize = 0;

        if decode_result == Some(SpudTypes::FieldNameId) {
            next_steps = self.read_field_name();

            self.next(next_steps).unwrap();

            None
        } else {
            let return_value: Value = match decode_result {
                Some(SpudTypes::Null) => {
                    next_steps = 1;

                    Value::Null
                }
                Some(SpudTypes::Bool) => {
                    self.next(1).unwrap();

                    let value: Value = match self.file_contents.get(self.index) {
                        Some(0) => Value::Bool(false),
                        Some(1) => Value::Bool(true),
                        _ => panic!("Unknown bool value: {}", self.file_contents[self.index]),
                    };

                    next_steps = 1;

                    value
                }
                Some(SpudTypes::U8) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(1);

                    Value::Number(Number::from(u8::from_le_bytes(
                        read_bytes.try_into().unwrap(),
                    )))
                }
                Some(SpudTypes::U16) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(2);

                    Value::Number(Number::from(u16::from_le_bytes(
                        read_bytes.try_into().unwrap(),
                    )))
                }
                Some(SpudTypes::U32) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(4);

                    Value::Number(Number::from(u32::from_le_bytes(
                        read_bytes.try_into().unwrap(),
                    )))
                }
                Some(SpudTypes::U64) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(8);

                    Value::Number(Number::from(u64::from_le_bytes(
                        read_bytes.try_into().unwrap(),
                    )))
                }
                Some(SpudTypes::I8) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(1);

                    Value::Number(Number::from(i8::from_le_bytes(
                        read_bytes.try_into().unwrap(),
                    )))
                }
                Some(SpudTypes::I16) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(2);

                    Value::Number(Number::from(i16::from_le_bytes(
                        read_bytes.try_into().unwrap(),
                    )))
                }
                Some(SpudTypes::I32) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(4);

                    Value::Number(Number::from(i32::from_le_bytes(
                        read_bytes.try_into().unwrap(),
                    )))
                }
                Some(SpudTypes::I64) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(8);

                    Value::Number(Number::from(i64::from_le_bytes(
                        read_bytes.try_into().unwrap(),
                    )))
                }
                Some(SpudTypes::F32) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(4);

                    Value::Number(
                        Number::from_f64(f32::from_le_bytes(read_bytes.try_into().unwrap()).into())
                            .unwrap(),
                    )
                }
                Some(SpudTypes::F64) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(8);

                    Value::Number(
                        Number::from_f64(f64::from_le_bytes(read_bytes.try_into().unwrap()))
                            .unwrap(),
                    )
                }
                Some(SpudTypes::String) => {
                    let string_len: usize = self.read_variable_length_data();

                    next_steps = string_len;

                    Value::String(
                        String::from_utf8(
                            self.file_contents[self.index..self.index + string_len].to_vec(),
                        )
                        .unwrap(),
                    )
                }
                Some(SpudTypes::BinaryBlob) => {
                    let blob_len: usize = self.read_variable_length_data();

                    let processed: Vec<u8> =
                        self.file_contents[self.index..self.index + blob_len].to_vec();

                    let mut output_array: Vec<Value> = vec![];

                    for processed_bit in &processed {
                        output_array.push(Value::Number(Number::from(*processed_bit)));
                    }

                    next_steps = blob_len;

                    Value::Array(output_array)
                }
                _ => {
                    tracing::error!("Unknown type: {bit} at index {}", self.index);

                    panic!("Closing...");
                }
            };

            self.next(next_steps).unwrap();

            Some(return_value)
        }
    }

    #[cfg(feature = "async")]
    /// # Panics
    ///
    /// Panics if the file has errors being written
    pub async fn build_file(&self, path: &str) {
        let path: &Path = Path::new(path);

        let file: Result<TokioFile, io::Error> = TokioFile::create(path).await;

        let res: Result<(), io::Error> = match file {
            Ok(mut file) => file.write_all(self.output_json.as_bytes()).await,
            Err(err) => {
                tracing::error!("Error creating file: {}", err);
                panic!("Closing...")
            }
        };

        match res {
            Ok(()) => {}
            Err(err) => {
                tracing::error!("Error writing file: {}", err);
                panic!("Closing...")
            }
        }
    }

    #[cfg(not(feature = "async"))]
    /// # Panics
    ///
    /// Panics if the file has errors being written
    pub fn build_file(&self, path: &str) {
        let path: &Path = Path::new(path);

        let file: Result<StdFile, io::Error> = StdFile::create(path);

        let res: Result<(), io::Error> = match file {
            Ok(mut file) => file.write_all(self.output_json.as_bytes()),
            Err(err) => {
                tracing::error!("Error creating file: {}", err);
                panic!("Closing...")
            }
        };

        match res {
            Ok(()) => {}
            Err(err) => {
                tracing::error!("Error writing file: {}", err);
                panic!("Closing...")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spud_builder::SpudBuilder;
    use crate::spud_types::SpudTypes;
    use serde_json::json;

    fn create_test_spud_file_bytes(add_data_fn: impl FnOnce(&mut SpudBuilder)) -> Vec<u8> {
        let mut builder = SpudBuilder::new();

        add_data_fn(&mut builder);

        let mut file_bytes: Vec<u8> = env!("SPUD_VERSION").as_bytes().to_vec();

        for (name_tuple, id) in &builder.field_names {
            file_bytes.push(name_tuple.1);
            file_bytes.extend_from_slice(name_tuple.0.as_bytes());
            file_bytes.push(*id);
        }

        file_bytes.push(SpudTypes::FieldNameListEnd as u8);
        file_bytes.extend_from_slice(&builder.data);
        file_bytes.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);

        file_bytes
    }

    #[test]
    fn test_decoder_new_valid_header() {
        let test_data = create_test_spud_file_bytes(|builder| {
            builder.add_null("field1");
        });

        let decoder = SpudDecoder::new(&test_data);

        assert_eq!(decoder.field_names.len(), 1);
        assert!(decoder.field_names.contains_key("field1"));
    }

    #[test]
    #[should_panic(expected = "Invalid spud file")]
    fn test_decoder_new_invalid_version() {
        let mut test_data = b"invalid_version_".to_vec();

        test_data.extend(create_test_spud_file_bytes(|builder| {
            builder.add_null("field1");
        }));

        let spud_version_len = env!("SPUD_VERSION").len();
        let _ = SpudDecoder::new(&test_data[spud_version_len..]);
    }

    #[test]
    #[should_panic(expected = "Invalid spud file, missing FieldNameListEnd byte.")]
    fn test_decoder_new_missing_field_name_list_end() {
        let mut file_bytes: Vec<u8> = env!("SPUD_VERSION").as_bytes().to_vec();

        let field_name = "test_field";

        file_bytes.push(field_name.len() as u8);
        file_bytes.extend_from_slice(field_name.as_bytes());
        file_bytes.push(2);

        file_bytes.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);

        let _ = SpudDecoder::new(&file_bytes);
    }

    #[test]
    fn test_decode_simple_null() {
        let test_data = create_test_spud_file_bytes(|builder| {
            builder.add_null("my_null");
        });

        let mut decoder = SpudDecoder::new(&test_data);

        let json_output = decoder.decode(false);
        let expected_json = json!({ "my_null": null }).to_string();

        assert_eq!(json_output, expected_json);
    }

    #[test]
    fn test_decode_simple_bool() {
        let test_data = create_test_spud_file_bytes(|builder| {
            builder.add_bool("my_true", true);
            builder.add_bool("my_false", false);
        });

        let mut decoder = SpudDecoder::new(&test_data);

        let json_output = decoder.decode(false);
        let expected_json_val: Value = serde_json::from_str(json_output).unwrap();

        let mut expected_map = serde_json::Map::new();

        expected_map.insert("my_true".to_string(), Value::Bool(true));
        expected_map.insert("my_false".to_string(), Value::Bool(false));

        assert_eq!(expected_json_val, Value::Object(expected_map));
    }

    #[test]
    fn test_decode_numbers() {
        let test_data = create_test_spud_file_bytes(|builder| {
            builder.add_number("my_u8", 123u8);
            builder.add_number("my_i32", -5000i32);
            builder.add_number("my_f64", 123.456f64);
        });

        let mut decoder = SpudDecoder::new(&test_data);

        let json_output = decoder.decode(false);
        let expected_json_val: Value = serde_json::from_str(json_output).unwrap();

        let mut expected_map = serde_json::Map::new();

        expected_map.insert("my_u8".to_string(), json!(123u8));
        expected_map.insert("my_i32".to_string(), json!(-5000i32));
        expected_map.insert("my_f64".to_string(), json!(123.456f64));

        assert_eq!(
            expected_json_val.as_object().unwrap().get("my_u8").unwrap(),
            &json!(123u8)
        );
        assert_eq!(
            expected_json_val
                .as_object()
                .unwrap()
                .get("my_i32")
                .unwrap(),
            &json!(-5000i32)
        );

        assert_eq!(
            expected_json_val
                .as_object()
                .unwrap()
                .get("my_f64")
                .unwrap()
                .as_f64()
                .unwrap(),
            123.456f64
        );
    }

    #[test]
    fn test_decode_string() {
        let test_data = create_test_spud_file_bytes(|builder| {
            builder.add_string("my_string", "hello spud decoder");
        });

        let mut decoder = SpudDecoder::new(&test_data);

        let json_output = decoder.decode(false);
        let expected_json = json!({ "my_string": "hello spud decoder" }).to_string();

        assert_eq!(json_output, expected_json);
    }

    #[test]
    fn test_decode_binary_blob() {
        let blob_data: Vec<u8> = vec![10, 20, 30, 40, 50];

        let test_data = create_test_spud_file_bytes(|builder| {
            builder.add_binary_blob("my_blob", &blob_data);
        });

        let mut decoder = SpudDecoder::new(&test_data);
        let json_output = decoder.decode(false);

        let expected_value = json!({ "my_blob": [10, 20, 30, 40, 50]});
        let output_value: Value = serde_json::from_str(json_output).unwrap();

        assert_eq!(output_value, expected_value);
    }

    #[test]
    fn test_decode_multiple_fields() {
        let test_data = create_test_spud_file_bytes(|builder| {
            builder.add_string("name", "Spuddy");
            builder.add_number("age", 3u8);
            builder.add_bool("is_cool", true);
        });

        let mut decoder = SpudDecoder::new(&test_data);
        let json_output = decoder.decode(true);

        let expected_json_val: Value = serde_json::from_str(json_output).unwrap();

        let mut expected_map = serde_json::Map::new();

        expected_map.insert("name".to_string(), json!("Spuddy"));
        expected_map.insert("age".to_string(), json!(3u8));
        expected_map.insert("is_cool".to_string(), json!(true));

        assert_eq!(expected_json_val, Value::Object(expected_map));
    }

    #[cfg(not(feature = "async"))]
    #[test]
    fn test_new_from_path_sync() {
        let dir = tempfile::tempdir().unwrap();

        let file_path = dir.path().join("test_sync.spud");

        let test_data_bytes = create_test_spud_file_bytes(|builder| {
            builder.add_string("message", "sync load test");
        });

        std::fs::write(&file_path, test_data_bytes).unwrap();

        let mut decoder = SpudDecoder::new_from_path(file_path.to_str().unwrap());
        let json_output = decoder.decode(false);
        let expected_json = json!({ "message": "sync load test" }).to_string();

        assert_eq!(json_output, expected_json);

        dir.close().unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_new_from_path_async() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("test_async.spud");

        let test_data_bytes = create_test_spud_file_bytes(|builder| {
            builder.add_string("message_async", "async load test");
        });

        tokio::fs::write(&file_path, test_data_bytes).await.unwrap();

        let mut decoder = SpudDecoder::new_from_path(file_path.to_str().unwrap()).await;

        let json_output = decoder.decode(false);
        let expected_json = json!({ "message_async": "async load test" }).to_string();

        assert_eq!(json_output, expected_json);

        dir.close().unwrap();
    }
}
