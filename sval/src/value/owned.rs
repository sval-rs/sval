/*!
Owned values.
*/

use crate::{
    std::{
        string::{
            String,
            ToString,
        },
        sync::Arc,
        vec::Vec,
    },
    stream::{
        self,
        Stream,
    },
    value::{
        self,
        Error,
    },
};

/**
An owned value.
*/
pub struct Value(ValueInner);

impl Value {
    pub fn new(v: impl value::Value) -> Self {
        let mut buf = Buf::new();

        match crate::stream(v, &mut buf) {
            Ok(()) => Value(ValueInner::Stream(buf.tokens)),
            Err(error) => Value(ValueInner::Error(error)),
        }
    }

    pub fn shared(v: impl Into<Arc<dyn value::Value + Send + Sync>>) -> Self {
        Value(ValueInner::Shared(v.into()))
    }
}

enum ValueInner {
    Error(value::Error),
    Shared(Arc<dyn value::Value + Send + Sync>),
    Stream(Vec<Token>),
}

impl value::Value for Value {
    fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
        use self::Kind::*;

        match self.0 {
            ValueInner::Error(ref e) => Err(Error::custom(e)),
            ValueInner::Shared(ref v) => v.stream(stream),
            ValueInner::Stream(ref v) => {
                for token in v {
                    match token.kind {
                        Signed(v) => stream.i64(v)?,
                        Unsigned(v) => stream.u64(v)?,
                        Float(v) => stream.f64(v)?,
                        BigSigned(v) => stream.i128(v)?,
                        BigUnsigned(v) => stream.u128(v)?,
                        Bool(v) => stream.bool(v)?,
                        Str(ref v) => stream.str(&*v)?,
                        Char(v) => stream.char(v)?,
                        None => stream.none()?,
                        MapBegin => stream.map_begin(Option::None)?,
                        MapKey => {
                            stream.map_key_begin()?;
                        }
                        MapValue => {
                            stream.map_value_begin()?;
                        }
                        MapEnd => stream.map_end()?,
                        SeqBegin => stream.seq_begin(Option::None)?,
                        SeqElem => {
                            stream.seq_elem_begin()?;
                        }
                        SeqEnd => stream.seq_end()?,
                    }
                }

                Ok(())
            }
        }
    }
}

pub(crate) struct Buf {
    depth: usize,
    tokens: Vec<Token>,
}

pub(crate) struct Token {
    #[cfg(feature = "serde")]
    depth: usize,
    kind: Kind,
}

#[derive(PartialEq)]
pub(crate) enum Kind {
    MapBegin,
    MapKey,
    MapValue,
    MapEnd,
    SeqBegin,
    SeqElem,
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

impl Buf {
    pub(crate) fn new() -> Buf {
        Buf {
            depth: 0,
            tokens: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, kind: Kind) {
        match kind {
            Kind::MapBegin | Kind::SeqBegin => {
                self.tokens.push(Token {
                    #[cfg(feature = "serde")]
                    depth: self.depth,
                    kind,
                });
                self.depth += 1;
            }
            Kind::MapEnd | Kind::SeqEnd => {
                self.depth -= 1;
                self.tokens.push(Token {
                    #[cfg(feature = "serde")]
                    depth: self.depth,
                    kind,
                });
            }
            kind => {
                self.tokens.push(Token {
                    #[cfg(feature = "serde")]
                    depth: self.depth,
                    kind,
                });
            }
        }
    }
}

impl Stream for Buf {
    fn fmt(&mut self, f: stream::Arguments) -> Result<(), stream::Error> {
        self.push(Kind::Str(f.to_string()));

        Ok(())
    }

    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        self.push(Kind::Signed(v));

        Ok(())
    }

    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        self.push(Kind::Unsigned(v));

        Ok(())
    }

    fn i128(&mut self, v: i128) -> Result<(), stream::Error> {
        self.push(Kind::BigSigned(v));

        Ok(())
    }

    fn u128(&mut self, v: u128) -> Result<(), stream::Error> {
        self.push(Kind::BigUnsigned(v));

        Ok(())
    }

    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        self.push(Kind::Float(v));

        Ok(())
    }

    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        self.push(Kind::Bool(v));

        Ok(())
    }

    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        self.push(Kind::Char(v));

        Ok(())
    }

    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        self.push(Kind::Str(v.to_string()));

        Ok(())
    }

    fn none(&mut self) -> Result<(), stream::Error> {
        self.push(Kind::None);

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        self.push(Kind::MapBegin);

        Ok(())
    }

    fn map_key(&mut self) -> Result<(), stream::Error> {
        self.push(Kind::MapKey);

        Ok(())
    }

    fn map_value(&mut self) -> Result<(), stream::Error> {
        self.push(Kind::MapValue);

        Ok(())
    }

    fn map_end(&mut self) -> Result<(), stream::Error> {
        self.push(Kind::MapEnd);

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        self.push(Kind::SeqBegin);

        Ok(())
    }

    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        self.push(Kind::SeqElem);

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), stream::Error> {
        self.push(Kind::SeqEnd);

        Ok(())
    }
}

#[cfg(feature = "serde")]
impl Token {
    pub(crate) fn depth(&self) -> usize {
        self.depth
    }

    pub(crate) fn kind(&self) -> &Kind {
        &self.kind
    }
}

#[cfg(feature = "serde")]
impl Buf {
    pub(crate) fn clear(&mut self) {
        self.tokens.clear();
        self.depth = 0;
    }

    pub(crate) fn depth(&self) -> usize {
        self.depth
    }

    pub(crate) fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}
