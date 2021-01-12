/*!
A small, no-std, object-safe, serialization-only framework.

The `sval` API is built around two key traits:

- [`Value`] is a trait for data with a streamable structure. It's like `serde::Serialize`.
- [`Stream`] is a trait for receiving the structure of a value. It's like `serde::Serializer`.

# Getting started

Add `sval` to your `Cargo.toml`:

```toml,ignore
[dependencies.sval]
version = "1.0.0-alpha.4"
```

# Supported formats

- [JSON](https://crates.io/crates/sval_json), the ubiquitous JavaScript Object Notation used by many HTTP APIs.

# Streaming values

The structure of a [`Value`] can be streamed to a [`Stream`].

## in a single call

For simple use-cases, use the [`stream`](function.stream.html) function to stream the structure of a value:

```no_run
# #[cfg(not(feature = "std"))]
# fn main() {}
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
sval::stream(MyStream, 42)?;
# Ok(())
# }
# use sval::stream::{self, Stream};
# struct MyStream;
# impl Stream for MyStream {
#     fn fmt(&mut self, _: stream::Arguments) -> stream::Result { unimplemented!() }
# }
```

where `42` is a [`Value`] and `MyStream` is a [`Stream`].

## over multiple calls

More involved use-cases may want to build up structure over time. Use a [`stream::OwnedStream`](stream/struct.OwnedStream.html)
to hang on to a stream and pass it values over time.

The following example wraps up a stream in an API that lets callers treat it like a map:

```no_run
# #[cfg(not(feature = "std"))]
# fn main() {}
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let my_stream = MyStream;
// Create a `Map` stream that wraps up another one
let mut stream = Map::new(my_stream)?;

// Stream it some entries
stream.entry("a", 42)?;
stream.entry("b", 17)?;

// Eventually we end the wrapper and return the original stream
let my_stream = stream.end()?;
# struct MyStream;
# struct Map;
# impl Map {
#     fn new<T>(_: T) -> Result<Self, Box<dyn std::error::Error>> { Ok(Map) }
#     fn entry<K, V>(&mut self, _: K, _: V) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
#     fn end(self) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
# }
# Ok(())
}
```

An implementation of `Map` could then look like this:

```no_run
use sval::{
    value::Value,
    stream::{self, OwnedStream},
};

struct Map {
    // Using `OwnedStream<MyStream>` instead of just `MyStream`
    // gives us better ergonomics and validation
    stream: OwnedStream<MyStream>,
}

impl Map {
    fn new(stream: MyStream) -> Result<Self, sval::Error> {
        let mut stream = OwnedStream::new(stream);
        stream.map_begin(None)?;

        Ok(Map {
            stream,
        })
    }

    fn entry(&mut self, k: impl Value, v: impl Value) -> Result<(), sval::Error> {
        self.stream.map_key(k)?;
        self.stream.map_value(v)?;

        Ok(())
    }

    fn end(mut self) -> Result<MyStream, sval::Error> {
        self.stream.map_end()?;

        Ok(self.stream.into_inner())
    }
}
# use sval::stream::Stream;
# struct MyStream;
# impl Stream for MyStream {
#     fn fmt(&mut self, _: stream::Arguments) -> stream::Result { unimplemented!() }
# }
```

# `serde` integration

Use the `serde` Cargo feature to enable integration with `serde`:

```toml,ignore
[dependencies.sval]
features = ["serde"]
```

When `serde` is available, the `Value` trait can also be derived
based on an existing `Serialize` implementation:

```ignore
#[macro_use]
extern crate sval;

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
# use sval::value::Value;
fn with_value(value: impl Value) {
    dbg!(sval::fmt::to_debug(&value));

    // Do something with the value
}
# }
```
*/

#![doc(html_root_url = "https://docs.rs/sval/1.0.0-alpha.4")]
#![no_std]

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! sval_if_alloc {
    (
        if #[cfg(feature = "alloc")]
        {
            $($with:tt)*
        }
        else
        {
            $($without:tt)*
        }
    ) => {
        $($with)*
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "alloc"))]
macro_rules! sval_if_alloc {
    (
        if #[cfg(feature = "alloc")]
        {
            $($with:tt)*
        }
        else
        {
            $($without:tt)*
        }
    ) => {
        $($without)*
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "std")]
macro_rules! sval_if_std {
    (
        if #[cfg(feature = "std")]
        {
            $($with:tt)*
        }
        else
        {
            $($without:tt)*
        }
    ) => {
        $($with)*
    };
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! sval_if_std {
    (
        if #[cfg(feature = "std")]
        {
            $($with:tt)*
        }
        else
        {
            $($without:tt)*
        }
    ) => {
        $($without)*
    };
}

#[doc(hidden)]
#[cfg(feature = "derive")]
pub mod derive;

#[doc(inline)]
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use sval_derive::*;

#[cfg(feature = "std")]
#[macro_use]
#[allow(unused_imports)]
extern crate std;

#[macro_use]
#[allow(unused_imports)]
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc as alloc_lib;
#[macro_use]
#[allow(unused_imports)]
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate core as core_lib;
#[cfg(all(feature = "alloc", not(feature = "std")))]
mod std {
    pub use crate::alloc_lib::{
        boxed,
        collections,
        rc,
        string,
        sync,
        vec,
    };

    pub use crate::core_lib::*;
}

#[cfg(all(not(feature = "std"), not(feature = "alloc")))]
#[macro_use]
#[allow(unused_imports)]
extern crate core as std;

#[macro_use]
mod error;
mod collect;

#[cfg(any(test, feature = "test"))]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub mod test;

#[cfg(feature = "fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub mod fmt;

#[cfg(feature = "serde1_lib")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;

pub mod stream;
pub mod value;

#[doc(inline)]
pub use self::error::Error;

use self::{
    stream::Stream,
    value::Value,
};

/**
Stream the structure of a [`Value`] using the given [`Stream`].

This method is a convenient way of calling [`OwnedStream::stream`](stream/struct.OwnedStream.html#method.stream).
*/
pub fn stream<S>(stream: S, value: impl Value) -> Result<S, Error>
where
    S: Stream,
{
    crate::stream::OwnedStream::stream(stream, value)
}
