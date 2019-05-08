use crate::{
    collect::{
        Collect,
        Error,
    },
    stream::stack::{
        DebugRefMut,
        DebugStack,
    },
    value,
};

/**
A value that's known upfront.
*/
pub(crate) struct Value<'a> {
    stack: DebugRefMut<'a, DebugStack>,
    value: &'a dyn value::Value,
}

impl<'a> Value<'a> {
    #[inline]
    pub(crate) fn new(value: &'a impl value::Value, stack: DebugRefMut<'a, DebugStack>) -> Self {
        Value { stack, value }
    }

    /**
    Stream this value.
    */
    #[inline]
    pub(crate) fn stream(self, mut stream: impl Collect) -> Result<(), Error> {
        let mut stream = value::Stream::new(&mut stream, self.stack);

        stream.any(self.value)?;

        Ok(())
    }
}
