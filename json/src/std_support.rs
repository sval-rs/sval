use sval::{
    stream::{
        self,
        Stream,
    },
    value::Value,
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
Write a [`Value`] to a string.
*/
pub fn to_string(v: impl Value) -> Result<String, sval::Error> {
    let mut out = String::new();
    crate::to_fmt(&mut out, v)?;

    Ok(out)
}

/**
Write a [`Value`] to a writer.
*/
pub fn to_writer(writer: impl Write, v: impl Value) -> Result<(), sval::Error> {
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
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# use std::str;
use sval_json::Writer;

let mut stream = Writer::new(Vec::<u8>::new());
sval::stream(&mut stream, &42)?;
let json = stream.end()?;

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
    Get the inner writer back out of the stream without ensuring it's valid.
    */
    pub fn into_inner(self) -> W {
        self.0.into_inner().0
    }
}

impl<'v, W> Stream<'v> for Writer<W>
where
    W: Write,
{
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.0.fmt(v)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.0.error(v)
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.0.i64(v)
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.0.u64(v)
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.0.i128(v)
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.0.u128(v)
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.0.f64(v)
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.0.bool(v)
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.0.char(v)
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.0.str(v)
    }

    fn none(&mut self) -> stream::Result {
        self.0.none()
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.map_begin(len)
    }

    fn map_key(&mut self) -> stream::Result {
        self.0.map_key()
    }

    fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
        self.0.map_key_collect(k)
    }

    fn map_value(&mut self) -> stream::Result {
        self.0.map_value()
    }

    fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
        self.0.map_value_collect(v)
    }

    fn map_end(&mut self) -> stream::Result {
        self.0.map_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.seq_begin(len)
    }

    fn seq_elem(&mut self) -> stream::Result {
        self.0.seq_elem()
    }

    fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
        self.0.seq_elem_collect(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        self.0.seq_end()
    }
}
