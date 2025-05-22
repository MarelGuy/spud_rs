use crate::{
    functions::add_value_len::add_value_length,
    spud_types::SpudTypes,
    types::{binary_blob::BinaryBlob as BinaryBlobStruct, spud_string::SpudString},
};

trait SpudPrimitiveWriter {
    fn write_primitive(self, data: &mut Vec<u8>);
}

macro_rules! impl_spud_primitive_writer_le {
    ($($t:ty),+ $(,)?) => {
        $(
            impl SpudPrimitiveWriter for $t {
                fn write_primitive(self, data: &mut Vec<u8>) {
                    data.extend_from_slice(&self.to_le_bytes());
                }
            }
        )+
    };
}

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
                    data.push(SpudTypes::$spud_type as u8);

                    $write_fn(self.clone(), data);
                }
            }
        )+
    };
}

impl<T: SpudTypesExt> SpudTypesExt for Vec<T> {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::ArrayStart
    }

    fn write_spud_bytes(&self, data: &mut Vec<u8>) {
        data.push(SpudTypes::ArrayStart as u8);

        for item in self {
            item.write_spud_bytes(data);
        }

        data.push(SpudTypes::ArrayEnd as u8);
    }
}

impl<T: SpudTypesExt> SpudTypesExt for &[T] {
    fn get_spud_type_tag(&self) -> SpudTypes {
        SpudTypes::ArrayStart
    }

    fn write_spud_bytes(&self, data: &mut Vec<u8>) {
        data.push(SpudTypes::ArrayStart as u8);

        for item in *self {
            item.write_spud_bytes(data);
        }

        data.push(SpudTypes::ArrayEnd as u8);
    }
}

impl_spud_primitive_writer_le!(u8, i8, i16, u16, i32, u32, f32, i64, u64, f64);
impl_spud_type_ext! {
    SpudString, String, write_string,
    BinaryBlobStruct<'_>, BinaryBlob, write_blob,
    bool, Bool, write_bool,
    i8, I8, write_primitive_value,
    u8, U8, write_primitive_value,
    i16, I16, write_primitive_value,
    u16, U16, write_primitive_value,
    i32, I32, write_primitive_value,
    u32, U32, write_primitive_value,
    f32, F32, write_primitive_value,
    i64, I64, write_primitive_value,
    u64, U64, write_primitive_value,
    f64, F64, write_primitive_value,
    (), Null, write_null,
}

fn write_string(value: &SpudString, data: &mut Vec<u8>) {
    add_value_length(data, value.0.len());

    data.extend_from_slice(value.0.as_bytes());
}

fn write_blob(value: &BinaryBlobStruct, data: &mut Vec<u8>) {
    add_value_length(data, value.0.len());

    data.extend_from_slice(value.0);
}

fn write_bool(value: bool, data: &mut Vec<u8>) {
    data.push(u8::from(value));
}

fn write_null(_value: (), data: &mut Vec<u8>) {
    data.push(SpudTypes::Null as u8);
}

fn write_primitive_value<T: SpudPrimitiveWriter>(value: T, data: &mut Vec<u8>) {
    value.write_primitive(data);
}
