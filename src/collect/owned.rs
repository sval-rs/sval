use crate::{
    collect::{
        value::Value,
        Collect,
        Error,
    },
    std::borrow::BorrowMut,
    stream::{
        stack::DebugStack,
        Arguments,
    },
    value,
};

pub(crate) struct OwnedCollect<TStream, TStack = DebugStack> {
    stack: TStack,
    stream: TStream,
}

impl<TStream, TStack> OwnedCollect<TStream, TStack>
where
    TStream: Collect,
    TStack: BorrowMut<DebugStack>,
{
    #[inline]
    pub(crate) fn new(stream: TStream, stack: TStack) -> Self {
        OwnedCollect { stack, stream }
    }

    #[inline]
    pub(crate) fn split(self) -> (TStream, TStack) {
        (self.stream, self.stack)
    }

    #[inline]
    pub fn any(&mut self, v: impl value::Value) -> Result<(), Error> {
        v.stream(&mut value::Stream::new(
            &mut self.stream, // TODO: This is the problem!
            self.stack.borrow_mut(),
        ))
    }

    #[inline]
    pub fn fmt(&mut self, f: Arguments) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.fmt(f)?;

        Ok(())
    }

    #[inline]
    pub fn i64(&mut self, v: i64) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.i64(v)?;

        Ok(())
    }

    #[inline]
    pub fn u64(&mut self, v: u64) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.u64(v)?;

        Ok(())
    }

    #[inline]
    pub fn i128(&mut self, v: i128) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.i128(v)?;

        Ok(())
    }

    #[inline]
    pub fn u128(&mut self, v: u128) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.u128(v)?;

        Ok(())
    }

    #[inline]
    pub fn f64(&mut self, v: f64) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.f64(v)?;

        Ok(())
    }

    #[inline]
    pub fn bool(&mut self, v: bool) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.bool(v)?;

        Ok(())
    }

    #[inline]
    pub fn char(&mut self, v: char) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.char(v)?;

        Ok(())
    }

    #[inline]
    pub fn str(&mut self, v: &str) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.str(v)?;

        Ok(())
    }

    #[inline]
    pub fn none(&mut self) -> Result<(), Error> {
        self.stack.borrow_mut().primitive()?;

        self.stream.none()?;

        Ok(())
    }

    #[inline]
    pub fn map_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        self.stack.borrow_mut().map_begin()?;

        self.stream.map_begin(len)?;

        Ok(())
    }

    #[inline]
    pub fn map_key(&mut self, k: impl value::Value) -> Result<(), Error> {
        self.stack.borrow_mut().map_key()?;

        self.stream
            .map_key_collect(Value::new(self.stack.borrow_mut(), &k))?;

        Ok(())
    }
    
    #[inline]
    pub fn map_value(&mut self, v: impl value::Value) -> Result<(), Error> {
        self.stack.borrow_mut().map_value()?;

        self.stream
            .map_value_collect(Value::new(self.stack.borrow_mut(), &v))?;

        Ok(())
    }

    #[inline]
    pub fn map_end(&mut self) -> Result<(), Error> {
        self.stack.borrow_mut().map_end()?;

        self.stream.map_end()?;

        Ok(())
    }

    #[inline]
    pub fn seq_begin(&mut self, len: Option<usize>) -> Result<(), Error> {
        self.stack.borrow_mut().seq_begin()?;

        self.stream.seq_begin(len)?;

        Ok(())
    }

    #[inline]
    pub fn seq_elem(&mut self, v: impl value::Value) -> Result<(), Error> {
        self.stack.borrow_mut().seq_elem()?;

        self.stream
            .seq_elem_collect(Value::new(self.stack.borrow_mut(), &v))?;

        Ok(())
    }

    #[inline]
    pub fn seq_end(&mut self) -> Result<(), Error> {
        self.stack.borrow_mut().seq_end()?;

        self.stream.seq_end()?;

        Ok(())
    }

    #[inline]
    pub fn map_key_begin(&mut self) -> Result<&mut Self, Error> {
        self.stack.borrow_mut().map_key()?;

        self.stream.map_key()?;

        Ok(self)
    }

    #[inline]
    pub fn map_value_begin(&mut self) -> Result<&mut Self, Error> {
        self.stack.borrow_mut().map_value()?;

        self.stream.map_value()?;

        Ok(self)
    }

    #[inline]
    pub fn seq_elem_begin(&mut self) -> Result<&mut Self, Error> {
        self.stack.borrow_mut().seq_elem()?;

        self.stream.seq_elem()?;

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
        
        stack.begin()?;
        stream.begin()?;

        Ok(OwnedCollect {
            stack,
            stream,
        })
    }

    #[inline]
    pub(crate) fn end(mut self) -> Result<TStream, Error> {
        self.stack.end()?;
        self.stream.end()?;

        Ok(self.stream)
    }
}
