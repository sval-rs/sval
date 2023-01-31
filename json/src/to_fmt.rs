use core::fmt::{self, Write};

use sval::Stream as _;

use crate::{tags, Error};

pub fn stream_to_fmt(fmt: impl Write, v: impl sval::Value) -> Result<(), Error> {
    let mut stream = Formatter::new(fmt);

    match v.stream(&mut stream) {
        Ok(()) => Ok(()),
        Err(_) => Err(stream.err.unwrap_or_else(Error::generic)),
    }
}

pub(crate) struct Formatter<W> {
    is_internally_tagged: bool,
    is_current_depth_empty: bool,
    is_text_quoted: bool,
    is_json_native: bool,
    text_handler: Option<TextHandler>,
    err: Option<Error>,
    out: W,
}

impl<W> Formatter<W> {
    pub fn new(out: W) -> Self {
        Formatter {
            is_internally_tagged: false,
            is_current_depth_empty: true,
            is_text_quoted: true,
            is_json_native: false,
            text_handler: None,
            err: None,
            out,
        }
    }

    fn err(&mut self, e: Error) -> sval::Error {
        self.err = Some(e);
        sval::Error::new()
    }
}

impl<'sval, W> sval::Stream<'sval> for Formatter<W>
where
    W: Write,
{
    fn null(&mut self) -> sval::Result {
        self.out
            .write_str("null")
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        self.out
            .write_str(if v { "true" } else { "false" })
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.is_text_quoted {
            self.out
                .write_char('"')
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        }

        Ok(())
    }

    fn text_fragment_computed(&mut self, v: &str) -> sval::Result {
        if let Some(ref mut handler) = self.text_handler {
            handler
                .text_fragment(v, &mut self.out)
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        } else if !self.is_json_native {
            escape_str(v, &mut self.out).map_err(|e| self.err(Error::from_fmt(e)))?;
        } else {
            self.out
                .write_str(v)
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        }

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        if self.is_text_quoted {
            self.out
                .write_char('"')
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        }

        Ok(())
    }

    fn u8(&mut self, v: u8) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn u16(&mut self, v: u16) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn u32(&mut self, v: u32) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn u64(&mut self, v: u64) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn u128(&mut self, v: u128) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn i8(&mut self, v: i8) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn i16(&mut self, v: i16) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn i32(&mut self, v: i32) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn i64(&mut self, v: i64) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn i128(&mut self, v: i128) -> sval::Result {
        self.out
            .write_str(itoa::Buffer::new().format(v))
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn f32(&mut self, v: f32) -> sval::Result {
        if v.is_nan() || v.is_infinite() {
            self.null()?;
        } else {
            self.out
                .write_str(ryu::Buffer::new().format_finite(v))
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        }

        Ok(())
    }

    fn f64(&mut self, v: f64) -> sval::Result {
        if v.is_nan() || v.is_infinite() {
            self.null()?;
        } else {
            self.out
                .write_str(ryu::Buffer::new().format_finite(v))
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        }

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        if !self.is_text_quoted {
            return Err(self.err(Error::invalid_key()));
        }

        self.is_current_depth_empty = true;
        self.out
            .write_char('{')
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.is_text_quoted = false;
        self.is_internally_tagged = false;

        if !self.is_current_depth_empty {
            self.out
                .write_str(",\"")
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        } else {
            self.out
                .write_char('"')
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        }

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.out
            .write_str("\":")
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        self.is_text_quoted = true;

        Ok(())
    }

    fn map_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn map_end(&mut self) -> sval::Result {
        self.out
            .write_char('}')
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        if !self.is_text_quoted {
            return Err(self.err(Error::invalid_key()));
        }

        self.is_current_depth_empty = true;

        self.out
            .write_char('[')
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        self.is_internally_tagged = false;

        if !self.is_current_depth_empty {
            self.out
                .write_char(',')
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        }

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.out
            .write_char(']')
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        Ok(())
    }

    fn enum_begin(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.is_internally_tagged = true;

        Ok(())
    }

    fn enum_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        if self.is_internally_tagged {
            self.internally_tagged_map_end()?;

            self.is_internally_tagged = false;
        }

        Ok(())
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        match tag {
            Some(&tags::JSON_NATIVE) => {
                self.is_json_native = true;
            }
            Some(&sval::tags::NUMBER) => {
                self.is_text_quoted = false;

                // If the number isn't guaranteed to be valid JSON then create an adapter
                if !self.is_json_native {
                    self.text_handler = Some(TextHandler::number());
                }
            }
            _ => (),
        }

        self.internally_tagged_begin(label)?;

        Ok(())
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        match tag {
            Some(&tags::JSON_NATIVE) => {
                self.is_json_native = false;
            }
            Some(&sval::tags::NUMBER) => {
                self.is_text_quoted = true;

                if !self.is_json_native {
                    if let Some(TextHandler::Number(mut number)) = self.text_handler.take() {
                        number
                            .end(&mut self.out)
                            .map_err(|e| self.err(Error::from_fmt(e)))?;
                    }
                }
            }
            _ => (),
        }

        self.internally_tagged_end(label)
    }

    fn tag(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.is_internally_tagged = false;

        match tag {
            Some(&sval::tags::RUST_OPTION_NONE) => self.null(),
            _ => {
                if let Some(label) = label {
                    self.value(label.as_str())
                } else {
                    self.null()
                }
            }
        }
    }

    fn record_begin(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        self.internally_tagged_begin(label)?;
        self.map_begin(num_entries_hint)
    }

    fn record_value_begin(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        self.is_internally_tagged = false;

        if !self.is_current_depth_empty {
            self.out
                .write_str(",\"")
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        } else {
            self.out
                .write_char('"')
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        }

        // If the field is JSON native then it doesn't require escaping
        if let Some(&tags::JSON_NATIVE) = tag {
            self.out
                .write_str(label.as_str())
                .map_err(|e| self.err(Error::from_fmt(e)))?;
        } else {
            escape_str(label.as_str(), &mut self.out).map_err(|e| self.err(Error::from_fmt(e)))?;
        }

        self.out
            .write_str("\":")
            .map_err(|e| self.err(Error::from_fmt(e)))?;

        self.map_value_begin()
    }

    fn record_end(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.map_end()?;
        self.internally_tagged_end(label)
    }

    fn tuple_begin(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        self.internally_tagged_begin(label)?;
        self.seq_begin(num_entries_hint)
    }

    fn tuple_end(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.seq_end()?;
        self.internally_tagged_end(label)
    }
}

impl<'sval, W> Formatter<W>
where
    W: Write,
{
    fn internally_tagged_begin(&mut self, label: Option<&sval::Label>) -> sval::Result {
        // If there's a label then begin a map, using the label as the key
        if self.is_internally_tagged {
            if let Some(label) = label {
                self.internally_tagged_map_begin(label)?;
            }

            self.is_internally_tagged = false;
        }

        Ok(())
    }

    fn internally_tagged_end(&mut self, label: Option<&sval::Label>) -> sval::Result {
        if label.is_some() {
            self.is_internally_tagged = true;
        }

        Ok(())
    }

    fn internally_tagged_map_begin(&mut self, label: &sval::Label) -> sval::Result {
        self.map_begin(Some(1))?;

        self.map_key_begin()?;
        escape_str(label.as_str(), &mut self.out).map_err(|e| self.err(Error::from_fmt(e)))?;
        self.map_key_end()?;

        self.map_value_begin()
    }

    fn internally_tagged_map_end(&mut self) -> sval::Result {
        self.map_value_end()?;
        self.map_end()?;

        Ok(())
    }
}

enum TextHandler {
    Number(NumberTextHandler),
}

struct NumberTextHandler {
    at_start: bool,
    sign_negative: bool,
    leading_zeroes: usize,
    is_nan_or_infinity: bool,
}

impl TextHandler {
    fn number() -> Self {
        TextHandler::Number(NumberTextHandler {
            sign_negative: false,
            leading_zeroes: 0,
            at_start: true,
            is_nan_or_infinity: false,
        })
    }

    fn text_fragment(&mut self, v: &str, out: impl Write) -> fmt::Result {
        match self {
            TextHandler::Number(number) => number.text_fragment(v, out),
        }
    }
}

impl NumberTextHandler {
    fn text_fragment(&mut self, v: &str, mut out: impl Write) -> fmt::Result {
        if !self.is_nan_or_infinity {
            let mut range = 0..0;

            for b in v.as_bytes() {
                match b {
                    // JSON numbers don't support leading zeroes (except for `0.x`)
                    // so we need to shift over them
                    b'0' if self.at_start => {
                        self.leading_zeroes += 1;
                        range.start += 1;
                        range.end += 1;
                    }
                    // If we're not skipping zeroes then shift over it to write later
                    b'0'..=b'9' => {
                        if self.at_start && self.sign_negative {
                            out.write_char('-')?;
                        }

                        self.at_start = false;
                        range.end += 1;
                    }
                    // If we encounter a decimal point we might need to write a leading `0`
                    b'.' => {
                        if self.at_start {
                            if self.sign_negative {
                                out.write_char('-')?;
                            }

                            out.write_char('0')?;
                        }

                        self.at_start = false;
                        range.end += 1;
                    }
                    // If we encounter a sign then stash it until we know the number is finite
                    // A value like `-inf` should still write `null`, not `-null`
                    b'-' if self.at_start => {
                        self.sign_negative = true;
                        range.start += 1;
                        range.end += 1;
                    }
                    // JSON doesn't support a leading `+` sign
                    b'+' if self.at_start => {
                        range.start += 1;
                        range.end += 1;
                    }
                    // `snan`, `nan`, `inf` in any casing should write `null`
                    b's' | b'n' | b'i' | b'S' | b'N' | b'I' => {
                        self.is_nan_or_infinity = true;
                        self.at_start = false;

                        out.write_str("null")?;

                        range.start = 0;
                        range.end = 0;

                        break;
                    }
                    _ => range.end += 1,
                }
            }

            out.write_str(&v[range])?;
        }

        Ok(())
    }

    fn end(&mut self, mut out: impl Write) -> fmt::Result {
        if self.at_start {
            out.write_char('0')?;
        }

        Ok(())
    }
}

/*
This `escape_str` implementation has been shamelessly lifted from dtolnay's `miniserde`:
https://github.com/dtolnay/miniserde
*/

#[inline(always)]
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
            BB => out.write_str("\\b")?,
            TT => out.write_str("\\t")?,
            NN => out.write_str("\\n")?,
            FF => out.write_str("\\f")?,
            RR => out.write_str("\\r")?,
            QU => out.write_str("\\\"")?,
            BS => out.write_str("\\\\")?,
            U => {
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
