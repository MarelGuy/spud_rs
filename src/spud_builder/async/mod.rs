mod builder;
mod object;

pub use builder::SpudBuilderAsync;
pub use object::SpudObjectAsync;

#[cfg(all(test, feature = "async"))]
mod tests {
    use core::str::FromStr;
    use std::sync::Arc;

    use tokio::sync::{Mutex, MutexGuard};

    use crate::{
        SpudBuilderAsync, SpudObjectAsync,
        spud_types::{SpudNumberTypes, SpudTypes},
        types::{BinaryBlob, SpudString},
    };

    #[tokio::test]
    async fn test_spud_builder_new() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        assert!(builder.field_names.lock().await.is_empty());
        assert!(builder.data.lock().await.is_empty());
        assert!(builder.objects.lock().await.0.is_empty());

        assert_eq!(builder.seen_ids.lock().await.len(), 256);
    }

    #[tokio::test]
    async fn test_spud_builder_object_empty() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let _: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[0..2],
            [
                SpudTypes::ObjectStart.as_u8(),
                SpudTypes::ObjectStart.as_u8()
            ]
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_null() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("null", ()).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 1], SpudTypes::Null.as_u8());
    }

    #[tokio::test]
    async fn test_spud_builder_object_bool() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("bool", true).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 2], SpudTypes::Bool.as_u8());
        assert_eq!(data[data.len() - 1], 1);
    }

    #[tokio::test]
    async fn test_spud_builder_object_u8() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("u8", 42u8).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[data.len() - 2],
            SpudTypes::Number(SpudNumberTypes::U8).as_u8()
        );
        assert_eq!(data[data.len() - 1], 42);
    }

    #[tokio::test]
    async fn test_spud_builder_object_u16() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("u16", 256u16).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[data.len() - 3],
            SpudTypes::Number(SpudNumberTypes::U16).as_u8()
        );
        assert_eq!(data[data.len() - 2..data.len()], [0, 1]);
    }

    #[tokio::test]
    async fn test_spud_builder_object_u32() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("u32", 65536u32).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[data.len() - 5],
            SpudTypes::Number(SpudNumberTypes::U32).as_u8()
        );
        assert_eq!(data[data.len() - 4..data.len()], [0, 0, 1, 0]);
    }

    #[tokio::test]
    async fn test_spud_builder_object_u64() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("u64", 4_294_967_296u64).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[data.len() - 9],
            SpudTypes::Number(SpudNumberTypes::U64).as_u8()
        );
        assert_eq!(data[data.len() - 8..data.len()], [0, 0, 0, 0, 1, 0, 0, 0]);
    }

    #[tokio::test]
    async fn test_spud_builder_object_u128() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value("u128", 18_446_744_073_709_551_616u128)
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[data.len() - 17],
            SpudTypes::Number(SpudNumberTypes::U128).as_u8()
        );
        assert_eq!(
            data[data.len() - 16..data.len()],
            [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_i8() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("i8", -128i8).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[data.len() - 2],
            SpudTypes::Number(SpudNumberTypes::I8).as_u8()
        );
        assert_eq!(data[data.len() - 1], 0x80); // -128 in two's complement
    }

    #[tokio::test]
    async fn test_spud_builder_object_i16() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("i16", -32768i16).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[data.len() - 3],
            SpudTypes::Number(SpudNumberTypes::I16).as_u8()
        );
        assert_eq!(&data[data.len() - 2..data.len()], [0x00, 0x80]); // -32768 in two's complement
    }

    #[tokio::test]
    async fn test_spud_builder_object_i32() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("i32", -2_147_483_648_i32).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[data.len() - 5],
            SpudTypes::Number(SpudNumberTypes::I32).as_u8()
        );
        assert_eq!(&data[data.len() - 4..data.len()], [0x00, 0x00, 0x00, 0x80]); // -2147483648 in two's complement
    }

    #[tokio::test]
    async fn test_spud_builder_object_i64() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value("i64", -9_223_372_036_854_775_808_i64)
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[data.len() - 9],
            SpudTypes::Number(SpudNumberTypes::I64).as_u8()
        );
        assert_eq!(
            &data[data.len() - 8..data.len()],
            [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80]
        ); // -9223372036854775808 in two's complement
    }

    #[tokio::test]
    async fn test_spud_builder_object_i128() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value(
                        "i128",
                        -170_141_183_460_469_231_731_687_303_715_884_105_728_i128,
                    )
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

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

    #[tokio::test]
    async fn test_spud_builder_object_f32() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("f32", 3.15f32).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

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

    #[tokio::test]
    async fn test_spud_builder_object_f64() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("f64", 3.15f64).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

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

    #[tokio::test]
    async fn test_spud_builder_object_decimal() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value(
                        "decimal",
                        rust_decimal::Decimal::from_f32_retain(1.0).unwrap(),
                    )
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 17], SpudTypes::Decimal.as_u8());

        let data_decimal_bytes: [u8; 16] = data[data.len() - 16..data.len()].try_into().unwrap();
        let decimal: [u8; 16] = rust_decimal::Decimal::from_f32_retain(1.0)
            .unwrap()
            .serialize();

        assert_eq!(data_decimal_bytes, decimal);
    }

    #[tokio::test]
    async fn test_spud_builder_object_string() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value("string", SpudString::from("Hello, SPUD!"))
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 15], SpudTypes::String.as_u8());
        assert_eq!(data[data.len() - 13], 12);
        assert_eq!(&data[data.len() - 12..data.len()], b"Hello, SPUD!");
    }

    #[tokio::test]
    async fn test_spud_builder_object_binary_blob() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value(
                        "binary_blob",
                        BinaryBlob::new(&[0x01, 0x02, 0x03, 0x04, 0x05]),
                    )
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 8], SpudTypes::BinaryBlob.as_u8());
        assert_eq!(data[data.len() - 6], 5);
        assert_eq!(
            &data[data.len() - 5..data.len()],
            &[0x01, 0x02, 0x03, 0x04, 0x05]
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_array_vec() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value("array", vec![1u8, 2u8, 3u8])
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

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

    #[tokio::test]
    async fn test_spud_builder_object_array_slice() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object.add_value("array", &[1u8, 2u8, 3u8]).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

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

    #[tokio::test]
    async fn test_spud_builder_object_array_vec_slice() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                let vec: Vec<u8> = vec![1u8, 2u8, 3u8];

                locked_object.add_value("array", vec.as_slice()).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

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

    #[tokio::test]
    async fn test_spud_builder_object_date() {
        use crate::types::Date;

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value("date", Date::from_str("2023-10-01").unwrap())
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 5], SpudTypes::Date.as_u8());
        assert_eq!(
            &data[data.len() - 4..data.len()],
            &Date::from_str("2023-10-01").unwrap().as_le_bytes()
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_time() {
        use crate::types::Time;

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value("time", Time::from_str("12:34:56.7890").unwrap())
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 8], SpudTypes::Time.as_u8());
        assert_eq!(
            &data[data.len() - 7..data.len()],
            &Time::from_str("12:34:56.7890").unwrap().as_le_bytes()
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_datetime() {
        use crate::types::DateTime;

        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value(
                        "datetime",
                        DateTime::from_str("2023-10-01 12:34:56.7890").unwrap(),
                    )
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 12], SpudTypes::DateTime.as_u8());
        assert_eq!(
            &data[data.len() - 11..data.len()],
            &DateTime::from_str("2023-10-01 12:34:56.7890")
                .unwrap()
                .as_le_bytes()
        );
    }

    #[tokio::test]
    async fn test_debug_spud_builder() {
        let builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value("test", SpudString::from("value"))
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let debug_str: String = format!("{builder:?}");

        assert!(debug_str.contains("SpudBuilderAsync"));
        assert!(debug_str.contains("field_names"));
        assert!(debug_str.contains("data"));
        assert!(debug_str.contains("objects"));
        assert!(debug_str.contains("seen_ids"));
    }

    #[tokio::test]
    async fn test_spud_builder_encode_and_build() {
        let mut builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value("test", SpudString::from("value"))
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        builder.encode().await.unwrap();
        builder
            .build_file("./.tmp/spud", "async_test")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_spud_builder_encode_and_build_with_objects() {
        let mut builder: SpudBuilderAsync = SpudBuilderAsync::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObjectAsync>>| {
                let locked_object: MutexGuard<'_, SpudObjectAsync> = obj.lock().await;

                locked_object
                    .add_value("test_outside", SpudString::from("value_outside"))
                    .await?;

                locked_object
                    .object(
                        "test_object",
                        async |inner_obj: Arc<Mutex<SpudObjectAsync>>| {
                            let inner_locked_object: MutexGuard<'_, SpudObjectAsync> =
                                inner_obj.lock().await;

                            inner_locked_object
                                .add_value("test_inside", SpudString::from("value_inside"))
                                .await?;

                            Ok(())
                        },
                    )
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        builder.encode().await.unwrap();
        builder
            .build_file("./.tmp/spud", "async_test_with_objects")
            .await
            .unwrap();
    }
}
