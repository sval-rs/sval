use crate::{
    stream::Stream,
    value,
};

/**
A value that can emit its structure to a stream.
*/
pub struct Value<'a>(&'a dyn value::Value);

impl<'a> Value<'a> {
    #[inline]
    pub fn new(value: &'a impl value::Value) -> Self {
        Value(value)
    }

    #[inline]
    pub fn stream(&self, mut stream: impl Stream) -> value::Result {
        self.0.stream(&mut value::Stream::new(&mut stream))?;

        Ok(())
    }
}

impl<'a> value::Value for Value<'a> {
    #[inline]
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        self.0.stream(stream)
    }
}
