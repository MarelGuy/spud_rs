#![allow(clippy::needless_pass_by_value)]

use indexmap::{IndexMap, map::Values};
use std::{pin::Pin, sync::Arc};

use tokio::sync::{Mutex, MutexGuard};

use crate::{
    SpudError, functions::generate_u8_id_async, spud_builder::spud_type_ext::SpudTypesExt,
    spud_types::SpudTypes, types::ObjectId,
};

use super::builder::ObjectMap;

/// Represents a SPUD object, which is a collection of fields and values.
/// It allows adding values to fields and manages the internal data structure for SPUD encoding.
#[derive(Debug)]
pub struct SpudObjectAsync {
    pub(crate) _oid: ObjectId,
    data: Arc<Mutex<Vec<u8>>>,
    field_names: Arc<Mutex<IndexMap<(String, u8), u8>>>,
    seen_ids: Arc<Mutex<Vec<bool>>>,
    objects: Arc<Mutex<ObjectMap>>,
}

impl SpudObjectAsync {
    pub(crate) async fn new(
        field_names: Arc<Mutex<IndexMap<(String, u8), u8>>>,
        seen_ids: Arc<Mutex<Vec<bool>>>,
        objects: Arc<Mutex<ObjectMap>>,
        data: Arc<Mutex<Vec<u8>>>,
    ) -> Result<Arc<Mutex<SpudObjectAsync>>, SpudError> {
        data.lock().await.extend_from_slice(&[
            SpudTypes::ObjectStart.as_u8(),
            SpudTypes::ObjectStart.as_u8(),
        ]);

        let oid: ObjectId = Self::generate_oid(&mut data.lock().await)?;

        let object: Arc<Mutex<SpudObjectAsync>> = Arc::new(Mutex::new(Self {
            _oid: oid,
            data,
            field_names,
            seen_ids,
            objects: Arc::new(Mutex::new(ObjectMap(IndexMap::new()))),
        }));

        objects.lock().await.0.insert(oid, Arc::clone(&object));

        Ok(object)
    }

    /// Adds a value to the object with the specified field name.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field to which the value will be added.
    /// * `value` - The value to be added, which must implement the `SpudTypesExt` trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spud_rs::{SpudBuilder, SpudObjectAsync};
    /// use tokio::sync::MutexGuard;
    ///
    /// let builder = SpudBuilder::new();
    ///
    /// builder.object(async |obj| {
    ///     let locked_obj: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;
    ///
    ///     locked_obj.add_value("field_name", 42u8).await?;
    ///
    ///     Ok(())
    /// });
    /// ```
    ///
    /// # Returns
    ///
    /// A mutable reference to the `SpudObjectAsync`, allowing for method chaining.
    ///
    /// # Errors
    ///
    /// If the field name is too long (greater than 255 characters) or if there is an error generating a unique ID, this method will return an error.
    ///
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    pub async fn add_value<T: SpudTypesExt>(
        &self,
        field_name: &str,
        value: T,
    ) -> Result<&Self, SpudError> {
        self.add_field_name(field_name).await?;

        value.write_spud_bytes(&mut *self.data.lock().await);

        Ok(self)
    }

    /// Creates a new `SpudObjectAsync` instance associated with this Object.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field to which the object will be added.
    ///
    /// # Errors
    ///
    /// Returns an error if the object cannot be created, typically due to internal issues with the builder's state.
    ///
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    pub async fn object<F, Fut>(&self, field_name: &str, f: F) -> Result<(), SpudError>
    where
        F: FnOnce(Arc<Mutex<SpudObjectAsync>>) -> Fut,
        Fut: Future<Output = Result<(), SpudError>>,
    {
        self.add_field_name(field_name).await?;

        let obj: Arc<Mutex<SpudObjectAsync>> = self.new_object().await?;

        f(obj).await?;

        self.data.lock().await.push(SpudTypes::ObjectEnd.as_u8());
        self.data.lock().await.push(SpudTypes::ObjectEnd.as_u8());

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

    pub(crate) fn encode<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<(), SpudError>> + Send + 'a>> {
        Box::pin(async move {
            let objects: MutexGuard<'_, ObjectMap> = self.objects.lock().await;
            let objects: Values<'_, ObjectId, Arc<Mutex<SpudObjectAsync>>> = objects.0.values();

            for object in objects {
                object.lock().await.encode().await?;
            }

            Ok(())
        })
    }

    async fn add_field_name(&self, field_name: &str) -> Result<&Self, SpudError> {
        let key: (String, u8) = (field_name.into(), u8::try_from(field_name.len())?);

        let id: u8 = if let Some(value) = self.field_names.lock().await.get(&key) {
            *value
        } else {
            let id: u8 = generate_u8_id_async(&mut self.seen_ids.lock().await)?;

            self.field_names.lock().await.insert(key, id);
            id
        };

        self.data.lock().await.push(SpudTypes::FieldNameId.as_u8());
        self.data.lock().await.push(id);

        Ok(self)
    }

    fn generate_oid(data: &mut MutexGuard<'_, Vec<u8>>) -> Result<ObjectId, SpudError> {
        let oid: ObjectId = ObjectId::new()?;

        data.extend_from_slice(oid.as_bytes());

        Ok(oid)
    }
}
