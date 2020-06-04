# `sval`

[![Build Status](https://travis-ci.com/sval-rs/sval.svg?branch=master)](https://travis-ci.com/sval-rs/sval)
[![Latest version](https://img.shields.io/crates/v/sval.svg)](https://crates.io/crates/sval)
[![Documentation Latest](https://docs.rs/sval/badge.svg)](https://docs.rs/sval)

A lightweight, no-std, object-safe, serialization-only API for structured values with `serde` and `std::fmt` support.

Producers of structured values use the `value` module. Consumers of structured values use the `stream` module.

`sval` offers a JSON-like data model, which is more limiting than `serde`'s, but capable enough to represent Rust data-structures in one form or another.

This library is designed to plug a no-std-object-safe sized hole in Rust's current serialization ecosystem. The driving use-case is structured logging, where individual events are typically small, and there's no complete schema that can tie values in any one event to values in another.

**`sval_json` and `sval_derive` are mostly pilfered from dtolnay's [excellent `miniserde` project](https://github.com/dtolnay/miniserde).**

# Supported formats

- [JSON](https://crates.io/crates/sval_json), the ubiquitous JavaScript Object Notation used by many HTTP APIs.

# Minimum `rustc`

This library requires Rust `1.31.0`.

# See also

- [`serde`](https://docs.rs/serde)
- [`miniserde`](https://docs.rs/miniserde)

# Cargo features

`sval` has the following optional features that can be enabled in your `Cargo.toml`:

- `std`: assume `std` is available and add support for `std` types. Implies `alloc`.
- `alloc`: assume a global allocator is available.
- `derive`: add support for `#[derive(Value)]`.
- `serde`: enable integration with `serde`. Some implementations of `sval::Value` may not be representable without the `alloc` feature.
- `fmt`: support converting any `Value` into a `Debug`.
- `arbitrary-depth`: support stateful values with any depth. Implies `alloc`.
- `test`: add helpers for testing implementations of `Value`. Implies `std`. You should avoid using this feature outside of `dev-dependencies`.

# How to use it

Add `sval` to your crate dependencies:

```toml
[dependencies.sval]
version = "0.5.2"
```

## To support my data-structures

Simple struct-like data-structures can derive `sval::Value`:

```toml
[dependencies.sval]
features = ["derive"]
```

```rust
#[macro_use]
extern crate sval;

#[derive(Value)]
struct MyData {
    id: u64,
    name: String,
}
```

Other data-structures can implement `sval::Value` manually:

```rust
use sval::value::{self, Value};

struct MyId(u64);

impl Value for MyId {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        stream.u64(self.0)
    }
}
```

## To format my data

The `sval_json` crate can format any `sval::Value` as JSON:

```toml
[dependencies.sval_json]
version = "0.5.2"
features = ["std"]
```

```rust
let my_json = sval_json::to_string(my_data)?;
```

## To integrate with `serde`

`sval` has out-of-the-box `serde` integration between `sval::Value`s and `serde::Serialize`s. Add the `serde` feature to `sval` to enable it:

```toml
[dependencies.sval]
features = ["serde"]
```

Use the `to_serialize` function to turn any `sval::Value` into a `serde::Serialize`:

```rust
let my_serialize = sval::serde::to_serialize(my_data);
```

Use the `to_value` function to turn any `serde::Serialize` into a `sval::Value`:

```rust
let my_value = sval::serde::to_value(my_data);
```

When the `serde` feature is available, structures that already derive `Serialize` can also always derive `Value`. The `Value` implementation will behave the same as `Serialize`:

```rust
#[derive(Serialize, Value)]
#[sval(derive_from = "serde")]
struct MyData {
    id: u64,
    name: String,
    #[serde(flatten)]
    props: serde_json::Map<String, serde_json::Value>,
}
```

## To integrate with `std::fmt`

`sval` can provide a compatible `Debug` implementation for any `sval::Value`. Add the `fmt` feature to `sval` to enable it:

```toml
[dependencies.sval]
features = ["fmt"]
```

Use the `to_debug` function to turn any `sval::Value` into a `std::fmt::Debug`:

```rust
fn with_value(value: impl Value) {
    dbg!(sval::fmt::to_debug(&value));

    ..
}
```
