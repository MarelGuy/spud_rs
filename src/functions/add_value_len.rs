use crate::spud_types::SpudTypes;

pub(crate) fn add_value_length(data: &mut Vec<u8>, value_len: usize) {
    if u8::try_from(value_len).is_ok() {
        data.push(SpudTypes::U8 as u8);
        data.extend_from_slice(&u8::try_from(value_len).unwrap().to_le_bytes());
    } else if u16::try_from(value_len).is_ok() {
        data.push(SpudTypes::U16 as u8);
        data.extend_from_slice(&u16::try_from(value_len).unwrap().to_le_bytes());
    } else if u32::try_from(value_len).is_ok() {
        data.push(SpudTypes::U32 as u8);
        data.extend_from_slice(&u32::try_from(value_len).unwrap().to_le_bytes());
    } else if u64::try_from(value_len).is_ok() {
        data.push(SpudTypes::U64 as u8);
        data.extend_from_slice(&(value_len as u64).to_le_bytes());
    }
}
