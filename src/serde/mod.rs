/*!
Integration between `sval` and `serde`.

Add the `serde` feature to your `Cargo.toml` to enable this module:

```toml,no_run
[dependencies.sval]
features = ["serde"]
```

# From `sval` to `serde`

A type that implements [`sval::Value`](../value/trait.Value.html) can be converted into
a type that implements `serde::Serialize`:

```
# struct MyValue;
# impl sval::value::Value for MyValue {
#     fn stream(&self, stream: &mut sval::value::Stream) -> Result<(), sval::value::Error> {
#         unimplemented!()
#     }
# }
# let my_value = MyValue;
let my_serialize = sval::serde::to_serialize(my_value);
```

When using `serde` without `alloc`, there are some limitations on what kinds of `sval::Value`s you
can convert into `serde::Serialize`s:

- Any type that uses [`value::Stream::map_key_begin`], [`value::Stream::map_value_begin`],
or [`value::Stream::seq_elem_begin`] would require buffering, so will return an error instead
in no-std environments.

# From `serde` to `sval`

A type that implements `serde::Serialize` can be converted into
a type that implements [`sval::Value`](../value/trait.Value.html):

```
# struct MySerialize;
# impl serde_lib::Serialize for MySerialize {
#     fn serialize<S: serde_lib::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
#         unimplemented!()
#     }
# }
# let my_serialize = MySerialize;
let my_value = sval::serde::to_value(my_serialize);
```

[`value::Stream::map_key_begin`]: ../value/struct.Stream.html#method.map_key_begin
[`value::Stream::map_value_begin`]: ../value/struct.Stream.html#method.map_value_begin
[`value::Stream::seq_elem_begin`]: ../value/struct.Stream.html#method.seq_elem_begin
*/

mod error;

mod to_serialize;
mod to_value;

use crate::{
    Error,
    Stream,
    Value,
};

use serde_lib::ser::{
    Serialize,
    Serializer,
};

/**
Convert a [`Value`] into a [`Serialize`].

If the `Value` uses nested maps or sequences where the keys, values
or elements aren't known upfront then this method will need to allocate
for them.
*/
pub fn to_serialize(value: impl Value) -> impl Serialize {
    to_serialize::ToSerialize(value)
}

/**
Serialize a [`Value`] using the given [`Serializer`].
*/
pub fn serialize<S>(serializer: S, value: impl Value) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    to_serialize(value).serialize(serializer)
}

/**
Convert a [`Serialize`] into a [`Value`].
*/
pub fn to_value(value: impl Serialize) -> impl Value {
    to_value::ToValue(value)
}

/**
Stream a [`Serialize`] using the given [`Stream`].
*/
pub fn stream<S>(stream: S, value: impl Serialize) -> Result<S, Error>
where
    S: Stream,
{
    crate::stream(stream, to_value(value))
}

#[doc(hidden)]
#[cfg(feature = "std")]
pub const IS_NO_STD: bool = false;

#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub const IS_NO_STD: bool = true;
