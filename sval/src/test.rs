#[cfg(feature = "std")]
mod std_support {
    use crate::{
        std::vec::Vec,
        value::{
            self,
            Value,
        },
    };

    pub(crate) use self::value::owned::Kind;

    pub(crate) fn tokens(v: impl Value) -> Vec<Kind> {
        v.to_owned().into_tokens()
    }
}

#[cfg(feature = "std")]
pub(crate) use self::std_support::*;
