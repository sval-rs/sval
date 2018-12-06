# `sval`

[![Build Status](https://travis-ci.com/KodrAus/sval.svg?branch=master)](https://travis-ci.com/KodrAus/sval)
[![Latest version](https://img.shields.io/crates/v/sval.svg)](https://crates.io/crates/sval)
[![Documentation Latest](https://docs.rs/sval/badge.svg)](https://docs.rs/sval)
[![Documentation Master](https://img.shields.io/badge/docs-master-lightgrey.svg)](https://sval-rs.github.io/sval/sval/index.html)

A prototype, lightweight, no-std, object-safe, serialization-only API for structured values with `serde` support.

Producers of structured values use the `value` module. Consumers of structured values use the `stream` module. `sval` offers a json-like data model, which is more limiting than `serde`'s, but capable enough to represent Rust datastructures in one form or another.

This library is designed to plug a no-std-object-safe sized hole in Rust's current serialization ecosystem. The driving use-case is structured logging, where individual events are typically small, and there's no complete schema that can tie values in any one event to values in another.

**`sval_json` and `sval_derive` are mostly pilfered from dtolnay's [excellent `miniserde` project](https://github.com/dtolnay/miniserde).**

# Minimum `rustc`

This library requires Rust `1.31.0`, which is currently in `beta`.

# See also

- [`serde`](https://docs.rs/serde)
- [`miniserde`](https://docs.rs/miniserde)

# How to use it

Add `sval` to your crate dependencies:

```toml
[dependencies.sval]
version = "0.0.2"
```

## To support my datastructures

Simple struct-like datastructures can derive `sval::Value`:

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

Other datastructures can implement `sval::Value` manually:

```rust
use sval::value::{self, Value};

struct MyId(u64);

impl Value for MyId {
    fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
        stream.u64(self.0)
    }
}
```

## To format my data

The `sval_json` crate can format any `sval::Value` as json:

```toml
[dependencies.sval_json]
version = "0.0.2"
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

Then convert between `serde` and `sval`:

```rust
let my_serialize = sval::serde::to_serialize(my_data);
```
