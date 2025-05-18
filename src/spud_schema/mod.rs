use std::collections::HashMap;

use crate::spud_types::SpudTypes;

#[derive(Debug, PartialEq, Default)]
pub struct SpudSchema(pub HashMap<String, SpudTypes>);

impl From<HashMap<String, SpudTypes>> for SpudSchema {
    fn from(m: HashMap<String, SpudTypes>) -> Self {
        Self(m)
    }
}

#[macro_export]
macro_rules! schema {
    () => {
        SpudSchema::from(std::collections::HashMap::<String, SpudTypes>::new())
    };
    ( $( $key:literal : $value:expr ),+ $(,)? ) => {
        {
            let mut map: std::collections::HashMap<String, SpudTypes> = std::collections::HashMap::new();

            $(
                map.insert($key.into(), $value);
            )+

            SpudSchema::from(map)
        }
    };
}
