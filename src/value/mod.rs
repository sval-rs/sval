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
# #[cfg(feature = "derive")]
# mod test {
use sval::Value;

#[derive(Value)]
pub struct Data {
    id: u32,
    title: String,
}
# }
```

The trait can also be implemented manually:

```
use sval::value::{self, Value};

pub struct Id(u64);

impl Value for Id {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.u64(self.0)
    }
}
```

## Sequences

A sequence can be visited by iterating over its elements:

```
use sval::value::{self, Value};

pub struct Seq(Vec<u64>);

impl Value for Seq {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.seq_begin(Some(self.0.len()))?;

        for v in &self.0 {
            stream.seq_elem(v)?;
        }

        stream.seq_end()
    }
}
```

## Maps

A map can be visited by iterating over its key-value pairs:

```
# fn main() {}
# #[cfg(feature = "std")]
# mod test {
use std::collections::BTreeMap;
use sval::value::{self, Value};

pub struct Map(BTreeMap<String, u64>);

impl Value for Map {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(Some(self.0.len()))?;

        for (k, v) in &self.0 {
            stream.map_key(k)?;
            stream.map_value(v)?;
        }

        stream.map_end()
    }
}
# }
```

## Structure that isn't known upfront

Types can stream a structure that's different than what they use internally.
In the following example, the `Map` type doesn't have any keys or values,
but serializes a nested map like `{"nested": {"key": 42}}`:

```
use sval::value::{self, Value};

pub struct Map;

impl Value for Map {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(Some(1))?;

        stream.map_key_begin()?.str("nested")?;
        stream.map_value_begin()?.map_begin(Some(1))?;
        stream.map_key_begin()?.str("key")?;
        stream.map_value_begin()?.u64(42)?;
        stream.map_end()?;

        stream.map_end()
    }
}
```
*/

mod impls;

#[cfg(feature = "alloc")]
pub(crate) mod owned;

pub use crate::stream::RefMutStream as Stream;

#[cfg(feature = "alloc")]
pub use self::owned::OwnedValue;

#[doc(inline)]
pub use crate::Error;

/**
A value with a streamable structure.

# Implementing `Value`

Implementations of `Value` are expected to conform to the following
model:

## Only a single primitive, map or sequence is streamed

The following `Value` is valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        // VALID: The stream can take the primitive
        // value 42
        stream.any(42)
    }
}
```

The following `Value` is not valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.any(42)?;

        // INVALID: The stream already received the
        // primitive value 42
        stream.any(43)
    }
}
```

## All maps and sequences are completed, and in the right order

The following `Value` is valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;
        stream.map_key("a")?;
        stream.map_value_begin()?.seq_begin(None)?;

        // VALID: The sequence is completed, then the map is completed
        stream.seq_end()?;
        stream.map_end()
    }
}
```

The following `Value` is not valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;
        stream.map_key("a")?;
        stream.map_value_begin()?.seq_begin(None)?;

        // INVALID: The map is completed before the sequence,
        // even though the sequence was started last.
        stream.map_end()?;
        stream.seq_end()
    }
}
```

The following `Value` is not valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        // INVALID: The map is never completed
        Ok(())
    }
}
```

## Map keys and values are received before their corresponding structure

The following `Value` is valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        // VALID: The `map_key` and `map_value` methods
        // always call the underlying stream correctly
        stream.map_key("a")?;
        stream.map_value("b")?;

        // VALID: `map_key` and `map_value` are called before
        // their actual values are given
        stream.map_key_begin()?.any("c")?;
        stream.map_value_begin()?.any("d")?;

        stream.map_end()
    }
}
```

The following `Value` is not valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        // INVALID: The underlying `map_key` and `map_value` methods
        // aren't being called before their actual values are given
        stream.any("a")?;
        stream.any("b")?;

        stream.map_end()
    }
}
```

## Map keys are received before values

The following `Value` is valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        // VALID: The key is streamed before the value
        stream.map_key("a")?;
        stream.map_value("b")?;

        stream.map_end()
    }
}
```

The following `Value` is not valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.map_begin(None)?;

        // INVALID: The value is streamed before the key
        stream.map_value("b")?;
        stream.map_key("a")?;

        stream.map_end()
    }
}
```

## Sequence elements are received before their corresponding structure

The following `Value` is valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.seq_begin(None)?;

        // VALID: The `seq_elem` method
        // always calls the underlying stream correctly
        stream.seq_elem("a")?;

        // VALID: `seq_elem` is called before
        // their actual values are given
        stream.seq_elem_begin()?.any("b")?;

        stream.seq_end()
    }
}
```

The following `Value` is not valid:

```
# use sval::value::{self, Value};
# struct MyValue;
impl Value for MyValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.seq_begin(None)?;

        // INVALID: The underlying `seq_elem` method
        // isn't being called before the actual value is given
        stream.any("a")?;

        stream.seq_end()
    }
}
```
*/
pub trait Value {
    /**
    Stream this value.

    # Examples

    Use a [`stream::OwnedStream`] to stream a value:

    ```no_run
    # #[cfg(not(feature = "std"))]
    # fn main() {}
    # #[cfg(feature = "std")]
    # fn main() -> Result<(), Box<dyn std::error::Error>> {
    use sval::stream::OwnedStream;

    let mut stream = OwnedStream::new(MyStream);
    stream.any(42)?;
    # Ok(())
    # }
    # use sval::stream::{self, Stream};
    # struct MyStream;
    # impl Stream for MyStream {
    #     fn fmt(&mut self, _: stream::Arguments) -> stream::Result { unimplemented!() }
    # }
    ```

    It's less convenient, but the `stream` method can be called directly
    instead of using `OwnedStream.any`:

    ```no_run
    # #[cfg(not(feature = "std"))]
    # fn main() {}
    # #[cfg(feature = "std")]
    # fn main() -> Result<(), Box<dyn std::error::Error>> {
    use sval::{
        stream::OwnedStream,
        Value,
    };

    let mut stream = OwnedStream::new(MyStream);
    42.stream(&mut stream.borrow_mut())?;
    # Ok(())
    # }
    # use sval::stream::{self, Stream};
    # struct MyStream;
    # impl Stream for MyStream {
    #     fn fmt(&mut self, _: stream::Arguments) -> stream::Result { unimplemented!() }
    # }
    ```

    [`sval::stream`]: ../fn.stream.html
    [`stream::OwnedStream`]: ../stream/struct.OwnedStream.html
    */
    fn stream(&self, stream: &mut Stream) -> Result;
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    #[inline]
    fn stream(&self, stream: &mut Stream) -> Result {
        (**self).stream(stream)
    }
}

/**
The type returned by streaming methods.
*/
pub type Result = crate::std::result::Result<(), Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_is_object_safe() {
        fn _safe(_: &dyn Value) {}
    }
}
