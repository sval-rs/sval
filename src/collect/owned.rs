use crate::{
    collect::{
        self,
        stack::{
            DebugBorrowMut,
            DebugStack,
        },
        value::Value,
        Collect,
        Error,
    },
    stream::Arguments,
    value,
};

pub(crate) struct OwnedCollect<TStream, TStack = DebugStack> {
    stack: TStack,
    stream: TStream,
}

impl<TStream, TStack> OwnedCollect<TStream, TStack>
where
    TStream: Collect,
    TStack: DebugBorrowMut<DebugStack>,
{
    #[inline]
    pub(crate) fn new(stream: TStream, stack: TStack) -> Self {
        OwnedCollect { stack, stream }
    }

    #[inline]
    pub fn any(&mut self, v: impl value::Value) -> collect::Result {
        let mut stream = value::Stream::new(&mut self.stream, self.stack.borrow_mut());

        stream.any(v)
    }

    #[inline]
    pub fn fmt(&mut self, f: Arguments) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.fmt(f)?;

            Ok(())
        })
    }

    #[inline]
    pub fn i64(&mut self, v: i64) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.i64(v)
        })
    }

    #[inline]
    pub fn u64(&mut self, v: u64) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.u64(v)?;

            Ok(())
        })
    }

    #[inline]
    pub fn i128(&mut self, v: i128) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.i128(v)
        })
    }

    #[inline]
    pub fn u128(&mut self, v: u128) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.u128(v)
        })
    }

    #[inline]
    pub fn f64(&mut self, v: f64) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.f64(v)
        })
    }

    #[inline]
    pub fn bool(&mut self, v: bool) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.bool(v)
        })
    }

    #[inline]
    pub fn char(&mut self, v: char) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.char(v)
        })
    }

    #[inline]
    pub fn str(&mut self, v: &str) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.str(v)
        })
    }

    #[inline]
    pub fn none(&mut self) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.primitive()?;
            stream.none()
        })
    }

    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.map_begin()?;
            stream.map_begin(len)
        })
    }

    #[inline]
    pub fn map_key(&mut self, k: impl value::Value) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.map_key()?;
            Ok(())
        })?;

        stream.map_key_collect(Value::new(&k, self.stack.borrow_mut()))
    }

    #[inline]
    pub fn map_value(&mut self, v: impl value::Value) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.map_value()?;
            Ok(())
        })?;

        stream.map_value_collect(Value::new(&v, self.stack.borrow_mut()))
    }

    #[inline]
    pub fn map_end(&mut self) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.map_end()?;
            stream.map_end()
        })
    }

    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.seq_begin()?;
            stream.seq_begin(len)
        })
    }

    #[inline]
    pub fn seq_elem(&mut self, v: impl value::Value) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.seq_elem()?;
            Ok(())
        })?;

        stream.seq_elem_collect(Value::new(&v, self.stack.borrow_mut()))
    }

    #[inline]
    pub fn seq_end(&mut self) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.seq_end()?;
            stream.seq_end()
        })
    }

    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Self, Error> {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.map_key()?;
            stream.map_key()
        })?;

        Ok(self)
    }

    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Self, Error> {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.map_value()?;
            stream.map_value()
        })?;

        Ok(self)
    }

    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, Error> {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.seq_elem()?;
            stream.seq_elem()
        })?;

        Ok(self)
    }
}

impl<TStream> OwnedCollect<TStream>
where
    TStream: Collect,
{
    #[inline]
    pub(crate) fn begin(mut stream: TStream) -> Result<Self, Error> {
        let mut stack = DebugStack::default();

        stack.borrow_mut().and_then(|mut stack| {
            stack.begin()?;
            stream.begin()?;

            Ok(())
        })?;

        Ok(OwnedCollect { stack, stream })
    }

    #[inline]
    pub(crate) fn end(mut self) -> Result<TStream, Error> {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stack.end()?;
            stream.end()?;

            Ok(())
        })?;

        Ok(self.stream)
    }
}
