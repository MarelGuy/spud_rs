use indexmap::IndexMap;
use std::{fmt, path::Path, sync::Arc};

use std::sync::Mutex;

use crate::{
    SpudError,
    functions::{check_path, initialise_header_sync},
    spud_types::SpudTypes,
    types::ObjectId,
};

use std::fs;

use super::SpudObjectSync;

#[derive(Default, Clone)]
pub(crate) struct ObjectMap(pub(crate) IndexMap<ObjectId, Arc<Mutex<SpudObjectSync>>>);

/// Represents a builder for creating SPUD objects.
///
/// This builder allows you to create and manage SPUD objects, encode them into a byte vector, and write them to a file.
///
/// # Example
/// ```rust
/// use spud_rs::SpudBuilderSync;
/// ```
///
/// # Notes
///
/// This builder is designed to be used in a synchronous context. There is an asynchronous version available if the `async` feature is enabled.
#[derive(Default, Clone)]
pub struct SpudBuilderSync {
    pub(crate) field_names: Arc<Mutex<IndexMap<(String, u8), u8>>>,
    pub(crate) data: Arc<Mutex<Vec<u8>>>,
    pub(crate) objects: Arc<Mutex<ObjectMap>>,
    pub(crate) seen_ids: Arc<Mutex<Vec<bool>>>,
}

impl SpudBuilderSync {
    #[must_use]
    /// Creates a new `SpudBuilderSync` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spud_rs::SpudBuilderSync;
    ///
    /// let builder = SpudBuilderSync::new();
    /// ```
    ///
    /// # Returns
    ///
    /// A new instance of `SpudBuilderSync`.
    pub fn new() -> Self {
        let mut seen_ids: Vec<bool> = vec![false; 256];

        seen_ids[0] = true;
        seen_ids[1] = true;

        Self {
            field_names: Arc::new(Mutex::new(IndexMap::new())),
            data: Arc::new(Mutex::new(Vec::new())),
            objects: Arc::new(Mutex::new(ObjectMap(IndexMap::new()))),
            seen_ids: Arc::new(Mutex::new(seen_ids)),
        }
    }

    /// Creates a new `SpudObjectSync` instance associated with this builder.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a reference to the `SpudObjectSync` and returns a `Result<(), SpudError>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spud_rs::SpudBuilderSync;
    ///
    /// let builder = SpudBuilderSync::new();
    ///
    /// builder.object(|obj| {
    ///     Ok(())
    /// });
    /// ```
    ///
    /// # Returns
    ///
    /// A new instance of `SpudObjectSync` that is linked to the builder's field names, seen IDs, and objects.
    ///
    /// # Errors
    ///
    /// Returns an error if the object cannot be created, typically due to internal issues with the builder's state.
    ///
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    ///
    /// # Note
    ///
    /// The `SpudObjectSync` created by this method will share the same field names, seen IDs, and objects as the builder.
    pub fn object<F>(&self, f: F) -> Result<(), SpudError>
    where
        F: FnOnce(&SpudObjectSync) -> Result<(), SpudError>,
    {
        let obj: Arc<Mutex<SpudObjectSync>> = self.new_object()?;

        f(&obj.lock().unwrap())?;

        self.data.lock().unwrap().push(SpudTypes::ObjectEnd.as_u8());
        self.data.lock().unwrap().push(SpudTypes::ObjectEnd.as_u8());

        Ok(())
    }

    fn new_object(&self) -> Result<Arc<Mutex<SpudObjectSync>>, SpudError> {
        SpudObjectSync::new(
            Arc::clone(&self.field_names),
            Arc::clone(&self.seen_ids),
            Arc::clone(&self.objects),
            Arc::clone(&self.data),
        )
    }

    /// Encodes all objects associated with this builder into a byte vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spud_rs::SpudBuilderSync;
    ///
    /// let builder = SpudBuilderSync::new();
    ///
    /// builder.object(|obj| {
    ///     Ok(())
    /// });
    ///
    /// let encoded_data = builder.encode().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if any of the objects cannot be encoded, typically due to issues with the data format or internal state.
    ///
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    pub fn encode(&self) -> Result<Vec<u8>, SpudError> {
        for object in self.objects.lock().unwrap().0.values() {
            object.lock().unwrap().encode()?;
        }

        let header: Vec<u8> = initialise_header_sync(
            &self.field_names.lock().unwrap(),
            &self.data.lock().unwrap(),
        );

        self.data.lock().unwrap().clear();
        self.data.lock().unwrap().extend_from_slice(&header);

        Ok(header)
    }

    /// Builds the SPUD file at the specified path with the given file name.
    ///
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

        fs::write(path, self.data.lock().unwrap().clone())?;

        Ok(())
    }
}

impl fmt::Debug for SpudBuilderSync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_builder: fmt::DebugStruct<'_, '_> = f.debug_struct("SpudBuilderSync");

        debug_builder.field("field_names", &self.field_names.lock().unwrap());
        debug_builder.field("data", &self.data.lock().unwrap());
        debug_builder.field("objects", &self.objects.lock().unwrap());

        let mut seen_ids_to_display: IndexMap<usize, bool> = IndexMap::new();

        for (index, &is_seen) in self.seen_ids.lock().unwrap().iter().enumerate() {
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
            debug_map.entry(&key, &value.lock().unwrap());
        }

        debug_map.finish()
    }
}
