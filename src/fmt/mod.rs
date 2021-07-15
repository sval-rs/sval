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

pub fn to_debug<V>(value: V) -> ToDebug<V>
where
    V: Value,
{
    ToDebug(value)
}

pub fn debug(f: &mut Formatter, value: impl Value) -> fmt::Result {
    to_debug(value).fmt(f)
}
