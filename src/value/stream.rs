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
pub struct Stream<'stream, 'value>(&'stream mut dyn stream::Stream<'value>);

impl<'stream, 'value> Stream<'stream, 'value> {
    /**
    Wrap an implementation of [`Stream`].

    [`Stream`]: ../stream/trait.Stream.html
    */
    #[inline]
    pub fn new(stream: &'stream mut impl stream::Stream<'value>) -> Self {
        Stream(stream)
    }

    /**
    Stream an implementation of [`Value`].

    [`Value`]: ./trait.Value.html
    */
    #[inline]
    pub fn any(&mut self, v: impl Value) -> stream::Result {
        v.stream(self)
    }

    /**
    Stream a debuggable type.
    */
    #[inline]
    pub fn debug(&mut self, v: impl Debug) -> stream::Result {
        self.0.fmt(stream::Arguments::debug(&v))
    }

    /**
    Stream a displayable type.
    */
    #[inline]
    pub fn display(&mut self, v: impl Display) -> stream::Result {
        self.0.fmt(stream::Arguments::display(&v))
    }

    /**
    Stream an error.

    This method is only available when the `std` feature is enabled.

    # Examples

    Errors that don't satisfy the trait bounds needed by this method can go through [`Source`](struct.Source.html):

    ```
    # #![cfg(feature = "std")]
    # use sval::value::{self, Value};
    # struct MyError {
    #    error: std::io::Error,
    # }
    impl Value for MyError {
        fn stream(&self, stream: &mut value::Stream) -> value::Result {
            use sval::stream::Source;

            stream.any(Source::new(&self.error))
        }
    }
    # fn main() {}
    ```
    */
    #[inline]
    #[cfg(feature = "std")]
    pub fn error(&mut self, v: &(dyn error::Error + 'static)) -> stream::Result {
        self.0.error(stream::Source::new(v))
    }

    /**
    Stream a signed integer.
    */
    #[inline]
    pub fn i64(&mut self, v: i64) -> stream::Result {
        self.0.i64(v)
    }

    /**
    Stream an unsigned integer.
    */
    #[inline]
    pub fn u64(&mut self, v: u64) -> stream::Result {
        self.0.u64(v)
    }

    /**
    Stream a 128-bit signed integer.
    */
    #[inline]
    pub fn i128(&mut self, v: i128) -> stream::Result {
        self.0.i128(v)
    }

    /**
    Stream a 128-bit unsigned integer.
    */
    #[inline]
    pub fn u128(&mut self, v: u128) -> stream::Result {
        self.0.u128(v)
    }

    /**
    Stream a floating point value.
    */
    #[inline]
    pub fn f64(&mut self, v: f64) -> stream::Result {
        self.0.f64(v)
    }

    /**
    Stream a boolean.
    */
    #[inline]
    pub fn bool(&mut self, v: bool) -> stream::Result {
        self.0.bool(v)
    }

    /**
    Stream a unicode character.
    */
    #[inline]
    pub fn char(&mut self, v: char) -> stream::Result {
        self.0.char(v)
    }

    /**
    Stream a UTF8 string.
    */
    #[inline]
    pub fn str(&mut self, v: &str) -> stream::Result {
        self.0.str(v)
    }

    /**
    Stream a UTF8 string.
    */
    #[inline]
    pub fn borrowed_str(&mut self, v: &'value str) -> stream::Result {
        self.0.borrowed_str(v)
    }

    /**
    Stream an empty value.
    */
    #[inline]
    pub fn none(&mut self) -> stream::Result {
        self.0.none()
    }

    /**
    Begin a map.
    */
    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.map_begin(len)
    }

    /**
    Stream a map key.
    */
    #[inline]
    pub fn map_key(&mut self, k: impl Value) -> stream::Result {
        self.0.map_key_collect(&stream::Value::new(&k))
    }

    /**
    Stream a map key.
    */
    #[inline]
    pub fn borrowed_map_key(&mut self, k: &'value impl Value) -> stream::Result {
        self.0.borrowed_map_key_collect(&stream::Value::new(k))
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn map_value(&mut self, v: impl Value) -> stream::Result {
        self.0.map_value_collect(&stream::Value::new(&v))
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn borrowed_map_value(&mut self, v: &'value impl Value) -> stream::Result {
        self.0.borrowed_map_value_collect(&stream::Value::new(v))
    }

    /**
    End a map.
    */
    #[inline]
    pub fn map_end(&mut self) -> stream::Result {
        self.0.map_end()
    }

    /**
    Begin a sequence.
    */
    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.0.seq_begin(len)
    }

    /**
    Stream a sequence element.
    */
    #[inline]
    pub fn seq_elem(&mut self, v: impl Value) -> stream::Result {
        self.0.seq_elem_collect(&stream::Value::new(&v))
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn borrowed_seq_elem(&mut self, v: &'value impl Value) -> stream::Result {
        self.0.borrowed_seq_elem_collect(&stream::Value::new(v))
    }

    /**
    End a sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> stream::Result {
        self.0.seq_end()
    }
}

impl<'stream, 'value> Stream<'stream, 'value> {
    /**
    Begin a map key.
    */
    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.0.map_key()?;

        Ok(self)
    }

    /**
    Begin a map value.
    */
    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.0.map_value()?;

        Ok(self)
    }

    /**
    Begin a sequence element.
    */
    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, crate::Error> {
        self.0.seq_elem()?;

        Ok(self)
    }
}

impl<'stream, 'value> stream::Stream<'value> for Stream<'stream, 'value> {
    #[inline]
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        self.any(v)
    }

    #[inline]
    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.any(v)
    }

    #[inline]
    fn i64(&mut self, v: i64) -> stream::Result {
        self.i64(v)
    }

    #[inline]
    fn u64(&mut self, v: u64) -> stream::Result {
        self.u64(v)
    }

    #[inline]
    fn i128(&mut self, v: i128) -> stream::Result {
        self.i128(v)
    }

    #[inline]
    fn u128(&mut self, v: u128) -> stream::Result {
        self.u128(v)
    }

    #[inline]
    fn f64(&mut self, v: f64) -> stream::Result {
        self.f64(v)
    }

    #[inline]
    fn bool(&mut self, v: bool) -> stream::Result {
        self.bool(v)
    }

    #[inline]
    fn char(&mut self, v: char) -> stream::Result {
        self.char(v)
    }

    #[inline]
    fn str(&mut self, v: &str) -> stream::Result {
        self.str(v)
    }

    #[inline]
    fn borrowed_str(&mut self, v: &'value str) -> stream::Result {
        self.borrowed_str(v)
    }

    #[inline]
    fn none(&mut self) -> stream::Result {
        self.none()
    }

    #[inline]
    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.map_begin(len)
    }

    #[inline]
    fn map_key(&mut self) -> stream::Result {
        self.map_key_begin().map(|_| ())
    }

    #[inline]
    fn map_key_collect(&mut self, k: &stream::Value) -> stream::Result {
        self.map_key(k).map(|_| ())
    }

    #[inline]
    fn map_value(&mut self) -> stream::Result {
        self.map_value_begin().map(|_| ())
    }

    #[inline]
    fn map_value_collect(&mut self, v: &stream::Value) -> stream::Result {
        self.map_value(v).map(|_| ())
    }

    #[inline]
    fn map_end(&mut self) -> stream::Result {
        self.map_end()
    }

    #[inline]
    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.seq_begin(len)
    }

    #[inline]
    fn seq_elem(&mut self) -> stream::Result {
        self.seq_elem_begin().map(|_| ())
    }

    #[inline]
    fn seq_elem_collect(&mut self, v: &stream::Value) -> stream::Result {
        self.seq_elem(v).map(|_| ())
    }

    #[inline]
    fn seq_end(&mut self) -> stream::Result {
        self.seq_end()
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
            stream.map_key("key")?;
            stream.map_value(42)?;
            stream.map_end()
        }

        fn takes_stream<'stream>(mut stream: impl stream::Stream<'stream>) -> stream::Result {
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
