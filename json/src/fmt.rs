use sval::stream::{
    self,
    stack,
    Stack,
    Stream,
};

use crate::std::{
    fmt::{
        self,
        Write,
    },
    mem,
};

/**
Write a [`sval::Value`] to a formatter.
*/
pub fn to_fmt(fmt: impl Write, v: impl sval::Value) -> Result<(), sval::Error> {
    let mut fmt = Fmt {
        stack: Stack::new(),
        delim: None,
        out: fmt,
    };

    sval::stream(v, &mut fmt)
}

struct Fmt<W> {
    stack: Stack,
    delim: Option<char>,
    out: W,
}

impl<W> Fmt<W>
where
    W: Write,
{
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

impl<W> Stream for Fmt<W>
where
    W: Write,
{
    #[inline]
    fn begin(&mut self) -> Result<(), stream::Error> {
        self.stack.begin()?;

        Ok(())
    }

    #[inline]
    fn fmt(&mut self, v: stream::Arguments) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        fmt::write(&mut Escape(&mut self.out), v)?;

        Ok(())
    }

    #[inline]
    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::msg(
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
    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::msg(
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
    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::msg(
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
    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::msg(
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
    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::msg(
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
    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.write_char(delim)?;
        }

        escape_str(&v, &mut self.out)?;

        Ok(())
    }

    #[inline]
    fn none(&mut self) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if pos.is_key() {
            return Err(stream::Error::msg(
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
    fn seq_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        if self.stack.seq_begin()?.is_key() {
            return Err(stream::Error::msg(
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
    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        self.stack.seq_elem()?;

        Ok(())
    }

    #[inline]
    fn seq_end(&mut self) -> Result<(), stream::Error> {
        let pos = self.stack.seq_end()?;

        self.delim = Self::next_delim(pos);
        self.out.write_char(']')?;

        Ok(())
    }

    #[inline]
    fn map_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        if self.stack.map_begin()?.is_key() {
            return Err(stream::Error::msg(
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
    fn map_key(&mut self) -> Result<(), stream::Error> {
        self.stack.map_key()?;

        Ok(())
    }

    #[inline]
    fn map_value(&mut self) -> Result<(), stream::Error> {
        self.stack.map_value()?;

        Ok(())
    }

    #[inline]
    fn map_end(&mut self) -> Result<(), stream::Error> {
        let pos = self.stack.map_end()?;

        self.delim = Self::next_delim(pos);
        self.out.write_char('}')?;

        Ok(())
    }

    #[inline]
    fn end(&mut self) -> Result<(), stream::Error> {
        self.stack.end()?;

        Ok(())
    }
}

/*
This `escape_str` implementation has been shamelessly lifted from dtolnay's `miniserde`:
https://github.com/dtolnay/miniserde
*/

#[inline]
fn escape_str(value: &str, mut out: impl Write) -> Result<(), fmt::Error> {
    out.write_char('"')?;

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

    out.write_char('"')?;

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
