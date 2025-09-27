# `sval`: Streaming, structured values

[![Rust](https://github.com/sval-rs/sval/workflows/sval/badge.svg)](https://github.com/sval-rs/sval/actions)
[![Latest version](https://img.shields.io/crates/v/sval.svg)](https://crates.io/crates/sval)
[![Documentation Latest](https://docs.rs/sval/badge.svg)](https://docs.rs/sval)

## What is it?

`sval` is a serialization-only framework for Rust. It has a simple, but expressive design that can express any Rust data
structure, plus some it can't yet. It was originally designed as a niche framework for structured logging, targeting
serialization to [JSON](), [protobuf](), and Rust's [Debug-style formatting](). The project has evolved beyond this point into a fully general
and capable framework for introspecting runtime data.

The core of `sval` is the [`Stream`]() trait. It defines the data model and features of the framework. `sval` makes
a few different API design decisions compared to [`serde`](), the de-facto choice, to better accommodate the needs of Rust diagnostic frameworks:

1. **Simple API.** The [`Stream`]() trait has only a few required members that all values are forwarded to.
   This makes it easy to write bespoke handling for specific data types without needing to implement unrelated methods.
2. **`dyn`-friendly.** The [`Stream`]() trait is internally mutable, so is trivial to make `dyn`-compatible without intermediate boxing, making
   it possible to use in no-std environments.
3. **Buffer-friendly.** The [`Stream`]() trait is non-recursive, so values can be buffered as a flat stream of tokens and replayed later.
4. **Borrowing as an optimization.** The [`Stream`]() trait may accept borrowed text or binary fragments for a specific lifetime `'sval`,
   but is also required to accept temporary ones too. This makes it possible to optimize away allocations where possible, but still force
   them if it's required.
5. **Broadly compatible.** `sval` imposes very few constraints of its own, so it can trivially translate implementations of [`serde::Serialize`]()
   into implementations of [`sval::Value`]().

`sval`'s data model takes inspiration from [CBOR](), specifically:

1. **Small core.** The base data model of `sval` is small. The required members of the [`Stream`]() trait only includes nulls, booleans,
   text, 64-bit signed integers, 64-bit floating point numbers, and sequences. All other types, like arbitrary-precision floating point numbers,
   records, and tuples, are representable in the base data model.
2. **Extensible tags.** Users can define _tags_ that extend `sval`'s data model with new semantics. Examples of tags include Rust's `Some` and `None`
   variants, constant-sized arrays, text that doesn't require JSON escaping, and anything else you might need.

## Getting started

Add `sval` to your `Cargo.toml`:

```toml
[dependencies.sval]
version = "2.14.1"
features = ["derive"]
```

Derive [`Value`]() on your Rust types, just like you do with `serde`:

```rust
#[derive(sval::Value)]
pub struct MyRecord<'a> {
    field_0: i32,
    field_1: bool,
    field_2: &'a str,
}
```

Types that derive or implement [`Value`]() can be streamed through an isntance of [`Stream`](). [`sval_json`]() is an example of a stream that
serializes values to JSON:

```toml
[dependencies.sval_json]
version = "2.14.1"
features = ["std"]
```

```rust
let my_record = MyRecord {
    field_0: 1,
    field_1: true,
    field_2: "some text",
};

// {"field_0":1,"field_1":true,"field_2":"some text"}
let json = sval_json::stream_to_string(my_record);
```

[`Stream`]()s can do more than just serialize data into an interchange format. [`sval_buffer`]() is a stream that buffers temporary values into owned,
thread-safe ones. [`sval_flatten`]() is a stream that removes a level of nesting from a field. The [`Value`]() trait's conversion methods,
like [`Value::to_text`](), are streams that attempt to cast an arbitrary value to a concrete type in `sval`'s data model.

Here's an example of a stream that attempts to extract a specific field of a value as an `i32`:

```rust
pub fn get_i32<'sval>(field: &str, value: impl sval::Value) -> Option<i32> {
    struct Extract<'a> {
        depth: usize,
        field: &'a str,
        matched_field: bool,
        extracted: Option<i32>,
    }

    impl<'a, 'sval> sval::Stream<'sval> for Extract<'a> {
        fn record_value_begin(&mut self, _: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
            self.matched_field = label.as_str() == self.field;
            Ok(())
        }

        fn record_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Label) -> sval::Result {
            Ok(())
        }

        fn i64(&mut self, v: i64) -> sval::Result {
            if self.matched_field {
                self.extracted = v.try_into().ok();
            }

            Ok(())
        }

        fn null(&mut self) -> sval::Result {
            Ok(())
        }
    
        fn bool(&mut self, _: bool) -> sval::Result {
            Ok(())
        }
    
        fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
            Ok(())
        }
    
        fn text_fragment_computed(&mut self, _: &str) -> sval::Result {
            Ok(())
        }
    
        fn text_end(&mut self) -> sval::Result {
            Ok(())
        }
    
        fn f64(&mut self, _: f64) -> sval::Result {
            Ok(())
        }
    
        fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
            self.depth += 1;
            Ok(())
        }
    
        fn seq_value_begin(&mut self) -> sval::Result {
            Ok(())
        }
    
        fn seq_value_end(&mut self) -> sval::Result {
            Ok(())
        }
    
        fn seq_end(&mut self) -> sval::Result {
            self.depth -= 1;
            Ok(())
        }
    }

    let mut stream = Extract {
        depth: 0,
        field,
        matched_field: false,
        extracted: None,
    };
    
    sval::stream(&mut stream, &value).ok()?;
    stream.extracted
}
```
