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
    #[inline]
    pub fn new(stream: &'s mut impl stream::Stream<'v>) -> Self {
        Stream(Owned(stream))
    }

    /**
    Wrap this stream so it can accept borrowed data of any lifetime.
    */
    #[inline]
    pub fn owned<'a, 'b>(&'a mut self) -> Stream<'a, 'b> {
        Stream(Owned(&mut self.0))
    }

    #[inline]
    pub fn borrowed<'a>(&'a mut self) -> Stream<'a, 'v> {
        Stream(Owned((self.0).0))
    }

    #[inline]
    fn inner(&mut self) -> &mut dyn stream::Stream<'v> {
        (self.0).0
    }

    /**
    Stream an implementation of [`Value`].

    [`Value`]: ./trait.Value.html
    */
    #[inline]
    pub fn any(&mut self, v: &'v (impl Value + ?Sized)) -> stream::Result {
        v.stream(self.borrowed())
    }

    /**
    Stream a debuggable type.
    */
    #[inline]
    pub fn debug(&mut self, v: &'v impl Debug) -> stream::Result {
        self.inner().fmt_borrowed(&stream::Arguments::debug(v))
    }

    /**
    Stream a displayable type.
    */
    #[inline]
    pub fn display(&mut self, v: &'v impl Display) -> stream::Result {
        self.inner().fmt_borrowed(&stream::Arguments::display(v))
    }

    /**
    Stream an error.

    This method is only available when the `std` feature is enabled.
    */
    #[inline]
    #[cfg(feature = "std")]
    pub fn error(&mut self, v: &'v (dyn error::Error + 'static)) -> stream::Result {
        self.inner().error_borrowed(&stream::Source::new(v))
    }

    /**
    Stream a signed integer.
    */
    #[inline]
    pub fn i64(&mut self, v: i64) -> stream::Result {
        self.inner().i64(v)
    }

    /**
    Stream an unsigned integer.
    */
    #[inline]
    pub fn u64(&mut self, v: u64) -> stream::Result {
        self.inner().u64(v)
    }

    /**
    Stream a 128-bit signed integer.
    */
    #[inline]
    pub fn i128(&mut self, v: i128) -> stream::Result {
        self.inner().i128(v)
    }

    /**
    Stream a 128-bit unsigned integer.
    */
    #[inline]
    pub fn u128(&mut self, v: u128) -> stream::Result {
        self.inner().u128(v)
    }

    /**
    Stream a floating point value.
    */
    #[inline]
    pub fn f64(&mut self, v: f64) -> stream::Result {
        self.inner().f64(v)
    }

    /**
    Stream a boolean.
    */
    #[inline]
    pub fn bool(&mut self, v: bool) -> stream::Result {
        self.inner().bool(v)
    }

    /**
    Stream a unicode character.
    */
    #[inline]
    pub fn char(&mut self, v: char) -> stream::Result {
        self.inner().char(v)
    }

    /**
    Stream a UTF8 string.
    */
    #[inline]
    pub fn str(&mut self, v: &'v str) -> stream::Result {
        self.inner().str_borrowed(v)
    }

    /**
    Stream an empty value.
    */
    #[inline]
    pub fn none(&mut self) -> stream::Result {
        self.inner().none()
    }

    /**
    Begin a map.
    */
    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().map_begin(len)
    }

    /**
    Stream a map key.
    */
    #[inline]
    pub fn map_key(&mut self, k: &'v impl Value) -> stream::Result {
        // NOTE: With specialization we could add a `?Sized` bound to `impl Value`
        // This would let us continue to forward to `collect_borrowed` for sized values
        self.inner().map_key_collect_borrowed(&stream::Value::new(k))
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn map_value(&mut self, v: &'v impl Value) -> stream::Result {
        // NOTE: With specialization we could add a `?Sized` bound to `impl Value`
        // This would let us continue to forward to `collect_borrowed` for sized values
        self.inner().map_value_collect_borrowed(&stream::Value::new(v))
    }

    /**
    End a map.
    */
    #[inline]
    pub fn map_end(&mut self) -> stream::Result {
        self.inner().map_end()
    }

    /**
    Begin a sequence.
    */
    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().seq_begin(len)
    }

    /**
    Stream a sequence element.
    */
    #[inline]
    pub fn seq_elem(&mut self, v: &'v impl Value) -> stream::Result {
        // NOTE: With specialization we could add a `?Sized` bound to `impl Value`
        // This would let us continue to forward to `collect_borrowed` for sized values
        self.inner().seq_elem_collect_borrowed(&stream::Value::new(v))
    }

    /**
    End a sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> stream::Result {
        self.inner().seq_end()
    }
}

impl<'s, 'v> Stream<'s, 'v> {
    /**
    Begin a map key.

    The map key must be followed by an item.
    */
    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.inner().map_key()?;

        Ok(self)
    }

    /**
    Begin a map value.

    The map value must be followed by an item.
    */
    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.inner().map_value()?;

        Ok(self)
    }

    /**
    Begin a sequence element.

    The sequence element must be followed by an item.
    */
    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.inner().seq_elem()?;

        Ok(self)
    }
}

impl<'s, 'v> stream::Stream<'v> for Stream<'s, 'v> {
    #[inline]
    fn fmt(&mut self, v: &stream::Arguments) -> stream::Result {
        self.inner().fmt(v)
    }

    #[inline]
    fn fmt_borrowed(&mut self, v: &stream::Arguments<'v>) -> stream::Result {
        self.inner().fmt_borrowed(v)
    }

    #[inline]
    fn error(&mut self, v: &stream::Source) -> stream::Result {
        self.inner().error(v)
    }

    #[inline]
    fn error_borrowed(&mut self, v: &stream::Source<'v>) -> stream::Result {
        self.inner().error_borrowed(v)
    }

    #[inline]
    fn i64(&mut self, v: i64) -> stream::Result {
        self.inner().i64(v)
    }

    #[inline]
    fn u64(&mut self, v: u64) -> stream::Result {
        self.inner().u64(v)
    }

    #[inline]
    fn i128(&mut self, v: i128) -> stream::Result {
        self.inner().i128(v)
    }

    #[inline]
    fn u128(&mut self, v: u128) -> stream::Result {
        self.inner().u128(v)
    }

    #[inline]
    fn f64(&mut self, v: f64) -> stream::Result {
        self.inner().f64(v)
    }

    #[inline]
    fn bool(&mut self, v: bool) -> stream::Result {
        self.inner().bool(v)
    }

    #[inline]
    fn char(&mut self, v: char) -> stream::Result {
        self.inner().char(v)
    }

    #[inline]
    fn str(&mut self, v: &str) -> stream::Result {
        self.inner().str(v)
    }

    #[inline]
    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.inner().str_borrowed(v)
    }

    #[inline]
    fn none(&mut self) -> stream::Result {
        self.inner().none()
    }

    #[inline]
    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().map_begin(len)
    }

    #[inline]
    fn map_key(&mut self) -> stream::Result {
        self.inner().map_key()
    }

    #[inline]
    fn map_key_collect(&mut self, k: &stream::Value) -> stream::Result {
        self.inner().map_key_collect(k)
    }

    #[inline]
    fn map_key_collect_borrowed(&mut self, k: &stream::Value<'v>) -> stream::Result {
        self.inner().map_key_collect_borrowed(k)
    }

    #[inline]
    fn map_value(&mut self) -> stream::Result {
        self.inner().map_value()
    }

    #[inline]
    fn map_value_collect(&mut self, v: &stream::Value) -> stream::Result {
        self.inner().map_value_collect(v)
    }

    #[inline]
    fn map_value_collect_borrowed(&mut self, v: &stream::Value<'v>) -> stream::Result {
        self.inner().map_value_collect_borrowed(v)
    }

    #[inline]
    fn map_end(&mut self) -> stream::Result {
        self.inner().map_end()
    }

    #[inline]
    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.inner().seq_begin(len)
    }

    #[inline]
    fn seq_elem(&mut self) -> stream::Result {
        self.inner().seq_elem()
    }

    #[inline]
    fn seq_elem_collect(&mut self, v: &stream::Value) -> stream::Result {
        self.inner().seq_elem_collect(v)
    }

    #[inline]
    fn seq_elem_collect_borrowed(&mut self, v: &stream::Value<'v>) -> stream::Result {
        self.inner().seq_elem_collect_borrowed(v)
    }

    #[inline]
    fn seq_end(&mut self) -> stream::Result {
        self.inner().seq_end()
    }
}

impl<'a, 'v, S> stream::Stream<'v> for Owned<S>
where
    S: stream::Stream<'a>,
{
    #[inline]
    fn fmt(&mut self, v: &stream::Arguments) -> stream::Result {
        self.0.fmt(v)
    }

    #[inline]
    fn fmt_borrowed(&mut self, v: &stream::Arguments<'v>) -> stream::Result {
        self.0.fmt(v)
    }

    #[inline]
    fn error(&mut self, v: &stream::Source) -> stream::Result {
        self.0.error(v)
    }

    #[inline]
    fn error_borrowed(&mut self, v: &stream::Source<'v>) -> stream::Result {
        self.0.error(v)
    }

    #[inline]
    fn i64(&mut self, v: i64) -> stream::Result {
        self.0.i64(v)
    }

    #[inline]
    fn u64(&mut self, v: u64) -> stream::Result {
        self.0.u64(v)
    }

    #[inline]
    fn i128(&mut self, v: i128) -> stream::Result {
        self.0.i128(v)
    }

    #[inline]
    fn u128(&mut self, v: u128) -> stream::Result {
        self.0.u128(v)
    }

    #[inline]
    fn f64(&mut self, v: f64) -> stream::Result {
        self.0.f64(v)
    }

    #[inline]
    fn bool(&mut self, v: bool) -> stream::Result {
        self.0.bool(v)
    }

    #[inline]
    fn char(&mut self, v: char) -> stream::Result {
        self.0.char(v)
    }

    #[inline]
    fn str(&mut self, v: &str) -> stream::Result {
        self.0.str(v)
    }

    #[inline]
    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.0.str(v)
    }

    #[inline]
    fn none(&mut self) -> stream::Result {
        self.0.none()
    }

    #[inline]
    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.map_begin(len)
    }

    #[inline]
    fn map_key(&mut self) -> stream::Result {
        self.0.map_key()
    }

    #[inline]
    fn map_key_collect(&mut self, k: &stream::Value) -> stream::Result {
        self.0.map_key_collect(k)
    }

    #[inline]
    fn map_key_collect_borrowed(&mut self, k: &stream::Value<'v>) -> stream::Result {
        self.0.map_key_collect(k)
    }

    #[inline]
    fn map_value(&mut self) -> stream::Result {
        self.0.map_value()
    }

    #[inline]
    fn map_value_collect(&mut self, v: &stream::Value) -> stream::Result {
        self.0.map_value_collect(v)
    }

    #[inline]
    fn map_value_collect_borrowed(&mut self, v: &stream::Value<'v>) -> stream::Result {
        self.0.map_value_collect(v)
    }

    #[inline]
    fn map_end(&mut self) -> stream::Result {
        self.0.map_end()
    }

    #[inline]
    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.seq_begin(len)
    }

    #[inline]
    fn seq_elem(&mut self) -> stream::Result {
        self.0.seq_elem()
    }

    #[inline]
    fn seq_elem_collect(&mut self, v: &stream::Value) -> stream::Result {
        self.0.seq_elem_collect(v)
    }

    #[inline]
    fn seq_elem_collect_borrowed(&mut self, v: &stream::Value<'v>) -> stream::Result {
        self.0.seq_elem_collect(v)
    }

    #[inline]
    fn seq_end(&mut self) -> stream::Result {
        self.0.seq_end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::stream::Stack;

    #[test]
    fn stream_method_resolution() {
        fn takes_value_stream(mut stream: Stream) -> stream::Result {
            stream.map_begin(None)?;
            stream.map_key(&"key")?;
            stream.map_value(&42)?;
            stream.map_end()
        }

        fn takes_stream<'s>(mut stream: impl stream::Stream<'s>) -> stream::Result {
            stream.map_begin(None)?;
            stream.map_key()?;
            stream.str("key")?;
            stream.map_value()?;
            stream.i64(42)?;
            stream.map_end()
        }

        takes_value_stream(Stream::new(&mut Stack::default()))
            .expect("failed to use ref mut stream");
        takes_stream(Stream::new(&mut Stack::default())).expect("failed to use stream");
    }
}
