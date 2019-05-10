/*!
Integration between `sval` and `std::fmt`.

Add the `fmt` feature to your `Cargo.toml` to enable this module:

```toml,no_run
[dependencies.sval]
features = ["fmt"]
```

# From `sval` to `std::fmt`

A type that implements [`sval::Value`](../value/trait.Value.html) can be converted into
a type that implements [`std::fmt::Debug`]:

```
# struct MyValue;
# impl sval::value::Value for MyValue {
#     fn stream(&self, stream: &mut sval::value::Stream) -> Result<(), sval::value::Error> {
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

/**
Convert a [`Value`] into a [`Debug`].

The formatted value is not guaranteed to be exactly the same as
a `Debug` implementation that might exist on the type.

This method doesn't need to allocate or perform any buffering.
*/
pub fn to_debug(value: impl Value) -> impl Debug {
    self::to_debug::ToDebug(value)
}

/**
Format a [`Value`] using the given [`Formatter`].
*/
pub fn debug(f: &mut Formatter, value: impl Value) -> fmt::Result {
    to_debug(value).fmt(f)
}
