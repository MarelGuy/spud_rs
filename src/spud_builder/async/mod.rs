mod builder;
mod object;

pub use builder::SpudBuilder;
pub use object::SpudObject;

#[cfg(all(test, feature = "async"))]
mod tests {
    use core::str::FromStr;
    use std::sync::Arc;

    use tokio::sync::{Mutex, MutexGuard};

    use crate::{
        SpudBuilder, SpudObject,
        spud_types::SpudTypes,
        types::{BinaryBlob, SpudString},
    };

    #[tokio::test]
    async fn test_spud_builder_new() {
        let builder: SpudBuilder = SpudBuilder::new();

        assert!(builder.field_names.lock().await.is_empty());
        assert!(builder.data.lock().await.is_empty());
        assert!(builder.objects.lock().await.0.is_empty());

        assert_eq!(builder.seen_ids.lock().await.len(), 256);
    }

    #[tokio::test]
    async fn test_spud_builder_object_empty() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let _: MutexGuard<'_, SpudObject> = obj.lock().await;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(
            data[0..2],
            [SpudTypes::ObjectStart as u8, SpudTypes::ObjectStart as u8]
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_null() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object.add_value("null", ()).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 1], SpudTypes::Null as u8);
    }

    #[tokio::test]
    async fn test_spud_builder_object_bool() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object.add_value("bool", true).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 2], SpudTypes::Bool as u8);
        assert_eq!(data[data.len() - 1], 1);
    }

    #[tokio::test]
    async fn test_spud_builder_object_u8() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object.add_value("u8", 42u8).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 2], SpudTypes::U8 as u8);
        assert_eq!(data[data.len() - 1], 42);
    }
    #[tokio::test]
    async fn test_spud_builder_object_u16() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object.add_value("u16", 256u16).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 3], SpudTypes::U16 as u8);
        assert_eq!(data[data.len() - 2..data.len()], [0, 1]);
    }

    #[tokio::test]
    async fn test_spud_builder_object_u32() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object.add_value("u32", 65536u32).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 5], SpudTypes::U32 as u8);
        assert_eq!(data[data.len() - 4..data.len()], [0, 0, 1, 0]);
    }

    #[tokio::test]
    async fn test_spud_builder_object_u64() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object.add_value("u64", 4_294_967_296u64).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 9], SpudTypes::U64 as u8);
        assert_eq!(data[data.len() - 8..data.len()], [0, 0, 0, 0, 1, 0, 0, 0]);
    }

    #[tokio::test]
    async fn test_spud_builder_object_f32() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object.add_value("f32", 3.15f32).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 5], SpudTypes::F32 as u8);
        assert!(
            (f32::from_le_bytes(data[data.len() - 4..data.len()].try_into().unwrap()) - 3.15f32)
                .abs()
                < 0.0001
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_f64() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object.add_value("f64", 3.15f64).await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 9], SpudTypes::F64 as u8);
        assert!(
            (f64::from_le_bytes(data[data.len() - 8..data.len()].try_into().unwrap()) - 3.15f64)
                .abs()
                < 0.0001
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_decimal() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

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

        assert_eq!(data[data.len() - 17], SpudTypes::Decimal as u8);

        let data_decimal_bytes: [u8; 16] = data[data.len() - 16..data.len()].try_into().unwrap();
        let decimal: [u8; 16] = rust_decimal::Decimal::from_f32_retain(1.0)
            .unwrap()
            .serialize();

        assert_eq!(data_decimal_bytes, decimal);
    }

    #[tokio::test]
    async fn test_spud_builder_object_string() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object
                    .add_value("string", SpudString::from("Hello, SPUD!"))
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 15], SpudTypes::String as u8);
        assert_eq!(data[data.len() - 13], 12);
        assert_eq!(&data[data.len() - 12..data.len()], b"Hello, SPUD!");
    }

    #[tokio::test]
    async fn test_spud_builder_object_binary_blob() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

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

        assert_eq!(data[data.len() - 8], SpudTypes::BinaryBlob as u8);
        assert_eq!(data[data.len() - 6], 5);
        assert_eq!(
            &data[data.len() - 5..data.len()],
            &[0x01, 0x02, 0x03, 0x04, 0x05]
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_array() {
        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object
                    .add_value("array", vec![1u8, 2u8, 3u8])
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 8], SpudTypes::ArrayStart as u8);
        assert_eq!(
            data[data.len() - 7..data.len() - 5],
            [SpudTypes::U8 as u8, 1]
        );
        assert_eq!(
            data[data.len() - 5..data.len() - 3],
            [SpudTypes::U8 as u8, 2]
        );
        assert_eq!(
            data[data.len() - 3..data.len() - 1],
            [SpudTypes::U8 as u8, 3]
        );
        assert_eq!(data[data.len() - 1], SpudTypes::ArrayEnd as u8);
    }

    #[tokio::test]
    async fn test_spud_builder_object_date() {
        use crate::types::Date;

        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object
                    .add_value("date", Date::from_str("2023-10-01").unwrap())
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 5], SpudTypes::Date as u8);
        assert_eq!(
            &data[data.len() - 4..data.len()],
            &Date::from_str("2023-10-01").unwrap().as_le_bytes()
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_time() {
        use crate::types::Time;

        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

                locked_object
                    .add_value("time", Time::from_str("12:34:56.7890").unwrap())
                    .await?;

                Ok(())
            })
            .await
            .unwrap();

        let data: MutexGuard<'_, Vec<u8>> = builder.data.lock().await;

        assert_eq!(data[data.len() - 8], SpudTypes::Time as u8);
        assert_eq!(
            &data[data.len() - 7..data.len()],
            &Time::from_str("12:34:56.7890").unwrap().as_le_bytes()
        );
    }

    #[tokio::test]
    async fn test_spud_builder_object_datetime() {
        use crate::types::DateTime;

        let builder: SpudBuilder = SpudBuilder::new();

        builder
            .object(async |obj: Arc<Mutex<SpudObject>>| {
                let locked_object: MutexGuard<'_, SpudObject> = obj.lock().await;

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

        assert_eq!(data[data.len() - 12], SpudTypes::DateTime as u8);
        assert_eq!(
            &data[data.len() - 11..data.len()],
            &DateTime::from_str("2023-10-01 12:34:56.7890")
                .unwrap()
                .as_le_bytes()
        );
    }
}
