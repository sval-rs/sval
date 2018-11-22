#[doc(inline)]
pub use crate::Error;

use std::fmt;

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
    fn fmt(&mut self, pos: Pos, args: fmt::Arguments) -> Result<(), Error>;

    fn u64(&mut self, pos: Pos, v: u64) -> Result<(), Error> {
        self.fmt(pos, format_args!("{:?}", v))
    }

    fn map_begin(&mut self, pos: Pos, len: Option<usize>) -> Result<(), Error> {
        let _ = pos;
        let _ = len;

        Ok(())
    }

    fn map_end(&mut self, pos: Pos) -> Result<(), Error> {
        let _ = pos;

        Ok(())
    }

    fn begin(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn end(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, T: ?Sized> Stream for &'a mut T
where
    T: Stream,
{
    fn fmt(&mut self, pos: Pos, args: std::fmt::Arguments) -> Result<(), Error> {
        (**self).fmt(pos, args)
    }

    fn u64(&mut self, pos: Pos, v: u64) -> Result<(), Error> {
        (**self).u64(pos, v)
    }

    fn map_begin(&mut self, pos: Pos, len: Option<usize>) -> Result<(), Error> {
        (**self).map_begin(pos, len)
    }

    fn map_end(&mut self, pos: Pos) -> Result<(), Error> {
        (**self).map_end(pos)
    }

    fn begin(&mut self) -> Result<(), Error> {
        (**self).begin()
    }

    fn end(&mut self) -> Result<(), Error> {
        (**self).end()
    }
}
