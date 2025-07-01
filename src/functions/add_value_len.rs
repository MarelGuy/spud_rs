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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_value_length_u8() {
        let mut data: Vec<u8> = Vec::with_capacity(1);

        add_value_length(&mut data, 42);
        assert_eq!(data, vec![SpudTypes::U8 as u8, 42]);
    }

    #[test]
    fn test_add_value_length_u16() {
        let mut data: Vec<u8> = Vec::with_capacity(2);

        add_value_length(&mut data, 256);
        assert_eq!(data, vec![SpudTypes::U16 as u8, 0, 1]);
    }

    #[test]
    fn test_add_value_length_u32() {
        let mut data: Vec<u8> = Vec::with_capacity(4);

        add_value_length(&mut data, 65536);
        assert_eq!(data, vec![SpudTypes::U32 as u8, 0, 0, 1, 0]);
    }

    #[test]
    fn test_add_value_length_u64() {
        let mut data: Vec<u8> = Vec::with_capacity(8);

        add_value_length(&mut data, 4_294_967_296);
        assert_eq!(data, vec![SpudTypes::U64 as u8, 0, 0, 0, 0, 1, 0, 0, 0]);
    }
}
