mod impls;

#[doc(inline)]
pub use crate::Error;

use crate::{
    std::fmt,
    stream,
};

/**
A value with a streamable structure.

Use the [`sval::stream`] function to stream a value.
*/
pub trait Value {
    /** Stream this value. */
    fn stream(&self, stream: &mut Stream) -> Result<(), Error>;
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn stream(&self, stream: &mut Stream) -> Result<(), Error> {
        (**self).stream(stream)
    }
}

/**
A value stream.
*/
pub struct Stream<'a> {
    #[cfg(any(debug_assertions, test))]
    stack: &'a mut stream::Stack,
    stream: &'a mut dyn stream::Stream,
}

impl<'a> Stream<'a> {
    /**
    Begin a new value stream.
    */
    #[inline]
    #[cfg(any(debug_assertions, test))]
    pub(crate) fn new(
        stack: &'a mut stream::Stack,
        stream: &'a mut dyn stream::Stream,
    ) -> Result<Self, Error> {
        Ok(Stream { stack, stream })
    }

    /**
    Begin a new value stream.
    */
    #[inline]
    #[cfg(all(not(debug_assertions), not(test)))]
    pub(crate) fn new(stream: &'a mut dyn stream::Stream) -> Result<Self, Error> {
        Ok(Stream { stream })
    }

    pub(crate) fn begin(&mut self) -> Result<(), Error> {
        self.stream.begin()?;

        Ok(())
    }

    /**
    Stream a value.
    */
    #[inline]
    pub fn any(&mut self, v: impl Value) -> Result<(), Error> {
        v.stream(self)
    }

    /**
    Stream format arguments.
    */
    #[inline]
    pub fn fmt(&mut self, f: fmt::Arguments) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.fmt(f)?;

        Ok(())
    }

    /**
    Stream a signed integer.
    */
    #[inline]
    pub fn i64(&mut self, v: i64) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.i64(v)?;

        Ok(())
    }

    /**
    Stream an unsigned integer.
    */
    #[inline]
    pub fn u64(&mut self, v: u64) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.u64(v)?;

        Ok(())
    }

    /**
    Stream a 128-bit signed integer.
    */
    #[inline]
    pub fn i128(&mut self, v: i128) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.i128(v)?;

        Ok(())
    }

    /**
    Stream a 128-bit unsigned integer.
    */
    #[inline]
    pub fn u128(&mut self, v: u128) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.u128(v)?;

        Ok(())
    }

    /**
    Stream a floating point value.
    */
    #[inline]
    pub fn f64(&mut self, v: f64) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.f64(v)?;

        Ok(())
    }

    /**
    Stream a boolean.
    */
    #[inline]
    pub fn bool(&mut self, v: bool) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.bool(v)?;

        Ok(())
    }

    /**
    Stream a unicode character.
    */
    #[inline]
    pub fn char(&mut self, v: char) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.char(v)?;

        Ok(())
    }

    /**
    Stream a UTF8 string.
    */
    #[inline]
    pub fn str(&mut self, v: &str) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.str(v)?;

        Ok(())
    }

    /**
    Stream an empty value.
    */
    #[inline]
    pub fn none(&mut self) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.primitive()?;

        self.stream.none()?;

        Ok(())
    }

    /**
    Begin a map.
    */
    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.map_begin()?;

        self.stream.map_begin(len)?;

        Ok(())
    }

    /**
    Begin a map key.
    */
    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Stream<'a>, Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.map_key()?;

        self.stream.map_key()?;

        Ok(self)
    }

    /**
    Stream a map key.
    */
    #[inline]
    pub fn map_key(&mut self, k: impl Value) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.map_key()?;

        let value = {
            #[cfg(any(debug_assertions, test))]
            {
                stream::Value::new(&mut self.stack, &k)
            }
            #[cfg(all(not(debug_assertions), not(test)))]
            {
                stream::Value::new(&k)
            }
        };

        self.stream.map_key_collect(value)?;

        Ok(())
    }

    /**
    Begin a map value.
    */
    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Stream<'a>, Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.map_value()?;

        self.stream.map_value()?;

        Ok(self)
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn map_value(&mut self, k: impl Value) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.map_value()?;

        let value = {
            #[cfg(any(debug_assertions, test))]
            {
                stream::Value::new(&mut self.stack, &k)
            }
            #[cfg(all(not(debug_assertions), not(test)))]
            {
                stream::Value::new(&k)
            }
        };

        self.stream.map_value_collect(value)?;

        Ok(())
    }

    /**
    End a map.
    */
    #[inline]
    pub fn map_end(&mut self) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.map_end()?;

        self.stream.map_end()?;

        Ok(())
    }

    /**
    Begin a sequence.
    */
    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.seq_begin()?;

        self.stream.seq_begin(len)?;

        Ok(())
    }

    /**
    Begin a sequence element.
    */
    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Stream<'a>, Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.seq_elem()?;

        self.stream.seq_elem()?;

        Ok(self)
    }

    /**
    Stream a sequence element.
    */
    #[inline]
    pub fn seq_elem(&mut self, k: impl Value) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.seq_elem()?;

        let value = {
            #[cfg(any(debug_assertions, test))]
            {
                stream::Value::new(&mut self.stack, &k)
            }
            #[cfg(all(not(debug_assertions), not(test)))]
            {
                stream::Value::new(&k)
            }
        };

        self.stream.seq_elem_collect(value)?;

        Ok(())
    }

    /**
    End a sequence.
    */
    #[inline]
    pub fn seq_end(&mut self) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.seq_end()?;

        self.stream.seq_end()?;

        Ok(())
    }

    /**
    End the stream.
    */
    #[inline]
    pub(crate) fn end(self) -> Result<(), Error> {
        #[cfg(any(debug_assertions, test))]
        self.stack.end()?;

        self.stream.end()
    }
}
