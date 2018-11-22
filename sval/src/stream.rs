#[doc(inline)]
pub use crate::Error;

use std::fmt;

#[derive(Clone, Copy)]
pub enum Expect {
    Root,
    Key,
    Value,
    Elem,
}

pub trait Stream {
    fn fmt(&mut self, expect: Expect, args: fmt::Arguments) -> Result<(), Error>;

    fn u64(&mut self, expect: Expect, v: u64) -> Result<(), Error> {
        self.fmt(expect, format_args!("{:?}", v))
    }

    fn map_begin(&mut self, _: Expect, _: Option<usize>) -> Result<(), Error> {
        Ok(())
    }

    fn map_end(&mut self, _: Expect) -> Result<(), Error> {
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
    fn fmt(&mut self, expect: Expect, args: std::fmt::Arguments) -> Result<(), Error> {
        (**self).fmt(expect, args)
    }

    fn u64(&mut self, expect: Expect, v: u64) -> Result<(), Error> {
        (**self).u64(expect, v)
    }

    fn map_begin(&mut self, expect: Expect, len: Option<usize>) -> Result<(), Error> {
        (**self).map_begin(expect, len)
    }

    fn map_end(&mut self, expect: Expect) -> Result<(), Error> {
        (**self).map_end(expect)
    }

    fn end(&mut self) -> Result<(), Error> {
        (**self).end()
    }
}
