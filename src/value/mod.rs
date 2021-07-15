mod impls;
mod stream;

#[cfg(feature = "alloc")]
pub(crate) mod owned;

pub use self::stream::Stream;

#[cfg(feature = "alloc")]
pub use self::owned::OwnedValue;

pub trait Value {
    fn stream<'s, 'v>(&'v self, stream: Stream<'s, 'v>) -> Result;

    fn stream_owned(&self, mut stream: Stream) -> Result {
        self.stream(stream.owned())
    }
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn stream<'s, 'v>(&'v self, stream: Stream<'s, 'v>) -> Result {
        (**self).stream(stream)
    }

    fn stream_owned(&self, stream: Stream) -> Result {
        (**self).stream_owned(stream)
    }
}

pub type Result<T = ()> = crate::std::result::Result<T, crate::Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_is_object_safe() {
        fn _safe(_: &dyn Value) {}
    }
}
