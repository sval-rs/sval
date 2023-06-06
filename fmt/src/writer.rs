use crate::tags;
use core::fmt::{self, Display, Write};

pub(crate) struct Writer<W> {
    is_current_depth_empty: bool,
    is_number: bool,
    out: W,
}

/**
A token-aware [`fmt::Write`].

This trait can be used to customize the way various tokens are written, such
as colorizing numbers and booleans differently.
*/
pub trait TokenWrite: Write {
    /**
    Get an escaper to use for text fragments.

    By default, an escaper that uses the same strategy as Rust's `Debug` is used.
    */
    fn text_escaper(&self) -> TextEscaper {
        TextEscaper::escape_debug()
    }

    /**
    Write a token fragment.
    */
    fn write_token_fragment<T: fmt::Display>(&mut self, tag: &sval::Tag, token: T) -> fmt::Result {
        let _ = tag;
        self.write_fmt(format_args!("{}", token))
    }

    /**
    Write a number.
    */
    fn write_u8(&mut self, value: u8) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_u16(&mut self, value: u16) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_u32(&mut self, value: u32) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_u64(&mut self, value: u64) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_u128(&mut self, value: u128) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i8(&mut self, value: i8) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i16(&mut self, value: i16) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i32(&mut self, value: i32) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i64(&mut self, value: i64) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_i128(&mut self, value: i128) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_f32(&mut self, value: f32) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write a number.
    */
    fn write_f64(&mut self, value: f64) -> fmt::Result {
        self.write_number(value)
    }

    /**
    Write null or unit.
    */
    fn write_null(&mut self) -> fmt::Result {
        self.write_atom("()")
    }

    /**
    Write a boolean.
    */
    fn write_bool(&mut self, value: bool) -> fmt::Result {
        self.write_atom(value)
    }

    /**
    Write a type name.
    */
    fn write_type(&mut self, ty: &str) -> fmt::Result {
        self.write_ident(ty)
    }

    /**
    Write a field name.
    */
    fn write_field(&mut self, field: &str) -> fmt::Result {
        self.write_ident(field)
    }

    /**
    Write an opening or closing quote.

    By default, a double quote (`"`) is used.
    */
    fn write_text_quote(&mut self) -> fmt::Result {
        self.write_token_fragment(&tags::TEXT, "\"")
    }

    /**
    Write a fragment of text.

    By default, text input is escaped for debug rendering.
    */
    fn write_text(&mut self, text: &str) -> fmt::Result {
        self.write_token_fragment(&tags::TEXT, text)
    }

    /**
    Write a fragment of tagged text.

    By default, tagged text values aren't escaped.
    */
    fn write_tagged_text(&mut self, tag: &sval::Tag, text: &str) -> fmt::Result {
        self.write_token_fragment(tag, text)
    }

    /**
    Write a number.
    */
    fn write_number<N: fmt::Display>(&mut self, num: N) -> fmt::Result {
        self.write_token_fragment(&tags::NUMBER, num)
    }

    /**
    Write an atom, like `true` or `()`.
    */
    fn write_atom<A: fmt::Display>(&mut self, atom: A) -> fmt::Result {
        self.write_token_fragment(&tags::ATOM, atom)
    }

    /**
    Write an identifier.
    */
    fn write_ident(&mut self, ident: &str) -> fmt::Result {
        self.write_token_fragment(&tags::IDENT, ident)
    }

    /**
    Write a fragment of punctuation, like `:` or `,`.
    */
    fn write_punct(&mut self, punct: &str) -> fmt::Result {
        self.write_token_fragment(&tags::PUNCT, punct)
    }

    /**
    Write whitespace.
    */
    fn write_ws(&mut self, ws: &str) -> fmt::Result {
        self.write_token_fragment(&tags::WS, ws)
    }
}

impl<'a, W: TokenWrite + ?Sized> TokenWrite for &'a mut W {
    fn text_escaper(&self) -> TextEscaper {
        (**self).text_escaper()
    }

    fn write_token_fragment<T: Display>(&mut self, tag: &sval::Tag, token: T) -> fmt::Result {
        (**self).write_token_fragment(tag, token)
    }

    fn write_u8(&mut self, value: u8) -> fmt::Result {
        (**self).write_u8(value)
    }

    fn write_u16(&mut self, value: u16) -> fmt::Result {
        (**self).write_u16(value)
    }

    fn write_u32(&mut self, value: u32) -> fmt::Result {
        (**self).write_u32(value)
    }

    fn write_u64(&mut self, value: u64) -> fmt::Result {
        (**self).write_u64(value)
    }

    fn write_u128(&mut self, value: u128) -> fmt::Result {
        (**self).write_u128(value)
    }

    fn write_i8(&mut self, value: i8) -> fmt::Result {
        (**self).write_i8(value)
    }

    fn write_i16(&mut self, value: i16) -> fmt::Result {
        (**self).write_i16(value)
    }

    fn write_i32(&mut self, value: i32) -> fmt::Result {
        (**self).write_i32(value)
    }

    fn write_i64(&mut self, value: i64) -> fmt::Result {
        (**self).write_i64(value)
    }

    fn write_i128(&mut self, value: i128) -> fmt::Result {
        (**self).write_i128(value)
    }

    fn write_f32(&mut self, value: f32) -> fmt::Result {
        (**self).write_f32(value)
    }

    fn write_f64(&mut self, value: f64) -> fmt::Result {
        (**self).write_f64(value)
    }

    fn write_null(&mut self) -> fmt::Result {
        (**self).write_null()
    }

    fn write_bool(&mut self, value: bool) -> fmt::Result {
        (**self).write_bool(value)
    }

    fn write_type(&mut self, ty: &str) -> fmt::Result {
        (**self).write_type(ty)
    }

    fn write_field(&mut self, field: &str) -> fmt::Result {
        (**self).write_field(field)
    }

    fn write_text_quote(&mut self) -> fmt::Result {
        (**self).write_text_quote()
    }

    fn write_text(&mut self, text: &str) -> fmt::Result {
        (**self).write_text(text)
    }

    fn write_number<N: fmt::Display>(&mut self, num: N) -> fmt::Result {
        (**self).write_number(num)
    }

    fn write_atom<A: fmt::Display>(&mut self, atom: A) -> fmt::Result {
        (**self).write_atom(atom)
    }

    fn write_ident(&mut self, ident: &str) -> fmt::Result {
        (**self).write_ident(ident)
    }

    fn write_punct(&mut self, punct: &str) -> fmt::Result {
        (**self).write_punct(punct)
    }

    fn write_tagged_text(&mut self, tag: &sval::Tag, text: &str) -> fmt::Result {
        (**self).write_tagged_text(tag, text)
    }

    fn write_ws(&mut self, ws: &str) -> fmt::Result {
        (**self).write_ws(ws)
    }
}

impl<'a> TokenWrite for fmt::Formatter<'a> {
    fn write_u8(&mut self, value: u8) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u16(&mut self, value: u16) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u32(&mut self, value: u32) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u64(&mut self, value: u64) -> fmt::Result {
        value.fmt(self)
    }

    fn write_u128(&mut self, value: u128) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i8(&mut self, value: i8) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i16(&mut self, value: i16) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i32(&mut self, value: i32) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i64(&mut self, value: i64) -> fmt::Result {
        value.fmt(self)
    }

    fn write_i128(&mut self, value: i128) -> fmt::Result {
        value.fmt(self)
    }

    fn write_f32(&mut self, value: f32) -> fmt::Result {
        value.fmt(self)
    }

    fn write_f64(&mut self, value: f64) -> fmt::Result {
        value.fmt(self)
    }
}

pub(crate) struct GenericWriter<W>(pub W);

impl<W: Write> Write for GenericWriter<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.0.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result {
        self.0.write_fmt(args)
    }
}

impl<W: Write> TokenWrite for GenericWriter<W> {
    fn write_u8(&mut self, value: u8) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u16(&mut self, value: u16) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u32(&mut self, value: u32) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u64(&mut self, value: u64) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_u128(&mut self, value: u128) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i8(&mut self, value: i8) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i16(&mut self, value: i16) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i32(&mut self, value: i32) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i64(&mut self, value: i64) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_i128(&mut self, value: i128) -> fmt::Result {
        self.write_str(itoa::Buffer::new().format(value))
    }

    fn write_f32(&mut self, value: f32) -> fmt::Result {
        self.write_str(ryu::Buffer::new().format(value))
    }

    fn write_f64(&mut self, value: f64) -> fmt::Result {
        self.write_str(ryu::Buffer::new().format(value))
    }
}

impl<W> Writer<W> {
    pub fn new(out: W) -> Self {
        Writer {
            is_current_depth_empty: true,
            is_number: false,
            out,
        }
    }
}

impl<'sval, W: TokenWrite> sval::Stream<'sval> for Writer<W> {
    fn null(&mut self) -> sval::Result {
        self.out.write_null().map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.out.write_bool(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        if self.is_number {
            Ok(())
        } else {
            self.out.write_text_quote().map_err(|_| sval::Error::new())
        }
    }

    fn tagged_text_fragment_computed(&mut self, tag: &sval::Tag, fragment: &str) -> sval::Result {
        if tag == &tags::NUMBER {
            self.out
                .write_number(fragment)
                .map_err(|_| sval::Error::new())
        } else {
            self.out
                .text_escaper()
                .escape(fragment, |fragment| {
                    self.out.write_tagged_text(tag, fragment)
                })
                .map_err(|_| sval::Error::new())
        }
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        if self.is_number {
            self.out
                .write_number(fragment)
                .map_err(|_| sval::Error::new())
        } else {
            self.out
                .text_escaper()
                .escape(fragment, |fragment| self.out.write_text(fragment))
                .map_err(|_| sval::Error::new())
        }
    }

    fn text_end(&mut self) -> sval::Result {
        if self.is_number {
            Ok(())
        } else {
            self.out.write_text_quote().map_err(|_| sval::Error::new())
        }
    }

    fn binary_begin(&mut self, num_bytes_hint: Option<usize>) -> sval::Result {
        self.seq_begin(num_bytes_hint)
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        for b in fragment {
            self.seq_value_begin()?;
            self.u8(*b)?;
            self.seq_value_end()?;
        }

        Ok(())
    }

    fn binary_end(&mut self) -> sval::Result {
        self.seq_end()
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.out.write_u8(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.out.write_u16(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.out.write_u32(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.out.write_u64(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.out.write_u128(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.out.write_i8(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.out.write_i16(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.out.write_i32(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.out.write_i64(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.out.write_i128(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.out.write_f32(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.out.write_f64(value).map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.is_number = false;
        self.is_current_depth_empty = true;

        self.out.write_punct("{").map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn map_key_begin(&mut self) -> sval::Result {
        if !self.is_current_depth_empty {
            self.out.write_punct(",").map_err(|_| sval::Error::new())?;
        }

        self.out.write_ws(" ").map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.out.write_punct(":").map_err(|_| sval::Error::new())?;
        self.out.write_ws(" ").map_err(|_| sval::Error::new())?;

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
        if !self.is_current_depth_empty {
            self.out.write_ws(" ").map_err(|_| sval::Error::new())?;
        }

        self.out.write_punct("}").map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        self.is_number = false;
        self.is_current_depth_empty = true;

        self.out.write_punct("[").map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        if !self.is_current_depth_empty {
            self.out.write_punct(",").map_err(|_| sval::Error::new())?;
            self.out.write_ws(" ").map_err(|_| sval::Error::new())?;
        }

        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        self.is_current_depth_empty = false;

        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.out.write_punct("]").map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn enum_begin(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        Ok(())
    }

    fn enum_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        Ok(())
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        if tag == Some(&tags::NUMBER) {
            self.is_number = true;
        }

        if let Some(label) = label {
            self.out
                .write_type(label.as_str())
                .map_err(|_| sval::Error::new())?;
            self.out.write_punct("(").map_err(|_| sval::Error::new())?;
        }

        Ok(())
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        if tag == Some(&tags::NUMBER) {
            self.is_number = false;
        }

        if label.is_some() {
            self.out.write_punct(")").map_err(|_| sval::Error::new())?;
        }

        Ok(())
    }

    fn tag(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        if let Some(label) = label {
            self.out
                .write_type(label.as_str())
                .map_err(|_| sval::Error::new())?;
        } else {
            self.null()?;
        }

        Ok(())
    }

    fn record_begin(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
        num_entries_hint: Option<usize>,
    ) -> sval::Result {
        if let Some(label) = label {
            self.out
                .write_type(label.as_str())
                .map_err(|_| sval::Error::new())?;
            self.out.write_ws(" ").map_err(|_| sval::Error::new())?;
        }

        self.map_begin(num_entries_hint)
    }

    fn record_value_begin(&mut self, _: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        self.map_key_begin()?;
        self.out
            .write_field(label.as_str())
            .map_err(|_| sval::Error::new())?;
        self.map_key_end()?;

        self.map_value_begin()
    }

    fn record_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Label) -> sval::Result {
        self.map_value_end()
    }

    fn record_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.map_end()
    }

    fn tuple_begin(
        &mut self,
        _: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
        _: Option<usize>,
    ) -> sval::Result {
        self.is_number = false;
        self.is_current_depth_empty = true;

        if let Some(label) = label {
            self.out
                .write_type(label.as_str())
                .map_err(|_| sval::Error::new())?;
        }

        self.out.write_punct("(").map_err(|_| sval::Error::new())?;

        Ok(())
    }

    fn tuple_value_begin(&mut self, _: Option<&sval::Tag>, _: &sval::Index) -> sval::Result {
        self.seq_value_begin()
    }

    fn tuple_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Index) -> sval::Result {
        self.seq_value_end()
    }

    fn tuple_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        self.out.write_punct(")").map_err(|_| sval::Error::new())?;

        Ok(())
    }
}

/**
A strategy for escaping fragments of text.
*/
pub struct TextEscaper(TextEscaperStrategy);

enum TextEscaperStrategy {
    NoEscaping,
    EscapeDebug,
}

impl TextEscaper {
    /**
    Don't perform any escaping.

    This strategy can be used for writers that either don't escape, or that implement
    their own custom escaping.
    */
    pub fn no_escaping() -> Self {
        TextEscaper(TextEscaperStrategy::NoEscaping)
    }

    /**
    Escape using the same strategy as Rust's `Debug`.
    */
    pub fn escape_debug() -> Self {
        TextEscaper(TextEscaperStrategy::EscapeDebug)
    }

    fn escape(&mut self, input: &str, mut output: impl FnMut(&str) -> fmt::Result) -> fmt::Result {
        match self.0 {
            TextEscaperStrategy::NoEscaping => output(input),
            TextEscaperStrategy::EscapeDebug => escape_debug(input, output),
        }
    }
}

fn escape_debug(input: &str, mut output: impl FnMut(&str) -> fmt::Result) -> fmt::Result {
    let mut from = 0;

    // Iterate over each character, escaping it if necessary
    for (i, c) in input.char_indices() {
        let esc = c.escape_debug();

        // A character is escaped if it produces more than an
        // escape sequence with more than a single character in it
        if esc.len() > 1 {
            let flush = &input[from..i];
            if flush.len() > 0 {
                output(flush)?;
            }

            let mut buf = [0; 4];
            for c in esc {
                output(c.encode_utf8(&mut buf))?;
            }

            // Skip over the original character without writing it
            from = i + c.len_utf8();
        }
    }

    let flush = &input[from..];
    if flush.len() > 0 {
        output(flush)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_no_escaping() {
        let mut actual = String::new();
        let mut escaper = TextEscaper::no_escaping();
        escaper.escape("hello", |i| Ok(actual.push_str(i))).unwrap();
        escaper.escape("\n", |i| Ok(actual.push_str(i))).unwrap();

        assert_eq!("hello\n", actual);
    }

    #[test]
    fn write_escape_debug() {
        let cases = [
            ("hello", r#"hello"#),
            ("\\", r#"\\"#),
            ("\r", r#"\r"#),
            ("\n", r#"\n"#),
            ("\t", r#"\t"#),
            ("\"", r#"\""#),
            ("'", r#"\'"#),
            ("⛰️", r#"⛰\u{fe0f}"#),
        ];

        for (ai, ae) in cases {
            for (bi, be) in cases {
                let mut expected = String::new();
                expected.push_str(ae);
                expected.push_str(be);

                let mut actual = String::new();
                let mut escaper = TextEscaper::escape_debug();
                escaper.escape(ai, |i| Ok(actual.push_str(i))).unwrap();
                escaper.escape(bi, |i| Ok(actual.push_str(i))).unwrap();

                assert_eq!(expected, actual);
            }
        }
    }
}
