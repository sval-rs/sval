use crate::{
    collect::{
        self,
        Collect,
    },
    stream::{
        stack::DebugStack,
        Arguments,
    },
    value::{
        Error,
        Value,
    },
};

/**
A borrowed value stream.

This type is a wrapper for a [`stream::Stream`] with a more ergonomic interface.

[`stream::Stream`]: ../stream/trait.Stream.html
*/
pub struct Stream<'a> {
    stack: &'a mut DebugStack,
    stream: &'a mut dyn Collect,
}

impl<'a> Stream<'a> {
    #[inline]
    pub(crate) fn new(stream: &'a mut dyn Collect, stack: &'a mut DebugStack) -> Self {
        Stream { stack, stream }
    }

    /**
    Stream a value.
    */
    #[inline]
    pub fn any(&mut self, v: impl Value) -> Result<(), Error> {
        v.stream(self)
    }

    /**
    Stream a format.
    */
    #[inline]
    pub fn fmt(&mut self, f: Arguments) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.fmt(f)?;

        Ok(())
    }

    /**
    Stream a signed integer.
    */
    #[inline]
    pub fn i64(&mut self, v: i64) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.i64(v)?;

        Ok(())
    }

    /**
    Stream an unsigned integer.
    */
    #[inline]
    pub fn u64(&mut self, v: u64) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.u64(v)?;

        Ok(())
    }

    /**
    Stream a 128-bit signed integer.
    */
    #[inline]
    pub fn i128(&mut self, v: i128) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.i128(v)?;

        Ok(())
    }

    /**
    Stream a 128-bit unsigned integer.
    */
    #[inline]
    pub fn u128(&mut self, v: u128) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.u128(v)?;

        Ok(())
    }

    /**
    Stream a floating point value.
    */
    #[inline]
    pub fn f64(&mut self, v: f64) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.f64(v)?;

        Ok(())
    }

    /**
    Stream a boolean.
    */
    #[inline]
    pub fn bool(&mut self, v: bool) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.bool(v)?;

        Ok(())
    }

    /**
    Stream a unicode character.
    */
    #[inline]
    pub fn char(&mut self, v: char) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.char(v)?;

        Ok(())
    }

    /**
    Stream a UTF8 string.
    */
    #[inline]
    pub fn str(&mut self, v: &str) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.str(v)?;

        Ok(())
    }

    /**
    Stream an empty value.
    */
    #[inline]
    pub fn none(&mut self) -> Result<(), Error> {
        self.stack.primitive()?;

        self.stream.none()?;

        Ok(())
    }

    /**
    Begin a map.
    */
    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        self.stack.map_begin()?;

        self.stream.map_begin(len)?;

        Ok(())
    }

    /**
    Stream a map key.
    */
    #[inline]
    pub fn map_key(&mut self, k: impl Value) -> Result<(), Error> {
        self.stack.map_key()?;

        self.stream
            .map_key_collect(collect::Value::new(self.stack, &k))?;

        Ok(())
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn map_value(&mut self, v: impl Value) -> Result<(), Error> {
        self.stack.map_value()?;

        self.stream
            .map_value_collect(collect::Value::new(self.stack, &v))?;

        Ok(())
    }

    /**
    End a map.
    */
    #[inline]
    pub fn map_end(&mut self) -> Result<(), Error> {
        self.stack.map_end()?;

        self.stream.map_end()?;

        Ok(())
    }

    /**
    Begin a sequence.
    */
    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        self.stack.seq_begin()?;

        self.stream.seq_begin(len)?;

        Ok(())
    }

    /**
    Stream a sequence element.
    */
    #[inline]
    pub fn seq_elem(&mut self, v: impl Value) -> Result<(), Error> {
        self.stack.seq_elem()?;

        self.stream
            .seq_elem_collect(collect::Value::new(self.stack, &v))?;

        Ok(())
    }

    /**
    End a sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> Result<(), Error> {
        self.stack.seq_end()?;

        self.stream.seq_end()?;

        Ok(())
    }
}

impl<'a> Stream<'a> {
    /**
    Begin a map key.
    */
    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Self, Error> {
        self.stack.map_key()?;

        self.stream.map_key()?;

        Ok(self)
    }

    /**
    Begin a map value.
    */
    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Self, Error> {
        self.stack.map_value()?;

        self.stream.map_value()?;

        Ok(self)
    }

    /**
    Begin a sequence element.
    */
    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, Error> {
        self.stack.seq_elem()?;

        self.stream.seq_elem()?;

        Ok(self)
    }
}
