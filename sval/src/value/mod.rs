/*!
A streamable value.
*/

#[macro_use]
mod macros;
mod stream;
mod impls;

pub(crate) mod collect;

#[doc(inline)]
pub use crate::Error;
pub use self::stream::Stream;

pub(crate) use self::stream::stream;

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
