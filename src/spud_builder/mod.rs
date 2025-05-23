#![allow(clippy::needless_pass_by_value)]
use indexmap::IndexMap;
use spud_type_ext::SpudTypesExt;
use std::{collections::HashMap, path::Path, process};

use crate::{
    functions::{check_path::check_path, initialise_header::initialise_header},
    types::object_id::ObjectId,
};

#[cfg(feature = "async")]
use tokio::fs::write;

#[cfg(not(feature = "async"))]
use std::fs;

use crate::spud_types::SpudTypes;

pub mod spud_type_ext;

#[derive(Default, Debug, Clone)]
pub struct SpudBuilder {
    pub data: Vec<u8>,
    pub field_names: IndexMap<(String, u8), u8>,
    pub field_names_index: u8,
}

impl SpudBuilder {
    #[must_use]
    pub fn new() -> Self {
        let id: [u8; 10] = ObjectId::new().0;

        let mut data: Vec<u8> = Vec::new();

        let mut field_names: IndexMap<(String, u8), u8> = IndexMap::new();
        let field_names_index: u8 = 2;

        field_names.insert(("id".into(), 2), 2);

        data.push(SpudTypes::FieldNameId as u8);
        data.push(2);

        data.push(SpudTypes::ObjectId as u8);
        data.extend_from_slice(&id);

        Self {
            data,
            field_names,
            field_names_index,
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

    // pub fn add_object(&mut self, field_name: &str, value: SpudBuilder) -> &mut Self {
    //     self.add_field_name(field_name);

    //     self.data.push(SpudTypes::ObjectStart as u8);

    //     self
    // }

    pub fn add_value<T: SpudTypesExt>(&mut self, field_name: &str, value: T) -> &mut Self {
        self.add_field_name(field_name);

        value.write_spud_bytes(&mut self.data);

        self
    }
}

impl SpudBuilder {
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

#[cfg(feature = "hashmap")]
impl SpudBuilder {
    pub(crate) fn add_hash_value(
        &mut self,
        field_name: &str,
        value: &dyn SpudTypesExt,
    ) -> &mut Self {
        self.add_field_name(field_name);

        value.write_spud_bytes(&mut self.data);

        self
    }
}

#[cfg(feature = "hashmap")]
impl From<HashMap<String, Box<dyn SpudTypesExt>>> for SpudBuilder {
    fn from(map: HashMap<String, Box<dyn SpudTypesExt>>) -> Self {
        let mut builder = SpudBuilder::new();

        for (key, value) in map {
            builder.add_hash_value(&key, &*value);
        }

        builder
    }
}
