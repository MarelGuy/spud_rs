use core::{fmt, ops::Deref};

/// Struct representing a binary blob for SPUD encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryBlob<'a>(&'a [u8]);

impl<'a> BinaryBlob<'a> {
    #[must_use]
    /// Creates a new `BinaryBlob` from a byte slice.
    pub fn new(value: &'a [u8]) -> Self {
        Self(value)
    }

    #[must_use]
    /// Returns the underlying byte slice of the `BinaryBlob`.
    pub fn bytes(&self) -> &'a [u8] {
        self.0
    }

    #[must_use]
    /// Returns the length of the `BinaryBlob`.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    /// Checks if the `BinaryBlob` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    /// Converts the `BinaryBlob` to a `Vec<u8>`.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
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
