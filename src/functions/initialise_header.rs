use indexmap::IndexMap;

use crate::spud_types::SpudTypes;

pub(crate) fn initialise_header(field_names: &IndexMap<(String, u8), u8>, data: &[u8]) -> Vec<u8> {
    let mut header: Vec<u8> = "SPUD-0.2.0".as_bytes().to_vec();

    for (name, id) in field_names {
        header.push(name.1);

        header.extend_from_slice(name.0.as_bytes());

        header.push(*id);
    }

    header.push(SpudTypes::FieldNameListEnd as u8);

    header.extend_from_slice(data);
    header.extend_from_slice(&[0xDE, 0xAD, 0xBE, 0xEF]);

    header
}
