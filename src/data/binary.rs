use crate::{Result, Stream, Value};

/**
An adapter that streams a slice of 8bit unsigned integers as binary.
*/
#[repr(transparent)]
pub struct Binary([u8]);

impl Binary {
    /**
    Treat a slice of 8bit unsigned integers as binary.
    */
    pub fn new<'a>(binary: &'a [u8]) -> &'a Self {
        // SAFETY: `Binary` and `[u8]` have the same ABI
        unsafe { &*(binary as *const _ as *const Binary) }
    }

    /**
    Get a reference to the underlying slice.
    */
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for Binary {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Value for Binary {
    fn stream<'sval, S: Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> Result {
        stream.binary_begin(Some(self.0.len()))?;
        stream.binary_fragment(&self.0)?;
        stream.binary_end()
    }

    fn to_binary(&self) -> Option<&[u8]> {
        Some(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_cast() {
        assert_eq!(Some(b"abc" as &[u8]), Binary::new(b"abc").to_binary());
    }
}
