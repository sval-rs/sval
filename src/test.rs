/*!
Helpers for testing value implementations.

> NOTE: The [`Token`](enum.Token.html) enum is expected to be non-exhaustively
used in tests, so additional members aren't considered
a breaking `semver` change.
*/

#[cfg(feature = "std")]
mod std_support {
    use crate::{
        value,
    };

    // TODO: Inline the enum and method
    // TODO: Scrape out the key, value, and elem variants

    /**
    The kind of token being produced.
    */
    pub use self::value::owned::Kind as Token;

    /**
    Collect a value into a sequence of tokens.
    */
    pub use self::value::owned::tokens;
}

#[cfg(feature = "std")]
pub use self::std_support::*;
