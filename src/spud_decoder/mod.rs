#![allow(clippy::too_many_lines)]
use std::{
    io::{self},
    path::Path,
};

use indexmap::IndexMap;
use rust_decimal::Decimal;
use serde_json::{Map, Number, Value};

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

/// The `SpudDecoder` is responsible for decoding SPUD files into a JSON format.
#[derive(Default, Debug, Clone)]
pub struct SpudDecoder {
    file_contents: Vec<u8>,
    current_object: Vec<u8>,
    index: usize,
    field_names: IndexMap<u8, String>,
    output: Vec<IndexMap<String, Value>>,
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
        let spud_version: &str = &std::env::var("SPUD_VERSION").unwrap();

        let spud_version_bytes: Vec<u8> = spud_version.as_bytes().to_vec();
        let spud_version_len: usize = spud_version_bytes.len();

        let (file_version, file_contents): (&[u8], &[u8]) = file.split_at(spud_version_len);

        assert!(file_version == spud_version_bytes, "Invalid spud file");

        let mut file_contents: Vec<u8> = file_contents.to_vec();

        let mut field_names: IndexMap<u8, String> = IndexMap::new();

        let field_name_list_end_byte_index: Option<usize> = file_contents
            .iter()
            .position(|&x| x == SpudTypes::FieldNameListEnd as u8);

        match field_name_list_end_byte_index {
            Some(index) => {
                let (field_names_bytes, file_content): (&[u8], &[u8]) =
                    file_contents.split_at(index + 1);

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

                    field_names.insert(field_id, String::from_utf8(field_name).unwrap());

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
            output: Vec::new(),
            current_byte: 0,
            current_field: String::new(),
            current_object: Vec::new(),
            output_json: String::new(),
        }
    }

    /// Decodes the SPUD file contents into a JSON string.
    /// # Arguments
    ///
    /// * `pretty` - Whether to format the JSON output with indentation.
    /// * `want_array` - Whether to wrap the output in an array, useless if the decoder finds more than one object.
    /// # Panics
    ///
    /// Panics if serde fails to serialize the file
    pub fn decode(&mut self, pretty: bool, want_array: bool) -> &str {
        let objects: Vec<Vec<u8>> = self.get_objects();

        for object in objects {
            self.current_object = object;
            self.index = 0;
            self.current_field.clear();

            let decoded_object: IndexMap<String, Value> = self.decode_object();
            self.output.push(decoded_object);
        }

        if self.output.len() == 1 && !want_array {
            let single_object = &self.output[0];
            self.output_json = if pretty {
                serde_json::to_string_pretty(single_object).unwrap()
            } else {
                serde_json::to_string(single_object).unwrap()
            };
        } else {
            self.output_json = if pretty {
                serde_json::to_string_pretty(&self.output).unwrap()
            } else {
                serde_json::to_string(&self.output).unwrap()
            };
        }

        self.output_json.as_str()
    }

    fn decode_object(&mut self) -> IndexMap<String, Value> {
        let mut object: IndexMap<String, Value> = IndexMap::new();

        self.next(2).unwrap();

        let id: Vec<u8> = self.read_bytes(10);

        let object_id: String = bs58::encode(&id).into_string();
        object.insert("oid".to_string(), Value::String(object_id));

        while self.index < self.current_object.len() {
            if self.current_byte == SpudTypes::ObjectEnd as u8 {
                break;
            }

            let field_value: Option<Value> = self.decode_byte(self.current_byte);

            if let Some(value) = field_value {
                object.insert(self.current_field.clone(), value);
            }
        }

        object
    }

    /// # Panics
    ///
    /// Will panic if the index is out of bounds
    fn next(&mut self, steps: usize) -> Result<(), ()> {
        if self.index + steps >= self.current_object.len() {
            tracing::error!("Index out of bounds, decoding failed.");

            return Err(());
        }

        self.index += steps;

        self.current_byte = self.current_object[self.index];

        Ok(())
    }

    fn read_bytes(&mut self, steps: usize) -> Vec<u8> {
        let result: Vec<u8> = self.current_object[self.index..self.index + steps].to_vec();

        self.next(steps).unwrap();

        result
    }

    fn get_objects(&self) -> Vec<Vec<u8>> {
        let mut objects: Vec<Vec<u8>> = vec![];

        let mut current_object: Vec<u8> = vec![];

        let mut old_byte: u8 = 0;

        for byte in &self.file_contents {
            if *byte == SpudTypes::ObjectStart as u8 && old_byte == SpudTypes::ObjectStart as u8 {
                current_object.clear();
            }

            if *byte == SpudTypes::ObjectEnd as u8 && old_byte == SpudTypes::ObjectEnd as u8 {
                current_object.push(*byte);
                objects.push(current_object.clone());
            } else {
                current_object.push(*byte);
            }

            old_byte = *byte;
        }

        objects
    }

    fn read_field_name(&mut self) -> usize {
        self.next(1).unwrap();

        let field_name_id: u8 = self.current_object[self.index];

        self.current_field = self.field_names.get(&field_name_id).cloned().unwrap();

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

    /// # Panics
    ///
    /// Will panic on unknown type
    fn decode_byte(&mut self, byte: u8) -> Option<Value> {
        let decode_result: Option<SpudTypes> = SpudTypes::from_u8(byte);

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

                    let value: Value = match self.current_object.get(self.index) {
                        Some(0) => Value::Bool(false),
                        Some(1) => Value::Bool(true),
                        _ => panic!("Unknown bool value: {}", self.current_object[self.index]),
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
                Some(SpudTypes::Decimal) => {
                    self.next(1).unwrap();

                    let read_bytes: Vec<u8> = self.read_bytes(16);

                    let decimal_value: Decimal =
                        Decimal::deserialize(read_bytes.try_into().unwrap());

                    Value::String(decimal_value.to_string())
                }
                Some(SpudTypes::String) => {
                    let string_len: usize = self.read_variable_length_data();

                    next_steps = string_len;

                    Value::String(
                        String::from_utf8(
                            self.current_object[self.index..self.index + string_len].to_vec(),
                        )
                        .unwrap(),
                    )
                }
                Some(SpudTypes::BinaryBlob) => {
                    let blob_len: usize = self.read_variable_length_data();

                    let processed: Vec<u8> =
                        self.current_object[self.index..self.index + blob_len].to_vec();

                    let mut output_array: Vec<Value> = vec![];

                    for processed_byte in &processed {
                        output_array.push(Value::Number(Number::from(*processed_byte)));
                    }

                    next_steps = blob_len;

                    Value::Array(output_array)
                }
                Some(SpudTypes::ArrayStart) => {
                    self.next(1).unwrap();

                    let mut output_array: Vec<Value> = vec![];

                    loop {
                        let byte: Option<SpudTypes> =
                            SpudTypes::from_u8(self.current_object[self.index]);

                        if byte == Some(SpudTypes::ArrayEnd) {
                            break;
                        }

                        let decoded_byte: Option<Value> =
                            self.decode_byte(self.current_object[self.index]);

                        if let Some(value) = decoded_byte {
                            output_array.push(value);
                        }
                    }

                    next_steps = 1;

                    Value::Array(output_array)
                }
                Some(SpudTypes::ObjectStart) => {
                    self.next(1).unwrap();

                    let mut output_object: Map<String, Value> = Map::new();

                    let parent_field: String = self.current_field.clone();

                    loop {
                        let byte: Option<SpudTypes> =
                            SpudTypes::from_u8(self.current_object[self.index]);

                        if byte == Some(SpudTypes::ObjectEnd) {
                            break;
                        }

                        let decoded_byte: Option<Value> =
                            self.decode_byte(self.current_object[self.index]);

                        if let Some(value) = decoded_byte {
                            output_object.insert(self.current_field.clone(), value);
                        }
                    }

                    next_steps = 1;

                    self.current_field = parent_field;

                    Value::Object(output_object)
                }
                _ => {
                    tracing::error!("Unknown type: {byte} at index {}", self.index);
                    panic!("Closing...");
                }
            };

            self.next(next_steps).unwrap();

            Some(return_value)
        }
    }
}

impl SpudDecoder {
    #[cfg(feature = "async")]
    #[must_use]
    /// Creates a new `SpudDecoder` instance from a file at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to read.
    ///
    /// # Panics
    ///
    /// Will panic if the path is invalid
    pub async fn new_from_path(path: &str) -> Self {
        let file: Vec<u8> = tokio_read(path).await.unwrap();

        Self::new(&file)
    }

    #[cfg(not(feature = "async"))]
    #[must_use]
    /// Creates a new `SpudDecoder` instance from a file at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to read.
    ///
    /// # Panics
    ///
    /// Will panic if the path is invalid
    ///
    /// # Notes
    ///
    /// There is an async version of this function available if the `async` feature is enabled.
    pub fn new_from_path(path: &str) -> Self {
        let file: Vec<u8> = std_read(path).unwrap();

        Self::new(&file)
    }

    #[cfg(feature = "async")]
    /// Builds a JSON file at the specified path with the given file name.
    ///  # Arguments
    ///
    /// * `path_str` - The path to the directory where the file will be created.
    /// * `file_name` - The name of the file to create.
    ///
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
    /// Builds a JSON file at the specified path with the given file name.
    ///  # Arguments
    ///
    /// * `path_str` - The path to the directory where the file will be created.
    /// * `file_name` - The name of the file to create.
    ///
    /// # Panics
    ///
    /// Panics if the file has errors being written
    ///
    /// # Notes
    ///
    /// There is an async version of this function available if the `async` feature is enabled.
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

#[cfg(feature = "serde")]
impl SpudDecoder {
    /// # Errors
    ///
    /// Will return an error if the deserialization fails
    pub fn deserialize<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_str(&self.output_json)
    }
}
