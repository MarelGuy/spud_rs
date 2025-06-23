use crate::spud_types::SpudTypes;

pub(crate) fn add_value_length(data: &mut Vec<u8>, value_len: usize) {
    macro_rules! try_push {
        ($ty:ty, $variant:expr) => {
            if let Ok(value) = <$ty>::try_from(value_len) {
                data.push($variant as u8);
                data.extend_from_slice(&value.to_le_bytes());
                return;
            }
        };
    }

    try_push!(u8, SpudTypes::U8);
    try_push!(u16, SpudTypes::U16);
    try_push!(u32, SpudTypes::U32);
    try_push!(u64, SpudTypes::U64);
}
