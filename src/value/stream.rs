use crate::{
    std::fmt::{
        Debug,
        Display,
    },
    stream,
    value::Value,
};

#[cfg(feature = "std")]
use crate::std::error;

pub struct Stream<'s, 'v> {
    owned: Owned<&'s mut dyn stream::Stream<'v>>,
}

struct Owned<S> {
    stream: S,
}

impl<S> Owned<S> {
    fn inner(&mut self) -> &mut S {
        &mut self.stream
    }
}

impl<'s, 'v> From<&'s mut dyn stream::Stream<'v>> for Stream<'s, 'v> {
    fn from(stream: &'s mut dyn stream::Stream<'v>) -> Self {
        Stream {
            owned: Owned { stream },
        }
    }
}

impl<'s, 'v> Stream<'s, 'v> {
    pub fn new(stream: &'s mut impl stream::Stream<'v>) -> Self {
        Stream {
            owned: Owned { stream },
        }
    }

    fn inner(&mut self) -> &mut dyn stream::Stream<'v> {
        self.owned.stream
    }

    pub fn owned<'a, 'b>(&'a mut self) -> Stream<'a, 'b> {
        Stream {
            owned: Owned {
                stream: &mut self.owned,
            },
        }
    }

    pub fn by_ref<'a>(&'a mut self) -> Stream<'a, 'v> {
        Stream {
            owned: Owned {
                stream: &mut self.owned.stream,
            },
        }
    }

    pub fn any(&mut self, v: &'v (impl Value + ?Sized)) -> stream::Result {
        v.stream(self.by_ref())
    }

    pub fn debug(&mut self, v: &'v impl Debug) -> stream::Result {
        self.inner().fmt_borrowed(stream::Arguments::debug(v))
    }

    pub fn display(&mut self, v: &'v impl Display) -> stream::Result {
        self.inner().fmt_borrowed(stream::Arguments::display(v))
    }

    #[cfg(feature = "std")]
    pub fn error(&mut self, v: &'v (dyn error::Error + 'static)) -> stream::Result {
        self.inner().error_borrowed(stream::Source::new(v))
    }

    pub fn i64(&mut self, v: i64) -> stream::Result {
        self.inner().i64(v)
    }

    pub fn u64(&mut self, v: u64) -> stream::Result {
        self.inner().u64(v)
    }

    pub fn i128(&mut self, v: i128) -> stream::Result {
        self.inner().i128(v)
    }

    pub fn u128(&mut self, v: u128) -> stream::Result {
        self.inner().u128(v)
    }

    pub fn f64(&mut self, v: f64) -> stream::Result {
        self.inner().f64(v)
    }

    pub fn bool(&mut self, v: bool) -> stream::Result {
        self.inner().bool(v)
    }

    pub fn char(&mut self, v: char) -> stream::Result {
        self.inner().char(v)
    }

    pub fn str(&mut self, v: &'v str) -> stream::Result {
        self.inner().str_borrowed(v)
    }

    pub fn none(&mut self) -> stream::Result {
        self.inner().none()
    }

    pub fn tag(&mut self, t: stream::Tag<'v>) -> stream::Result {
        self.inner().tag_borrowed(t)
    }

    pub fn tagged(&mut self, t: stream::Tag<'v>, v: &'v impl Value) -> stream::Result {
        self.inner()
            .tagged_collect_borrowed(t, stream::Value::new(v))
    }

    pub fn ident(&mut self, v: stream::Ident<'v>) -> stream::Result {
        self.inner().ident(v)
    }

    pub fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().map_begin(len)
    }

    pub fn map_key(&mut self, k: &'v impl Value) -> stream::Result {
        // NOTE: With specialization we could add a `?Sized` bound to `impl Value`
        // This would let us continue to forward to `collect_borrowed` for sized values
        self.inner().map_key_collect_borrowed(stream::Value::new(k))
    }

    pub fn map_value(&mut self, v: &'v impl Value) -> stream::Result {
        // NOTE: With specialization we could add a `?Sized` bound to `impl Value`
        // This would let us continue to forward to `collect_borrowed` for sized values
        self.inner()
            .map_value_collect_borrowed(stream::Value::new(v))
    }

    pub fn map_end(&mut self) -> stream::Result {
        self.inner().map_end()
    }

    pub fn tagged_map_begin(&mut self, t: stream::Tag<'v>, len: Option<usize>) -> stream::Result {
        self.inner().tagged_map_begin_borrowed(t, len)
    }

    pub fn tagged_map_end(&mut self) -> stream::Result {
        self.inner().tagged_map_end()
    }

    pub fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().seq_begin(len)
    }

    pub fn seq_elem(&mut self, v: &'v impl Value) -> stream::Result {
        // NOTE: With specialization we could add a `?Sized` bound to `impl Value`
        // This would let us continue to forward to `collect_borrowed` for sized values
        self.inner()
            .seq_elem_collect_borrowed(stream::Value::new(v))
    }

    pub fn seq_end(&mut self) -> stream::Result {
        self.inner().seq_end()
    }

    pub fn tagged_seq_begin(&mut self, t: stream::Tag<'v>, len: Option<usize>) -> stream::Result {
        self.inner().tagged_seq_begin_borrowed(t, len)
    }

    pub fn tagged_seq_end(&mut self) -> stream::Result {
        self.inner().tagged_seq_end()
    }
}

impl<'s, 'v> Stream<'s, 'v> {
    pub fn map_key_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.inner().map_key()?;

        Ok(self)
    }

    pub fn map_value_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.inner().map_value()?;

        Ok(self)
    }

    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.inner().seq_elem()?;

        Ok(self)
    }
}

impl<'s, 'v> stream::Stream<'v> for Stream<'s, 'v> {
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.inner().fmt(v)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.inner().error(v)
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.inner().i64(v)
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.inner().u64(v)
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.inner().i128(v)
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.inner().u128(v)
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.inner().f64(v)
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.inner().bool(v)
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.inner().char(v)
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.inner().str(v)
    }

    fn tag(&mut self, t: stream::Tag) -> stream::Result {
        self.inner().tag(t)
    }

    fn ident(&mut self, v: stream::Ident) -> stream::Result {
        self.inner().ident(v)
    }

    fn none(&mut self) -> stream::Result {
        self.inner().none()
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().map_begin(len)
    }

    fn map_key(&mut self) -> stream::Result {
        self.inner().map_key()
    }

    fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
        self.inner().map_key_collect(k)
    }

    fn map_value(&mut self) -> stream::Result {
        self.inner().map_value()
    }

    fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
        self.inner().map_value_collect(v)
    }

    fn map_end(&mut self) -> stream::Result {
        self.inner().map_end()
    }

    fn tagged_map_begin(&mut self, tag: stream::Tag, len: Option<usize>) -> stream::Result {
        self.inner().tagged_map_begin(tag, len)
    }

    fn tagged_map_end(&mut self) -> stream::Result {
        self.inner().tagged_map_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().seq_begin(len)
    }

    fn seq_elem(&mut self) -> stream::Result {
        self.inner().seq_elem()
    }

    fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
        self.inner().seq_elem_collect(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        self.inner().seq_end()
    }

    fn tagged_seq_begin(&mut self, tag: stream::Tag, len: Option<usize>) -> stream::Result {
        self.inner().tagged_seq_begin(tag, len)
    }

    fn tagged_seq_end(&mut self) -> stream::Result {
        self.inner().tagged_seq_end()
    }

    fn tagged_collect(&mut self, t: stream::Tag, v: stream::Value) -> stream::Result {
        self.inner().tagged_collect(t, v)
    }

    fn fmt_borrowed(&mut self, v: stream::Arguments<'v>) -> stream::Result {
        self.inner().fmt_borrowed(v)
    }

    fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
        self.inner().error_borrowed(v)
    }

    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.inner().str_borrowed(v)
    }

    fn tag_borrowed(&mut self, t: stream::Tag<'v>) -> stream::Result {
        self.inner().tag_borrowed(t)
    }

    fn ident_borrowed(&mut self, v: stream::Ident<'v>) -> stream::Result {
        self.inner().ident_borrowed(v)
    }

    fn tagged_map_begin_borrowed(
        &mut self,
        tag: stream::Tag<'v>,
        len: Option<usize>,
    ) -> stream::Result {
        self.inner().tagged_map_begin_borrowed(tag, len)
    }

    fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
        self.inner().map_key_collect_borrowed(k)
    }

    fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.inner().map_value_collect_borrowed(v)
    }

    fn tagged_seq_begin_borrowed(
        &mut self,
        tag: stream::Tag<'v>,
        len: Option<usize>,
    ) -> stream::Result {
        self.inner().tagged_seq_begin_borrowed(tag, len)
    }

    fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.inner().seq_elem_collect_borrowed(v)
    }

    fn tagged_collect_borrowed(
        &mut self,
        t: stream::Tag<'v>,
        v: stream::Value<'v>,
    ) -> stream::Result {
        self.inner().tagged_collect_borrowed(t, v)
    }
}

impl<'a, 'v, S> stream::Stream<'v> for Owned<S>
where
    S: stream::Stream<'a>,
{
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.inner().fmt(v)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.inner().error(v)
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.inner().i64(v)
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.inner().u64(v)
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.inner().i128(v)
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.inner().u128(v)
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.inner().f64(v)
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.inner().bool(v)
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.inner().char(v)
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.inner().str(v)
    }

    fn none(&mut self) -> stream::Result {
        self.inner().none()
    }

    fn tag(&mut self, t: stream::Tag) -> stream::Result {
        self.inner().tag(t)
    }

    fn ident(&mut self, v: stream::Ident) -> stream::Result {
        self.inner().ident(v)
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().map_begin(len)
    }

    fn map_key(&mut self) -> stream::Result {
        self.inner().map_key()
    }

    fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
        self.inner().map_key_collect(k)
    }

    fn map_value(&mut self) -> stream::Result {
        self.inner().map_value()
    }

    fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
        self.inner().map_value_collect(v)
    }

    fn map_end(&mut self) -> stream::Result {
        self.inner().map_end()
    }

    fn tagged_map_begin(&mut self, tag: stream::Tag, len: Option<usize>) -> stream::Result {
        self.inner().tagged_map_begin(tag, len)
    }

    fn tagged_map_end(&mut self) -> stream::Result {
        self.inner().tagged_map_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().seq_begin(len)
    }

    fn seq_elem(&mut self) -> stream::Result {
        self.inner().seq_elem()
    }

    fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
        self.inner().seq_elem_collect(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        self.inner().seq_end()
    }

    fn tagged_seq_begin(&mut self, tag: stream::Tag, len: Option<usize>) -> stream::Result {
        self.inner().tagged_seq_begin(tag, len)
    }

    fn tagged_seq_end(&mut self) -> stream::Result {
        self.inner().tagged_seq_end()
    }

    fn tagged_collect(&mut self, t: stream::Tag, v: stream::Value) -> stream::Result {
        self.inner().tagged_collect(t, v)
    }

    fn fmt_borrowed(&mut self, v: stream::Arguments<'v>) -> stream::Result {
        self.inner().fmt(v)
    }

    fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
        self.inner().error(v)
    }

    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.inner().str(v)
    }

    fn tag_borrowed(&mut self, t: stream::Tag<'v>) -> stream::Result {
        self.inner().tag(t)
    }

    fn ident_borrowed(&mut self, v: stream::Ident<'v>) -> stream::Result {
        self.inner().ident(v)
    }

    fn tagged_map_begin_borrowed(
        &mut self,
        tag: stream::Tag<'v>,
        len: Option<usize>,
    ) -> stream::Result {
        self.inner().tagged_map_begin(tag, len)
    }

    fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
        self.inner().map_key_collect(k)
    }

    fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.inner().map_value_collect(v)
    }

    fn tagged_seq_begin_borrowed(
        &mut self,
        tag: stream::Tag<'v>,
        len: Option<usize>,
    ) -> stream::Result {
        self.inner().tagged_seq_begin(tag, len)
    }

    fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.inner().seq_elem_collect(v)
    }

    fn tagged_collect_borrowed(
        &mut self,
        t: stream::Tag<'v>,
        v: stream::Value<'v>,
    ) -> stream::Result {
        self.inner().tagged_collect(t, v)
    }
}
