use crate::SpudTypes;

pub trait SpudTypesExt {
    fn get_spud_type_tag(&self) -> SpudTypes;
}

macro_rules! impl_spud_type_ext {
    ($($t:ty, $spud_type:ident),+) => {
        $(
            impl SpudTypesExt for $t {
                fn get_spud_type_tag(&self) -> SpudTypes {
                    SpudTypes::$spud_type
                }
            }
        )+
    };
}

impl_spud_type_ext! {
    i8, I8,
    i16, I16,
    i32, I32,
    i64, I64,
    u8, U8,
    u16, U16,
    u32, U32,
    u64, U64,
    f32, F32,
    f64, F64,
    String, String,
    &str, String,
    Vec<u8>, BinaryBlob,
    &[u8], BinaryBlob,
    bool, Bool
}
