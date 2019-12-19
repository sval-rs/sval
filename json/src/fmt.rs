use sval::stream::{
    self,
    stack,
    Stack,
    Stream,
};

use crate::{
    std::{
        fmt::{
            self,
            Write,
        },
        mem,
    },
    End,
};

/**
Write a [`sval::Value`] to a formatter.
*/
pub fn to_fmt<W>(fmt: W, v: impl sval::Value) -> Result<W, sval::Error>
where
    W: Write,
{
    let fmt = Formatter::new(fmt);

    sval::stream(fmt, v).map(|fmt| fmt.into_inner_unchecked())
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
use sval::stream::OwnedStream;
use sval_json::Formatter;

let mut stream = OwnedStream::new(Formatter::new(String::new()));
stream.any(42)?;
let json = stream.into_inner().end()?;

assert_eq!("42", json);
# Ok(())
# }
```
*/
pub struct Formatter<W> {
    stack: Stack,
    delim: Option<char>,
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
            stack: Stack::new(),
            delim: None,
            out,
        }
    }

    /**
    Whether the stream has seen a complete, valid json structure.
    */
    pub fn is_valid(&self) -> bool {
        self.stack.can_end()
    }

    /**
    Complete the stream and return the inner writer.

    If the writer contains incomplete json then this method will fail.
    The returned error can be used to pull the original stream back out.
    */
    pub fn end(mut self) -> Result<W, End<Self>> {
        match self.stack.end() {
            Ok(()) => Ok(self.out),
            Err(e) => Err(End::new(e, self)),
        }
    }

    /**
    Get the inner writer back out of the stream without ensuring it's valid.
    */
    pub fn into_inner_unchecked(self) -> W {
        self.out
    }

    #[inline]
    fn next_delim(pos: stack::Pos) -> Option<char> {
        if pos.is_value() || pos.is_elem() {
            return Some(',');
        }

        if pos.is_key() {
            return Some(':');
        }

        return None;
    }
}

impl<W> Stream for Formatter<W>
where
    W: Write,
{
    #[inline]
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        let pos = self.stack.primitive()?;

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        self.out.write_char('"')?;
        fmt::write(&mut Escape(&mut self.out), v)?;
        self.out.write_char('"')?;

        Ok(())
    }

    #[inline]
    fn i128(&mut self, v: i128) -> stream::Result {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    #[inline]
    fn u128(&mut self, v: u128) -> stream::Result {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        self.out.write_str(itoa::Buffer::new().format(v))?;

        Ok(())
    }

    #[inline]
    fn f64(&mut self, v: f64) -> stream::Result {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        self.out.write_str(ryu::Buffer::new().format(v))?;

        Ok(())
    }

    #[inline]
    fn bool(&mut self, v: bool) -> stream::Result {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        self.out.write_str(if v { "true" } else { "false" })?;

        Ok(())
    }

    #[inline]
    fn char(&mut self, v: char) -> stream::Result {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        self.out.write_char(v)?;

        Ok(())
    }

    #[inline]
    fn str(&mut self, v: &str) -> stream::Result {
        let pos = self.stack.primitive()?;

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        self.out.write_char('"')?;
        escape_str(&v, &mut self.out)?;
        self.out.write_char('"')?;

        Ok(())
    }

    #[inline]
    fn none(&mut self) -> stream::Result {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        self.out.write_str("null")?;

        Ok(())
    }

    #[inline]
    fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
        if self.stack.seq_begin()?.is_key() {
            return Err(stream::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        if let Some(delim) = self.delim.take() {
            self.out.write_char(delim)?;
        }

        self.out.write_char('[')?;

        Ok(())
    }

    #[inline]
    fn seq_elem(&mut self) -> stream::Result {
        self.stack.seq_elem()?;

        Ok(())
    }

    #[inline]
    fn seq_end(&mut self) -> stream::Result {
        let pos = self.stack.seq_end()?;

        self.delim = Self::next_delim(pos);
        self.out.write_char(']')?;

        Ok(())
    }

    #[inline]
    fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
        if self.stack.map_begin()?.is_key() {
            return Err(stream::Error::unsupported(
                "only strings are supported as json keys",
            ));
        }

        if let Some(delim) = self.delim.take() {
            self.out.write_char(delim)?;
        }

        self.out.write_char('{')?;

        Ok(())
    }

    #[inline]
    fn map_key(&mut self) -> stream::Result {
        self.stack.map_key()?;

        Ok(())
    }

    #[inline]
    fn map_value(&mut self) -> stream::Result {
        self.stack.map_value()?;

        Ok(())
    }

    #[inline]
    fn map_end(&mut self) -> stream::Result {
        let pos = self.stack.map_end()?;

        self.delim = Self::next_delim(pos);
        self.out.write_char('}')?;

        Ok(())
    }
}

/*
This `escape_str` implementation has been shamelessly lifted from dtolnay's `miniserde`:
https://github.com/dtolnay/miniserde
*/

#[inline]
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
