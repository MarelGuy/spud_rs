#![allow(clippy::needless_pass_by_value)]

use core::cell::RefCell;
use indexmap::IndexMap;
use std::rc::Rc;

use crate::{functions::generate_u8_id, spud_types::SpudTypes, types::ObjectId};

use super::{SpudTypesExt, builder::ObjectMap};

/// Represents a SPUD object, which is a collection of fields and values.
/// It allows adding values to fields and manages the internal data structure for SPUD encoding.
#[derive(Debug)]
pub struct SpudObject {
    pub(crate) _oid: ObjectId,
    data: Rc<RefCell<Vec<u8>>>,
    field_names: Rc<RefCell<IndexMap<(String, u8), u8>>>,
    seen_ids: Rc<RefCell<Vec<bool>>>,
}

impl SpudObject {
    #[must_use]
    pub(crate) fn new(
        field_names: Rc<RefCell<IndexMap<(String, u8), u8>>>,
        seen_ids: Rc<RefCell<Vec<bool>>>,
        objects: Rc<RefCell<ObjectMap>>,
    ) -> Self {
        let data: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));

        data.borrow_mut()
            .extend_from_slice(&[SpudTypes::ObjectStart as u8, SpudTypes::ObjectStart as u8]);

        let oid: ObjectId = Self::generate_oid(&mut data.borrow_mut());

        objects.borrow_mut().0.insert(oid, Rc::clone(&data));

        Self {
            _oid: oid,
            data,
            field_names,
            seen_ids,
        }
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
    /// let mut builder = SpudBuilder::new();
    /// let mut object = builder.new_object();
    ///
    /// object.add_value("example_field", SpudString::from("example_value"));
    /// // The object now contains the field "example_field" with the value "example_value".
    /// ```
    ///
    /// # Returns
    /// A mutable reference to the `SpudObject`, allowing for method chaining.
    pub fn add_value<T: SpudTypesExt>(&mut self, field_name: &str, value: T) -> &mut Self {
        self.add_field_name(field_name);

        value.write_spud_bytes(&mut self.data.borrow_mut());

        self
    }

    fn add_field_name(&mut self, field_name: &str) -> &mut Self {
        let key: (String, u8) = (
            field_name.into(),
            if let Ok(value) = u8::try_from(field_name.len()) {
                value
            } else {
                tracing::error!("Field name is too long: {field_name}");

                panic!("Closing...")
            },
        );

        let id: u8 = if let Some(value) = self.field_names.borrow().get(&key) {
            *value
        } else {
            let id: u8 = generate_u8_id(&mut self.seen_ids.borrow_mut());

            self.field_names.borrow_mut().insert(key, id);
            id
        };

        self.data.borrow_mut().push(SpudTypes::FieldNameId as u8);
        self.data.borrow_mut().push(id);

        self
    }

    fn generate_oid(data: &mut Vec<u8>) -> ObjectId {
        let oid: ObjectId = ObjectId::new();

        data.push(SpudTypes::ObjectId as u8);
        data.extend_from_slice(&oid.0);

        oid
    }
}
