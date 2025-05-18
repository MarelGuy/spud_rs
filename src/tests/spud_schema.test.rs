#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        schema,
        spud_schema::{SpudSchema, spud_schema_types::SpudSchemaTypes},
    };

    #[test]
    fn test_empty_schema() {
        let empty_schema = schema!();
        assert_eq!(empty_schema, SpudSchema::default());
    }

    #[test]
    fn test_schema_single_pair() {
        let schema = schema!("key": SpudSchemaTypes::String);
        let mut expected = HashMap::new();
        expected.insert("key".to_string(), SpudSchemaTypes::String);
        assert_eq!(schema, SpudSchema::from(expected));
    }

    #[test]
    fn test_schema_with_multiple_pairs() {
        let schema = schema! {
            "name": SpudSchemaTypes::String,
            "age": SpudSchemaTypes::Number,
            "is_student": SpudSchemaTypes::Bool,
        };

        let mut expected = HashMap::new();
        expected.insert("name".to_string(), SpudSchemaTypes::String);
        expected.insert("age".to_string(), SpudSchemaTypes::Number);
        expected.insert("is_student".to_string(), SpudSchemaTypes::Bool);

        assert_eq!(schema, SpudSchema::from(expected));
    }

    #[test]
    fn test_schema_with_trailing_comma() {
        let schema_with_comma = schema! {
            "field1": SpudSchemaTypes::String,
            "field2": SpudSchemaTypes::Number,
        };

        let mut expected_map = HashMap::new();
        expected_map.insert("field1".to_string(), SpudSchemaTypes::String);
        expected_map.insert("field2".to_string(), SpudSchemaTypes::Number);
        let expected_schema = SpudSchema::from(expected_map);

        assert_eq!(schema_with_comma, expected_schema);
    }

    #[test]
    fn test_schema_macro_string_keys() {
        let schema = schema! {
            "key1": SpudSchemaTypes::String,
            "key2": SpudSchemaTypes::Number,
        };

        let expected_schema = SpudSchema::from({
            let mut map = HashMap::new();
            map.insert("key1".to_string(), SpudSchemaTypes::String);
            map.insert("key2".to_string(), SpudSchemaTypes::Number);
            map
        });

        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_schema_with_different_types() {
        let schema = schema! {
            "string_field": SpudSchemaTypes::String,
            "number_field": SpudSchemaTypes::Number,
            "bool_field": SpudSchemaTypes::Bool,
            "null_field": SpudSchemaTypes::Null,
            "binary_field": SpudSchemaTypes::BinaryBlob,
        };

        let expected_schema = SpudSchema::from(HashMap::from([
            ("string_field".to_string(), SpudSchemaTypes::String),
            ("number_field".to_string(), SpudSchemaTypes::Number),
            ("bool_field".to_string(), SpudSchemaTypes::Bool),
            ("null_field".to_string(), SpudSchemaTypes::Null),
            ("binary_field".to_string(), SpudSchemaTypes::BinaryBlob),
        ]));

        assert_eq!(schema, expected_schema);
    }

    #[test]
    fn test_schema_duplicate_keys() {
        let schema = schema!(
            "key1": SpudSchemaTypes::String,
            "key2": SpudSchemaTypes::Number,
            "key1": SpudSchemaTypes::Bool
        );

        let expected = {
            let mut map = HashMap::new();
            map.insert("key1".to_string(), SpudSchemaTypes::Bool);
            map.insert("key2".to_string(), SpudSchemaTypes::Number);
            SpudSchema::from(map)
        };

        assert_eq!(schema, expected);
    }

    #[test]
    fn test_schema_macro_with_whitespace_and_newlines() {
        let schema = schema! {
            "key1"  :  SpudSchemaTypes::String,
            "key2" : SpudSchemaTypes::Number,

            "key3":SpudSchemaTypes::Bool,
        };

        let mut expected = HashMap::new();
        expected.insert("key1".to_string(), SpudSchemaTypes::String);
        expected.insert("key2".to_string(), SpudSchemaTypes::Number);
        expected.insert("key3".to_string(), SpudSchemaTypes::Bool);

        assert_eq!(schema, SpudSchema::from(expected));
    }

    #[test]
    fn test_schema_with_special_characters() {
        let schema = schema! {
            "key with spaces": SpudSchemaTypes::String,
            "key-with-dashes": SpudSchemaTypes::Number,
            "key_with_underscores": SpudSchemaTypes::Bool,
            "key.with.dots": SpudSchemaTypes::Null,
            "key@with@at": SpudSchemaTypes::BinaryBlob,
        };

        let mut expected = HashMap::new();
        expected.insert("key with spaces".to_string(), SpudSchemaTypes::String);
        expected.insert("key-with-dashes".to_string(), SpudSchemaTypes::Number);
        expected.insert("key_with_underscores".to_string(), SpudSchemaTypes::Bool);
        expected.insert("key.with.dots".to_string(), SpudSchemaTypes::Null);
        expected.insert("key@with@at".to_string(), SpudSchemaTypes::BinaryBlob);

        assert_eq!(schema, SpudSchema::from(expected));
    }
}
