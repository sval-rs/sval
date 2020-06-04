# `sval_json`

[![Build Status](https://travis-ci.com/sval-rs/sval.svg?branch=master)](https://travis-ci.com/sval-rs/sval)
[![Latest version](https://img.shields.io/crates/v/sval_json.svg)](https://crates.io/crates/sval_json)
[![Documentation Latest](https://docs.rs/sval_json/badge.svg)](https://docs.rs/sval_json)
[![Documentation Master](https://img.shields.io/badge/docs-master-lightgrey.svg)](https://sval-rs.github.io/sval/sval_json/index.html)

A no-std JSON implementation for the [`sval`](https://crates.io/crates/sval) serialization framework.

**`sval_json` is mostly pilfered from dtolnay's [excellent `miniserde` project](https://github.com/dtolnay/miniserde).**

# Minimum `rustc`

This library requires Rust `1.31.0`.

# Cargo features

`sval_json` has the following optional features that can be enabled in your `Cargo.toml`:

- `std`: assume `std` is available and add support for `std` types.

# How to use it

Add `sval_json` to your crate dependencies:

```toml
[dependencies.sval_json]
version = "0.5.2"
```

## To write JSON to a `fmt::Write`

```rust
let json = sval_json::to_fmt(MyWrite, 42)?;
```

## To write JSON to a `String`

Add the `std` feature to your `Cargo.toml` to enable writing to a `String`:

```toml
[dependencies.sval_json]
features = ["std"]
```

```rust
let json = sval_json::to_string(42)?;
```

## To write JSON to a `io::Write`

Add the `std` feature to your `Cargo.toml` to enable writing to an `io::Write`:

```toml
[dependencies.sval_json]
features = ["std"]
```

```rust
let json = sval_json::to_writer(MyWrite, 42)?;
```
