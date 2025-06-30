use indexmap::IndexMap;
use std::{fmt, path::Path, sync::Arc};

use tokio::{
    runtime::Runtime,
    sync::{Mutex, MutexGuard},
};

use crate::{
    SpudError,
    functions::{check_path, initialise_header},
    types::ObjectId,
};

use tokio::fs::write;

use super::SpudObject;

#[derive(Default, Clone)]
pub(crate) struct ObjectMap(pub(crate) IndexMap<ObjectId, Arc<Mutex<SpudObject>>>);

/// Represents a builder for creating SPUD objects.
///
/// This builder allows you to create and manage SPUD objects, encode them into a byte vector, and write them to a file.
///
/// # Example
/// ```rust
/// use spud::SpudBuilder;
/// ```
pub struct SpudBuilder {
    pub(crate) field_names: Arc<Mutex<IndexMap<(String, u8), u8>>>,
    pub(crate) data: Arc<Mutex<Vec<u8>>>,
    pub(crate) objects: Arc<Mutex<ObjectMap>>,
    pub(crate) seen_ids: Arc<Mutex<Vec<bool>>>,
    pub(crate) rt: Arc<Runtime>,
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
    pub fn new(rt: Runtime) -> Self {
        let mut seen_ids: Vec<bool> = vec![false; 256];

        seen_ids[0] = true;
        seen_ids[1] = true;

        let rt: Arc<Runtime> = Arc::new(rt);

        Self {
            field_names: Arc::new(Mutex::new(IndexMap::new())),
            data: Arc::new(Mutex::new(Vec::new())),
            objects: Arc::new(Mutex::new(ObjectMap(IndexMap::new()))),
            seen_ids: Arc::new(Mutex::new(seen_ids)),
            rt,
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
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    ///
    /// # Note
    /// The `SpudObject` created by this method will share the same field names, seen IDs, and objects as the builder, allowing for consistent data management.
    /// Nothing is cloned, SPUD uses `Rc` and `RefCell` to manage shared ownership and mutability.
    pub fn object<F>(&self, f: F) -> Result<(), SpudError>
    where
        F: FnOnce(&SpudObject) -> Result<(), SpudError>,
    {
        let obj: Arc<Mutex<SpudObject>> = self.new_object()?;

        let locked_obj: MutexGuard<'_, SpudObject> = self.rt.block_on(obj.lock());

        f(&locked_obj)?;

        Ok(())
    }

    fn new_object(&self) -> Result<Arc<Mutex<SpudObject>>, SpudError> {
        self.rt.block_on(SpudObject::new(
            Arc::clone(&self.field_names),
            Arc::clone(&self.seen_ids),
            Arc::clone(&self.objects),
            Arc::clone(&self.data),
            Arc::clone(&self.rt),
        ))
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
    ///
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    pub async fn encode(&self) -> Result<Vec<u8>, SpudError> {
        for object in self.objects.lock().await.0.values() {
            object.lock().await.encode().await?;
        }

        Ok(self.data.lock().await.clone())
    }
}

impl fmt::Debug for SpudBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_builder: fmt::DebugStruct<'_, '_> = f.debug_struct("SpudBuilder");

        let field_names: MutexGuard<'_, IndexMap<(String, u8), u8>> =
            self.rt.block_on(self.field_names.lock());
        debug_builder.field("field_names", &*field_names);

        let data: MutexGuard<'_, Vec<u8>> = self.rt.block_on(self.data.lock());
        debug_builder.field("data", &*data);

        let objects: MutexGuard<'_, ObjectMap> = self.rt.block_on(self.objects.lock());
        debug_builder.field("objects", &*objects);

        let seen_ids: MutexGuard<'_, Vec<bool>> = self.rt.block_on(self.seen_ids.lock());

        let mut seen_ids_to_display: IndexMap<usize, bool> = IndexMap::new();

        for (index, &is_seen) in seen_ids.iter().enumerate() {
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
        let rt: Runtime = Runtime::new().unwrap();

        for (key, value) in &self.0 {
            rt.block_on(async {
                debug_map.entry(&key, &value.lock().await);
            });
        }

        debug_map.finish()
    }
}

impl SpudBuilder {
    /// Builds the SPUD file at the specified path with the given file name.
    ///  # Arguments
    ///
    /// * `path_str` - The path to the directory where the file will be created.
    /// * `file_name` - The name of the file to create.
    ///
    /// # Errors
    ///
    /// Returns an error if the path is invalid
    ///
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    pub async fn build_file(&mut self, path_str: &str, file_name: &str) -> Result<(), SpudError> {
        let path_str: String = check_path(path_str, file_name)?;

        let path: &Path = Path::new(&path_str);

        let header: Vec<u8> =
            initialise_header(&self.field_names.lock().await, &self.data.lock().await);

        write(path, header).await?;

        Ok(())
    }
}
