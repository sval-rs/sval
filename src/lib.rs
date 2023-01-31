/*!
Structured, streaming values.

`sval` is a serialization framework that treats data as a flat stream of tokens.
The source of that data could be some Rust object or parsed from some encoding.
It's well suited to self-describing, text-based formats like JSON.

# A note on docs

Even though this library's API is stable, these docs themselves are still a
work-in-progress.

# Getting started

Add `sval` to your `Cargo.toml`:

```toml
[dependencies.sval]
version = "0.0.0"
```

By default, `sval` doesn't depend on Rust's standard library or integrate
with its collection types. To include them, add the `alloc` or `std` features:

```toml
[dependencies.sval]
version = "0.0.0"
features = ["std"]
```

# The `Value` trait

[`Value`] is a trait for data types to implement that surfaces their structure
through visitors called _streams_. `Value` is like `serde`'s `Serialize`.

Many standard types in Rust implement the `Value` trait.

`Value` can be derived when the `derive` feature is enabled.

# The `Stream` trait

[`Stream`] is a trait for data formats and visitors to implement that observes
the structure of _values_. `Stream` is like `serde`'s `Serializer`.

# Data-model

`sval`'s data-model is defined by the [`Stream`] trait. It includes:

- Null
- Booleans (`true`, `false`)
- Text blobs
- Binary blobs
- Integers (`u8`-`u128`, `i8`-`i128`)
- Binary floating points (`f32`-`f64`)
- Maps
- Sequences
- Records
- Tuples
- Enums
- Tags

# Tags

[`Tag`] is a type for extending `sval`'s data-model with new kinds of values.
Rust's own `()` and `Option<T>` types are expressed as tags. Other examples of
tags include text that encodes RFC3339 timestamps or RFC4122 UUIDs.

The [`tags`] module contains built-in tags. Other libraries may define their own tags too.

# Buffering

Complex or arbitrarily-sized values like strings, maps, and sequences can all be
streamed as chunks across multiple calls to avoid intermediate buffering when it's not necessary.

# Object safety

The [`Value`] and [`Stream`] traits aren't object-safe themselves, but object-safe
wrappers are provided by the [`sval_dynamic`] crate. This wrapper works in no-std.
*/

#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]

#[cfg(feature = "std")]
extern crate std;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate core;

#[cfg(all(feature = "alloc", not(feature = "std")))]
mod std {
    pub use crate::{
        alloc::{borrow, boxed, collections, string, vec},
        core::{convert, fmt, hash, marker, mem, ops, result, str},
    };
}

#[cfg(all(not(feature = "alloc"), not(feature = "std")))]
extern crate core as std;

mod data;
mod result;
mod stream;
mod value;

#[doc(inline)]
pub use self::{data::*, result::*, stream::*, value::*};

/**
A generic streaming result.
*/
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/**
Stream a value through a stream.
*/
pub fn stream<'sval, S: Stream<'sval> + ?Sized, V: Value + ?Sized>(
    stream: &mut S,
    value: &'sval V,
) -> Result {
    stream.value(value)
}

/**
Stream a value through a stream with an arbitrarily short lifetime.
*/
pub fn stream_computed<'sval, S: Stream<'sval> + ?Sized, V: Value + ?Sized>(
    stream: &mut S,
    value: &V,
) -> Result {
    stream.value_computed(value)
}
