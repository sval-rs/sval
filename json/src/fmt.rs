use sval::{
    stream::{
        self,
        Stream,
    },
    value::Value,
};

use crate::std::fmt::{
    self,
    Write,
};

/**
Write a [`Value`] to a formatter.
*/
pub fn to_fmt(fmt: impl Write, v: impl Value) -> Result<(), sval::Error> {
    sval::stream_owned(Formatter::new(fmt), v)
}

/**
A stream for writing structured data as json.

The stream internally wraps a [`std::fmt::Write`].

# Examples

Create an owned json stream:

```
# #[cfg(not(feature = "std"))]
# fn main() {}
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use sval_json::Formatter;

let mut stream = Formatter::new(String::new());
sval::stream(&mut stream, &42)?;
let json = stream.end()?;

assert_eq!("42", json);
# Ok(())
# }
```
*/
pub struct Formatter<W> {
    is_key: bool,
    is_current_depth_empty: bool,
    out: W,
}

impl<W> Formatter<W>
where
    W: Write,
{
    /**
    Create a new json stream.
    */
    pub fn new(out: W) -> Self {
        Formatter {
            is_key: false,
            is_current_depth_empty: true,
            out,
        }
    }

    /**
    Get the inner writer back out of the stream without ensuring it's valid.
    */
    pub fn into_inner(self) -> W {
        self.out
    }
}

impl<'v, W> Stream<'v> for Formatter<W>
where
    W: Write,
{
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.out.write_char('"')?;
        fmt::write(&mut Escape(&mut self.out), format_args!("{}", v))?;
        self.out.write_char('"')?;

        Ok(())
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.fmt(stream::Arguments::display(&v))
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.out.write_str(ryu::Buffer::new().format(v))?;

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.out.write_str(if v { "true" } else { "false" })?;

        Ok(())
    }

    fn char(&mut self, v: char) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.out.write_char(v)?;

        Ok(())
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.out.write_char('"')?;
        escape_str(&v, &mut self.out)?;
        self.out.write_char('"')?;

        Ok(())
    }

    fn none(&mut self) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.out.write_str("null")?;

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.is_current_depth_empty = true;
        self.out.write_char('{')?;

        Ok(())
    }

    fn map_key(&mut self) -> stream::Result {
        self.is_key = true;

        if !self.is_current_depth_empty {
            self.out.write_char(',')?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
        self.map_key()?;
        k.stream(self)?;

        Ok(())
    }

    fn map_value(&mut self) -> stream::Result {
        self.is_key = false;

        self.out.write_char(':')?;

        Ok(())
    }

    fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
        self.map_value()?;
        v.stream(self)?;

        Ok(())
    }

    fn map_end(&mut self) -> stream::Result {
        self.is_current_depth_empty = false;

        self.out.write_char('}')?;

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
        if self.is_key {
            return Err(sval::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        self.is_current_depth_empty = true;

        self.out.write_char('[')?;

        Ok(())
    }

    fn seq_elem(&mut self) -> stream::Result {
        if !self.is_current_depth_empty {
            self.out.write_char(',')?;
        }

        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
        self.seq_elem()?;
        v.stream(self)?;

        Ok(())
    }

    fn seq_end(&mut self) -> stream::Result {
        self.is_current_depth_empty = false;

        self.out.write_char(']')?;

        Ok(())
    }
}

/*
This `escape_str` implementation has been shamelessly lifted from dtolnay's `miniserde`:
https://github.com/dtolnay/miniserde
*/

fn escape_str(value: &str, mut out: impl Write) -> Result<(), fmt::Error> {
    let bytes = value.as_bytes();
    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }

        if start < i {
            out.write_str(&value[start..i])?;
        }

        match escape {
            self::BB => out.write_str("\\b")?,
            self::TT => out.write_str("\\t")?,
            self::NN => out.write_str("\\n")?,
            self::FF => out.write_str("\\f")?,
            self::RR => out.write_str("\\r")?,
            self::QU => out.write_str("\\\"")?,
            self::BS => out.write_str("\\\\")?,
            self::U => {
                static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
                out.write_str("\\u00")?;
                out.write_char(HEX_DIGITS[(byte >> 4) as usize] as char)?;
                out.write_char(HEX_DIGITS[(byte & 0xF) as usize] as char)?;
            }
            _ => unreachable!(),
        }

        start = i + 1;
    }

    if start != bytes.len() {
        out.write_str(&value[start..])?;
    }

    Ok(())
}

const BB: u8 = b'b'; // \x08
const TT: u8 = b't'; // \x09
const NN: u8 = b'n'; // \x0A
const FF: u8 = b'f'; // \x0C
const RR: u8 = b'r'; // \x0D
const QU: u8 = b'"'; // \x22
const BS: u8 = b'\\'; // \x5C
const U: u8 = b'u'; // \x00...\x1F except the ones above

// Lookup table of escape sequences. A value of b'x' at index i means that byte
// i is escaped as "\x" in JSON. A value of 0 means that byte i is not escaped.
#[rustfmt::skip]
static ESCAPE: [u8; 256] = [
    //  1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    U,  U,  U,  U,  U,  U,  U,  U, BB, TT, NN,  U, FF, RR,  U,  U, // 0
    U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U,  U, // 1
    0,  0, QU,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 2
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 3
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 4
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, BS,  0,  0,  0, // 5
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 6
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 7
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 8
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // 9
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // A
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // B
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // C
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // D
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // E
    0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0, // F
];

struct Escape<W>(W);

impl<W> Write for Escape<W>
where
    W: Write,
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        escape_str(s, &mut self.0)
    }
}
