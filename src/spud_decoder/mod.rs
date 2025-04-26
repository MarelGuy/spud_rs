use std::{collections::HashMap, fs};

use crate::SpudTypes;

pub struct SpudDecoder {
    file_contents: Vec<u8>,
    current_byte: u8,
    index: usize,
    field_names: HashMap<String, u8>,
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
            current_byte: 0,
            field_names,
        }
    }

    #[must_use]
    /// # Panics
    ///
    /// Will panic if the path is invalid
    pub fn new_from_path(path: &str) -> Self {
        let file: Vec<u8> = fs::read(path).unwrap();

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

        print!("\"{field_name}\": ");

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
            _ => unreachable!(), // Should be caught by the first match
        }
    }

    /// # Panics
    ///
    /// Will panic on unknown type
    pub fn decode(&mut self) {
        let result: u8 = self.file_contents[self.index];

        let decode_result: Option<SpudTypes> = SpudTypes::from_u8(result);

        let mut next_steps: usize = 0;

        match decode_result {
            Some(SpudTypes::FieldNameId) => {
                next_steps = self.read_field_name();
            }
            Some(SpudTypes::Null) => {
                print!("null");

                next_steps = 1;
            }
            Some(SpudTypes::Bool) => {
                self.next(1).unwrap();

                match self.file_contents.get(self.index) {
                    Some(0) => print!("false"),
                    Some(1) => print!("true"),
                    _ => panic!("Unknown bool value: {}", self.file_contents[self.index]),
                }

                next_steps = 1;
            }
            Some(SpudTypes::U8) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(1);

                print!("{}", u8::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::U16) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(2);

                print!("{}", u16::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::U32) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(4);

                print!("{}", u32::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::U64) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(8);

                print!("{}", u64::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::I8) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(1);

                print!("{}", i8::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::I16) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(2);

                print!("{}", i16::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::I32) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(4);

                print!("{}", i32::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::I64) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(8);

                print!("{}", i64::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::F32) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(4);

                print!("{}", f32::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::F64) => {
                self.next(1).unwrap();

                let read_bytes: Vec<u8> = self.read_bytes(8);

                print!("{}", f64::from_le_bytes(read_bytes.try_into().unwrap()));
            }
            Some(SpudTypes::String) => {
                let string_len: usize = self.read_variable_length_data();
                print!(
                    "\"{}\"",
                    String::from_utf8(
                        self.file_contents[self.index..self.index + string_len].to_vec()
                    )
                    .unwrap()
                );
                next_steps = string_len;
            }
            Some(SpudTypes::BinaryBlob) => {
                let blob_len: usize = self.read_variable_length_data();
                print!(
                    "{:?}",
                    self.file_contents[self.index..self.index + blob_len].to_vec()
                );
                next_steps = blob_len;
            }
            _ => {
                if self.check_end(0) {
                    return;
                }
                println!("Unknown type: {result}");
                self.next(1).unwrap();
            }
        }

        println!();

        self.next(next_steps).unwrap();

        self.decode();
    }
}
