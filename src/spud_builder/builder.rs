#![allow(clippy::needless_pass_by_value)]

use core::cell::RefCell;
use indexmap::IndexMap;
use std::{fmt, path::Path, rc::Rc};

use crate::{
    SpudError,
    functions::{check_path, initialise_header},
    types::ObjectId,
};

#[cfg(feature = "async")]
use tokio::fs::write;

#[cfg(not(feature = "async"))]
use std::fs;

use super::SpudObject;

#[derive(Default, Clone)]
pub(crate) struct ObjectMap(pub(crate) IndexMap<ObjectId, Rc<RefCell<SpudObject>>>);

/// Represents a builder for creating SPUD objects.
///
/// This builder allows you to create and manage SPUD objects, encode them into a byte vector, and write them to a file.
///
/// # Example
/// ```rust
/// use spud::SpudBuilder;
/// ```
#[derive(Default, Clone)]
pub struct SpudBuilder {
    pub(crate) field_names: Rc<RefCell<IndexMap<(String, u8), u8>>>,
    pub(crate) data: Rc<RefCell<Vec<u8>>>,
    pub(crate) objects: Rc<RefCell<ObjectMap>>,
    pub(crate) seen_ids: Rc<RefCell<Vec<bool>>>,
}

impl SpudBuilder {
    #[must_use]
    /// Creates a new `SpudBuilder` instance.
    ///
    /// # Examples
    /// ```rust
    /// use spud::SpudBuilder;
    /// let builder = SpudBuilder::new();
    /// ```
    ///
    /// # Returns
    /// A new instance of `SpudBuilder`.
    pub fn new() -> Self {
        let mut seen_ids: Vec<bool> = vec![false; 256];

        seen_ids[0] = true;
        seen_ids[1] = true;

        Self {
            data: Rc::new(RefCell::new(Vec::new())),
            field_names: Rc::new(RefCell::new(IndexMap::new())),
            objects: Rc::new(RefCell::new(ObjectMap(IndexMap::new()))),
            seen_ids: Rc::new(RefCell::new(seen_ids)),
        }
    }

    /// Creates a new `SpudObject` instance associated with this builder.
    /// # Examples
    ///
    /// ```rust
    /// use spud::SpudBuilder;
    ///
    /// let builder = SpudBuilder::new();
    ///
    /// builder.object(|obj| {
    ///     OK(())
    /// });
    /// ```
    ///
    /// # Returns
    /// A new instance of `SpudObject` that is linked to the builder's field names, seen IDs, and objects.
    ///
    /// # Errors
    ///
    /// Returns an error if the object cannot be created, typically due to internal issues with the builder's state.
    ///
    /// # Note
    /// The `SpudObject` created by this method will share the same field names, seen IDs, and objects as the builder, allowing for consistent data management.
    /// Nothing is cloned, SPUD uses `Rc` and `RefCell` to manage shared ownership and mutability.
    pub fn object<F>(&self, f: F) -> Result<(), SpudError>
    where
        F: FnOnce(&SpudObject) -> Result<(), SpudError>,
    {
        let obj = self.new_object()?;
        f(&obj.borrow())?;
        Ok(())
    }

    fn new_object(&self) -> Result<Rc<RefCell<SpudObject>>, SpudError> {
        SpudObject::new(
            Rc::clone(&self.field_names),
            Rc::clone(&self.seen_ids),
            Rc::clone(&self.objects),
            Rc::clone(&self.data),
        )
    }

    /// Encodes all objects associated with this builder into a byte vector.
    ///
    /// # Examples
    /// ```rust
    /// use spud::SpudBuilder;
    ///
    /// let builder = SpudBuilder::new();
    ///
    /// builder.object(|obj| {
    ///     OK(())
    /// });
    ///
    /// let encoded_data = builder.encode();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if any of the objects cannot be encoded, typically due to issues with the data format or internal state.
    pub fn encode(&self) -> Result<Vec<u8>, SpudError> {
        for object in self.objects.borrow().0.values() {
            object.borrow().encode()?;
        }

        Ok(self.data.borrow().clone())
    }
}

impl fmt::Debug for SpudBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_builder: fmt::DebugStruct<'_, '_> = f.debug_struct("SpudBuilder");

        debug_builder.field("field_names", &self.field_names.borrow());
        debug_builder.field("data", &self.data.borrow());
        debug_builder.field("objects", &self.objects.borrow());

        let mut seen_ids_to_display = IndexMap::new();

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
    /// Builds the SPUD file at the specified path with the given file name.
    ///  # Arguments
    ///
    /// * `path_str` - The path to the directory where the file will be created.
    /// * `file_name` - The name of the file to create.
    ///
    /// # Errors
    ///
    /// Returns an error if the path is invalid
    pub async fn build_file(&mut self, path_str: &str, file_name: &str) -> Result<(), SpudError> {
        let path_str: String = check_path(path_str, file_name)?;

        let path: &Path = Path::new(&path_str);

        let header: Vec<u8> = initialise_header(&self.field_names.borrow(), &self.data.borrow());

        write(path, header).await?;

        Ok(())
    }

    #[cfg(not(feature = "async"))]
    /// Builds the SPUD file at the specified path with the given file name.
    ///  # Arguments
    ///
    /// * `path_str` - The path to the directory where the file will be created.
    /// * `file_name` - The name of the file to create.
    ///
    /// # Panics
    ///
    /// Will panic if the path is invalid
    ///
    /// # Errors
    ///
    /// Returns an error if the path is invalid
    ///
    /// # Notes
    ///
    /// There is an async version of this function available if the `async` feature is enabled.
    pub fn build_file(&mut self, path_str: &str, file_name: &str) -> Result<(), SpudError> {
        let path_str: String = check_path(path_str, file_name)?;

        let path: &Path = Path::new(&path_str);

        let header: Vec<u8> = initialise_header(&self.field_names.borrow(), &self.data.borrow());

        fs::write(path, header)?;

        Ok(())
    }
}
