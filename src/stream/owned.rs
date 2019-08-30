use crate::{
    collect::{
        self,
        OwnedCollect,
        RefMutCollect,
    },
    stream::{
        self,
        Arguments,
        Error,
        Stream,
    },
    value::Value,
};

/**
An owned stream wrapper.

`OwnedStream` is an ergonomic wrapper over a raw [`Stream`] that makes it
easier to stream complex types.
*/
pub struct OwnedStream<S>(OwnedCollect<collect::Default<S>>);

impl<S> OwnedStream<S>
where
    S: Stream,
{
    /**
    Stream a value.
    */
    #[inline]
    pub fn stream(stream: S, value: impl Value) -> Result<S, Error> {
        let mut stream = Self::new(stream);
        stream.any(value)?;
        Ok(stream.into_inner())
    }

    /**
    Begin an owned stream.
    */
    #[inline]
    pub fn new(stream: S) -> Self {
        OwnedStream(OwnedCollect::new(collect::Default(stream)))
    }

    /**
    Unwrap the inner stream.
    */
    #[inline]
    pub fn into_inner(self) -> S {
        self.0.into_inner().0
    }

    /**
    Get a reference to the stream that can be used by a value.
    */
    #[inline]
    pub fn borrow_mut(&mut self) -> RefMutStream {
        RefMutStream(self.0.borrow_mut())
    }

    /**
    Stream a value.
    */
    #[inline]
    pub fn any(&mut self, v: impl Value) -> stream::Result {
        self.0.any(v)
    }

    /**
    Stream a format.
    */
    #[inline]
    pub fn fmt(&mut self, f: Arguments) -> stream::Result {
        self.0.fmt(f)
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
        self.0.map_key(k)
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn map_value(&mut self, v: impl Value) -> stream::Result {
        self.0.map_value(v)
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
        self.0.seq_elem(v)
    }

    /**
    End a sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> stream::Result {
        self.0.seq_end()
    }
}

impl<S> OwnedStream<S>
where
    S: Stream,
{
    /**
    Begin a map key.
    */
    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Self, Error> {
        self.0.map_key_begin()?;

        Ok(self)
    }

    /**
    Begin a map value.
    */
    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Self, Error> {
        self.0.map_value_begin()?;

        Ok(self)
    }

    /**
    Begin a sequence element.
    */
    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, Error> {
        self.0.seq_elem_begin()?;

        Ok(self)
    }
}

/**
A borrowed stream wrapper.

This is the result of calling `OwnedStream.borrow_mut`.
*/
pub struct RefMutStream<'a>(RefMutCollect<'a>);

impl<'a> RefMutStream<'a> {
    #[inline]
    pub(crate) fn new(collect: RefMutCollect<'a>) -> Self {
        RefMutStream(collect)
    }

    /**
    Stream a value.
    */
    #[inline]
    pub fn any(&mut self, v: impl Value) -> stream::Result {
        self.0.any(v)
    }

    /**
    Stream a format.
    */
    #[inline]
    pub fn fmt(&mut self, f: Arguments) -> stream::Result {
        self.0.fmt(f)
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
        self.0.map_key(k)
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn map_value(&mut self, v: impl Value) -> stream::Result {
        self.0.map_value(v)
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
        self.0.seq_elem(v)
    }

    /**
    End a sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> stream::Result {
        self.0.seq_end()
    }
}

impl<'a> RefMutStream<'a> {
    /**
    Begin a map key.
    */
    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Self, Error> {
        self.0.map_key_begin()?;

        Ok(self)
    }

    /**
    Begin a map value.
    */
    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Self, Error> {
        self.0.map_value_begin()?;

        Ok(self)
    }

    /**
    Begin a sequence element.
    */
    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, Error> {
        self.0.seq_elem_begin()?;

        Ok(self)
    }
}
