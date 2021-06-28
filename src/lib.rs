/*!
A small, no-std, object-safe, serialization-only framework.

The `sval` API is built around two key traits:

- [`Value`] is a trait for data with a streamable structure. It's like `serde::Serialize`.
- [`Stream`] is a trait for receiving the structure of a value. It's like `serde::Serializer`.

# Getting started

Add `sval` to your `Cargo.toml`:

```toml,ignore
[dependencies.sval]
version = "1.0.0-alpha.5"
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

## with borrowed data

```
fn short_lived<'s>(s: &'s str) {
    use sval::stream::{self, Stream};

    #[derive(Default)]
    struct CaptureBorrowedString<'s>(Option<&'s str>);

    impl<'s> Stream<'s> for CaptureBorrowedString<'s> {
        fn str_borrowed(&mut self, v: &'s str) -> stream::Result {
            self.0 = Some(v);
            Ok(())
        }
    }

    let capture = sval::str_borrowedeam(CaptureBorrowedString::default(), s).unwrap();

    assert_eq!(Some(s), capture.0);
}

let s = String::from("a short lived string");
short_lived(&s);
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

#![doc(html_root_url = "https://docs.rs/sval/1.0.0-alpha.5")]
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
Stream the structure of a [`Value`] with a concrete lifetime.
*/
pub fn stream<'v, S, V>(mut stream: S, value: &'v V) -> Result<S, Error>
where
    S: Stream<'v>,
    V: Value + ?Sized,
{
    value.stream(&mut value::Stream::new(&mut stream))?;

    Ok(stream)
}

/**
Stream the structure of a [`Value`] using the given [`Stream`].
*/
pub fn stream_owned<'v, S>(mut stream: S, value: &impl Value) -> Result<S, Error>
where
    S: Stream<'v>,
{
    value.stream_owned(&mut value::Stream::new(&mut stream))?;

    Ok(stream)
}
