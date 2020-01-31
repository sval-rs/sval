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

#[cfg(feature = "alloc")]
mod alloc_support {
    use crate::{
        std::{
            string::String,
            vec::Vec,
        },
        stream::{
            self,
            OwnedStream,
            Stream,
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

    /**
    Ensure a stream is valid for various inputs.

    This is useful for smoke-testing stream implementations to make sure they explicitly
    implement all methods on `Stream`. The stream will only be given technically valid inputs.

    Any new methods added to `Stream` may cause this method to panic until the given `Stream` is updated.
    */
    pub fn stream_exhaustive<S>(build: impl Fn() -> S, check: impl Fn(Result<S, stream::Error>))
    where
        S: Stream,
    {
        use crate::std::{
            boxed::Box,
            collections::BTreeMap,
        };

        let values: Vec<Box<dyn Value>> = vec![
            Box::new(u8::max_value()),
            Box::new(u16::max_value()),
            Box::new(u32::max_value()),
            Box::new(u64::max_value()),
            Box::new(u128::max_value()),
            Box::new(i8::min_value()),
            Box::new(i16::min_value()),
            Box::new(i32::min_value()),
            Box::new(i64::min_value()),
            Box::new(i128::min_value()),
            Box::new(4.25827473958372f32),
            Box::new(4271.00000000001f64),
            Box::new('山'),
            Box::new("🍔∈🌏"),
            Box::new(Some(1)),
            Box::new(Option::None::<u8>),
            Box::new(vec![1, 2, 3, 4]),
            Box::new({
                let v: Vec<Box<dyn Value>> = vec![Box::new(1), Box::new('a')];
                v
            }),
            Box::new({
                let mut v = BTreeMap::new();
                v.insert(String::from("a"), 1);
                v.insert(String::from("b"), 2);
                v.insert(String::from("c"), 3);
                v.insert(String::from("d"), 4);
                v
            }),
            Box::new({
                let mut v = BTreeMap::new();
                v.insert(1, 1);
                v.insert(2, 2);
                v.insert(3, 3);
                v.insert(4, 4);
                v
            }),
            Box::new({
                let mut v: BTreeMap<String, Box<dyn Value>> = BTreeMap::new();
                v.insert(String::from("a"), Box::new(1));
                v.insert(String::from("b"), Box::new('a'));
                v
            }),
            Box::new({
                let v: Vec<Box<dyn Value>> = vec![
                    Box::new(1),
                    Box::new({
                        let mut v: BTreeMap<String, Box<dyn Value>> = BTreeMap::new();
                        v.insert(String::from("a"), Box::new(1));
                        v.insert(String::from("b"), Box::new('a'));
                        v
                    }),
                ];
                v
            }),
        ];

        macro_rules! check {
            ($build:expr, $value:expr) => {
                let r = OwnedStream::stream($build, &$value);

                if let Err(e) = &r {
                    // We only care about errors from the stream when a method isn't overriden
                    // If the stream intentionally returns unsupported then this condition isn't hit
                    if e.is_default_unsupported() {
                        let tokens = tokens(&$value);
                        panic!("value `{:?}` is unsupported (a method on `Stream` hasn't been overriden)", tokens);
                    }
                }

                check(r);
            };
        }

        // Check fmt separately for lifetime reasons
        check!(build(), format_args!("A {} value", "🍔∈🌏"));

        for value in values {
            check!(build(), value);
        }
    }
}

#[cfg(feature = "alloc")]
pub use self::alloc_support::*;
