use crate::tags;
use core::fmt::{self, Display, Write};

pub(crate) struct Writer<W> {
    is_current_depth_empty: bool,
    is_number: bool,
    escaper: TextEscaper,
    flush_text_tag: Option<sval::Tag>,
    out: W,
}

/**
A token-aware [`fmt::Write`].

This trait can be used to customize the way various tokens are written, such
as colorizing numbers and booleans differently.
*/
pub trait TokenWrite: Write {
    /**
    Get an escaper to use for text.

    Implementors can override this method to change the strategy for escaping input.
    */
    fn text_escaper(&self) -> TextEscaper {
        TextEscaper::escape_idempotent()
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

pub(crate) struct StreamWriter<S>(pub S);

impl<'sval, S: sval::Stream<'sval>> Write for StreamWriter<S> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.text_fragment_computed(s).map_err(|_| fmt::Error)
    }
}

struct TaggedTextFragmentWriter<'a, S> {
    tag: &'a sval::Tag,
    stream: S,
}

impl<'a, 'b, S: sval::Stream<'b>> Write for TaggedTextFragmentWriter<'a, S> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.stream
            .tagged_text_fragment_computed(self.tag, s)
            .map_err(|_| fmt::Error)
    }
}

impl<'sval, S: sval::Stream<'sval>> TokenWrite for StreamWriter<S> {
    fn write_token_fragment<T: fmt::Display>(&mut self, tag: &sval::Tag, token: T) -> fmt::Result {
        write!(
            TaggedTextFragmentWriter {
                stream: &mut self.0,
                tag,
            },
            "{}",
            token
        )
    }
}

impl<W> Writer<W> {
    pub fn new(out: W) -> Self {
        Writer {
            is_current_depth_empty: true,
            is_number: false,
            flush_text_tag: None,
            escaper: TextEscaper::no_escaping(),
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
        self.escaper = self.out.text_escaper();

        if self.is_number {
            Ok(())
        } else {
            self.out.write_text_quote().map_err(|_| sval::Error::new())
        }
    }

    fn tagged_text_fragment_computed(&mut self, tag: &sval::Tag, fragment: &str) -> sval::Result {
        self.flush_text_tag = Some(tag.clone());

        if tag == &tags::NUMBER {
            self.out
                .write_number(fragment)
                .map_err(|_| sval::Error::new())
        } else {
            self.escaper
                .write(fragment, |fragment| {
                    self.out.write_tagged_text(tag, fragment)
                })
                .map_err(|_| sval::Error::new())
        }
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.flush_text_tag = None;

        if self.is_number {
            self.out
                .write_number(fragment)
                .map_err(|_| sval::Error::new())
        } else {
            self.escaper
                .write(fragment, |fragment| self.out.write_text(fragment))
                .map_err(|_| sval::Error::new())
        }
    }

    fn text_end(&mut self) -> sval::Result {
        if let Some(tag) = self.flush_text_tag.take() {
            self.escaper
                .flush(|fragment| self.out.write_tagged_text(&tag, fragment))
                .map_err(|_| sval::Error::new())?;
        } else {
            self.escaper
                .flush(|fragment| self.out.write_text(fragment))
                .map_err(|_| sval::Error::new())?;
        }

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
Encapsulates a strategy for escaping text fragments.
*/
pub struct TextEscaper(EscaperStrategy);

enum EscaperStrategy {
    Idempotent(IdempotentEscaper),
    NoEscaping,
}

enum IdempotentEscaper {
    Normal,
    SeenBackslash,
}

impl TextEscaper {
    /**
    Don't escape any input.
    */
    pub fn no_escaping() -> Self {
        TextEscaper(EscaperStrategy::NoEscaping)
    }

    /**
    Escape the input, unless it already appears to be escaped.
    */
    pub fn escape_idempotent() -> Self {
        TextEscaper(EscaperStrategy::Idempotent(IdempotentEscaper::Normal))
    }

    fn write(&mut self, input: &str, mut output: impl FnMut(&str) -> fmt::Result) -> fmt::Result {
        match self.0 {
            EscaperStrategy::Idempotent(ref mut escaper) => escaper.write(input, output),
            EscaperStrategy::NoEscaping => output(input),
        }
    }

    fn flush(&mut self, output: impl FnMut(&str) -> fmt::Result) -> fmt::Result {
        match self.0 {
            EscaperStrategy::Idempotent(ref mut escaper) => escaper.flush(output),
            EscaperStrategy::NoEscaping => Ok(()),
        }
    }
}

impl IdempotentEscaper {
    fn write(&mut self, input: &str, mut output: impl FnMut(&str) -> fmt::Result) -> fmt::Result {
        let mut from = 0;

        for (i, c) in input.char_indices() {
            if let IdempotentEscaper::SeenBackslash = self {
                *self = IdempotentEscaper::Normal;

                let flush = &input[from..i];
                if flush.len() > 0 {
                    output(flush)?;
                }

                match c {
                    // If the character following the backslash looks like
                    // an escape then write the backslash as-is. We don't just
                    // increment an index here because the backslash may have
                    // come from a previous write
                    'r' | 'n' | 't' | '\\' | 'u' => {
                        output("\\")?;
                        from = i;
                        continue;
                    }
                    // If the character following the backslash doesn't look
                    // like an escape then escape the backslash
                    _ => {
                        let esc = c.escape_debug();

                        for c in esc {
                            let mut buf = [0; 4];
                            output(c.encode_utf8(&mut buf))?;
                        }

                        from = i + c.len_utf8();
                    }
                }
            }

            // Backslash is handled explicitly
            if c == '\\' {
                from = i + c.len_utf8();

                *self = IdempotentEscaper::SeenBackslash;
                continue;
            }

            // Single-quotes aren't escaped
            if c == '\'' {
                continue;
            }

            let esc = c.escape_debug();

            // A character is escaped if its number of escaped characters
            // is not 1; that means there's at least a leading `\` in there
            if esc.len() != 1 {
                output(&input[from..i])?;

                for c in esc {
                    let mut buf = [0; 4];
                    output(c.encode_utf8(&mut buf))?;
                }

                from = i + c.len_utf8();
            }
        }

        // Flush the rest of the buffer
        let flush = &input[from..];
        if flush.len() > 0 {
            output(flush)?;
        }

        Ok(())
    }

    fn flush(&mut self, mut output: impl FnMut(&str) -> fmt::Result) -> fmt::Result {
        if let IdempotentEscaper::SeenBackslash = self {
            *self = IdempotentEscaper::Normal;

            let esc = '\\'.escape_debug();

            for c in esc {
                let mut buf = [0; 4];
                output(c.encode_utf8(&mut buf))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_escaping() {
        let mut escaper = TextEscaper::no_escaping();

        let mut out = String::new();

        escaper
            .write("hello\rworld\\", |s| {
                out.push_str(s);
                Ok(())
            })
            .unwrap();

        assert_eq!("hello\rworld\\", out);
    }

    #[test]
    fn idempotent_write_escape() {
        for (input, expected) in [
            ("hello", r#"hello"#),
            ("\\", r#"\\"#),
            ("\\\\", r#"\\"#),
            ("\r", r#"\r"#),
            ("\\r", r#"\r"#),
        ] {
            let mut escaper = TextEscaper::escape_idempotent();

            let mut out = String::new();

            escaper
                .write(input, |s| {
                    out.push_str(s);
                    Ok(())
                })
                .unwrap();
            escaper
                .flush(|s| {
                    out.push_str(s);
                    Ok(())
                })
                .unwrap();

            assert_eq!(expected, out);
        }
    }

    #[test]
    fn idempotent_write_escape_across_boundaries() {
        for i in ["\\", "n", "r"] {
            let mut escaper = TextEscaper::escape_idempotent();

            let mut out = String::new();

            escaper
                .write("\\", |s| {
                    out.push_str(s);
                    Ok(())
                })
                .unwrap();

            assert_eq!("", out);

            escaper
                .write(i, |s| {
                    out.push_str(s);
                    Ok(())
                })
                .unwrap();

            escaper
                .flush(|s| {
                    out.push_str(s);
                    Ok(())
                })
                .unwrap();

            assert_eq!(format!("\\{}", i), out);
        }
    }

    #[test]
    fn idempotent_flush_escape_across_boundaries() {
        let mut escaper = TextEscaper::escape_idempotent();

        let mut out = String::new();

        escaper
            .write("\\", |s| {
                out.push_str(s);
                Ok(())
            })
            .unwrap();

        escaper
            .flush(|s| {
                out.push_str(s);
                Ok(())
            })
            .unwrap();

        assert_eq!("\\\\", out);
    }
}
