/*!
Helpers for testing value implementations.

Add the `test` feature to your `Cargo.toml` to enable this module:

```toml,no_run
[dependencies.sval]
features = ["test"]
```

**NOTE:** The [`Token`](enum.Token.html) enum is expected to be non-exhaustively
used in tests, so additional members aren't considered
a breaking `semver` change.
*/

#[cfg(feature = "std")]
mod std_support {
    use crate::{
        std::{
            string::String,
            vec::Vec,
        },
        value::{
            owned::Kind,
            OwnedValue,
            Value,
        },
    };

    /**
    The kind of token being produced.
    */
    #[derive(Debug, Clone, PartialEq)]
    pub enum Token {
        MapBegin(Option<usize>),
        MapEnd,
        SeqBegin(Option<usize>),
        SeqEnd,
        Signed(i64),
        Unsigned(u64),
        Float(f64),
        BigSigned(i128),
        BigUnsigned(u128),
        Bool(bool),
        Str(String),
        Char(char),
        None,
    }

    /**
    Collect a value into a sequence of tokens.
    */
    pub fn tokens(v: impl Value) -> Vec<Token> {
        OwnedValue::collect(v)
            .tokens()
            .unwrap()
            .iter()
            .filter_map(|token| match token.kind {
                Kind::MapBegin(len) => Some(Token::MapBegin(len)),
                Kind::MapEnd => Some(Token::MapEnd),
                Kind::SeqBegin(len) => Some(Token::SeqBegin(len)),
                Kind::SeqEnd => Some(Token::SeqEnd),
                Kind::Signed(v) => Some(Token::Signed(v)),
                Kind::Unsigned(v) => Some(Token::Unsigned(v)),
                Kind::BigSigned(v) => Some(Token::BigSigned(v)),
                Kind::BigUnsigned(v) => Some(Token::BigUnsigned(v)),
                Kind::Float(v) => Some(Token::Float(v)),
                Kind::Bool(v) => Some(Token::Bool(v)),
                Kind::Char(v) => Some(Token::Char(v)),
                Kind::Str(ref v) => Some(Token::Str((**v).into())),
                Kind::None => Some(Token::None),
                _ => None,
            })
            .collect()
    }
}

#[cfg(feature = "std")]
pub use self::std_support::*;
