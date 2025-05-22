#![allow(clippy::needless_pass_by_value)]
use spud_type_ext::SpudTypesExt;
use std::{collections::HashMap, path::Path, process};

use crate::functions::{check_path::check_path, initialise_header::initialise_header};

#[cfg(feature = "async")]
use tokio::fs::write;

#[cfg(not(feature = "async"))]
use std::fs;

use crate::spud_types::SpudTypes;

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

    pub fn add_binary_blob(&mut self, field_name: &str, value: &[u8]) -> &mut Self {
        self.add_field_name(field_name);

        value.write_spud_bytes(&mut self.data);

        self
    }

    pub fn add_value<T: SpudTypesExt>(&mut self, field_name: &str, value: T) -> &mut Self {
        self.add_field_name(field_name);

        value.write_spud_bytes(&mut self.data);

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
