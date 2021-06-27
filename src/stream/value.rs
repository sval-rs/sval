use crate::{
    stream::Stream,
    value,
};

/**
A value that can emit its structure to a stream.
*/
pub struct Value<'v>(&'v dyn value::Value);

impl<'v> Value<'v> {
    /**
    Wrap an implementation of [`Value`].

    [`Value`]: ../value/trait.Value.html
    */
    #[inline]
    pub fn new(value: &'v impl value::Value) -> Self {
        Value(value)
    }

    /**
    Stream this value using an implementation of [`Stream`].

    [`Stream`]: ./trait.Stream.html
    */
    #[inline]
    pub fn stream<'a>(&self, mut stream: impl Stream<'a>) -> value::Result {
        self.0.stream(&mut value::Stream::new(&mut stream))?;

        Ok(())
    }

    /**
    Stream this value using an implementation of [`Stream`].

    [`Stream`]: ./trait.Stream.html
    */
    #[inline]
    pub fn borrowed_stream(&self, mut stream: impl Stream<'v>) -> value::Result {
        self.0.stream(&mut value::Stream::new(&mut stream))?;

        Ok(())
    }
}

impl<'a> value::Value for Value<'a> {
    #[inline]
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        self.0.stream(stream)
    }

    #[inline]
    fn borrowed_stream<'s, 'v>(&'v self, stream: &mut value::Stream<'s, 'v>) -> value::Result {
        self.0.borrowed_stream(stream)
    }
}
