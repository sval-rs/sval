/*!
A small, no-std, object-safe, serialization-only framework.

The `sval` API is built around two key traits:

- [`Value`] is a trait for data with a streamable structure. It's like `serde::Serialize`.
- [`Stream`] is a trait for receiving the structure of a value. It's like `serde::Serializer`.

# Getting started

Add `sval` to your `Cargo.toml`:

```toml,ignore
[dependencies.sval]
version = "0.5.0"
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
to hang on to a stream and pass it values over time:

```no_run
# #[cfg(not(feature = "std"))]
# fn main() {}
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use sval::{
    Value,
    stream::{self, OwnedStream},
};

// We begin the wrapper over `MyStream`
let mut stream = StreamPairs::new()?;

// Pairs can be streamed independently
stream.pair("a", 42)?;
stream.pair("b", 17)?;

// Eventually we end the wrapper and return the underlying `MyStream`
let my_stream = stream.end()?;

struct StreamPairs {
    // Using `OwnedStream<MyStream>` instead of just `MyStream`
    // gives us better ergonomics and validation
    stream: OwnedStream<MyStream>,
}

impl StreamPairs {
    fn new() -> Result<Self, stream::Error> {
        let mut stream = OwnedStream::new(MyStream);
        stream.map_begin(None)?;

        Ok(StreamPairs {
            stream,
        })
    }

    fn pair(&mut self, k: impl Value, v: impl Value) -> Result<(), stream::Error> {
        self.stream.map_key(k)?;
        self.stream.map_value(v)?;

        Ok(())
    }

    fn end(mut self) -> Result<MyStream, stream::Error> {
        self.stream.map_end()?;

        Ok(self.stream.into_inner())
    }
}
# Ok(())
# }
# use sval::stream::{self, Stream};
# struct MyStream;
# impl Stream for MyStream {
#     fn fmt(&mut self, _: stream::Arguments) -> stream::Result { unimplemented!() }
# }
```

The above example captures an `OwnedStream<MyStream>` and then allows multiple key-value pairs to be
streamed through it before finishing.

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

In no-std environments, `serde` support can be enabled using the `serde_no_std` feature
instead:

```toml,ignore
[dependencies.sval]
features = ["serde_no_std"]
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

#![doc(html_root_url = "https://docs.rs/sval/0.5.0")]
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
#[cfg(feature = "derive")]
pub mod derive;

#[doc(inline)]
#[cfg(feature = "derive")]
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
        vec,
        string,
        rc,
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
pub mod test;

#[cfg(feature = "fmt")]
pub mod fmt;

#[cfg(feature = "serde_lib")]
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

This method is a convenient way of calling [`OwnedStream::stream`](stream/struct.OwnedStream.html#method.stream).
*/
pub fn stream<S>(stream: S, value: impl Value) -> Result<S, Error>
where
    S: Stream,
{
    crate::stream::OwnedStream::stream(stream, value)
}
