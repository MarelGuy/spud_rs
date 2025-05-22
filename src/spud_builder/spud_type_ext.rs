use crate::{functions::add_value_len::add_value_length, spud_types::SpudTypes};

pub trait SpudTypesExt {
    fn get_spud_type_tag(&self) -> SpudTypes;
    fn write_spud_bytes(&self, data: &mut Vec<u8>);
}

macro_rules! impl_spud_type_ext {
    ($($t:ty, $spud_type:ident, $write_fn:path),+ $(,)?) => {
        $(
            impl SpudTypesExt for $t {
                fn get_spud_type_tag(&self) -> SpudTypes {
                    SpudTypes::$spud_type
                }

                fn write_spud_bytes(&self, data: &mut Vec<u8>) {
                    $write_fn(self.clone(), data);
                }
            }
        )+
    };
}

fn write_string(value: String, data: &mut Vec<u8>) {
    data.push(SpudTypes::String as u8);

    add_value_length(data, value.len());

    data.extend_from_slice(value.as_bytes());
}

fn write_str(value: &str, data: &mut Vec<u8>) {
    data.push(SpudTypes::String as u8);

    add_value_length(data, value.len());

    data.extend_from_slice(value.as_bytes());
}

fn write_blob(value: Vec<u8>, data: &mut Vec<u8>) {
    data.push(SpudTypes::BinaryBlob as u8);

    add_value_length(data, value.len());

    data.extend_from_slice(&value);
}

fn write_blob_ref(value: &[u8], data: &mut Vec<u8>) {
    data.push(SpudTypes::BinaryBlob as u8);

    add_value_length(data, value.len());

    data.extend_from_slice(value);
}

fn write_bool(value: bool, data: &mut Vec<u8>) {
    data.push(SpudTypes::Bool as u8);
    data.push(u8::from(value));
}

fn write_i8_tagged(value: i8, data: &mut Vec<u8>) {
    data.push(SpudTypes::I8 as u8);
    data.extend_from_slice(&value.to_le_bytes());
}

fn write_u8_tagged(value: u8, data: &mut Vec<u8>) {
    data.push(SpudTypes::U8 as u8);
    data.push(value);
}

fn write_i16_tagged(value: i16, data: &mut Vec<u8>) {
    data.push(SpudTypes::I16 as u8);
    data.extend_from_slice(&value.to_le_bytes());
}

fn write_u16_tagged(value: u16, data: &mut Vec<u8>) {
    data.push(SpudTypes::U16 as u8);
    data.extend_from_slice(&value.to_le_bytes());
}

fn write_i32_tagged(value: i32, data: &mut Vec<u8>) {
    data.push(SpudTypes::I32 as u8);
    data.extend_from_slice(&value.to_le_bytes());
}

fn write_u32_tagged(value: u32, data: &mut Vec<u8>) {
    data.push(SpudTypes::U32 as u8);
    data.extend_from_slice(&value.to_le_bytes());
}

fn write_f32_tagged(value: f32, data: &mut Vec<u8>) {
    data.push(SpudTypes::F32 as u8);
    data.extend_from_slice(&value.to_le_bytes());
}

fn write_i64_tagged(value: i64, data: &mut Vec<u8>) {
    data.push(SpudTypes::I64 as u8);
    data.extend_from_slice(&value.to_le_bytes());
}

fn write_u64_tagged(value: u64, data: &mut Vec<u8>) {
    data.push(SpudTypes::U64 as u8);
    data.extend_from_slice(&value.to_le_bytes());
}

fn write_f64_tagged(value: f64, data: &mut Vec<u8>) {
    data.push(SpudTypes::F64 as u8);
    data.extend_from_slice(&value.to_le_bytes());
}

impl_spud_type_ext! {
    String, String, write_string,
    &String, String, write_str,
    &str, String, write_str,
    Vec<u8>, BinaryBlob, write_blob,
    &[u8], BinaryBlob, write_blob_ref,
    bool, Bool, write_bool,
    i8, I8, write_i8_tagged,
    u8, U8, write_u8_tagged,
    i16, I16, write_i16_tagged,
    u16, U16, write_u16_tagged,
    i32, I32, write_i32_tagged,
    u32, U32, write_u32_tagged,
    f32, F32, write_f32_tagged,
    i64, I64, write_i64_tagged,
    u64, U64, write_u64_tagged,
    f64, F64, write_f64_tagged
}

impl<T: SpudTypesExt> SpudTypesExt for Option<T> {
    fn get_spud_type_tag(&self) -> SpudTypes {
        match self {
            Some(val) => val.get_spud_type_tag(),
            None => SpudTypes::Null,
        }
    }

    fn write_spud_bytes(&self, data: &mut Vec<u8>) {
        match self {
            Some(val) => val.write_spud_bytes(data),
            None => {
                data.push(SpudTypes::Null as u8);
            }
        }
    }
}
