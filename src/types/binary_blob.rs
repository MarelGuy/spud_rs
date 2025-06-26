use core::{fmt, ops::Deref};

/// Struct representing a binary blob for SPUD encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a> Deref for BinaryBlob<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for BinaryBlob<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0 {
            write!(f, "{byte:02x}")?;
        }

        Ok(())
    }
}
