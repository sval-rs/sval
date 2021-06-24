use crate::{
    value,
    stream::Stream,
};

/**
A value that can emit its structure to a stream.
*/
pub struct Value<'a> {
    value: &'a dyn value::Value,
}

impl<'a> Value<'a> {
    #[inline]
    pub fn new(value: &'a impl value::Value) -> Self {
        Value { value }
    }

    #[inline]
    pub fn stream(&self, mut stream: impl Stream) -> value::Result {
        self.value
            .stream(&mut value::Stream::new(&mut stream))?;

        Ok(())
    }
}
