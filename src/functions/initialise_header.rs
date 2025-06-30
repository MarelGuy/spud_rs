use indexmap::IndexMap;

use crate::{SPUD_VERSION, spud_types::SpudTypes};

#[cfg(not(feature = "async"))]
type FieldNames<'a> = std::sync::MutexGuard<'a, IndexMap<(String, u8), u8>>;

#[cfg(feature = "async")]
type FieldNames<'a> = tokio::sync::MutexGuard<'a, IndexMap<(String, u8), u8>>;

pub(crate) fn initialise_header(field_names: &FieldNames, data: &[u8]) -> Vec<u8> {
    let mut header: Vec<u8> = SPUD_VERSION.as_bytes().to_vec();

    for (name, id) in field_names.iter() {
        header.push(name.1);

        header.extend_from_slice(name.0.as_bytes());

        header.push(*id);
    }

    header.push(SpudTypes::FieldNameListEnd as u8);

    header.extend_from_slice(data);
    header.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);

    header
}
