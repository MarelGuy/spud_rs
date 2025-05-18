#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{schema, spud_schema::SpudSchema, spud_types::SpudTypes};

    #[test]
    fn test_empty_schema() {
        let empty_schema = schema!();
        assert_eq!(empty_schema, SpudSchema::default());
    }

    #[test]
    fn test_schema_single_pair() {
        let schema = schema!("key": SpudTypes::String);
        let mut expected = HashMap::new();
        expected.insert("key".to_string(), SpudTypes::String);
        assert_eq!(schema, SpudSchema::from(expected));
    }

    #[test]
    fn test_schema_with_multiple_pairs() {
        let schema = schema! {
            "name": SpudTypes::String,
            "age": SpudTypes::I32,
            "is_student": SpudTypes::Bool,
        };

        let mut expected = HashMap::new();
        expected.insert("name".to_string(), SpudTypes::String);
        expected.insert("age".to_string(), SpudTypes::I32);
        expected.insert("is_student".to_string(), SpudTypes::Bool);

        assert_eq!(schema, SpudSchema::from(expected));
    }

    #[test]
    fn test_schema_with_trailing_comma() {
        let schema_with_comma = schema! {
            "field1": SpudTypes::String,
            "field2": SpudTypes::I32,
        };

        let mut expected_map = HashMap::new();
        expected_map.insert("field1".to_string(), SpudTypes::String);
        expected_map.insert("field2".to_string(), SpudTypes::I32);
        let expected_schema = SpudSchema::from(expected_map);

        assert_eq!(schema_with_comma, expected_schema);
    }

    #[test]
    fn test_schema_macro_string_keys() {
        let schema = schema! {
            "key1": SpudTypes::String,
            "key2": SpudTypes::I32,
        };

        let expected_schema = SpudSchema::from({
            let mut map = HashMap::new();
            map.insert("key1".to_string(), SpudTypes::String);
            map.insert("key2".to_string(), SpudTypes::I32);
            map
        });

        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_schema_with_different_types() {
        let schema = schema! {
            "string_field": SpudTypes::String,
            "number_field": SpudTypes::I32,
            "bool_field": SpudTypes::Bool,
            "null_field": SpudTypes::Null,
            "binary_field": SpudTypes::BinaryBlob,
        };

        let expected_schema = SpudSchema::from(HashMap::from([
            ("string_field".to_string(), SpudTypes::String),
            ("number_field".to_string(), SpudTypes::I32),
            ("bool_field".to_string(), SpudTypes::Bool),
            ("null_field".to_string(), SpudTypes::Null),
            ("binary_field".to_string(), SpudTypes::BinaryBlob),
        ]));

        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_schema_duplicate_keys() {
        let schema = schema!(
            "key1": SpudTypes::String,
            "key2": SpudTypes::I32,
            "key1": SpudTypes::Bool
        );

        let expected = {
            let mut map = HashMap::new();
            map.insert("key1".to_string(), SpudTypes::Bool);
            map.insert("key2".to_string(), SpudTypes::I32);
            SpudSchema::from(map)
        };

        assert_eq!(schema, expected);
    }

    #[test]
    fn test_schema_macro_with_whitespace_and_newlines() {
        let schema = schema! {
            "key1"  :  SpudTypes::String,
            "key2" : SpudTypes::I32,

            "key3":SpudTypes::Bool,
        };

        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), SpudTypes::String);
        expected.insert("key2".to_string(), SpudTypes::I32);
        expected.insert("key3".to_string(), SpudTypes::Bool);

        assert_eq!(schema, SpudSchema::from(expected));
    }

    #[test]
    fn test_schema_with_special_characters() {
        let schema = schema! {
            "key with spaces": SpudTypes::String,
            "key-with-dashes": SpudTypes::I32,
            "key_with_underscores": SpudTypes::Bool,
            "key.with.dots": SpudTypes::Null,
            "key@with@at": SpudTypes::BinaryBlob,
        };

        let mut expected = HashMap::new();
        expected.insert("key with spaces".to_string(), SpudTypes::String);
        expected.insert("key-with-dashes".to_string(), SpudTypes::I32);
        expected.insert("key_with_underscores".to_string(), SpudTypes::Bool);
        expected.insert("key.with.dots".to_string(), SpudTypes::Null);
        expected.insert("key@with@at".to_string(), SpudTypes::BinaryBlob);

        assert_eq!(schema, SpudSchema::from(expected));
    }
}
