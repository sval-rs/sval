/*!
A streamable value.
*/

mod impls;
mod stream;

#[cfg(feature = "std")]
pub(crate) mod owned;

pub use self::stream::Stream;

#[cfg(feature = "std")]
pub use self::owned::OwnedValue;

#[doc(inline)]
pub use crate::Error;

/**
A value with a streamable structure.

Use the [`sval::stream`](../fn.stream.html) function to stream a value.

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
    /** Stream this value. */
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
pub type Result = std::result::Result<(), Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_is_object_safe() {
        fn _safe(_: &dyn Value) {}
    }
}
