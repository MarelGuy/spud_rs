#![allow(clippy::needless_pass_by_value)]

use indexmap::{IndexMap, map::Values};
use std::{pin::Pin, sync::Arc};

use tokio::sync::{Mutex, MutexGuard};

use crate::{
    SpudError, functions::generate_u8_id, spud_builder::spud_type_ext::SpudTypesExt,
    spud_types::SpudTypes, types::ObjectId,
};

use super::builder::ObjectMap;

/// Represents a SPUD object, which is a collection of fields and values.
/// It allows adding values to fields and manages the internal data structure for SPUD encoding.
#[derive(Debug)]
pub struct SpudObject {
    pub(crate) _oid: ObjectId,
    data: Arc<Mutex<Vec<u8>>>,
    field_names: Arc<Mutex<IndexMap<(String, u8), u8>>>,
    seen_ids: Arc<Mutex<Vec<bool>>>,
    objects: Arc<Mutex<ObjectMap>>,
}

impl SpudObject {
    pub(crate) async fn new(
        field_names: Arc<Mutex<IndexMap<(String, u8), u8>>>,
        seen_ids: Arc<Mutex<Vec<bool>>>,
        objects: Arc<Mutex<ObjectMap>>,
        data: Arc<Mutex<Vec<u8>>>,
    ) -> Result<Arc<Mutex<SpudObject>>, SpudError> {
        data.lock()
            .await
            .extend_from_slice(&[SpudTypes::ObjectStart as u8, SpudTypes::ObjectStart as u8]);

        let oid: ObjectId = Self::generate_oid(&mut data.lock().await)?;

        let object: Arc<Mutex<SpudObject>> = Arc::new(Mutex::new(Self {
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
    /// * `field_name` - The name of the field to which the value will be added.
    /// * `value` - The value to be added, which must implement the `SpudTypesExt` trait.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use spud::{SpudBuilder, SpudObject, types::SpudString};
    ///
    /// let builder = SpudBuilder::new();
    ///
    /// builder.object(|obj| {
    ///     obj.add_value("example_field", SpudString::from("example_value"));
    ///
    ///     OK(())
    /// });
    ///
    /// // The object now contains the field "example_field" with the value "example_value".
    /// ```
    ///
    /// # Returns
    /// A mutable reference to the `SpudObject`, allowing for method chaining.
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

    /// Creates a new `SpudObject` instance associated with this Object.
    ///
    /// # Errors
    ///
    /// Returns an error if the object cannot be created, typically due to internal issues with the builder's state.
    ///
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    pub async fn object<F>(&self, field_name: &str, f: F) -> Result<(), SpudError>
    where
        F: FnOnce(&SpudObject) -> Result<(), SpudError>,
    {
        self.add_field_name(field_name).await?;

        let obj: Arc<Mutex<SpudObject>> = self.new_object().await?;

        f(&*obj.lock().await)?;

        Ok(())
    }

    async fn new_object(&self) -> Result<Arc<Mutex<SpudObject>>, SpudError> {
        SpudObject::new(
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
            let mut data: MutexGuard<'_, Vec<u8>> = self.data.lock().await;

            data.push(SpudTypes::ObjectEnd as u8);
            data.push(SpudTypes::ObjectEnd as u8);

            let objects: MutexGuard<'_, ObjectMap> = self.objects.lock().await;
            let objects: Values<'_, ObjectId, Arc<Mutex<SpudObject>>> = objects.0.values();

            drop(data);

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
            let id: u8 = generate_u8_id(&mut self.seen_ids.lock().await)?;

            self.field_names.lock().await.insert(key, id);
            id
        };

        self.data.lock().await.push(SpudTypes::FieldNameId as u8);
        self.data.lock().await.push(id);

        Ok(self)
    }

    fn generate_oid(data: &mut MutexGuard<'_, Vec<u8>>) -> Result<ObjectId, SpudError> {
        let oid: ObjectId = ObjectId::new()?;

        data.extend_from_slice(oid.as_bytes());

        Ok(oid)
    }
}
