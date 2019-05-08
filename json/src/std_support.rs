use sval::stream::{
    self,
    Stream,
};

use crate::std::string::String;
use crate::{
    fmt::Formatter,
    std::{
        fmt,
        io::Write,
    },
};

/**
Write a [`sval::Value`] to a string.
*/
pub fn to_string(v: impl sval::Value) -> Result<String, sval::Error> {
    let mut out = String::new();

    crate::to_fmt(&mut out, v)?;

    Ok(out)
}

/**
Write a [`sval::Value`] to a writer.
*/
pub fn to_writer(writer: impl Write, v: impl sval::Value) -> Result<(), sval::Error> {
    crate::to_fmt(FmtToIo(writer), v)
}

struct FmtToIo<W>(W);

impl<W> fmt::Write for FmtToIo<W>
where
    W: Write,
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.0.write(s.as_bytes()).map_err(|_| fmt::Error)?;

        Ok(())
    }
}

/**
A stream for writing structured data as json.

The stream internally wraps a [`std::io::Write`].

# Examples

Create an owned json stream:

```
# fn main() -> Result<(), Box<std::error::Error>> {
# use std::str;
use sval::stream::OwnedStream;
use sval_json::Writer;

let mut stream = OwnedStream::begin(Writer::new(Vec::<u8>::new()))?;
stream.any(42)?;
let json = stream.end()?.into_inner();

assert_eq!(Some("42"), str::from_utf8(&json).ok());
# Ok(())
# }
```
*/
pub struct Writer<W>(Formatter<FmtToIo<W>>);

impl<W> Writer<W>
where
    W: Write,
{
    /**
    Create a new json stream.
    */
    pub fn new(out: W) -> Self {
        Writer(Formatter::new(FmtToIo(out)))
    }

    /**
    Get the inner writer back out of the stream.

    There is no validation done to ensure the data written is valid.
    */
    pub fn into_inner(self) -> W {
        self.0.into_inner().0
    }
}

impl<W> Stream for Writer<W>
where
    W: Write,
{
    #[inline]
    fn begin(&mut self) -> stream::Result {
        self.0.begin()
    }

    #[inline]
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.0.fmt(v)
    }

    #[inline]
    fn i64(&mut self, v: i64) -> stream::Result {
        self.0.i64(v)
    }

    #[inline]
    fn u64(&mut self, v: u64) -> stream::Result {
        self.0.u64(v)
    }

    #[inline]
    fn f64(&mut self, v: f64) -> stream::Result {
        self.0.f64(v)
    }

    #[inline]
    fn bool(&mut self, v: bool) -> stream::Result {
        self.0.bool(v)
    }

    #[inline]
    fn char(&mut self, v: char) -> stream::Result {
        self.0.char(v)
    }

    #[inline]
    fn str(&mut self, v: &str) -> stream::Result {
        self.0.str(v)
    }

    #[inline]
    fn none(&mut self) -> stream::Result {
        self.0.none()
    }

    #[inline]
    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.seq_begin(len)
    }

    #[inline]
    fn seq_elem(&mut self) -> stream::Result {
        self.0.seq_elem()
    }

    #[inline]
    fn seq_end(&mut self) -> stream::Result {
        self.0.seq_end()
    }

    #[inline]
    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.map_begin(len)
    }

    #[inline]
    fn map_key(&mut self) -> stream::Result {
        self.0.map_key()
    }

    #[inline]
    fn map_value(&mut self) -> stream::Result {
        self.0.map_value()
    }

    #[inline]
    fn map_end(&mut self) -> stream::Result {
        self.0.map_end()
    }

    #[inline]
    fn end(&mut self) -> stream::Result {
        self.0.end()
    }
}
