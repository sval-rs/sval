/*!
A streamable value.

# The `Value` trait

A [`Value`] is a type that has structure, like a number, string, map, or sequence.

## Deriving `Value`

Use the `derive` Cargo feature to support automatic implementations of the `Value` trait:

```toml,ignore
[dependencies.sval]
features = ["derive"]
```

Then derive the `Value` for struct-like datastructures:

```
# fn main() {}
# #[cfg(all(feature = "std", feature = "derive"))]
# mod test {
use sval::Value;

#[derive(Value)]
pub struct Data {
    id: u32,
    title: String,
}
# }
```
*/

mod impls;
mod stream;

#[cfg(feature = "alloc")]
pub(crate) mod owned;

pub use self::stream::Stream;

#[cfg(feature = "alloc")]
pub use self::owned::OwnedValue;

/**
A value with a streamable structure.
*/
pub trait Value {
    /**
    Stream this value.

    Data passed to the stream may satisfy the `'v` lifetime so that
    the stream can hold onto it across calls.
    */
    fn stream<'s, 'v>(&'v self, stream: &mut Stream<'s, 'v>) -> Result;

    /**
    Stream this value.

    Data passed to the stream may have an arbitrarily short lifetime.
    That means the stream can't borrow that data across calls, but the
    value is free to produce any short-lived values it needs.
    */
    fn stream_owned(&self, stream: &mut Stream) -> Result {
        self.stream(&mut stream.owned())
    }
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn stream<'s, 'v>(&'v self, stream: &mut Stream<'s, 'v>) -> Result {
        (**self).stream(stream)
    }

    fn stream_owned(&self, stream: &mut Stream) -> Result {
        (**self).stream_owned(stream)
    }
}

/**
The type returned by streaming methods.
*/
pub type Result<T = ()> = crate::std::result::Result<T, crate::Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_is_object_safe() {
        fn _safe(_: &dyn Value) {}
    }
}
