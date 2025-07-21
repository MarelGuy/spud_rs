use indexmap::IndexMap;

use std::{fmt, future::Future, path::Path, sync::Arc};

use tokio::sync::{Mutex, MutexGuard};

use crate::{
    SpudError,
    functions::{check_path, initialise_header_async},
    types::ObjectId,
};

use tokio::fs::write;

use super::SpudObjectAsync;

#[derive(Default, Clone)]
pub(crate) struct ObjectMap(pub(crate) IndexMap<ObjectId, Arc<Mutex<SpudObjectAsync>>>);

#[derive(Default, Clone)]
/// Represents a builder for creating SPUD objects.
///
/// This builder allows you to create and manage SPUD objects, encode them into a byte vector, and write them to a file.
///
/// # Example
/// ```rust
/// use spud_rs::SpudBuilderAsync;
/// ```
pub struct SpudBuilderAsync {
    pub(crate) field_names: Arc<Mutex<IndexMap<(String, u8), u8>>>,
    pub(crate) data: Arc<Mutex<Vec<u8>>>,
    pub(crate) objects: Arc<Mutex<ObjectMap>>,
    pub(crate) seen_ids: Arc<Mutex<Vec<bool>>>,
}

impl SpudBuilderAsync {
    #[must_use]
    /// Creates a new `SpudBuilderAsync` instance.
    ///
    /// # Examples
    /// ```rust
    /// use spud_rs::SpudBuilderAsync;
    ///
    /// let builder = SpudBuilderAsync::new();
    ///
    /// ```
    ///
    /// # Returns
    ///
    /// A new instance of `SpudBuilderAsync`.
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

    /// Creates a new `SpudObjectAsync` instance associated with this builder.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a reference to the `SpudObjectAsync` and returns a `Result<(), SpudError>`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spud_rs::{SpudBuilderAsync, SpudObjectAsync};
    /// use tokio::sync::MutexGuard;
    ///
    /// async fn foo() -> Result<(), spud_rs::SpudError> {
    ///     let builder = SpudBuilderAsync::new();
    ///
    ///     builder.object(async |obj| {
    ///         let locked_obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;
    ///
    ///         Ok(())
    ///        }).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// A new instance of `SpudObjectAsync` that is linked to the builder's field names, seen IDs, and objects.
    ///
    /// # Errors
    ///
    /// Returns an error if the object cannot be created, typically due to internal issues with the builder's state.
    ///
    /// # Note
    ///
    /// The `SpudObjectAsync` created by this method will share the same field names, seen IDs, and objects as the builder.
    pub async fn object<F, Fut>(&self, f: F) -> Result<(), SpudError>
    where
        F: FnOnce(Arc<Mutex<SpudObjectAsync>>) -> Fut,
        Fut: Future<Output = Result<(), SpudError>>,
    {
        let obj: Arc<Mutex<SpudObjectAsync>> = self.new_object().await?;

        f(obj).await?;

        Ok(())
    }

    async fn new_object(&self) -> Result<Arc<Mutex<SpudObjectAsync>>, SpudError> {
        SpudObjectAsync::new(
            Arc::clone(&self.field_names),
            Arc::clone(&self.seen_ids),
            Arc::clone(&self.objects),
            Arc::clone(&self.data),
        )
        .await
    }

    /// Encodes all objects associated with this builder into a byte vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spud_rs::{SpudBuilderAsync, SpudObjectAsync};
    /// use tokio::sync::MutexGuard;
    ///
    /// async fn foo() -> Result<(), spud_rs::SpudError> {
    ///     let builder = SpudBuilderAsync::new();
    ///
    ///     builder.object(async |obj| {
    ///         let locked_obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;
    ///
    ///         locked_obj.add_value("field_name", 42u8).await?;
    ///
    ///         Ok(())
    ///     });
    ///
    ///     let encoded_data = builder.encode().await?;
    ///     Ok(())
    /// }
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

        let header: Vec<u8> =
            initialise_header_async(&self.field_names.lock().await, &self.data.lock().await);

        self.data.lock().await.clear();
        self.data.lock().await.extend_from_slice(&header);

        Ok(header)
    }

    /// Builds the SPUD file at the specified path with the given file name.
    ///
    ///  # Arguments
    ///
    /// * `path_str` - The path to the directory where the file will be created.
    /// * `file_name` - The name of the file to create.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spud_rs::{SpudBuilderAsync, SpudObjectAsync};
    /// use tokio::sync::MutexGuard;
    ///
    /// async fn foo() -> Result<(), spud_rs::SpudError> {
    ///     let mut builder = SpudBuilderAsync::new();
    ///
    ///     builder.object(async |obj| {
    ///         let locked_obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;
    ///
    ///         locked_obj.add_value("val", 1u8).await?;
    ///
    ///         Ok(())
    ///     }).await?;
    ///
    ///     builder.encode().await?;
    ///
    ///     builder.build_file("./tmp", "file_name").await?;
    ///
    ///     Ok(())
    /// }
    /// ```
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

        write(path, self.data.lock().await.clone()).await?;

        Ok(())
    }
}

impl fmt::Debug for SpudBuilderAsync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_builder: fmt::DebugStruct<'_, '_> = f.debug_struct("SpudBuilderAsync");

        let field_names: MutexGuard<'_, IndexMap<(String, u8), u8>> =
            if let Ok(guard) = self.field_names.try_lock() {
                guard
            } else {
                return Err(fmt::Error);
            };

        debug_builder.field("field_names", &*field_names);

        let data: MutexGuard<'_, Vec<u8>> = if let Ok(guard) = self.data.try_lock() {
            guard
        } else {
            return Err(fmt::Error);
        };

        debug_builder.field("data", &*data);

        let objects: MutexGuard<'_, ObjectMap> = if let Ok(guard) = self.objects.try_lock() {
            guard
        } else {
            return Err(fmt::Error);
        };

        debug_builder.field("objects", &*objects);

        let seen_ids: MutexGuard<'_, Vec<bool>> = if let Ok(guard) = self.seen_ids.try_lock() {
            guard
        } else {
            return Err(fmt::Error);
        };

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

        for (key, value) in &self.0 {
            debug_map.entry(&key, &value.try_lock());
        }

        debug_map.finish()
    }
}
