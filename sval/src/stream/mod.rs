/*!
A stream for datastructures.
*/

mod stack;

use crate::std::fmt;

#[doc(inline)]
pub use crate::Error;

pub use self::{
    fmt::Arguments,
    stack::{
        Pos,
        Stack,
    },
};

/**
A value stream.

The `Stream` trait has a flat, stateless structure, but it may need to work with
nested values. Implementations can use a [`Stack`] to track state for them.
*/
pub trait Stream {
    /**
    Begin the stream.

    This method must be called before interacting with the stream
    in any other way.
    */
    fn begin(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /**
    Stream a format.
    */
    fn fmt(&mut self, args: Arguments) -> Result<(), Error>;

    /**
    Stream a signed integer.
    */
    fn i64(&mut self, v: i64) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream an unsigned integer.
    */
    fn u64(&mut self, v: u64) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a 128bit signed integer.
    */
    fn i128(&mut self, v: i128) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a 128bit unsigned integer.
    */
    fn u128(&mut self, v: u128) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a floating point value.
    */
    fn f64(&mut self, v: f64) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a boolean.
    */
    fn bool(&mut self, v: bool) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream a unicode character.
    */
    fn char(&mut self, v: char) -> Result<(), Error> {
        let mut b = [0; 4];
        self.str(&*v.encode_utf8(&mut b))
    }

    /**
    Stream a UTF-8 string slice.
    */
    fn str(&mut self, v: &str) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", v))
    }

    /**
    Stream an empty value.
    */
    fn none(&mut self) -> Result<(), Error> {
        self.fmt(format_args!("{:?}", ()))
    }

    /**
    Begin a map.
    */
    fn map_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        let _ = len;
        Ok(())
    }

    /**
    Begin a map key.

    The key will be implicitly ended by the stream methods that follow it.
    */
    fn map_key(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /**
    Begin a map value.

    The value will be implicitly ended by the stream methods that follow it.
    */
    fn map_value(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /**
    End a map.
    */
    fn map_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /**
    Begin a sequence.
    */
    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        let _ = len;
        Ok(())
    }

    /**
    Begin a sequence element.

    The element will be implicitly ended by the stream methods that follow it.
    */
    fn seq_elem(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /**
    End a sequence.
    */
    fn seq_end(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /**
    End the stream.
    */
    fn end(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, T: ?Sized> Stream for &'a mut T
where
    T: Stream,
{
    fn begin(&mut self) -> Result<(), Error> {
        (**self).begin()
    }

    fn fmt(&mut self, args: Arguments) -> Result<(), Error> {
        (**self).fmt(args)
    }

    fn i64(&mut self, v: i64) -> Result<(), Error> {
        (**self).i64(v)
    }

    fn u64(&mut self, v: u64) -> Result<(), Error> {
        (**self).u64(v)
    }

    fn i128(&mut self, v: i128) -> Result<(), Error> {
        (**self).i128(v)
    }

    fn u128(&mut self, v: u128) -> Result<(), Error> {
        (**self).u128(v)
    }

    fn f64(&mut self, v: f64) -> Result<(), Error> {
        (**self).f64(v)
    }

    fn bool(&mut self, v: bool) -> Result<(), Error> {
        (**self).bool(v)
    }

    fn char(&mut self, v: char) -> Result<(), Error> {
        (**self).char(v)
    }

    fn str(&mut self, v: &str) -> Result<(), Error> {
        (**self).str(v)
    }

    fn none(&mut self) -> Result<(), Error> {
        (**self).none()
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        (**self).map_begin(len)
    }

    fn map_key(&mut self) -> Result<(), Error> {
        (**self).map_key()
    }

    fn map_value(&mut self) -> Result<(), Error> {
        (**self).map_value()
    }

    fn map_end(&mut self) -> Result<(), Error> {
        (**self).map_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        (**self).seq_begin(len)
    }

    fn seq_elem(&mut self) -> Result<(), Error> {
        (**self).seq_elem()
    }

    fn seq_end(&mut self) -> Result<(), Error> {
        (**self).seq_end()
    }

    fn end(&mut self) -> Result<(), Error> {
        (**self).end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_is_object_safe() {
        fn _safe(_: &mut dyn Stream) {}
    }
}
