# `sval`: Streaming, structured values

[![Rust](https://github.com/sval-rs/sval/workflows/sval/badge.svg)](https://github.com/sval-rs/sval/actions)
[![Latest version](https://img.shields.io/crates/v/sval.svg)](https://crates.io/crates/sval)
[![Documentation Latest](https://docs.rs/sval/badge.svg)](https://docs.rs/sval)

## What is it?

`sval` is a serialization-only framework for Rust. It has a simple, but expressive design that can express any Rust data structure, plus some it can't yet. It was originally designed as a niche framework for structured logging, targeting serialization to [JSON](), [protobuf](), and Rust's [Debug-style formatting](). The project has evolved beyond this point into a fully general and capable framework for introspecting runtime data.

The core of `sval` is the [`Stream`]() trait. It defines the data model and features of the framework. `sval` makes a few different API design decisions compared to [`serde`](), the de-facto choice, to better accommodate the needs of Rust diagnostic frameworks:

1. **Simple API.** The [`Stream`]() trait has only a few required members that all values are forwarded to. This makes it easy to write bespoke handling for specific data types without needing to implement unrelated methods.
2. **`dyn`-friendly.** The [`Stream`]() trait is internally mutable, so is trivial to make `dyn`-compatible without intermediate boxing, making it possible to use in no-std environments.
3. **Buffer-friendly.** The [`Stream`]() trait is non-recursive, so values can be buffered as a flat stream of tokens and replayed later.
4. **Borrowing as an optimization.** The [`Stream`]() trait may accept borrowed text or binary fragments for a specific lifetime `'sval`, but is also required to accept temporary ones too. This makes it possible to optimize away allocations where possible, but still force them if it's required.
5. **Broadly compatible.** `sval` imposes very few constraints of its own, so it can trivially translate implementations of [`serde::Serialize`]() into implementations of [`sval::Value`]().

`sval`'s data model takes inspiration from [CBOR](), specifically:

1. **Small core.** The base data model of `sval` is small. The required members of the [`Stream`]() trait only includes nulls, booleans, text, 64-bit signed integers, 64-bit floating point numbers, and sequences. All other types, like arbitrary-precision floating point numbers, records, and tuples, are representable in the base data model.
2. **Extensible tags.** Users can define _tags_ that extend `sval`'s data model with new semantics. Examples of tags include Rust's `Some` and `None` variants, constant-sized arrays, text that doesn't require JSON escaping, and anything else you might need.

## Getting started

This section is a high-level guided tour of `sval`'s design and API. To get started, add `sval` to your `Cargo.toml`:

```toml
[dependencies.sval]
version = "2.14.1"
features = ["derive"]
```

### Serializing values

As a quick example, here's how you can use `sval` to serialize a runtime value as JSON.

First, add [`sval_json`]() to your `Cargo.toml`:

```toml
[dependencies.sval_json]
version = "2.14.1"
features = ["std"]
```

Next, derive the [`Value`]() trait on the type you want to serialize, including on any other types it uses in its fields:

```rust
#[derive(sval::Value)]
pub struct MyRecord<'a> {
    field_0: i32,
    field_1: bool,
    field_2: &'a str,
}
```

Finally, use [`stream_to_string`]() to serialize an instance of your type as JSON:

```rust
let my_record = MyRecord {
    field_0: 1,
    field_1: true,
    field_2: "some text",
};

// Produces:
//
// {"field_0":1,"field_1":true,"field_2":"some text"}
let json: String = sval_json::stream_to_string(my_record)?;
```

### The `Value` trait

The previous example didn't reveal a lot of detail about how `sval` works, only that there's a [`Value`]() trait involved, and it somehow allows us to convert an instance of the `MyRecord` struct into a JSON object. Using [`cargo expand`](), we can peek behind the covers and see what the `Value` trait does. The previous example expands to something like this:

```rust
impl<'a> sval::Value for MyRecord<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        let type_label = Some(&sval::Label::new("MyRecord").with_tag(&sval::tags::VALUE_IDENT));
        let type_index = None;

        stream.record_tuple_begin(None, type_label, type_index, Some(3))?;

        let mut field_index = 0;

        // field_0
        {
            field_index += 1;

            let field_index = &sval::Index::from(field_index - 1).with_tag(&sval::tags::VALUE_OFFSET);
            let field_label = &sval::Label::new("field_0").with_tag(&sval::tags::VALUE_IDENT);

            stream.record_tuple_value_begin(None, field_label, field_index)?;
            stream.value(&self.field_0)?;
            stream.record_tuple_value_end(None, field_label, field_index)?;
        }

        // field_1
        {
            field_index += 1;

            let field_index = &sval::Index::from(field_index - 1).with_tag(&sval::tags::VALUE_OFFSET);
            let field_label = &sval::Label::new("field_1").with_tag(&sval::tags::VALUE_IDENT);

            stream.record_tuple_value_begin(None, field_label, field_index)?;
            stream.value(&self.field_1)?;
            stream.record_tuple_value_end(None, field_label, field_index)?;
        }

        // field_2
        {
            field_index += 1;

            let field_index = &sval::Index::from(field_index - 1).with_tag(&sval::tags::VALUE_OFFSET);
            let field_label = &sval::Label::new("field_2").with_tag(&sval::tags::VALUE_IDENT);
            
            stream.record_tuple_value_begin(None, field_label, field_index)?;
            stream.value(&self.field_2)?;
            stream.record_tuple_value_end(None, field_label, field_index)?;
        }

        stream.record_tuple_end(None, type_label, type_index)
    }
}
```

The [`Value`]() trait has a single required method, `stream`, which is responsible for driving an instance of a [`Stream`]() with its fields. The [`Stream`]() trait defines `sval`'s data model and the mechanics of how data is described in it. In this example, the `MyRecord` struct is represented as a _record tuple_, a type that can be either a _record_ with fields named by a [`Label`](), or a _tuple_ with fields indexed by an [`Index`](). Labels and indexes can be annotated with a [`Tag`]() which add user-defined semantics to them. In this case, the labels carry the [`VALUE_IDENT`]() tag meaning they're valid Rust identifiers, and the indexes carry the [`VALUE_OFFSET`]() tag meanings they're zero-indexed field offsets. The specific type of [`Stream`]() can decide whether to treat the `MyRecord` type as either a record (in the case of JSON) or a tuple (in the case of protobuf), and whether it understands that tags it sees or not.

### The `Stream` trait

Something to notice about the [`Stream`]() API in the expanded `MyRecord` example is that it is _flat_. The call to `record_tuple_begin` doesn't return a new type like `serde`'s `struct_begin` does. The implementor of [`Value`]() is responsible for issuing the correct sequence of [`Stream`]() calls as it works through its structure. The [`Stream`]() can then rely on markers like `record_tuple_value_begin` and `record_tuple_value_end` to know what position within a value it is without needing to track that state itself. The flat API makes dyn-compatibility and buffering simpler, but makes implementing non-trivial streams more difficult, because you can't rely on recursive to manage state.

Recall the way `MyRecord` was converted into JSON earlier:

```rust
let json: String = sval_json::stream_to_string(my_record)?;
```

Internally, [`stream_to_string`]() uses an instance of [`Stream`]() that writes JSON tokens for each piece of the value it encounters. For example, `record_tuple_begin` and `record_tuple_end` will emit the corresponding `{` `}` characters for a JSON object.

`sval`'s data model is _layered_. The required methods on [`Stream`]() represent the base data model that more expressive constructs map down to. Here's what a minimal [`Stream`]() that just formats values in `sval`'s base data model looks like:

```rust
pub struct MyStream;

impl<'sval> sval::Stream<'sval> for MyStream {
    fn null(&mut self) -> sval::Result {
        print!("null");
        Ok(())
    }

    fn bool(&mut self, v: bool) -> sval::Result {
        print!("{}", v);
        Ok(())
    }

    fn i64(&mut self, v: i64) -> sval::Result {
        print!("{}", v);
        Ok(())
    }

    fn f64(&mut self, v: f64) -> sval::Result {
        print!("{}", v);
        Ok(())
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        print!("\"");
        Ok(())
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        print!("{}", fragment.escape_debug());

        Ok(())
    }

    fn text_end(&mut self) -> sval::Result {
        print!("\"");
        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
        print!("[");
        Ok(())
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        print!(",");
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        print!("]");
        Ok(())
    }
}

let my_record = MyRecord {
    field_0: 1,
    field_1: true,
    field_2: "some text",
};

// Prints:
//
// [["field_0",1,],["field_1",true,],["field_2","some text",],],
sval::stream(&mut MyStream, my_record);
```

Recall that the `MyRecord` struct mapped to a `record_tuple` in `sval`'s data model. `record_tuple`s in turn are represented in the base data model as a sequence of 2-dimensional sequences where the first element is the field label and the second is its value.

[`Stream`]()s aren't limited to just serializing data into interchange formats. They can manipulate or interrogate a value any way it likes. Here's an example of a [`Stream`]() that attempts to extract a specific field of a value as an `i32`:

```rust
pub fn get_i32<'sval>(field: &str, value: impl sval::Value) -> Option<i32> {
    struct Extract<'a> {
        depth: usize,
        field: &'a str,
        matched_field: bool,
        extracted: Option<i32>,
    }

    impl<'a, 'sval> sval::Stream<'sval> for Extract<'a> {
        // Rust structs that derive `Value`, like `MyRecord` from earlier, are records in `sval`'s data model.
        // Each field of the record starts with a call to `record_value_begin` with its name.
        fn record_value_begin(&mut self, _: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
            self.matched_field = label.as_str() == self.field;
            Ok(())
        }

        fn record_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Label) -> sval::Result {
            Ok(())
        }

        // We're looking for an `i32`, so will attempt to cast an integer we find.
        // `sval` will forward any convertible integer to `i64` by default.
        // We could also override the `i32` method here.
        fn i64(&mut self, v: i64) -> sval::Result {
            if self.matched_field {
                self.extracted = v.try_into().ok();
            }

            Ok(())
        }

        // `sval` will forward all complex types as sequences by default.
        // We're only interested in top-level fields of records here, so whenever
        // we encounter a sequence we increment/decrement our depth to tell how
        // deeply nested we are.
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

        // These other methods are required members of `Stream`.
        // We're not interested in them in this example.
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

let my_record = MyRecord {
    field_0: 1,
    field_1: true,
    field_2: "some text",
};

assert_eq!(Some(1), get_i32("field_0", &my_record));
```

### Chunking strings

Strings in `sval` don't need to be streamed in a single call. As an example, say we have a template type like this:

```rust
enum Part<'a> {
    Literal(&'a str),
    Property(&'a str),
}

pub struct Template<'a>(&'a [Part<'a>]);
```

If we wanted to serialize `Template` to a string, we could implement [`Value`](), handling each literal and property as a separate fragment:

```rust
impl<'a> sval::Value for Template<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        stream.text_begin(None)?;

        for part in self.0 {
            match part {
                Part::Literal(lit) => stream.text_fragment(lit)?,
                Part::Property(prop) => {
                    stream.text_fragment("{")?;
                    stream.text_fragment(prop)?;
                    stream.text_fragment("}")?;
                }
            }
        }

        stream.text_end()
    }
}
```

When streamed as JSON, `Template` would produce something like this:

```rust
let template = Template(&[
    Part::Literal("some literal text and "),
    Part::Property("x"),
    Part::Literal(" and more literal text"),
]);

// Produces:
//
// "some literal text and {x} and more literal text"
let json = sval_json::stream_to_string(template)?;
```

### Borrowed data

The [`Stream`]() trait carries a `'sval` lifetime it can use to accept borrowed text and binary values. Borrowing in `sval` is an optimization. Even if a [`Stream`]() uses a concrete `'sval` lifetime, it still needs to handle computed values. Here's an example of a [`Stream`]() that attempts to extract a borrowed string from a value by making use of the `'sval` lifetime:

```rust
pub fn to_text(value: &(impl Value + ?Sized)) -> Option<&str> {
    struct Extract<'sval> {
        extracted: Option<&'sval str>,
        seen_fragment: bool,
    }

    impl<'sval> Stream<'sval> for Extract<'sval> {
        fn text_begin(&mut self, _: Option<usize>) -> Result {
            Ok(())
        }

        // `text_fragment` accepts a string borrowed for `'sval`.
        //
        // Implementations of `Value` will send borrowed data if they can
        fn text_fragment(&mut self, fragment: &'sval str) -> Result {
            // Allow either independent strings, or fragments of a single borrowed string
            if !self.seen_fragment {
                self.extracted = Some(fragment);
                self.seen_fragment = true;
            } else {
                self.extracted = None;
            }

            Ok(())
        }

        // `text_fragment_computed` accepts a string for an arbitrarily short lifetime.
        //
        // The fragment can't be borrowed outside of the function call, so would need to
        // be buffered.
        fn text_fragment_computed(&mut self, _: &str) -> Result {
            self.extracted = None;
            self.seen_fragment = true;

            sval::error()
        }

        fn text_end(&mut self) -> Result {
            Ok(())
        }

        fn null(&mut self) -> Result {
            sval::error()
        }

        fn bool(&mut self, _: bool) -> Result {
            sval::error()
        }

        fn i64(&mut self, _: i64) -> Result {
            sval::error()
        }

        fn f64(&mut self, _: f64) -> Result {
            sval::error()
        }

        fn seq_begin(&mut self, _: Option<usize>) -> Result {
            sval::error()
        }

        fn seq_value_begin(&mut self) -> Result {
            sval::error()
        }

        fn seq_value_end(&mut self) -> Result {
            sval::error()
        }

        fn seq_end(&mut self) -> Result {
            sval::error()
        }
    }

    let mut extract = Extract {
        extracted: None,
        seen_fragment: false,
    };

    value.stream(&mut extract).ok()?;
    extract.extracted
}
```

Implementations of [`Value`]() should provide a [`Stream`]() with borrowed data where possible, and only compute it if it needs to.

### Error handling

`sval`'s [`Error`]() type doesn't carry any state of its own. It only signals early termination of the [`Stream`]() which may be because its job is done, or because it failed. It's up to the [`Stream`]() to carry whatever state it needs to provide meaningful errors.

## Ecosystem

`sval` is a general framework with specific serialization formats and utilities provided as external libraries:

- [`sval_fmt`](): Colorized Rust-style debug formatting.
- [`sval_json`](): Serialize values as JSON in a `serde`-compatible format.
- [`sval_protobuf`](): Serialize values as protobuf messages.
- [`sval_serde`](): Convert between `serde` and `sval`.
- [`sval_buffer`](): Losslessly buffers any [`Value`]() into an owned, thread-safe variant.
- [`sval_flatten`](): Flatten the fields of a value onto its parent, like `#[serde(flatten)]`.
- [`sval_nested`](): Buffer `sval`'s flat [`Stream`]() API into a recursive one like `serde`'s. For types that `#[derive(Value)]`, the translation is non-allocating.
- [`sval_ref`](): A variant of [`Value`]() for types that are internally borrowed (like `MyType<'a>`) instead of externally (like `&'a MyType`).
