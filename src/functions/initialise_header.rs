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

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "async"))]
    use std::sync::Mutex;
    #[cfg(feature = "async")]
    use tokio::sync::Mutex;

    use super::*;

    #[test]
    fn test_initialise_header() {
        let mut field_names: IndexMap<(String, u8), u8> = IndexMap::new();
        field_names.insert(("field1".to_string(), 1), 1);
        field_names.insert(("field2".to_string(), 2), 2);

        let data: Vec<u8> = vec![0x01, 0x02, 0x03];

        #[cfg(not(feature = "async"))]
        let field_names: Mutex<IndexMap<(String, u8), u8>> = Mutex::new(field_names);

        #[cfg(feature = "async")]
        let field_names = Mutex::new(field_names);

        let header: Vec<u8> = initialise_header(&field_names.try_lock().unwrap(), &data);

        assert_eq!(&header[0..SPUD_VERSION.len()], SPUD_VERSION.as_bytes());
        assert_eq!(&header[SPUD_VERSION.len()..SPUD_VERSION.len() + 1], &[1]);
        assert_eq!(
            &header[SPUD_VERSION.len() + 1..SPUD_VERSION.len() + 7],
            b"field1"
        );
        assert_eq!(
            &header[SPUD_VERSION.len() + 7..SPUD_VERSION.len() + 8],
            &[1]
        );
        assert_eq!(
            &header[SPUD_VERSION.len() + 8..SPUD_VERSION.len() + 9],
            &[2]
        );
        assert_eq!(
            &header[SPUD_VERSION.len() + 9..SPUD_VERSION.len() + 15],
            b"field2"
        );
        assert_eq!(
            &header[SPUD_VERSION.len() + 15..],
            &[2, 1, 1, 2, 3, 0xDE, 0xAD, 0xBE, 0xEF]
        );
    }
}
