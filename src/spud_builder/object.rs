#![allow(clippy::needless_pass_by_value)]

use core::cell::RefCell;
use indexmap::{IndexMap, map::Values};
use std::{cell::Ref, rc::Rc};

use crate::{SpudError, functions::generate_u8_id, spud_types::SpudTypes, types::ObjectId};

use super::{SpudTypesExt, builder::ObjectMap};

/// Represents a SPUD object, which is a collection of fields and values.
/// It allows adding values to fields and manages the internal data structure for SPUD encoding.
#[derive(Debug)]
pub struct SpudObject {
    pub(crate) _oid: ObjectId,
    data: Rc<RefCell<Vec<u8>>>,
    field_names: Rc<RefCell<IndexMap<(String, u8), u8>>>,
    seen_ids: Rc<RefCell<Vec<bool>>>,
    objects: Rc<RefCell<ObjectMap>>,
}

impl SpudObject {
    pub(crate) fn new(
        field_names: Rc<RefCell<IndexMap<(String, u8), u8>>>,
        seen_ids: Rc<RefCell<Vec<bool>>>,
        objects: Rc<RefCell<ObjectMap>>,
    ) -> Result<Rc<RefCell<SpudObject>>, SpudError> {
        let data: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));

        data.borrow_mut()
            .extend_from_slice(&[SpudTypes::ObjectStart as u8, SpudTypes::ObjectStart as u8]);

        let oid: ObjectId = Self::generate_oid(&mut data.borrow_mut())?;

        let object: Rc<RefCell<SpudObject>> = Rc::new(RefCell::new(Self {
            _oid: oid,
            data,
            field_names,
            seen_ids,
            objects: Rc::new(RefCell::new(ObjectMap(IndexMap::new()))),
        }));

        objects.borrow_mut().0.insert(oid, Rc::clone(&object));

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
    pub fn add_value<T: SpudTypesExt>(
        &self,
        field_name: &str,
        value: T,
    ) -> Result<&Self, SpudError> {
        self.add_field_name(field_name)?;

        value.write_spud_bytes(&mut self.data.borrow_mut());

        Ok(self)
    }

    /// Creates a new `SpudObject` instance associated with this Object.
    ///
    /// # Errors
    ///
    /// Returns an error if the object cannot be created, typically due to internal issues with the builder's state.
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
        )
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>, SpudError> {
        let mut data: Vec<u8> = self.data.borrow().clone();

        data.push(SpudTypes::ObjectEnd as u8);
        data.push(SpudTypes::ObjectEnd as u8);

        let objects: Ref<'_, ObjectMap> = self.objects.borrow();
        let objects: Values<'_, ObjectId, Rc<RefCell<SpudObject>>> = objects.0.values();

        for object in objects {
            let encoded_object_data: Vec<u8> = object.borrow().encode()?;

            data.extend_from_slice(&encoded_object_data);
        }

        Ok(data)
    }

    fn add_field_name(&self, field_name: &str) -> Result<&Self, SpudError> {
        let key: (String, u8) = (field_name.into(), u8::try_from(field_name.len())?);

        let id: u8 = if let Some(value) = self.field_names.borrow().get(&key) {
            *value
        } else {
            let id: u8 = generate_u8_id(&mut self.seen_ids.borrow_mut())?;

            self.field_names.borrow_mut().insert(key, id);
            id
        };

        self.data.borrow_mut().push(SpudTypes::FieldNameId as u8);
        self.data.borrow_mut().push(id);

        Ok(self)
    }

    fn generate_oid(data: &mut Vec<u8>) -> Result<ObjectId, SpudError> {
        let oid: ObjectId = ObjectId::new()?;

        data.push(SpudTypes::ObjectId as u8);
        data.extend_from_slice(oid.as_bytes());

        Ok(oid)
    }
}
