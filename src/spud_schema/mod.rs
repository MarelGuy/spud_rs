pub mod spud_schema_types;

use std::collections::HashMap;

use spud_schema_types::SpudSchemaTypes;

#[derive(Debug, PartialEq, Default)]
pub struct SpudSchema(pub HashMap<String, SpudSchemaTypes>);

impl From<HashMap<String, SpudSchemaTypes>> for SpudSchema {
    fn from(m: HashMap<String, SpudSchemaTypes>) -> Self {
        Self(m)
    }
}

#[macro_export]
macro_rules! schema {
    () => {
        SpudSchema::from(std::collections::HashMap::<String, SpudSchemaTypes>::new())
    };
    ( $( $key:literal : $value:expr ),+ $(,)? ) => {
        {
            let mut map: std::collections::HashMap<String, SpudSchemaTypes> = std::collections::HashMap::new();

            $(
                map.insert($key.into(), $value);
            )+

            SpudSchema::from(map)
        }
    };
}
