use std::path::Path;

use indexmap::IndexMap;
use serde_json::Value;

#[cfg(feature = "async")]
use tokio::{
    fs::{File as TokioFile, read as tokio_read},
    io::AsyncWriteExt,
};

#[cfg(feature = "sync")]
use std::{
    fs::{File as StdFile, read as std_read},
    io::Write,
};

use crate::{SPUD_VERSION, SpudError, spud_decoder::DecoderObject, spud_types::SpudTypes};

/// The `SpudDecoder` is responsible for decoding SPUD files into a JSON format.
#[derive(Default, Debug, Clone)]
pub struct SpudDecoder {
    file_contents: Vec<u8>,
    field_names: IndexMap<u8, String>,
    output_json: String,
}

impl SpudDecoder {
    /// # Errors
    ///
    /// Returns an error if the file is not a valid spud file
    ///
    /// # Panics
    ///
    /// Panics if the SPUD version environment variable is not set or if the file is invalid.
    pub fn new(file: &[u8]) -> Result<Self, SpudError> {
        let spud_version_bytes: Vec<u8> = SPUD_VERSION.as_bytes().to_vec();
        let spud_version_len: usize = spud_version_bytes.len();

        let (file_version, file_contents): (&[u8], &[u8]) = file.split_at(spud_version_len);

        if file_version != spud_version_bytes {
            return Err(SpudError::DecodingError(
                "Invalid SPUD file: version mismatch".to_owned(),
            ));
        }

        let mut file_contents: Vec<u8> = file_contents.to_vec();

        let mut field_names: IndexMap<u8, String> = IndexMap::new();

        let field_name_list_end_byte_index: Option<usize> = file_contents
            .iter()
            .position(|&x| x == SpudTypes::FieldNameListEnd.as_u8());

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

                    let decoded_field_name: String = String::from_utf8(field_name)?;

                    field_names.insert(field_id, decoded_field_name);

                    if field_names_bytes[cursor] == SpudTypes::FieldNameListEnd.as_u8() {
                        break;
                    }
                }

                file_contents = file_content.to_vec();
            }
            None => Err(SpudError::DecodingError(
                "Invalid SPUD file: missing field name list end byte".to_owned(),
            ))?,
        }

        Ok(Self {
            file_contents,
            field_names,
            output_json: String::new(),
        })
    }

    /// Decodes the SPUD file contents into a JSON string.
    /// # Arguments
    ///
    /// * `pretty` - Whether to format the JSON output with indentation.
    /// * `want_array` - Whether to wrap the output in an array, useless if the decoder finds more than one object.
    /// # Errors
    ///
    /// Returns an error if serde fails to serialize the file
    pub fn decode(&mut self, pretty: bool, want_array: bool) -> Result<&str, SpudError> {
        let objects: Vec<IndexMap<String, Value>> = self.decode_objects()?;

        let output_json: Result<String, serde_json::Error> = if objects.len() == 1 && !want_array {
            let single_object: &IndexMap<String, Value> = &objects[0];

            if pretty {
                serde_json::to_string_pretty(single_object)
            } else {
                serde_json::to_string(single_object)
            }
        } else if pretty {
            serde_json::to_string_pretty(&objects)
        } else {
            serde_json::to_string(&objects)
        };

        match output_json {
            Ok(json) => {
                self.output_json = json;
            }
            Err(err) => {
                Err(SpudError::DecodingError(format!(
                    "Failed to serialize JSON: {err}"
                )))?;
            }
        }

        Ok(self.output_json.as_str())
    }

    fn decode_objects(&mut self) -> Result<Vec<IndexMap<String, Value>>, SpudError> {
        let mut decoded_objects: Vec<IndexMap<String, Value>> = Vec::new();
        let mut i: usize = 0;

        while i < self.file_contents.len() {
            if self.file_contents.get(i) == Some(&SpudTypes::ObjectStart.as_u8())
                && self.file_contents.get(i + 1) == Some(&SpudTypes::ObjectStart.as_u8())
            {
                let start: usize = i;

                let mut depth: i32 = 0;
                let mut end: usize = 0;
                let mut j: usize = i;

                while let Some(&byte) = self.file_contents.get(j) {
                    if byte == SpudTypes::ObjectStart.as_u8()
                        && self.file_contents.get(j + 1) == Some(&SpudTypes::ObjectStart.as_u8())
                    {
                        depth += 1;
                        j += 1;
                    } else if byte == SpudTypes::ObjectEnd.as_u8()
                        && self.file_contents.get(j + 1) == Some(&SpudTypes::ObjectEnd.as_u8())
                    {
                        depth -= 1;
                        j += 1;

                        if depth == 0 {
                            end = j + 1;

                            break;
                        }
                    }

                    j += 1;
                }

                if end > start {
                    let object_bytes: &[u8] = &self.file_contents[start..end];

                    let mut decoder: DecoderObject<'_> =
                        DecoderObject::new(object_bytes, &self.field_names);

                    decoded_objects.push(decoder.decode()?);

                    i = end;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        Ok(decoded_objects)
    }
}

#[cfg(feature = "sync")]
impl SpudDecoder {
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
    /// # Errors
    ///
    /// Will return an error if the path is invalid
    ///
    /// # Notes
    ///
    /// There is an async version of this function available if the `async` feature is enabled.
    pub fn new_from_path(path: &str) -> Result<Self, SpudError> {
        let file: Vec<u8> = std_read(path)?;

        Self::new(&file)
    }

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
    /// # Errors
    ///
    /// Will return an error if the file has errors being written
    ///
    /// # Notes
    ///
    /// There is an async version of this function available if the `async` feature is enabled.
    pub fn build_file(&self, path: &str) -> Result<(), SpudError> {
        StdFile::create(Path::new(path))?.write_all(self.output_json.as_bytes())?;

        Ok(())
    }
}

#[cfg(feature = "async")]
impl SpudDecoder {
    /// Creates a new `SpudDecoder` instance from a file at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file to read.
    ///
    /// # Errors
    ///
    /// Will return an error if the path is invalid
    pub async fn new_from_path_async(path: &str) -> Result<Self, SpudError> {
        let file: Vec<u8> = tokio_read(path).await?;

        Self::new(&file)
    }

    /// Builds a JSON file at the specified path with the given file name.
    ///  # Arguments
    ///
    /// * `path_str` - The path to the directory where the file will be created.
    /// * `file_name` - The name of the file to create.
    ///
    /// # Errors
    ///
    /// Will return an error if the file has errors being written
    pub async fn build_file_async(&self, path: &str) -> Result<(), SpudError> {
        TokioFile::create(Path::new(path))
            .await?
            .write_all(self.output_json.as_bytes())
            .await?;

        Ok(())
    }
}
