use core::fmt::{self, Debug, Write};

pub(crate) struct Writer<W> {
    is_current_depth_empty: bool,
    is_text_number: bool,
    out: W,
}

/**
A token-aware [`fmt::Write`].

This trait can be used to customize the way various tokens are written, such
as colorizing numbers and booleans differently.
*/
pub trait TokenWrite: Write {
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
    Write a number.
    */
    fn write_number<N: fmt::Display>(&mut self, num: N) -> fmt::Result {
        self.write_fmt(format_args!("{}", num))
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
    Write an atom, like `true` or `()`.
    */
    fn write_atom<A: fmt::Display>(&mut self, atom: A) -> fmt::Result {
        self.write_fmt(format_args!("{}", atom))
    }

    /**
    Write a fragment of punctuation, like `:` or `,`.
    */
    fn write_punct(&mut self, punct: &str) -> fmt::Result {
        self.write_str(punct)
    }

    /**
    Write a type name.
    */
    fn write_type(&mut self, ty: &str) -> fmt::Result {
        self.write_ident(ty)
    }

    /**
    Write an identifier.
    */
    fn write_ident(&mut self, ident: &str) -> fmt::Result {
        self.write_str(ident)
    }

    /**
    Write a fragment of text.
    */
    fn write_text(&mut self, text: &str) -> fmt::Result {
        self.write_str(text)
    }

    /**
    Write a field name.
    */
    fn write_field(&mut self, field: &str) -> fmt::Result {
        self.write_ident(field)
    }

    /**
    Write whitespace.
    */
    fn write_ws(&mut self, ws: &str) -> fmt::Result {
        self.write_str(ws)
    }
}

impl<'a, W: TokenWrite + ?Sized> TokenWrite for &'a mut W {
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

    fn write_number<N: fmt::Display>(&mut self, num: N) -> fmt::Result {
        (**self).write_number(num)
    }

    fn write_null(&mut self) -> fmt::Result {
        (**self).write_null()
    }

    fn write_bool(&mut self, value: bool) -> fmt::Result {
        (**self).write_bool(value)
    }

    fn write_atom<A: fmt::Display>(&mut self, atom: A) -> fmt::Result {
        (**self).write_atom(atom)
    }

    fn write_punct(&mut self, punct: &str) -> fmt::Result {
        (**self).write_punct(punct)
    }

    fn write_type(&mut self, ty: &str) -> fmt::Result {
        (**self).write_type(ty)
    }

    fn write_ident(&mut self, ident: &str) -> fmt::Result {
        (**self).write_ident(ident)
    }

    fn write_text(&mut self, text: &str) -> fmt::Result {
        (**self).write_text(text)
    }

    fn write_field(&mut self, field: &str) -> fmt::Result {
        (**self).write_field(field)
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
            is_text_number: false,
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
        if !self.is_text_number {
            self.out.write_text("\"").map_err(|_| sval::Error::new())?;
        }

        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        if !self.is_text_number {
            // Inlined from `impl Debug for str`
            // This avoids writing the outer quotes for the string
            // and handles the `'` case
            // NOTE: The vast (vast) majority of formatting time is spent here
            // Optimizing this would be a big win
            let mut from = 0;

            for (i, c) in fragment.char_indices() {
                let esc = c.escape_debug();

                // If char needs escaping, flush backlog so far and write, else skip
                if c != '\'' && esc.len() != 1 {
                    self.out
                        .write_text(&fragment[from..i])
                        .map_err(|_| sval::Error::new())?;
                    for c in esc {
                        let mut buf = [0; 4];
                        self.out
                            .write_text(c.encode_utf8(&mut buf))
                            .map_err(|_| sval::Error::new())?;
                    }
                    from = i + c.len_utf8();
                }
            }

            self.out
                .write_text(&fragment[from..])
                .map_err(|_| sval::Error::new())?;
        } else {
            self.out
                .write_number(fragment)
                .map_err(|_| sval::Error::new())?;
        }

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        if !self.is_text_number {
            self.out.write_text("\"").map_err(|_| sval::Error::new())?;
        }

        Ok(())
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
        self.is_text_number = false;
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
        self.is_text_number = false;
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
        self.is_text_number = false;

        match tag {
            Some(&sval::tags::NUMBER) => {
                self.is_text_number = true;

                Ok(())
            }
            _ => {
                if let Some(label) = label {
                    self.out
                        .write_type(label.as_str())
                        .map_err(|_| sval::Error::new())?;
                    self.out.write_punct("(").map_err(|_| sval::Error::new())?;
                }

                Ok(())
            }
        }
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        match tag {
            Some(&sval::tags::NUMBER) => {
                self.is_text_number = false;

                Ok(())
            }
            _ => {
                if label.is_some() {
                    self.out.write_punct(")").map_err(|_| sval::Error::new())?;
                }

                Ok(())
            }
        }
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
        self.is_text_number = false;
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
