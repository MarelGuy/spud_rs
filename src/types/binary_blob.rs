pub struct BinaryBlob<'a>(pub &'a [u8]);

impl<'a> BinaryBlob<'a> {
    #[must_use]
    pub fn new(value: &'a [u8]) -> Self {
        Self(value)
    }
}
