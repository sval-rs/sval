#[doc(inline)]
pub use crate::Error;

use crate::std::fmt;

pub use self::fmt::Arguments;

/**
The expected position in the stream.
*/
#[derive(Clone, Copy)]
pub enum Pos {
    /** The root of the stream. */
    Root,
    /** A key within a map. */
    Key,
    /** A value within a map. */
    Value,
    /** An element within a sequence. */
    Elem,
}

/**
A value stream.

The `Stream` trait has a flat structure, but it may need to work with
nested values. The caller is expected to provide a [`Pos`] that tells
the stream how to interpret the values it's being given.
*/
pub trait Stream {
    /** Begin the stream. */
    fn begin(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /** Stream a format. */
    fn fmt(&mut self, pos: Pos, args: Arguments) -> Result<(), Error>;

    /** Stream a signed integer. */
    fn i64(&mut self, pos: Pos, v: i64) -> Result<(), Error> {
        self.fmt(pos, format_args!("{:?}", v))
    }

    /** Stream an unsigned integer. */
    fn u64(&mut self, pos: Pos, v: u64) -> Result<(), Error> {
        self.fmt(pos, format_args!("{:?}", v))
    }

    /** Stream a 128bit signed integer. */
    fn i128(&mut self, pos: Pos, v: i128) -> Result<(), Error> {
        self.fmt(pos, format_args!("{:?}", v))
    }

    /** Stream a 128bit unsigned integer. */
    fn u128(&mut self, pos: Pos, v: u128) -> Result<(), Error> {
        self.fmt(pos, format_args!("{:?}", v))
    }

    /** Stream a floating point value. */
    fn f64(&mut self, pos: Pos, v: f64) -> Result<(), Error> {
        self.fmt(pos, format_args!("{:?}", v))
    }

    /** Stream a boolean. */
    fn bool(&mut self, pos: Pos, v: bool) -> Result<(), Error> {
        self.fmt(pos, format_args!("{:?}", v))
    }

    /** Stream a unicode character. */
    fn char(&mut self, pos: Pos, v: char) -> Result<(), Error> {
        let mut b = [0; 4];
        self.str(pos, &*v.encode_utf8(&mut b))
    }

    /** Stream a UTF-8 string slice. */
    fn str(&mut self, pos: Pos, v: &str) -> Result<(), Error> {
        self.fmt(pos, format_args!("{:?}", v))
    }

    /** Stream an empty value. */
    fn none(&mut self, pos: Pos) -> Result<(), Error> {
        self.fmt(pos, format_args!("{:?}", ()))
    }

    /** Begin a map. */
    fn map_begin(&mut self, pos: Pos, len: Option<usize>) -> Result<(), Error> {
        let _ = pos;
        let _ = len;

        Ok(())
    }

    /** End a map. */
    fn map_end(&mut self, pos: Pos) -> Result<(), Error> {
        let _ = pos;

        Ok(())
    }

    /** Begin a sequence. */
    fn seq_begin(&mut self, pos: Pos, len: Option<usize>) -> Result<(), Error> {
        let _ = pos;
        let _ = len;

        Ok(())
    }

    /** End a sequence. */
    fn seq_end(&mut self, pos: Pos) -> Result<(), Error> {
        let _ = pos;

        Ok(())
    }

    /** End the stream. */
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

    fn fmt(&mut self, pos: Pos, args: Arguments) -> Result<(), Error> {
        (**self).fmt(pos, args)
    }

    fn i64(&mut self, pos: Pos, v: i64) -> Result<(), Error> {
        (**self).i64(pos, v)
    }

    fn u64(&mut self, pos: Pos, v: u64) -> Result<(), Error> {
        (**self).u64(pos, v)
    }

    fn i128(&mut self, pos: Pos, v: i128) -> Result<(), Error> {
        (**self).i128(pos, v)
    }

    fn u128(&mut self, pos: Pos, v: u128) -> Result<(), Error> {
        (**self).u128(pos, v)
    }

    fn f64(&mut self, pos: Pos, v: f64) -> Result<(), Error> {
        (**self).f64(pos, v)
    }

    fn bool(&mut self, pos: Pos, v: bool) -> Result<(), Error> {
        (**self).bool(pos, v)
    }

    fn char(&mut self, pos: Pos, v: char) -> Result<(), Error> {
        (**self).char(pos, v)
    }

    fn str(&mut self, pos: Pos, v: &str) -> Result<(), Error> {
        (**self).str(pos, v)
    }

    fn none(&mut self, pos: Pos) -> Result<(), Error> {
        (**self).none(pos)
    }

    fn map_begin(&mut self, pos: Pos, len: Option<usize>) -> Result<(), Error> {
        (**self).map_begin(pos, len)
    }

    fn map_end(&mut self, pos: Pos) -> Result<(), Error> {
        (**self).map_end(pos)
    }

    fn seq_begin(&mut self, pos: Pos, len: Option<usize>) -> Result<(), Error> {
        (**self).seq_begin(pos, len)
    }

    fn seq_end(&mut self, pos: Pos) -> Result<(), Error> {
        (**self).seq_end(pos)
    }

    fn end(&mut self) -> Result<(), Error> {
        (**self).end()
    }
}
