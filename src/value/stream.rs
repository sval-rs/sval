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

/**
A stream that can receive the structure of a value.
*/
pub struct Stream<'s, 'v>(Owned<&'s mut dyn stream::Stream<'v>>);

impl<'s, 'v> From<&'s mut dyn stream::Stream<'v>> for Stream<'s, 'v> {
    fn from(stream: &'s mut dyn stream::Stream<'v>) -> Self {
        Stream(Owned(stream))
    }
}

struct Owned<S>(S);

impl<'s, 'v> Stream<'s, 'v> {
    /**
    Wrap an implementation of [`Stream`].

    [`Stream`]: ../stream/trait.Stream.html
    */
    pub fn new(stream: &'s mut impl stream::Stream<'v>) -> Self {
        Stream(Owned(stream))
    }

    /**
    Wrap this stream so it can accept borrowed data of any lifetime.
    */
    pub fn owned<'a, 'b>(&'a mut self) -> Stream<'a, 'b> {
        Stream(Owned(&mut self.0))
    }

    pub fn borrowed<'a>(&'a mut self) -> Stream<'a, 'v> {
        Stream(Owned((self.0).0))
    }

    fn inner(&mut self) -> &mut dyn stream::Stream<'v> {
        (self.0).0
    }

    /**
    Stream an implementation of [`Value`].

    [`Value`]: ./trait.Value.html
    */
    pub fn any(&mut self, v: &'v (impl Value + ?Sized)) -> stream::Result {
        v.stream(self.borrowed())
    }

    /**
    Stream a debuggable type.
    */
    pub fn debug(&mut self, v: &'v impl Debug) -> stream::Result {
        self.inner().fmt_borrowed(stream::Arguments::debug(v))
    }

    /**
    Stream a displayable type.
    */
    pub fn display(&mut self, v: &'v impl Display) -> stream::Result {
        self.inner().fmt_borrowed(stream::Arguments::display(v))
    }

    /**
    Stream an error.

    This method is only available when the `std` feature is enabled.
    */
    #[cfg(feature = "std")]
    pub fn error(&mut self, v: &'v (dyn error::Error + 'static)) -> stream::Result {
        self.inner().error_borrowed(stream::Source::new(v))
    }

    /**
    Stream a signed integer.
    */
    pub fn i64(&mut self, v: i64) -> stream::Result {
        self.inner().i64(v)
    }

    /**
    Stream an unsigned integer.
    */
    pub fn u64(&mut self, v: u64) -> stream::Result {
        self.inner().u64(v)
    }

    /**
    Stream a 128-bit signed integer.
    */
    pub fn i128(&mut self, v: i128) -> stream::Result {
        self.inner().i128(v)
    }

    /**
    Stream a 128-bit unsigned integer.
    */
    pub fn u128(&mut self, v: u128) -> stream::Result {
        self.inner().u128(v)
    }

    /**
    Stream a floating point value.
    */
    pub fn f64(&mut self, v: f64) -> stream::Result {
        self.inner().f64(v)
    }

    /**
    Stream a boolean.
    */
    pub fn bool(&mut self, v: bool) -> stream::Result {
        self.inner().bool(v)
    }

    /**
    Stream a unicode character.
    */
    pub fn char(&mut self, v: char) -> stream::Result {
        self.inner().char(v)
    }

    /**
    Stream a UTF8 string.
    */
    pub fn str(&mut self, v: &'v str) -> stream::Result {
        self.inner().str_borrowed(v)
    }

    /**
    Stream an empty value.
    */
    pub fn none(&mut self) -> stream::Result {
        self.inner().none()
    }

    /**
    Begin a map.
    */
    pub fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().map_begin(len)
    }

    /**
    Stream a map key.
    */
    pub fn map_key(&mut self, k: &'v impl Value) -> stream::Result {
        // NOTE: With specialization we could add a `?Sized` bound to `impl Value`
        // This would let us continue to forward to `collect_borrowed` for sized values
        self.inner().map_key_collect_borrowed(stream::Value::new(k))
    }

    /**
    Stream a map value.
    */
    pub fn map_value(&mut self, v: &'v impl Value) -> stream::Result {
        // NOTE: With specialization we could add a `?Sized` bound to `impl Value`
        // This would let us continue to forward to `collect_borrowed` for sized values
        self.inner()
            .map_value_collect_borrowed(stream::Value::new(v))
    }

    /**
    End a map.
    */
    pub fn map_end(&mut self) -> stream::Result {
        self.inner().map_end()
    }

    /**
    Begin a sequence.
    */
    pub fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().seq_begin(len)
    }

    /**
    Stream a sequence element.
    */
    pub fn seq_elem(&mut self, v: &'v impl Value) -> stream::Result {
        // NOTE: With specialization we could add a `?Sized` bound to `impl Value`
        // This would let us continue to forward to `collect_borrowed` for sized values
        self.inner()
            .seq_elem_collect_borrowed(stream::Value::new(v))
    }

    /**
    End a sequence.
    */
    pub fn seq_end(&mut self) -> stream::Result {
        self.inner().seq_end()
    }
}

impl<'s, 'v> Stream<'s, 'v> {
    /**
    Begin a map key.

    The map key must be followed by an item.
    */
    pub fn map_key_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.inner().map_key()?;

        Ok(self)
    }

    /**
    Begin a map value.

    The map value must be followed by an item.
    */
    pub fn map_value_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.inner().map_value()?;

        Ok(self)
    }

    /**
    Begin a sequence element.

    The sequence element must be followed by an item.
    */
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.inner().seq_elem()?;

        Ok(self)
    }
}

impl<'s, 'v> stream::Stream<'v> for Stream<'s, 'v> {
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.inner().fmt(v)
    }

    fn fmt_borrowed(&mut self, v: stream::Arguments<'v>) -> stream::Result {
        self.inner().fmt_borrowed(v)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.inner().error(v)
    }

    fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
        self.inner().error_borrowed(v)
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

    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.inner().str_borrowed(v)
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

    fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
        self.inner().map_key_collect_borrowed(k)
    }

    fn map_value(&mut self) -> stream::Result {
        self.inner().map_value()
    }

    fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
        self.inner().map_value_collect(v)
    }

    fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.inner().map_value_collect_borrowed(v)
    }

    fn map_end(&mut self) -> stream::Result {
        self.inner().map_end()
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

    fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.inner().seq_elem_collect_borrowed(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        self.inner().seq_end()
    }
}

impl<'a, 'v, S> stream::Stream<'v> for Owned<S>
where
    S: stream::Stream<'a>,
{
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.0.fmt(v)
    }

    fn fmt_borrowed(&mut self, v: stream::Arguments<'v>) -> stream::Result {
        self.0.fmt(v)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.0.error(v)
    }

    fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
        self.0.error(v)
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.0.i64(v)
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.0.u64(v)
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.0.i128(v)
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.0.u128(v)
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.0.f64(v)
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.0.bool(v)
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.0.char(v)
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.0.str(v)
    }

    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.0.str(v)
    }

    fn none(&mut self) -> stream::Result {
        self.0.none()
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.map_begin(len)
    }

    fn map_key(&mut self) -> stream::Result {
        self.0.map_key()
    }

    fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
        self.0.map_key_collect(k)
    }

    fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
        self.0.map_key_collect(k)
    }

    fn map_value(&mut self) -> stream::Result {
        self.0.map_value()
    }

    fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
        self.0.map_value_collect(v)
    }

    fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.0.map_value_collect(v)
    }

    fn map_end(&mut self) -> stream::Result {
        self.0.map_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.seq_begin(len)
    }

    fn seq_elem(&mut self) -> stream::Result {
        self.0.seq_elem()
    }

    fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
        self.0.seq_elem_collect(v)
    }

    fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.0.seq_elem_collect(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        self.0.seq_end()
    }
}
