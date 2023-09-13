use core::{fmt::Write as _, mem};

use sval::{Index, Label, Stream};
use sval_buffer::TextBuf;

pub(crate) trait LabelStream<'sval>: Stream<'sval> {
    fn label(&mut self, label: &Label) -> sval::Result;
    fn index(&mut self, index: &Index) -> sval::Result;

    fn take(&mut self) -> LabelBuf<'sval>;
}

impl<'a, 'sval, S: LabelStream<'sval> + ?Sized> LabelStream<'sval> for &'a mut S {
    fn label(&mut self, label: &Label) -> sval::Result {
        (**self).label(label)
    }

    fn index(&mut self, index: &Index) -> sval::Result {
        (**self).index(index)
    }

    fn take(&mut self) -> LabelBuf<'sval> {
        (**self).take()
    }
}

pub(crate) enum LabelBuf<'sval> {
    Empty,
    Text(TextBuf<'sval>),
    I128(i128),
    U128(u128),
    F64(f64),
}

impl<'sval> Default for LabelBuf<'sval> {
    fn default() -> Self {
        LabelBuf::Empty
    }
}

impl<'sval> LabelBuf<'sval> {
    pub(crate) fn new() -> Self {
        LabelBuf::Empty
    }

    pub(crate) fn from_index(index: &Index) -> sval::Result<Self> {
        let mut buf = LabelBuf::new();
        buf.index(index)?;

        Ok(buf)
    }

    pub(crate) fn i128(&mut self, v: impl TryInto<i128>) -> sval::Result {
        *self = LabelBuf::I128(v.try_into().map_err(|_| sval::Error::new())?);
        Ok(())
    }

    pub(crate) fn u128(&mut self, v: impl TryInto<u128>) -> sval::Result {
        *self = LabelBuf::U128(v.try_into().map_err(|_| sval::Error::new())?);
        Ok(())
    }

    pub(crate) fn f64(&mut self, v: impl TryInto<f64>) -> sval::Result {
        *self = LabelBuf::F64(v.try_into().map_err(|_| sval::Error::new())?);
        Ok(())
    }

    pub(crate) fn null(&mut self) -> sval::Result {
        self.text_fragment("null")
    }

    pub(crate) fn bool(&mut self, v: bool) -> sval::Result {
        self.text_fragment(if v { "true" } else { "false" })
    }

    pub(crate) fn label(&mut self, label: &Label) -> sval::Result {
        if let Some(label) = label.as_static_str() {
            self.text_fragment(label)
        } else {
            self.text_fragment_computed(label.as_str())
        }
    }

    pub(crate) fn index(&mut self, index: &Index) -> sval::Result {
        if let Some(index) = index.to_isize() {
            self.i128(index)
        } else if let Some(index) = index.to_usize() {
            self.u128(index)
        } else {
            let buf = self.text_buf()?;
            write!(buf, "{}", index).map_err(|_| sval::Error::new())
        }
    }

    pub(crate) fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.text_buf()?
            .push_fragment(fragment)
            .map_err(|_| sval::Error::new())
    }

    pub(crate) fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.text_buf()?
            .push_fragment_computed(fragment)
            .map_err(|_| sval::Error::new())
    }

    pub(crate) fn text_buf(&mut self) -> sval::Result<&mut TextBuf<'sval>> {
        match self {
            LabelBuf::Text(buf) => Ok(buf),
            _ => {
                *self = LabelBuf::Text(TextBuf::new());
                if let LabelBuf::Text(buf) = self {
                    Ok(buf)
                } else {
                    unreachable!()
                }
            }
        }
    }

    pub(crate) fn with_label(&self, f: impl FnOnce(&Label) -> sval::Result) -> sval::Result {
        match self {
            LabelBuf::Empty => f(&Label::new_computed("")),
            LabelBuf::Text(text) => f(&Label::new_computed(text.as_str())),
            LabelBuf::I128(v) => {
                let mut buf = itoa::Buffer::new();
                f(&Label::new_computed(buf.format(*v)))
            }
            LabelBuf::U128(v) => {
                let mut buf = itoa::Buffer::new();
                f(&Label::new_computed(buf.format(*v)))
            }
            LabelBuf::F64(v) => {
                let mut buf = ryu::Buffer::new();
                f(&Label::new_computed(buf.format(*v)))
            }
        }
    }
}

impl<'sval> LabelStream<'sval> for LabelBuf<'sval> {
    fn label(&mut self, label: &Label) -> sval::Result {
        self.label(label)
    }

    fn index(&mut self, index: &Index) -> sval::Result {
        self.index(index)
    }

    fn take(&mut self) -> LabelBuf<'sval> {
        mem::take(self)
    }
}

impl<'sval> Stream<'sval> for LabelBuf<'sval> {
    fn null(&mut self) -> sval::Result {
        self.null()
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.bool(value)
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.text_fragment_computed(fragment)
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.text_fragment(fragment)
    }

    fn text_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.u128(value)
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.u128(value)
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.i128(value)
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.i128(value)
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.f64(value)
    }

    fn map_key_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn map_key_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        sval::error()
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        sval::error()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        sval::error()
    }

    fn seq_end(&mut self) -> sval::Result {
        sval::error()
    }
}

pub(crate) struct Empty;

impl<'sval> LabelStream<'sval> for Empty {
    fn label(&mut self, _: &Label) -> sval::Result {
        Ok(())
    }

    fn index(&mut self, _: &Index) -> sval::Result {
        Ok(())
    }

    fn take(&mut self) -> LabelBuf<'sval> {
        Default::default()
    }
}

impl<'sval> Stream<'sval> for Empty {
    fn null(&mut self) -> sval::Result {
        Ok(())
    }

    fn bool(&mut self, _: bool) -> sval::Result {
        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    fn text_fragment_computed(&mut self, _: &str) -> sval::Result {
        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn i64(&mut self, _: i64) -> sval::Result {
        Ok(())
    }

    fn f64(&mut self, _: f64) -> sval::Result {
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        Ok(())
    }
}
