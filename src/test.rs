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
            fmt,
            string::String,
            vec::Vec,
        },
        stream::{
            self,
            OwnedStream,
            Stream,
        },
        value::{
            owned::{
                TokenKind,
                OwnedSource,
            },
            OwnedValue,
            Value,
        },
    };

    /**
    The kind of token being produced.
    */
    #[non_exhaustive]
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
        Error(Source),
        None,
    }

    /**
    An error source.
    */
    #[derive(Clone, PartialEq)]
    pub struct Source(OwnedSource);

    impl fmt::Debug for Source {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    impl fmt::Display for Source {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
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
                TokenKind::MapBegin(len) => Some(Token::MapBegin(len)),
                TokenKind::MapEnd => Some(Token::MapEnd),
                TokenKind::SeqBegin(len) => Some(Token::SeqBegin(len)),
                TokenKind::SeqEnd => Some(Token::SeqEnd),
                TokenKind::Signed(v) => Some(Token::Signed(v)),
                TokenKind::Unsigned(v) => Some(Token::Unsigned(v)),
                TokenKind::BigSigned(v) => Some(Token::BigSigned(v)),
                TokenKind::BigUnsigned(v) => Some(Token::BigUnsigned(v)),
                TokenKind::Float(v) => Some(Token::Float(v)),
                TokenKind::Bool(v) => Some(Token::Bool(v)),
                TokenKind::Char(v) => Some(Token::Char(v)),
                TokenKind::Str(ref v) => Some(Token::Str((**v).into())),
                TokenKind::None => Some(Token::None),
                TokenKind::Error(ref err) => Some(Token::Error(Source((**err).clone()))),
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
    pub fn stream_exhaustive<S>(build: impl Fn() -> S, check: impl Fn(Result<S, crate::Error>))
    where
        S: Stream,
    {
        use crate::std::{
            boxed::Box,
            collections::BTreeMap,
        };

        let source = {
            #[cfg(not(feature = "std"))]
            {
                Source(OwnedSource::empty())
            }

            #[cfg(feature = "std")]
            {
                use crate::std::io;

                Source::new(&io::Error::from(io::ErrorKind::Other))
            }
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
            Box::new(stream::Source::from(&source.0)),
        ];

        macro_rules! check {
            ($build:expr, $value:expr) => {
                let r = OwnedStream::stream($build, &$value);

                if let Err(e) = &r {
                    // We only care about errors from the stream when a method isn't overridden
                    // If the stream intentionally returns unsupported then this condition isn't hit
                    if e.is_default_unsupported() {
                        let tokens = tokens(&$value);
                        panic!("value `{:?}` is unsupported (a method on `Stream` hasn't been overridden)", tokens);
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

    #[cfg(feature = "std")]
    mod std_support {
        use super::*;

        use crate::std::error::Error;

        impl Source {
            /**
            Create a new test source.
            */
            pub fn new(err: &dyn Error) -> Self {
                Source(OwnedSource::collect(err))
            }
        }
    }
}

#[cfg(feature = "alloc")]
pub use self::alloc_support::*;
