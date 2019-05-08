/*!
A stream for datastructures.
*/

pub(crate) mod owned;
pub mod stack;

use crate::std::fmt;

#[doc(inline)]
pub use crate::Error;

pub use self::{
    fmt::Arguments,
    owned::OwnedStream,
    stack::Stack,
};

/**
A raw value stream.

The `Stream` trait has a flat, stateless structure, but it may need to work with
nested values. Implementations can use a [`Stack`] to track state for them.

The [`OwnedStream`] type is an ergonomic wrapper over a raw `Stream` that adds
the concept of [`Value`]s.

[`Value`]: ../trait.Value.html
*/
pub trait Stream {
    /**
    Begin the stream.

    This method must be called before interacting with the stream
    in any other way.
    */
    fn begin(&mut self) -> Result {
        Ok(())
    }

    /**
    Stream a format.
    */
    fn fmt(&mut self, args: Arguments) -> Result;

    /**
    Stream a signed integer.
    */
    fn i64(&mut self, v: i64) -> Result {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream an unsigned integer.
    */
    fn u64(&mut self, v: u64) -> Result {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a 128bit signed integer.
    */
    fn i128(&mut self, v: i128) -> Result {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a 128bit unsigned integer.
    */
    fn u128(&mut self, v: u128) -> Result {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a floating point value.
    */
    fn f64(&mut self, v: f64) -> Result {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a boolean.
    */
    fn bool(&mut self, v: bool) -> Result {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a unicode character.
    */
    fn char(&mut self, v: char) -> Result {
        let mut b = [0; 4];
        self.str(&*v.encode_utf8(&mut b))
    }

    /**
    Stream a UTF-8 string slice.
    */
    fn str(&mut self, v: &str) -> Result {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream an empty value.
    */
    fn none(&mut self) -> Result {
        self.fmt(format_args!("{:?}", ()))
    }

    /**
    Begin a map.
    */
    fn map_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Ok(())
    }

    /**
    Begin a map key.

    The key will be implicitly ended by the stream methods that follow it.
    */
    fn map_key(&mut self) -> Result {
        Ok(())
    }

    /**
    Begin a map value.

    The value will be implicitly ended by the stream methods that follow it.
    */
    fn map_value(&mut self) -> Result {
        Ok(())
    }

    /**
    End a map.
    */
    fn map_end(&mut self) -> Result {
        Ok(())
    }

    /**
    Begin a sequence.
    */
    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        let _ = len;
        Ok(())
    }

    /**
    Begin a sequence element.

    The element will be implicitly ended by the stream methods that follow it.
    */
    fn seq_elem(&mut self) -> Result {
        Ok(())
    }

    /**
    End a sequence.
    */
    fn seq_end(&mut self) -> Result {
        Ok(())
    }

    /**
    End the stream.
    */
    fn end(&mut self) -> Result {
        Ok(())
    }
}

impl<'a, T: ?Sized> Stream for &'a mut T
where
    T: Stream,
{
    #[inline]
    fn begin(&mut self) -> Result {
        (**self).begin()
    }

    #[inline]
    fn fmt(&mut self, args: Arguments) -> Result {
        (**self).fmt(args)
    }

    #[inline]
    fn i64(&mut self, v: i64) -> Result {
        (**self).i64(v)
    }

    #[inline]
    fn u64(&mut self, v: u64) -> Result {
        (**self).u64(v)
    }

    #[inline]
    fn i128(&mut self, v: i128) -> Result {
        (**self).i128(v)
    }

    #[inline]
    fn u128(&mut self, v: u128) -> Result {
        (**self).u128(v)
    }

    #[inline]
    fn f64(&mut self, v: f64) -> Result {
        (**self).f64(v)
    }

    #[inline]
    fn bool(&mut self, v: bool) -> Result {
        (**self).bool(v)
    }

    #[inline]
    fn char(&mut self, v: char) -> Result {
        (**self).char(v)
    }

    #[inline]
    fn str(&mut self, v: &str) -> Result {
        (**self).str(v)
    }

    #[inline]
    fn none(&mut self) -> Result {
        (**self).none()
    }

    #[inline]
    fn map_begin(&mut self, len: Option<usize>) -> Result {
        (**self).map_begin(len)
    }

    #[inline]
    fn map_key(&mut self) -> Result {
        (**self).map_key()
    }

    #[inline]
    fn map_value(&mut self) -> Result {
        (**self).map_value()
    }

    #[inline]
    fn map_end(&mut self) -> Result {
        (**self).map_end()
    }

    #[inline]
    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        (**self).seq_begin(len)
    }

    #[inline]
    fn seq_elem(&mut self) -> Result {
        (**self).seq_elem()
    }

    #[inline]
    fn seq_end(&mut self) -> Result {
        (**self).seq_end()
    }

    #[inline]
    fn end(&mut self) -> Result {
        (**self).end()
    }
}

/**
The type returned by streaming methods.
*/
pub type Result = std::result::Result<(), Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_is_object_safe() {
        fn _safe(_: &mut dyn Stream) {}
    }
}
