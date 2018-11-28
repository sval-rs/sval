/*!
Extensions for `stream::Stream` for collecting
keys, values, and sequences that are known upfront.

This is useful for `serde` integration where we can avoid
allocating for nested datastructures that are already known.
*/

use crate::{
    std::marker::PhantomData,
    stream::{
        self,
        Stream as StreamTrait,
    },
    value,
    Error,
};

pub(crate) trait Stream: stream::Stream {
    fn map_key_collect(&mut self, k: Value) -> Result<(), Error>;

    fn map_value_collect(&mut self, v: Value) -> Result<(), Error>;

    fn seq_elem_collect(&mut self, v: Value) -> Result<(), Error>;
}

impl<'a, S: ?Sized> Stream for &'a mut S
where
    S: Stream,
{
    fn map_key_collect(&mut self, k: Value) -> Result<(), Error> {
        (**self).map_key_collect(k)
    }

    fn map_value_collect(&mut self, v: Value) -> Result<(), Error> {
        (**self).map_value_collect(v)
    }

    fn seq_elem_collect(&mut self, v: Value) -> Result<(), Error> {
        (**self).seq_elem_collect(v)
    }
}

pub(crate) struct Value<'a> {
    stack: DebugStack<'a>,
    value: &'a dyn value::Value,
}

impl<'a> Value<'a> {
    #[inline]
    pub(super) fn new(stack: value::DebugStack<'a>, value: &'a impl value::Value) -> Self {
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
        let stack = self.stack.take()?;
        let mut stream = value::Stream {
            stack,
            stream: &mut stream,
        };

        stream.any(&self.value)?;

        Ok(())
    }
}

pub(crate) struct Default<S>(pub(crate) S);

impl<S> Stream for Default<S>
where
    S: stream::Stream,
{
    fn map_key_collect(&mut self, k: Value) -> Result<(), Error> {
        self.map_key()?;
        k.stream(self)?;

        Ok(())
    }

    fn map_value_collect(&mut self, v: Value) -> Result<(), Error> {
        self.map_value()?;
        v.stream(self)?;

        Ok(())
    }

    fn seq_elem_collect(&mut self, v: Value) -> Result<(), Error> {
        self.seq_elem()?;
        v.stream(self)?;

        Ok(())
    }
}

impl<S> stream::Stream for Default<S>
where
    S: stream::Stream,
{
    fn begin(&mut self) -> Result<(), Error> {
        self.0.begin()
    }

    fn fmt(&mut self, args: stream::Arguments) -> Result<(), Error> {
        self.0.fmt(args)
    }

    fn i64(&mut self, v: i64) -> Result<(), Error> {
        self.0.i64(v)
    }

    fn u64(&mut self, v: u64) -> Result<(), Error> {
        self.0.u64(v)
    }

    fn i128(&mut self, v: i128) -> Result<(), Error> {
        self.0.i128(v)
    }

    fn u128(&mut self, v: u128) -> Result<(), Error> {
        self.0.u128(v)
    }

    fn f64(&mut self, v: f64) -> Result<(), Error> {
        self.0.f64(v)
    }

    fn bool(&mut self, v: bool) -> Result<(), Error> {
        self.0.bool(v)
    }

    fn char(&mut self, v: char) -> Result<(), Error> {
        self.0.char(v)
    }

    fn str(&mut self, v: &str) -> Result<(), Error> {
        self.0.str(v)
    }

    fn none(&mut self) -> Result<(), Error> {
        self.0.none()
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        self.0.map_begin(len)
    }

    fn map_key(&mut self) -> Result<(), Error> {
        self.0.map_key()
    }

    fn map_value(&mut self) -> Result<(), Error> {
        self.0.map_value()
    }

    fn map_end(&mut self) -> Result<(), Error> {
        self.0.map_end()
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        self.0.seq_begin(len)
    }

    fn seq_elem(&mut self) -> Result<(), Error> {
        self.0.seq_elem()
    }

    fn seq_end(&mut self) -> Result<(), Error> {
        self.0.seq_end()
    }

    fn end(&mut self) -> Result<(), Error> {
        self.0.end()
    }
}

struct DebugStack<'a> {
    #[cfg(any(debug_assertions, test))]
    stack: crate::std::cell::Cell<Option<value::DebugStack<'a>>>,
    _m: PhantomData<&'a mut stream::Stack>,
}

impl<'a> DebugStack<'a> {
    fn new(stack: value::DebugStack<'a>) -> Self {
        cfg_debug_stack! {
            if #[debug_stack] {
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

    fn take(&self) -> Result<value::DebugStack<'a>, Error> {
        cfg_debug_stack! {
            if #[debug_stack] {
                self.stack
                    .take()
                    .ok_or_else(|| Error::msg("attempt to re-use value"))
            }
            else {
                Ok(value::DebugStack {
                    _m: PhantomData,
                })
            }
        }
    }
}
