mod builder;
mod object;

pub use builder::SpudBuilderSync;
pub use object::SpudObjectSync;

#[cfg(all(test, feature = "sync"))]
mod tests {
    use core::str::FromStr;

    use std::sync::MutexGuard;

    use crate::{
        SpudBuilderSync, SpudObjectSync,
        spud_types::{SpudNumberTypes, SpudTypes},
        types::{BinaryBlob, SpudString},
    };

    #[test]
    fn test_spud_builder_new() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        assert!(builder.field_names.lock().unwrap().is_empty());
        assert!(builder.data.lock().unwrap().is_empty());
        assert!(builder.objects.lock().unwrap().0.is_empty());

        assert_eq!(builder.seen_ids.lock().unwrap().len(), 256);
    }

    #[test]
    fn test_spud_builder_object_empty() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder.object(|_| Ok(())).unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[0..2],
            [
                SpudTypes::ObjectStart.as_u8(),
                SpudTypes::ObjectStart.as_u8()
            ]
        );
    }

    #[test]
    fn test_spud_builder_object_null() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("null", ())?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 1], SpudTypes::Null.as_u8());
    }

    #[test]
    fn test_spud_builder_object_bool() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("bool", true)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 2], SpudTypes::Bool.as_u8());
        assert_eq!(data[data.len() - 1], 1);
    }

    #[test]
    fn test_spud_builder_object_u8() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("u8", 42u8)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 2],
            SpudTypes::Number(SpudNumberTypes::U8).as_u8()
        );
        assert_eq!(data[data.len() - 1], 42);
    }

    #[test]
    fn test_spud_builder_object_u16() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("u16", 256u16)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 3],
            SpudTypes::Number(SpudNumberTypes::U16).as_u8()
        );
        assert_eq!(data[data.len() - 2..data.len()], [0, 1]);
    }

    #[test]
    fn test_spud_builder_object_u32() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("u32", 65536u32)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 5],
            SpudTypes::Number(SpudNumberTypes::U32).as_u8()
        );
        assert_eq!(data[data.len() - 4..data.len()], [0, 0, 1, 0]);
    }

    #[test]
    fn test_spud_builder_object_u64() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("u64", 4_294_967_296u64)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 9],
            SpudTypes::Number(SpudNumberTypes::U64).as_u8()
        );
        assert_eq!(data[data.len() - 8..data.len()], [0, 0, 0, 0, 1, 0, 0, 0]);
    }

    #[test]
    fn test_spud_builder_object_u128() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("u128", 18_446_744_073_709_551_616u128)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 17],
            SpudTypes::Number(SpudNumberTypes::U128).as_u8()
        );
        assert_eq!(
            data[data.len() - 16..data.len()],
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_spud_builder_object_i8() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("i8", -128i8)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 2],
            SpudTypes::Number(SpudNumberTypes::I8).as_u8()
        );
        assert_eq!(data[data.len() - 1], 0x80); // -128 in two's complement
    }

    #[test]
    fn test_spud_builder_object_i16() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("i16", -32768i16)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 3],
            SpudTypes::Number(SpudNumberTypes::I16).as_u8()
        );
        assert_eq!(&data[data.len() - 2..data.len()], [0x00, 0x80]); // -32768 in two's complement
    }

    #[test]
    fn test_spud_builder_object_i32() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("i32", -2_147_483_648_i32)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 5],
            SpudTypes::Number(SpudNumberTypes::I32).as_u8()
        );
        assert_eq!(&data[data.len() - 4..data.len()], [0x00, 0x00, 0x00, 0x80]); // -2147483648 in two's complement
    }

    #[test]
    fn test_spud_builder_object_i64() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("i64", -9_223_372_036_854_775_808_i64)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 9],
            SpudTypes::Number(SpudNumberTypes::I64).as_u8()
        );
        assert_eq!(
            &data[data.len() - 8..data.len()],
            [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80]
        ); // -9223372036854775808 in two's complement
    }

    #[test]
    fn test_spud_builder_object_i128() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value(
                    "i128",
                    -170_141_183_460_469_231_731_687_303_715_884_105_728_i128,
                )?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 17],
            SpudTypes::Number(SpudNumberTypes::I128).as_u8()
        );
        assert_eq!(
            &data[data.len() - 16..data.len()],
            [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x80
            ]
        ); // -170141183460469231731687303715884105728 in two's complement
    }

    #[test]
    fn test_spud_builder_object_f32() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("f32", 3.15f32)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 5],
            SpudTypes::Number(SpudNumberTypes::F32).as_u8()
        );
        assert!(
            (f32::from_le_bytes(data[data.len() - 4..data.len()].try_into().unwrap()) - 3.15f32)
                .abs()
                < 0.0001
        );
    }

    #[test]
    fn test_spud_builder_object_f64() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("f64", 3.15f64)?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(
            data[data.len() - 9],
            SpudTypes::Number(SpudNumberTypes::F64).as_u8()
        );
        assert!(
            (f64::from_le_bytes(data[data.len() - 8..data.len()].try_into().unwrap()) - 3.15f64)
                .abs()
                < 0.0001
        );
    }

    #[test]
    fn test_spud_builder_object_decimal() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value(
                    "decimal",
                    rust_decimal::Decimal::from_f32_retain(1.0).unwrap(),
                )?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 17], SpudTypes::Decimal.as_u8());

        let data_decimal_bytes: [u8; 16] = data[data.len() - 16..data.len()].try_into().unwrap();
        let decimal: [u8; 16] = rust_decimal::Decimal::from_f32_retain(1.0)
            .unwrap()
            .serialize();

        assert_eq!(data_decimal_bytes, decimal);
    }

    #[test]
    fn test_spud_builder_object_string() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("string", SpudString::from("Hello, SPUD!"))?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 15], SpudTypes::String.as_u8());
        assert_eq!(data[data.len() - 13], 12);
        assert_eq!(&data[data.len() - 12..data.len()], b"Hello, SPUD!");
    }

    #[test]
    fn test_spud_builder_object_binary_blob() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value(
                    "binary_blob",
                    BinaryBlob::new(&[0x01, 0x02, 0x03, 0x04, 0x05]),
                )?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 8], SpudTypes::BinaryBlob.as_u8());
        assert_eq!(data[data.len() - 6], 5);
        assert_eq!(
            &data[data.len() - 5..data.len()],
            &[0x01, 0x02, 0x03, 0x04, 0x05]
        );
    }

    #[test]
    fn test_spud_builder_object_array_vec() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("array", vec![1u8, 2u8, 3u8])?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 8], SpudTypes::ArrayStart.as_u8());
        assert_eq!(
            data[data.len() - 7..data.len() - 5],
            [SpudTypes::Number(SpudNumberTypes::U8).as_u8(), 1]
        );
        assert_eq!(
            data[data.len() - 5..data.len() - 3],
            [SpudTypes::Number(SpudNumberTypes::U8).as_u8(), 2]
        );
        assert_eq!(
            data[data.len() - 3..data.len() - 1],
            [SpudTypes::Number(SpudNumberTypes::U8).as_u8(), 3]
        );
        assert_eq!(data[data.len() - 1], SpudTypes::ArrayEnd.as_u8());
    }

    #[test]
    fn test_spud_builder_object_array_slice() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("array", &[1u8, 2u8, 3u8])?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 8], SpudTypes::ArrayStart.as_u8());
        assert_eq!(
            data[data.len() - 7..data.len() - 5],
            [SpudTypes::Number(SpudNumberTypes::U8).as_u8(), 1]
        );
        assert_eq!(
            data[data.len() - 5..data.len() - 3],
            [SpudTypes::Number(SpudNumberTypes::U8).as_u8(), 2]
        );
        assert_eq!(
            data[data.len() - 3..data.len() - 1],
            [SpudTypes::Number(SpudNumberTypes::U8).as_u8(), 3]
        );
        assert_eq!(data[data.len() - 1], SpudTypes::ArrayEnd.as_u8());
    }

    #[test]
    fn test_spud_builder_object_array_vec_slice() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                let vec: Vec<u8> = vec![1u8, 2u8, 3u8];

                obj.add_value("array", vec.as_slice())?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 8], SpudTypes::ArrayStart.as_u8());
        assert_eq!(
            data[data.len() - 7..data.len() - 5],
            [SpudTypes::Number(SpudNumberTypes::U8).as_u8(), 1]
        );
        assert_eq!(
            data[data.len() - 5..data.len() - 3],
            [SpudTypes::Number(SpudNumberTypes::U8).as_u8(), 2]
        );
        assert_eq!(
            data[data.len() - 3..data.len() - 1],
            [SpudTypes::Number(SpudNumberTypes::U8).as_u8(), 3]
        );
        assert_eq!(data[data.len() - 1], SpudTypes::ArrayEnd.as_u8());
    }

    #[test]
    fn test_spud_builder_object_date() {
        use crate::types::Date;

        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("date", Date::from_str("2023-10-01").unwrap())?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 5], SpudTypes::Date.as_u8());
        assert_eq!(
            &data[data.len() - 4..data.len()],
            &Date::from_str("2023-10-01").unwrap().as_le_bytes()
        );
    }

    #[test]
    fn test_spud_builder_object_time() {
        use crate::types::Time;

        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("time", Time::from_str("12:34:56.7890").unwrap())?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 8], SpudTypes::Time.as_u8());
        assert_eq!(
            &data[data.len() - 7..data.len()],
            &Time::from_str("12:34:56.7890").unwrap().as_le_bytes()
        );
    }

    #[test]
    fn test_spud_builder_object_datetime() {
        use crate::types::DateTime;

        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value(
                    "datetime",
                    DateTime::from_str("2023-10-01 12:34:56.7890").unwrap(),
                )?;

                Ok(())
            })
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().unwrap();

        assert_eq!(data[data.len() - 12], SpudTypes::DateTime.as_u8());
        assert_eq!(
            &data[data.len() - 11..data.len()],
            &DateTime::from_str("2023-10-01 12:34:56.7890")
                .unwrap()
                .as_le_bytes()
        );
    }

    #[test]
    fn test_debug_spud_builder() {
        let builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("test", SpudString::from("value"))?;

                Ok(())
            })
            .unwrap();

        let debug_str: String = format!("{builder:?}");

        assert!(debug_str.contains("SpudBuilderSync"));
        assert!(debug_str.contains("field_names"));
        assert!(debug_str.contains("data"));
        assert!(debug_str.contains("objects"));
        assert!(debug_str.contains("seen_ids"));
    }
    #[test]
    fn test_spud_builder_encode_and_build() {
        let mut builder: SpudBuilderSync = SpudBuilderSync::new();

        builder
            .object(|obj: &SpudObjectSync| {
                obj.add_value("test", SpudString::from("value"))?;

                Ok(())
            })
            .unwrap();

        builder.encode().unwrap();
        builder.build_file("./.tmp", "test").unwrap();
    }
}
