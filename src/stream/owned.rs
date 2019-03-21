use crate::{
    collect::{
        self,
        OwnedCollect,
    },
    stream::{
        Arguments,
        Error,
        Stream,
    },
    value::Value,
};

/**
An owned stream.
*/
pub struct OwnedStream<S>(OwnedCollect<collect::Default<S>>);

impl<S> OwnedStream<S>
where
    S: Stream,
{
    /**
    Begin an owned stream.
    */
    #[inline]
    pub fn begin(stream: S) -> Result<Self, Error> {
        Ok(OwnedStream(OwnedCollect::begin(collect::Default(stream))?))
    }

    /**
    Complete an owned stream.
    */
    #[inline]
    pub fn end(self) -> Result<S, Error> {
        Ok(self.0.end()?.0)
    }

    /**
    Stream a value.
    */
    #[inline]
    pub fn any(&mut self, v: impl Value) -> Result<(), Error> {
        self.0.any(v)
    }

    /**
    Stream a format.
    */
    #[inline]
    pub fn fmt(&mut self, f: Arguments) -> Result<(), Error> {
        self.0.fmt(f)
    }

    /**
    Stream a signed integer.
    */
    #[inline]
    pub fn i64(&mut self, v: i64) -> Result<(), Error> {
        self.0.i64(v)
    }

    /**
    Stream an unsigned integer.
    */
    #[inline]
    pub fn u64(&mut self, v: u64) -> Result<(), Error> {
        self.0.u64(v)
    }

    /**
    Stream a 128-bit signed integer.
    */
    #[inline]
    pub fn i128(&mut self, v: i128) -> Result<(), Error> {
        self.0.i128(v)
    }

    /**
    Stream a 128-bit unsigned integer.
    */
    #[inline]
    pub fn u128(&mut self, v: u128) -> Result<(), Error> {
        self.0.u128(v)
    }

    /**
    Stream a floating point value.
    */
    #[inline]
    pub fn f64(&mut self, v: f64) -> Result<(), Error> {
        self.0.f64(v)
    }

    /**
    Stream a boolean.
    */
    #[inline]
    pub fn bool(&mut self, v: bool) -> Result<(), Error> {
        self.0.bool(v)
    }

    /**
    Stream a unicode character.
    */
    #[inline]
    pub fn char(&mut self, v: char) -> Result<(), Error> {
        self.0.char(v)
    }

    /**
    Stream a UTF8 string.
    */
    #[inline]
    pub fn str(&mut self, v: &str) -> Result<(), Error> {
        self.0.str(v)
    }

    /**
    Stream an empty value.
    */
    #[inline]
    pub fn none(&mut self) -> Result<(), Error> {
        self.0.none()
    }

    /**
    Begin a map.
    */
    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        self.0.map_begin(len)
    }

    /**
    Stream a map key.
    */
    #[inline]
    pub fn map_key(&mut self, k: impl Value) -> Result<(), Error> {
        self.0.map_key(k)
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn map_value(&mut self, v: impl Value) -> Result<(), Error> {
        self.0.map_value(v)
    }

    /**
    End a map.
    */
    #[inline]
    pub fn map_end(&mut self) -> Result<(), Error> {
        self.0.map_end()
    }

    /**
    Begin a sequence.
    */
    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        self.0.seq_begin(len)
    }

    /**
    Stream a sequence element.
    */
    #[inline]
    pub fn seq_elem(&mut self, v: impl Value) -> Result<(), Error> {
        self.0.seq_elem(v)
    }

    /**
    End a sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> Result<(), Error> {
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
