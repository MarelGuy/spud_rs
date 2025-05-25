#![allow(clippy::needless_pass_by_value)]

use core::cell::RefCell;
use indexmap::IndexMap;
use std::{collections::HashMap, fmt, path::Path, process, rc::Rc};

use crate::{
    functions::{check_path, initialise_header},
    spud_types::SpudTypes,
    types::ObjectId,
};

#[cfg(feature = "async")]
use tokio::fs::write;

#[cfg(not(feature = "async"))]
use std::fs;

#[cfg(feature = "serde")]
use super::SpudSerializer;

use super::SpudObject;

#[derive(Default, Clone)]
pub(crate) struct ObjectMap(pub(crate) IndexMap<ObjectId, Rc<RefCell<Vec<u8>>>>);

#[derive(Default, Clone)]
pub struct SpudBuilder {
    pub(crate) field_names: Rc<RefCell<IndexMap<(String, u8), u8>>>,
    pub(crate) data: Rc<RefCell<Vec<u8>>>,
    pub(crate) objects: Rc<RefCell<ObjectMap>>,
    pub(crate) seen_ids: Rc<RefCell<Vec<bool>>>,
}

impl SpudBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(Vec::new())),
            field_names: Rc::new(RefCell::new(IndexMap::new())),
            objects: Rc::new(RefCell::new(ObjectMap(IndexMap::new()))),
            seen_ids: Rc::new(RefCell::new(vec![false; 256])),
        }
    }

    #[must_use]
    pub fn new_object(&self) -> SpudObject {
        SpudObject::new(
            Rc::clone(&self.field_names),
            Rc::clone(&self.seen_ids),
            Rc::clone(&self.objects),
        )
    }

    pub fn encode(&mut self) {
        for object in self.objects.borrow().0.values() {
            self.data.borrow_mut().extend_from_slice(&object.borrow());

            self.data
                .borrow_mut()
                .extend_from_slice(&[SpudTypes::ObjectEnd as u8, SpudTypes::ObjectEnd as u8]);
        }
    }
}

impl fmt::Debug for SpudBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_builder: fmt::DebugStruct<'_, '_> = f.debug_struct("SpudBuilder");

        debug_builder.field("field_names", &self.field_names.borrow());
        debug_builder.field("data", &self.data.borrow());
        debug_builder.field("objects", &self.objects.borrow());

        let mut seen_ids_to_display = HashMap::new();

        for (index, &is_seen) in self.seen_ids.borrow().iter().enumerate() {
            if is_seen {
                seen_ids_to_display.insert(index, true);
            }
        }
        debug_builder.field("seen_ids", &seen_ids_to_display);

        debug_builder.finish()
    }
}

impl fmt::Debug for ObjectMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_map: fmt::DebugMap<'_, '_> = f.debug_map();

        for (key, value) in &self.0 {
            debug_map.entry(&key, &value.borrow());
        }

        debug_map.finish()
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

        let header: Vec<u8> = initialise_header(&self.field_names.borrow(), &self.data.borrow());

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

impl SpudBuilder {
    /// # Panics
    ///
    /// Will panic if the serialization fails
    #[cfg(feature = "serde")]
    pub fn serialize<T: serde::ser::Serialize>(&mut self, value: T) -> &mut Self {
        let serializer: SpudSerializer = SpudSerializer::new(self);

        value.serialize(serializer).unwrap();

        self
    }
}
