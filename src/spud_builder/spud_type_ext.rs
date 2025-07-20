use rust_decimal::Decimal;

use crate::{
    functions::add_value_length,
    spud_types::{SpudNumberTypes, SpudTypes},
    types::{BinaryBlob as BinaryBlobStruct, Date, DateTime, SpudString, Time},
};

trait SpudPrimitiveWriter {
    fn write_primitive(self, data: &mut Vec<u8>);
}

pub trait SpudTypesExt {
    fn write_spud_bytes(&self, data: &mut Vec<u8>);
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

macro_rules! impl_spud_type_ext {
    // For enum variants with parentheses (e.g., Number(SpudNumberTypes::I8))
    ($($t:ty, $spud_type:ident ( $($variant:tt)* ), $write_fn:path),+ $(,)?) => {
        $(
            impl SpudTypesExt for $t {
                fn write_spud_bytes(&self, data: &mut Vec<u8>) {
                    data.push(SpudTypes::$spud_type($($variant)*).as_u8());
                    $write_fn(*self, data);
                }
            }
        )+
    };
    // For simple enum variants (e.g., Bool, Null, Date, etc.)
    ($($t:ty, $spud_type:ident, $write_fn:path),+ $(,)?) => {
        $(
            impl SpudTypesExt for $t {
                fn write_spud_bytes(&self, data: &mut Vec<u8>) {
                    data.push(SpudTypes::$spud_type.as_u8());
                    $write_fn(*self, data);
                }
            }
        )+
    };
}

impl_spud_primitive_writer_le!(u8, i8, i16, u16, i32, u32, f32, i64, u64, f64, i128, u128);

impl_spud_type_ext! {
    i8, Number(SpudNumberTypes::I8), write_primitive_value,
    u8, Number(SpudNumberTypes::U8), write_primitive_value,
    i16, Number(SpudNumberTypes::I16), write_primitive_value,
    u16, Number(SpudNumberTypes::U16), write_primitive_value,
    i32, Number(SpudNumberTypes::I32), write_primitive_value,
    u32, Number(SpudNumberTypes::U32), write_primitive_value,
    f32, Number(SpudNumberTypes::F32), write_primitive_value,
    i64, Number(SpudNumberTypes::I64), write_primitive_value,
    u64, Number(SpudNumberTypes::U64), write_primitive_value,
    f64, Number(SpudNumberTypes::F64), write_primitive_value,
    i128, Number(SpudNumberTypes::I128), write_primitive_value,
    u128, Number(SpudNumberTypes::U128), write_primitive_value,
}

impl_spud_type_ext! {
    Decimal, Decimal, write_decimal,
    bool, Bool, write_bool,
    (), Null, write_null,
    Date, Date, write_date,
    Time, Time, write_time,
    DateTime, DateTime, write_datetime,
}

fn write_bool(value: bool, data: &mut Vec<u8>) {
    data.push(u8::from(value));
}

fn write_null(_value: (), data: &mut Vec<u8>) {
    data.push(SpudTypes::Null.as_u8());
}

fn write_primitive_value<T: SpudPrimitiveWriter>(value: T, data: &mut Vec<u8>) {
    value.write_primitive(data);
}

fn write_decimal(value: Decimal, data: &mut Vec<u8>) {
    let value_data: [u8; 16] = value.serialize();

    data.extend_from_slice(&value_data);
}

fn write_date(value: Date, data: &mut Vec<u8>) {
    data.extend_from_slice(&value.as_le_bytes());
}

fn write_time(value: Time, data: &mut Vec<u8>) {
    data.extend_from_slice(&value.as_le_bytes());
}

fn write_datetime(value: DateTime, data: &mut Vec<u8>) {
    data.extend_from_slice(&value.as_le_bytes());
}

fn write_slice<T: SpudTypesExt>(slice: &[T], data: &mut Vec<u8>) {
    data.push(SpudTypes::ArrayStart.as_u8());

    for item in slice {
        item.write_spud_bytes(data);
    }

    data.push(SpudTypes::ArrayEnd.as_u8());
}

impl<T: SpudTypesExt> SpudTypesExt for Vec<T> {
    fn write_spud_bytes(&self, data: &mut Vec<u8>) {
        write_slice(self, data);
    }
}

impl<T: SpudTypesExt> SpudTypesExt for &[T] {
    fn write_spud_bytes(&self, data: &mut Vec<u8>) {
        write_slice(self, data);
    }
}

impl<T: SpudTypesExt, const L: usize> SpudTypesExt for &[T; L] {
    fn write_spud_bytes(&self, data: &mut Vec<u8>) {
        write_slice(*self, data);
    }
}

impl SpudTypesExt for SpudString {
    fn write_spud_bytes(&self, data: &mut Vec<u8>) {
        data.push(SpudTypes::String.as_u8());

        add_value_length(data, self.len());

        data.extend_from_slice(self.as_bytes());
    }
}

impl SpudTypesExt for BinaryBlobStruct<'_> {
    fn write_spud_bytes(&self, data: &mut Vec<u8>) {
        data.push(SpudTypes::BinaryBlob.as_u8());

        add_value_length(data, self.len());

        data.extend_from_slice(self.bytes());
    }
}
