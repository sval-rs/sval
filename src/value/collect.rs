/*!
Extensions for `stream::Stream` for collecting
keys, values, and sequences that are known upfront.

This is useful for `serde` integration where we can avoid
allocating for nested datastructures that are already known.
*/

use crate::{
    std::marker::PhantomData,
    stream,
    value::{
        self,
        Error,
    },
};

// FIXME: Moving the `*_collect` methods onto the base `Stream`
// trait is a little more efficient (a few % improvement against `serde`)
// in the general case because it can save a virtual call per key/value/elem.
// The reason this hasn't been done already is just to reduce
// the API surface area for now. It should be revisited sometime.

/**
An extension to `stream::Stream` for items that are known upfront.
*/
pub(crate) trait Stream: stream::Stream {
    fn map_key_collect(&mut self, k: Value) -> Result<(), stream::Error>;

    fn map_value_collect(&mut self, v: Value) -> Result<(), stream::Error>;

    fn seq_elem_collect(&mut self, v: Value) -> Result<(), stream::Error>;
}

impl<'a, S: ?Sized> Stream for &'a mut S
where
    S: Stream,
{
    #[inline]
    fn map_key_collect(&mut self, k: Value) -> Result<(), stream::Error> {
        (**self).map_key_collect(k)
    }

    #[inline]
    fn map_value_collect(&mut self, v: Value) -> Result<(), stream::Error> {
        (**self).map_value_collect(v)
    }

    #[inline]
    fn seq_elem_collect(&mut self, v: Value) -> Result<(), stream::Error> {
        (**self).seq_elem_collect(v)
    }
}

/**
A value that's known upfront.
*/
pub(crate) struct Value<'a> {
    stack: DebugStack<'a>,
    value: &'a dyn value::Value,
}

impl<'a> Value<'a> {
    #[inline]
    pub(super) fn new(stack: value::stream::DebugStack<'a>, value: &'a impl value::Value) -> Self {
        Value {
            stack: DebugStack::new(stack),
            value,
        }
    }

    /**
    Stream this value.

    The value may only be streamed once.
    Subsequent calls to `stream` may fail.
    */
    #[inline]
    pub(crate) fn stream(&self, mut stream: impl Stream) -> Result<(), Error> {
        let mut stream = value::Stream::new(self.stack.take()?, &mut stream);

        stream.any(&self.value)?;

        Ok(())
    }
}

/**
Default implementations for stream extensions.
*/
pub(crate) struct Default<S>(pub(crate) S);

impl<S> Stream for Default<S>
where
    S: stream::Stream,
{
    #[inline]
    fn map_key_collect(&mut self, k: Value) -> Result<(), stream::Error> {
        stream::Stream::map_key(self)?;
        k.stream(self)?;

        Ok(())
    }

    #[inline]
    fn map_value_collect(&mut self, v: Value) -> Result<(), stream::Error> {
        stream::Stream::map_value(self)?;
        v.stream(self)?;

        Ok(())
    }

    #[inline]
    fn seq_elem_collect(&mut self, v: Value) -> Result<(), stream::Error> {
        stream::Stream::seq_elem(self)?;
        v.stream(self)?;

        Ok(())
    }
}

impl<S> stream::Stream for Default<S>
where
    S: stream::Stream,
{
    #[inline]
    fn begin(&mut self) -> Result<(), stream::Error> {
        self.0.begin()
    }

    #[inline]
    fn fmt(&mut self, args: stream::Arguments) -> Result<(), stream::Error> {
        self.0.fmt(args)
    }

    #[inline]
    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        self.0.i64(v)
    }

    #[inline]
    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        self.0.u64(v)
    }

    #[inline]
    fn i128(&mut self, v: i128) -> Result<(), stream::Error> {
        self.0.i128(v)
    }

    #[inline]
    fn u128(&mut self, v: u128) -> Result<(), stream::Error> {
        self.0.u128(v)
    }

    #[inline]
    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        self.0.f64(v)
    }

    #[inline]
    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        self.0.bool(v)
    }

    #[inline]
    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        self.0.char(v)
    }

    #[inline]
    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        self.0.str(v)
    }

    #[inline]
    fn none(&mut self) -> Result<(), stream::Error> {
        self.0.none()
    }

    #[inline]
    fn map_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        self.0.map_begin(len)
    }

    #[inline]
    fn map_key(&mut self) -> Result<(), stream::Error> {
        self.0.map_key()
    }

    #[inline]
    fn map_value(&mut self) -> Result<(), stream::Error> {
        self.0.map_value()
    }

    #[inline]
    fn map_end(&mut self) -> Result<(), stream::Error> {
        self.0.map_end()
    }

    #[inline]
    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        self.0.seq_begin(len)
    }

    #[inline]
    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        self.0.seq_elem()
    }

    #[inline]
    fn seq_end(&mut self) -> Result<(), stream::Error> {
        self.0.seq_end()
    }

    #[inline]
    fn end(&mut self) -> Result<(), stream::Error> {
        self.0.end()
    }
}

struct DebugStack<'a> {
    #[cfg(debug_assertions)]
    stack: crate::std::cell::Cell<Option<value::stream::DebugStack<'a>>>,
    _m: PhantomData<&'a mut stream::Stack>,
}

impl<'a> DebugStack<'a> {
    #[inline]
    fn new(stack: value::stream::DebugStack<'a>) -> Self {
        cfg_debug_stack! {
            if #[debug_assertions] {
                DebugStack {
                    stack: crate::std::cell::Cell::new(Some(stack)),
                    _m: PhantomData,
                }
            }
            else {
                let _ = stack;

                DebugStack {
                    _m: PhantomData,
                }
            }
        }
    }

    #[inline]
    fn take(&self) -> Result<value::stream::DebugStack<'a>, Error> {
        cfg_debug_stack! {
            if #[debug_assertions] {
                self.stack
                    .take()
                    .ok_or_else(|| Error::msg("attempt to re-use value"))
            }
            else {
                Ok(value::stream::DebugStack {
                    _m: PhantomData,
                })
            }
        }
    }
}

#[cfg(all(test, not(debug_assertions)))]
mod tests {
    use super::*;

    use crate::std::mem;

    #[test]
    fn debug_stack_is_zero_sized() {
        assert_eq!(0, mem::size_of::<DebugStack>());
    }
}
