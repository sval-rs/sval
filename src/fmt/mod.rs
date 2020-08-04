/*!
Integration between `sval` and `std::fmt`.

Add the `fmt` feature to your `Cargo.toml` to enable this module:

```toml,no_run
[dependencies.sval]
features = ["fmt"]
```

# From `sval` to `std::fmt`

A type that implements [`Value`](../value/trait.Value.html) can be converted into
a type that implements [`std::fmt::Debug`]:

```
# use sval::value::{self, Value};
# struct MyValue;
# impl Value for MyValue {
#     fn stream(&self, stream: &mut value::Stream) -> value::Result {
#         unimplemented!()
#     }
# }
# let my_value = MyValue;
let my_debug = sval::fmt::to_debug(my_value);
```
*/

use crate::{
    std::fmt::{
        self,
        Debug,
        Formatter,
    },
    Value,
};

mod to_debug;

pub use self::to_debug::ToDebug;

/**
Convert a [`Value`] into a [`Debug`].

The formatted value is not guaranteed to be exactly the same as
a `Debug` implementation that might exist on the type.

This method doesn't need to allocate or perform any buffering.
*/
pub fn to_debug<V>(value: V) -> ToDebug<V>
where
    V: Value,
{
    ToDebug(value)
}

/**
Format a [`Value`] using the given [`Formatter`].
*/
pub fn debug(f: &mut Formatter, value: impl Value) -> fmt::Result {
    to_debug(value).fmt(f)
}
