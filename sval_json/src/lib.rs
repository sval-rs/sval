use std::mem;

use sval::stream::{self, Stream};

#[inline]
pub fn to_string(v: impl sval::Value) -> Result<String, sval::Error> {
    let mut fmt = Fmt {
        stack: stream::Stack::new(),
        delim: None,
        out: String::new(),
    };

    sval::stream(v, &mut fmt)?;

    Ok(fmt.out)
}

struct Fmt {
    stack: stream::Stack,
    delim: Option<char>,
    out: String,
}

impl Fmt {
    fn next_delim(pos: stream::Pos) -> Option<char> {
        use sval::stream::Pos::*;

        match pos {
            Root => None,
            Key => Some(':'),
            Value | Elem => Some(','),
        }
    }
}

impl Stream for Fmt {
    fn begin(&mut self) -> Result<(), stream::Error> {
        self.stack.begin()?;

        Ok(())
    }

    fn fmt(&mut self, v: stream::Arguments) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.push(delim);
        }

        escape_str(&v.to_string(), &mut self.out);

        Ok(())
    }

    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let stream::Pos::Key = pos {
            return Err(stream::Error::msg("only strings are supported as json keys"));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.push(delim);
        }

        self.out.push_str(itoa::Buffer::new().format(v));

        Ok(())
    }

    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let stream::Pos::Key = pos {
            return Err(stream::Error::msg("only strings are supported as json keys"));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.push(delim);
        }

        self.out.push_str(itoa::Buffer::new().format(v));

        Ok(())
    }

    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let stream::Pos::Key = pos {
            return Err(stream::Error::msg("only strings are supported as json keys"));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.push(delim);
        }

        self.out.push_str(ryu::Buffer::new().format(v));

        Ok(())
    }

    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let stream::Pos::Key = pos {
            return Err(stream::Error::msg("only strings are supported as json keys"));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.push(delim);
        }

        self.out.push_str(if v { "true" } else { "false" });

        Ok(())
    }

    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let stream::Pos::Key = pos {
            return Err(stream::Error::msg("only strings are supported as json keys"));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.push(delim);
        }

        self.out.push(v);

        Ok(())
    }

    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.push(delim);
        }

        escape_str(&v, &mut self.out);

        Ok(())
    }

    fn none(&mut self) -> Result<(), stream::Error> {
        let pos = self.stack.primitive()?;

        if let stream::Pos::Key = pos {
            return Err(stream::Error::msg("only strings are supported as json keys"));
        }

        if let Some(delim) = mem::replace(&mut self.delim, Self::next_delim(pos)) {
            self.out.push(delim);
        }

        self.out.push_str("null");

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        if let stream::Pos::Key = self.stack.seq_begin()? {
            return Err(stream::Error::msg("only strings are supported as json keys"));
        }

        if let Some(delim) = self.delim.take() {
            self.out.push(delim);
        }

        self.out.push('[');

        Ok(())
    }

    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        self.stack.seq_elem()?;

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), stream::Error> {
        let pos = self.stack.seq_end()?;

        self.delim = Self::next_delim(pos);
        self.out.push(']');

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        if let stream::Pos::Key = self.stack.map_begin()? {
            return Err(stream::Error::msg("only strings are supported as json keys"));
        }

        if let Some(delim) = self.delim.take() {
            self.out.push(delim);
        }

        self.out.push('{');

        Ok(())
    }

    fn map_key(&mut self) -> Result<(), stream::Error> {
        self.stack.map_key()?;

        Ok(())
    }

    fn map_value(&mut self) -> Result<(), stream::Error> {
        self.stack.map_value()?;

        Ok(())
    }

    fn map_end(&mut self) -> Result<(), stream::Error> {
        let pos = self.stack.map_end()?;

        self.delim = Self::next_delim(pos);
        self.out.push('}');

        Ok(())
    }

    fn end(&mut self) -> Result<(), stream::Error> {
        self.stack.end()?;

        Ok(())
    }
}

fn escape_str(value: &str, out: &mut String) {
    out.push('"');

    let bytes = value.as_bytes();
    let mut start = 0;

    for (i, &byte) in bytes.iter().enumerate() {
        let escape = ESCAPE[byte as usize];
        if escape == 0 {
            continue;
        }

        if start < i {
            out.push_str(&value[start..i]);
        }

        match escape {
            self::BB => out.push_str("\\b"),
            self::TT => out.push_str("\\t"),
            self::NN => out.push_str("\\n"),
            self::FF => out.push_str("\\f"),
            self::RR => out.push_str("\\r"),
            self::QU => out.push_str("\\\""),
            self::BS => out.push_str("\\\\"),
            self::U => {
                static HEX_DIGITS: [u8; 16] = *b"0123456789abcdef";
                out.push_str("\\u00");
                out.push(HEX_DIGITS[(byte >> 4) as usize] as char);
                out.push(HEX_DIGITS[(byte & 0xF) as usize] as char);
            }
            _ => unreachable!(),
        }

        start = i + 1;
    }

    if start != bytes.len() {
        out.push_str(&value[start..]);
    }

    out.push('"');
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
