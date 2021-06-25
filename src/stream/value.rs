use crate::{
    stream::Stream,
    value,
};

/**
A value that can emit its structure to a stream.
*/
pub struct Value<'a>(&'a dyn value::Value);

impl<'a> Value<'a> {
    /**
    Wrap an implementation of [`Value`].

    [`Value`]: ../value/trait.Value.html
    */
    #[inline]
    pub fn new(value: &'a impl value::Value) -> Self {
        Value(value)
    }

    /**
    Stream this value using an implementation of [`Stream`].

    [`Stream`]: ./trait.Stream.html
    */
    #[inline]
    pub fn stream<'stream>(&self, mut stream: impl Stream<'stream>) -> value::Result {
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
