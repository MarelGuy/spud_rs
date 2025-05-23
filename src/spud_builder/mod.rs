#![allow(clippy::needless_pass_by_value)]

mod spud_type_ext;

pub(crate) use spud_type_ext::SpudTypesExt;

use core::panic;
use indexmap::IndexMap;
use std::{path::Path, process};

use crate::{
    functions::{check_path, generate_u8_id, initialise_header},
    spud_types::SpudTypes,
    types::ObjectId,
};

#[cfg(feature = "async")]
use tokio::fs::write;

#[cfg(not(feature = "async"))]
use std::fs;

#[derive(Default, Debug, Clone)]
pub struct SpudBuilder {
    oid: ObjectId,
    data: Vec<u8>,
    field_names: IndexMap<(String, u8), u8>,
    seen_ids: Vec<bool>,
    assigned_objects: Vec<ObjectId>,
}

impl SpudBuilder {
    #[must_use]
    pub fn new() -> Self {
        let mut data: Vec<u8> = Vec::new();
        let mut field_names: IndexMap<(String, u8), u8> = IndexMap::new();
        let mut seen_ids: Vec<bool> = vec![false; 256];

        let oid: ObjectId = Self::generate_oid(&mut seen_ids, &mut field_names, &mut data);

        Self {
            oid,
            data,
            field_names,
            seen_ids,
            assigned_objects: Vec::new(),
        }
    }

    #[must_use]
    pub fn as_inner_object_for(main_object: &mut SpudBuilder) -> Self {
        let mut data: Vec<u8> = vec![SpudTypes::ObjectStart as u8];
        let mut field_names: IndexMap<(String, u8), u8> = main_object.field_names.clone();
        let mut seen_ids: Vec<bool> = main_object.seen_ids.clone();

        let oid: ObjectId = Self::generate_oid(&mut seen_ids, &mut field_names, &mut data);

        main_object.assigned_objects.push(oid.clone());

        Self {
            oid,
            data,
            field_names,
            seen_ids,
            assigned_objects: Vec::new(),
        }
    }

    /// # Panics
    ///
    /// Will panic if the object was not created from the object
    pub fn add_object(&mut self, field_name: &str, object: &mut SpudBuilder) {
        if !self.assigned_objects.contains(&object.oid) {
            tracing::error!(
                "You didn't create the object with OID \"{}\" from the object with the OID: \"{}\"",
                object.oid,
                self.oid
            );
            panic!("Closing...");
        }

        self.add_field_name(field_name);

        object.data.push(SpudTypes::ObjectEnd as u8);

        self.data.extend(object.data.clone());

        self.field_names.extend(object.field_names.clone());
        self.seen_ids.extend(object.seen_ids.clone());
    }

    fn add_field_name(&mut self, field_name: &str) -> &mut Self {
        let key: (String, u8) = (field_name.into(), u8::try_from(field_name.len()).unwrap());

        let id: u8 = if let Some(value) = self.field_names.get(&key) {
            *value
        } else {
            let id: u8 = generate_u8_id(&mut self.seen_ids);

            self.field_names.insert(key, id);
            id
        };

        self.data.push(SpudTypes::FieldNameId as u8);
        self.data.push(id);

        self
    }

    fn generate_oid(
        seen_ids: &mut Vec<bool>,
        field_names: &mut IndexMap<(String, u8), u8>,
        data: &mut Vec<u8>,
    ) -> ObjectId {
        let oid: ObjectId = ObjectId::new();

        let key: (String, u8) = ("id".into(), 2);

        let id: u8 = if let Some(value) = field_names.get(&key) {
            *value
        } else {
            let id: u8 = generate_u8_id(seen_ids);

            field_names.insert(key, id);

            id
        };

        data.push(SpudTypes::FieldNameId as u8);
        data.push(id);

        data.push(SpudTypes::ObjectId as u8);
        data.extend_from_slice(&oid.0);

        oid
    }

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
