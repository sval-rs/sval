/*!
A small, no-std, object-safe, serialization-only framework.

The `sval` API is built around two key traits:

- [`Value`] is a trait for data with a streamable structure. It's like `serde::Serialize`.
- [`Stream`] is a trait for receiving the structure of a value. It's like `serde::Serializer`.

# Getting started

Add `sval` to your `Cargo.toml`:

```toml,ignore
[dependencies.sval]
version = "0.1.4"
```

# Streaming values

```no_run
# #[cfg(not(feature = "std"))]
# fn main() {}
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<std::error::Error>> {
sval::stream(42, MyStream)?;
# Ok(())
# }
# use sval::stream::{self, Stream};
# struct MyStream;
# impl Stream for MyStream {
#     fn fmt(&mut self, _: stream::Arguments) -> stream::Result { unimplemented!() }
# }
```

where `42` is a [`Value`] and `MyStream` is a [`Stream`].

# Implementing the `Value` trait

Use the `derive` Cargo feature to allow `Value` to be derived:

```toml,ignore
[dependencies.sval]
features = ["derive"]
```

Then derive the [`Value`] trait for simple datastructures:

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

## for a sequence

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

## for a map

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

## for values that aren't known upfront

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

# Implementing the `Stream` trait

## without state

Implement the [`Stream`] trait to visit the structure of a [`Value`]:

```
use sval::stream::{self, Stream};

struct Fmt;

impl Stream for Fmt {
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        println!("{}", v);

        Ok(())
    }
}
```

A `Stream` might only care about a single kind of value.
The following example overrides the provided `u64` method
to see whether a given value is a `u64`:

```
use sval::{
    Value,
    stream::{self, Stream}
};

assert!(is_u64(42u64));

pub fn is_u64(v: impl Value) -> bool {
    let mut stream = IsU64(None);

    sval::stream(v, &mut stream)
        .map(|_| stream.0.is_some())
        .unwrap_or(false)
}

struct IsU64(Option<u64>);
impl Stream for IsU64 {
    fn u64(&mut self, v: u64) -> stream::Result {
        self.0 = Some(v);

        Ok(())
    }

    fn fmt(&mut self, _: stream::Arguments) -> stream::Result {
        Err(stream::Error::msg("not a u64"))
    }
}
```

## with state

There are more methods on `Stream` that can be overriden for more complex
datastructures like sequences and maps. The following example uses a
[`stream::Stack`] to track the state of any sequences and maps and ensure
they're valid:

```
use std::{fmt, mem};
use sval::stream::{self, stack, Stream, Stack};

struct Fmt {
    stack: stream::Stack,
    delim: &'static str,
}

impl Fmt {
    fn next_delim(pos: stack::Pos) -> &'static str {
        if pos.is_key() {
            return ": ";
        }

        if pos.is_value() || pos.is_elem() {
            return ", ";
        }

        return "";
    }
}

impl Stream for Fmt {
    fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
        let pos = self.stack.primitive()?;

        let delim = mem::replace(&mut self.delim, Self::next_delim(pos));
        print!("{}{:?}", delim, v);

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
        self.stack.seq_begin()?;

        let delim = mem::replace(&mut self.delim, "");
        print!("{}[", delim);

        Ok(())
    }

    fn seq_elem(&mut self) -> stream::Result {
        self.stack.seq_elem()?;

        Ok(())
    }

    fn seq_end(&mut self) -> stream::Result {
        let pos = self.stack.seq_end()?;

        self.delim = Self::next_delim(pos);
        print!("]");

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
        self.stack.map_begin()?;

        let delim = mem::replace(&mut self.delim, "");
        print!("{}{{", delim);

        Ok(())
    }

    fn map_key(&mut self) -> stream::Result {
        self.stack.map_key()?;

        Ok(())
    }

    fn map_value(&mut self) -> stream::Result {
        self.stack.map_value()?;

        Ok(())
    }

    fn map_end(&mut self) -> stream::Result {
        let pos = self.stack.map_end()?;

        self.delim = Self::next_delim(pos);
        print!("}}");

        Ok(())
    }

    fn end(&mut self) -> stream::Result {
        self.stack.end()?;

        println!();

        Ok(())
    }
}
```

The `Stack` type has a fixed depth, so deeply nested structures
aren't supported.

# `serde` integration

Use the `serde` Cargo feature to enable integration with `serde`:

```toml,ignore
[dependencies.sval]
features = ["serde"]
```

When `serde` is available, the `Value` trait can also be derived
based on an existing `Serialize` implementation:

```ignore
use sval::Value;

#[derive(Serialize, Value)]
#[sval(derive_from = "serde")]
pub enum Data {
    Variant(i32, String),
}
# }
```

# `std::fmt` integration

Use the `fmt` Cargo feature to enable extended integration with `std::fmt`:

```toml,ignore
[dependencies.sval]
features = ["fmt"]
```

When `fmt` is available, arbitrary `Value`s can be treated like `std::fmt::Debug`:

```
# fn main() {}
# #[cfg(feature = "fmt")]
# mod test {
fn with_value(value: impl sval::Value) {
    dbg!(sval::fmt::to_debug(&value));

    // Do something with the value
}
# }
```
*/

#![doc(html_root_url = "https://docs.rs/sval/0.1.4")]
#![no_std]

#[macro_use]
mod macros;

#[doc(hidden)]
#[cfg(feature = "derive")]
pub mod derive;

#[doc(inline)]
#[cfg(feature = "derive")]
pub use sval_derive::*;

#[cfg(feature = "std")]
#[macro_use]
#[allow(unused_imports)]
extern crate std;

#[cfg(not(feature = "std"))]
#[macro_use]
#[allow(unused_imports)]
extern crate core as std;

#[macro_use]
mod error;
mod collect;

#[cfg(any(test, feature = "test"))]
pub mod test;

#[cfg(feature = "fmt")]
pub mod fmt;

#[cfg(feature = "serde")]
pub mod serde;

pub mod stream;
pub mod value;

pub use self::{
    error::Error,
    stream::Stream,
    value::Value,
};

/**
Stream the structure of a [`Value`] using the given [`Stream`].
*/
pub fn stream(value: impl Value, stream: impl Stream) -> Result<(), Error> {
    let mut stream = crate::stream::OwnedStream::begin(stream)?;
    stream.any(value)?;
    stream.end()?;

    Ok(())
}
