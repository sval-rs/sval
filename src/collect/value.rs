use crate::{
    collect::{
        self,
        stack::{
            DebugRefMut,
            DebugStack,
        },
        Collect,
        OwnedCollect,
    },
    value,
};

pub(crate) struct Value<'a> {
    stack: DebugRefMut<'a, DebugStack>,
    value: &'a dyn value::Value,
}

impl<'a> Value<'a> {
    #[inline]
    pub(crate) fn new(value: &'a impl value::Value, stack: DebugRefMut<'a, DebugStack>) -> Self {
        Value { stack, value }
    }

    #[inline]
    pub(crate) fn stream(self, stream: impl Collect) -> collect::Result {
        let mut stream = OwnedCollect::new(stream, self.stack);

        self.value
            .stream(&mut value::Stream::new(stream.borrow_mut()))?;

        Ok(())
    }
}
