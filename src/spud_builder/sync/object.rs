#![allow(clippy::needless_pass_by_value)]

use indexmap::{IndexMap, map::Values};
use std::sync::{Arc, Mutex, MutexGuard};

use crate::{
    SpudError, functions::generate_u8_id_sync, spud_builder::spud_type_ext::SpudTypesExt,
    spud_types::SpudTypes, types::ObjectId,
};

use super::builder::ObjectMap;

/// Represents a SPUD object, which is a collection of fields and values.
/// It allows adding values to fields and manages the internal data structure for SPUD encoding.
#[derive(Debug)]
pub struct SpudObjectSync {
    pub(crate) _oid: ObjectId,
    data: Arc<Mutex<Vec<u8>>>,
    field_names: Arc<Mutex<IndexMap<(String, u8), u8>>>,
    seen_ids: Arc<Mutex<Vec<bool>>>,
    objects: Arc<Mutex<ObjectMap>>,
}

impl SpudObjectSync {
    pub(crate) fn new(
        field_names: Arc<Mutex<IndexMap<(String, u8), u8>>>,
        seen_ids: Arc<Mutex<Vec<bool>>>,
        objects: Arc<Mutex<ObjectMap>>,
        data: Arc<Mutex<Vec<u8>>>,
    ) -> Result<Arc<Mutex<SpudObjectSync>>, SpudError> {
        data.lock().unwrap().extend_from_slice(&[
            SpudTypes::ObjectStart.as_u8(),
            SpudTypes::ObjectStart.as_u8(),
        ]);

        let oid: ObjectId = Self::generate_oid(&mut data.lock().unwrap())?;

        let object: Arc<Mutex<SpudObjectSync>> = Arc::new(Mutex::new(Self {
            _oid: oid,
            data,
            field_names,
            seen_ids,
            objects: Arc::new(Mutex::new(ObjectMap(IndexMap::new()))),
        }));

        objects.lock().unwrap().0.insert(oid, Arc::clone(&object));

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
    /// use spud_rs::{SpudBuilder, SpudObjectSync, types::SpudString};
    ///
    /// let builder = SpudBuilder::new();
    ///
    /// builder.object(|obj| {
    ///     obj.add_value("example_field", SpudString::from("example_value"));
    ///
    ///     Ok(())
    /// });
    ///
    /// // The object now contains the field "example_field" with the value "example_value".
    /// ```
    ///
    /// # Returns
    /// A mutable reference to the `SpudObjectSync`, allowing for method chaining.
    ///
    /// # Errors
    ///
    /// If the field name is too long (greater than 255 characters) or if there is an error generating a unique ID, this method will return an error.
    ///
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    pub fn add_value<T: SpudTypesExt>(
        &self,
        field_name: &str,
        value: T,
    ) -> Result<&Self, SpudError> {
        self.add_field_name(field_name)?;

        value.write_spud_bytes(&mut self.data.lock().unwrap());

        Ok(self)
    }

    /// Creates a new `SpudObjectSync` instance associated with this Object.
    ///
    /// # Errors
    ///
    /// Returns an error if the object cannot be created, typically due to internal issues with the builder's state.
    ///
    /// # Panics
    ///
    /// Panics if the Mutex cannot be locked, which is unlikely but can happen in case of a deadlock or other synchronization issues.
    pub fn object<F>(&self, field_name: &str, f: F) -> Result<(), SpudError>
    where
        F: FnOnce(&SpudObjectSync) -> Result<(), SpudError>,
    {
        self.add_field_name(field_name)?;

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

    pub(crate) fn encode(&self) -> Result<(), SpudError> {
        let objects: MutexGuard<'_, ObjectMap> = self.objects.lock().unwrap();
        let objects: Values<'_, ObjectId, Arc<Mutex<SpudObjectSync>>> = objects.0.values();

        for object in objects {
            object.lock().unwrap().encode()?;
        }

        Ok(())
    }

    fn add_field_name(&self, field_name: &str) -> Result<&Self, SpudError> {
        let key: (String, u8) = (field_name.into(), u8::try_from(field_name.len())?);

        let id: u8 = if let Some(value) = self.field_names.lock().unwrap().get(&key) {
            *value
        } else {
            let id: u8 = generate_u8_id_sync(&mut self.seen_ids.lock().unwrap())?;

            self.field_names.lock().unwrap().insert(key, id);
            id
        };

        self.data
            .lock()
            .unwrap()
            .push(SpudTypes::FieldNameId.as_u8());
        self.data.lock().unwrap().push(id);

        Ok(self)
    }

    fn generate_oid(data: &mut Vec<u8>) -> Result<ObjectId, SpudError> {
        let oid: ObjectId = ObjectId::new()?;

        data.extend_from_slice(oid.as_bytes());

        Ok(oid)
    }
}
