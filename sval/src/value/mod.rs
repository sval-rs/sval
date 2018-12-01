/*!
A streamable value.
*/

#[macro_use]
mod macros;
mod impls;
mod stream;

pub(crate) mod collect;

#[cfg(feature = "std")]
pub mod owned;

pub(crate) use self::stream::stream;

pub use self::stream::Stream;

#[doc(inline)]
pub use crate::Error;

/**
A value with a streamable structure.

Use the [`sval::stream`](../fn.stream.html) function to stream a value.
*/
pub trait Value {
    /** Stream this value. */
    fn stream(&self, stream: &mut Stream) -> Result<(), Error>;
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        (**self).stream(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_is_object_safe() {
        fn _safe(_: &dyn Value) {}
    }
}
