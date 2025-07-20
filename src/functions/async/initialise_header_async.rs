use indexmap::IndexMap;

use crate::{SPUD_VERSION, spud_types::SpudTypes};

type FieldNames<'a> = tokio::sync::MutexGuard<'a, IndexMap<(String, u8), u8>>;

pub(crate) fn initialise_header_async(field_names: &FieldNames, data: &[u8]) -> Vec<u8> {
    let mut header: Vec<u8> = SPUD_VERSION.as_bytes().to_vec();

    for (name, id) in field_names.iter() {
        header.push(name.1);

        header.extend_from_slice(name.0.as_bytes());

        header.push(*id);
    }

    header.push(SpudTypes::FieldNameListEnd.as_u8());

    header.extend_from_slice(data);
    header.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);

    header
}

#[cfg(test)]
mod tests {
    use tokio::sync::Mutex;

    use super::*;

    #[test]
    fn test_initialise_header() {
        let mut field_names: IndexMap<(String, u8), u8> = IndexMap::new();

        let field_name_1: String = "foo".into();
        let field_name_2: String = "bar".into();

        let field_name_1_len: u8 = field_name_1.len().try_into().unwrap();
        let field_name_2_len: u8 = field_name_2.len().try_into().unwrap();

        field_names.insert((field_name_1, field_name_1_len), 1);
        field_names.insert((field_name_2, field_name_2_len), 2);

        let data: Vec<u8> = vec![];

        #[cfg(not(feature = "async"))]
        let field_names: Mutex<IndexMap<(String, u8), u8>> = Mutex::new(field_names);

        #[cfg(feature = "async")]
        let field_names = Mutex::new(field_names);

        let header: Vec<u8> = initialise_header_async(&field_names.try_lock().unwrap(), &data);

        assert_eq!(
            header.len(),
            SPUD_VERSION.len()
                + field_name_1_len as usize
                + 2 // 1 byte for field name length, 1 byte for field ID
                + field_name_2_len as usize
                + 2 // 1 byte for field name length, 1 byte for field ID
                + 1 // 1 byte for FieldNameListEnd
                + data.len()
                + 4 // 4 bytes for the end marker (0xDE, 0xAD, 0xBE, 0xEF)
        );
        assert_eq!(&header[..SPUD_VERSION.len()], SPUD_VERSION.as_bytes());
    }
}
