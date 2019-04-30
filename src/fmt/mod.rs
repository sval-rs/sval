/*!
Integration between `sval` and `std::fmt`.

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
*/
pub fn to_debug(value: impl Value) -> impl Debug {
    self::to_debug::ToDebug(value)
}

/**
Format a [`Value`] using the given [`Formatter`].
*/
pub fn debug(value: impl Value, f: &mut Formatter) -> fmt::Result {
    to_debug(value).fmt(f)
}
