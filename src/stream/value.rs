use crate::{
    stream::Stream,
    value,
};

/**
A value that can emit its structure to a stream.
*/
pub struct Value<'v>(&'v dyn value::Value);

impl<'v> From<&'v dyn value::Value> for Value<'v> {
    fn from(value: &'v dyn value::Value) -> Self {
        Value(value)
    }
}

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
    pub fn stream(&self, mut stream: &mut (impl Stream<'v> + ?Sized)) -> value::Result {
        self.0.stream(&mut value::Stream::new(&mut stream))?;

        Ok(())
    }

    /**
    Stream this value using an implementation of [`Stream`].

    [`Stream`]: ./trait.Stream.html
    */
    #[inline]
    pub fn stream_owned<'a>(&self, mut stream: &mut (impl Stream<'a> + ?Sized)) -> value::Result {
        self.0.stream_owned(&mut value::Stream::new(&mut stream))?;

        Ok(())
    }
}

impl<'a> value::Value for Value<'a> {
    #[inline]
    fn stream<'s, 'v>(&'v self, stream: &mut value::Stream<'s, 'v>) -> value::Result {
        self.0.stream(stream)
    }
}
