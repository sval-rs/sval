use crate::{
    collect::{
        self,
        stack::{
            DebugBorrowMut,
            DebugRefMut,
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
    pub(crate) fn into_inner(self) -> TStream {
        self.stream
    }

    #[inline]
    pub(crate) fn borrow_mut(&mut self) -> RefMutCollect {
        RefMutCollect(OwnedCollect::new(&mut self.stream, self.stack.borrow_mut()))
    }

    #[inline]
    pub fn any(&mut self, v: impl value::Value) -> collect::Result {
        v.stream(&mut value::Stream::new(self.borrow_mut()))
    }

    #[inline]
    pub fn fmt(&mut self, f: Arguments) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.fmt(f)?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn i64(&mut self, v: i64) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.i64(v)?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn u64(&mut self, v: u64) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.u64(v)?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn i128(&mut self, v: i128) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.i128(v)?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn u128(&mut self, v: u128) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.u128(v)?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn f64(&mut self, v: f64) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.f64(v)?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn bool(&mut self, v: bool) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.bool(v)?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn char(&mut self, v: char) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.char(v)?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn str(&mut self, v: &str) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.str(v)?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn none(&mut self) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.none()?;
            stack.primitive();

            Ok(())
        })
    }

    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.map_begin(len)?;
            stack.map_begin();

            Ok(())
        })
    }

    #[inline]
    pub fn map_key(&mut self, k: impl value::Value) -> collect::Result {
        self.map_key_begin()?;

        let stream = &mut self.stream;
        stream.map_key_collect(Value::new(&k, self.stack.borrow_mut()))
    }

    #[inline]
    pub fn map_value(&mut self, v: impl value::Value) -> collect::Result {
        self.map_value_begin()?;

        let stream = &mut self.stream;
        stream.map_value_collect(Value::new(&v, self.stack.borrow_mut()))
    }

    #[inline]
    pub fn map_end(&mut self) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.map_end()?;
            stack.map_end();

            Ok(())
        })
    }

    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.seq_begin(len)?;
            stack.seq_begin();

            Ok(())
        })
    }

    #[inline]
    pub fn seq_elem(&mut self, v: impl value::Value) -> collect::Result {
        self.seq_elem_begin()?;
        
        let stream = &mut self.stream;
        stream.seq_elem_collect(Value::new(&v, self.stack.borrow_mut()))
    }

    #[inline]
    pub fn seq_end(&mut self) -> collect::Result {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.seq_end()?;
            stack.seq_end();

            Ok(())
        })
    }

    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Self, Error> {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.map_key()?;
            stack.map_key();

            Ok(())
        })?;

        Ok(self)
    }

    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Self, Error> {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.map_value()?;
            stack.map_value();

            Ok(())
        })?;

        Ok(self)
    }

    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, Error> {
        let stream = &mut self.stream;
        self.stack.borrow_mut().and_then(|mut stack| {
            stream.seq_elem()?;
            stack.seq_elem();

            Ok(())
        })?;

        Ok(self)
    }
}

pub(crate) struct RefMutCollect<'a>(OwnedCollect<&'a mut dyn Collect, DebugRefMut<'a, DebugStack>>);

impl<'a> RefMutCollect<'a> {
    #[inline]
    pub fn fmt(&mut self, f: Arguments) -> value::Result {
        self.0.fmt(f)
    }

    #[inline]
    pub fn any(&mut self, v: impl value::Value) -> collect::Result {
        self.0.any(v)
    }

    #[inline]
    pub fn i64(&mut self, v: i64) -> value::Result {
        self.0.i64(v)
    }

    #[inline]
    pub fn u64(&mut self, v: u64) -> value::Result {
        self.0.u64(v)
    }

    #[inline]
    pub fn i128(&mut self, v: i128) -> value::Result {
        self.0.i128(v)
    }

    #[inline]
    pub fn u128(&mut self, v: u128) -> value::Result {
        self.0.u128(v)
    }

    #[inline]
    pub fn f64(&mut self, v: f64) -> value::Result {
        self.0.f64(v)
    }

    #[inline]
    pub fn bool(&mut self, v: bool) -> value::Result {
        self.0.bool(v)
    }

    #[inline]
    pub fn char(&mut self, v: char) -> value::Result {
        self.0.char(v)
    }

    #[inline]
    pub fn str(&mut self, v: &str) -> value::Result {
        self.0.str(v)
    }

    #[inline]
    pub fn none(&mut self) -> value::Result {
        self.0.none()
    }

    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> value::Result {
        self.0.map_begin(len)
    }

    #[inline]
    pub fn map_key(&mut self, k: impl value::Value) -> value::Result {
        self.0.map_key(k)
    }

    #[inline]
    pub fn map_value(&mut self, v: impl value::Value) -> value::Result {
        self.0.map_value(v)
    }

    #[inline]
    pub fn map_end(&mut self) -> value::Result {
        self.0.map_end()
    }

    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> value::Result {
        self.0.seq_begin(len)
    }

    #[inline]
    pub fn seq_elem(&mut self, v: impl value::Value) -> value::Result {
        self.0.seq_elem(v)
    }

    #[inline]
    pub fn seq_end(&mut self) -> value::Result {
        self.0.seq_end()
    }
}

impl<'a> RefMutCollect<'a> {
    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Self, Error> {
        self.0.map_key_begin()?;

        Ok(self)
    }

    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Self, Error> {
        self.0.map_value_begin()?;

        Ok(self)
    }

    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, Error> {
        self.0.seq_elem_begin()?;

        Ok(self)
    }
}
