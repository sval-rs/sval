use crate::{
    collect::{
        self,
        Collect,
        OwnedCollect,
    },
    value,
};

pub(crate) struct Value<'a> {
    value: &'a dyn value::Value,
}

impl<'a> Value<'a> {
    #[inline]
    pub(crate) fn new(value: &'a impl value::Value) -> Self {
        Value { value }
    }

    #[inline]
    pub(crate) fn stream(self, stream: impl Collect) -> collect::Result {
        let mut stream = OwnedCollect::new(stream);

        self.value
            .stream(&mut value::Stream::new(stream.borrow_mut()))?;

        Ok(())
    }
}
