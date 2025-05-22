pub struct SpudString(pub String);

impl From<&str> for SpudString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for SpudString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&String> for SpudString {
    fn from(value: &String) -> Self {
        Self(value.clone())
    }
}
