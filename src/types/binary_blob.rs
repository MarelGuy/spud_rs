/// Struct representing a binary blob for SPUD encoding.
pub struct BinaryBlob<'a>(pub(crate) &'a [u8]);

impl<'a> BinaryBlob<'a> {
    #[must_use]
    pub fn new(value: &'a [u8]) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a [u8]> for BinaryBlob<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self::new(value)
    }
}
