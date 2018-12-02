/**
Helpers for testing [`sval::Value`] implementations.

> NOTE: The [`Token`] enum is expected to be non-exhaustively
used in tests, so additional members aren't considered
a breaking `semver` change.
*/

#[cfg(feature = "std")]
mod std_support {
    use crate::{
        value,
    };

    /**
    The kind of token being produced.
    */
    #[doc(inline)]
    pub use self::value::owned::Kind as Token;

    /**
    Collect a value into a sequence of tokens.
    */
    #[doc(inline)]
    pub use self::value::owned::tokens;
}

#[cfg(feature = "std")]
pub use self::std_support::*;
