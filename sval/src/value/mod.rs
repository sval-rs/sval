/*!
A streamable value.
*/

#[macro_use]
mod macros;

mod impls;

pub(crate) mod collect;

#[doc(inline)]
pub use crate::Error;

use crate::{
    std::marker::PhantomData,
    stream,
};

/**
A value with a streamable structure.

Use the [`sval::stream`](../fn.stream.html) function to stream a value.
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
    stack: DebugStack<'a>,
    stream: &'a mut dyn collect::Stream,
}

impl<'a> Stream<'a> {
    #[inline]
    pub(crate) fn stream(value: impl Value, mut stream: impl collect::Stream) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                let mut stack = stream::Stack::default();
                let mut stream = Stream {
                    stack: DebugStack {
                        stack: &mut stack,
                        _m: PhantomData
                    },
                    stream: &mut stream
                };

                stream.begin()?;
                value.stream(&mut stream)?;
                stream.end()?;
            }
            else {
                let mut stream = Stream {
                    stack: DebugStack {
                        _m: PhantomData
                    },
                    stream: &mut stream
                };

                stream.begin()?;
                value.stream(&mut stream)?;
                stream.end()?;
            }
        }

        Ok(())
    }

    fn begin(&mut self) -> Result<(), Error> {
        self.stack.begin()?;

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
    pub fn fmt(&mut self, f: stream::Arguments) -> Result<(), Error> {
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
            .map_key_collect(collect::Value::new(self.stack.borrow_mut(), &k))?;

        Ok(())
    }

    /**
    Stream a map value.
    */
    #[inline]
    pub fn map_value(&mut self, v: impl Value) -> Result<(), Error> {
        self.stack.map_value()?;

        self.stream
            .map_value_collect(collect::Value::new(self.stack.borrow_mut(), &v))?;

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
            .seq_elem_collect(collect::Value::new(self.stack.borrow_mut(), &v))?;

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

    /**
    End the stream.
    */
    #[inline]
    fn end(mut self) -> Result<(), Error> {
        self.stack.end()?;

        self.stream.end()
    }
}

impl<'a> Stream<'a> {
    /**
    Begin a map key.
    */
    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Stream<'a>, Error> {
        self.stack.map_key()?;

        self.stream.map_key()?;

        Ok(self)
    }

    /**
    Begin a map value.
    */
    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Stream<'a>, Error> {
        self.stack.map_value()?;

        self.stream.map_value()?;

        Ok(self)
    }

    /**
    Begin a sequence element.
    */
    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Stream<'a>, Error> {
        self.stack.seq_elem()?;

        self.stream.seq_elem()?;

        Ok(self)
    }
}

struct DebugStack<'a> {
    #[cfg(any(debug_assertions, test))]
    stack: &'a mut stream::Stack,
    _m: PhantomData<&'a mut stream::Stack>,
}

impl<'a> DebugStack<'a> {
    #[inline]
    fn borrow_mut(&mut self) -> DebugStack {
        cfg_debug_stack! {
            if #[debug_stack] {
                DebugStack {
                    stack: self.stack,
                    _m: PhantomData,
                }
            }
            else {
                DebugStack {
                    _m: PhantomData,
                }
            }
        }
    }

    #[inline]
    pub fn begin(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.begin()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn primitive(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.primitive()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn map_begin(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.map_begin()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn map_key(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.map_key()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn map_value(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.map_value()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn map_end(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.map_end()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn seq_begin(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.seq_begin()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn seq_elem(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.seq_elem()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn seq_end(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.seq_end()?;
            }
        }

        Ok(())
    }

    #[inline]
    pub fn end(&mut self) -> Result<(), Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack.end()?;
            }
        }

        Ok(())
    }
}
