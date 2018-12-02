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
        Value,
    },
};

/**
An owned value.

Owned values are safe to share and are cheap to clone.
*/
pub struct OwnedValue(ValueInner);

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    MapBegin(Option<usize>),
    MapKey,
    MapValue,
    MapEnd,
    SeqBegin(Option<usize>),
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

impl OwnedValue {
    pub fn from_value(v: impl Value) -> Self {
        let mut buf = Buf::new();

        match crate::stream(v, &mut buf) {
            Ok(()) => OwnedValue(ValueInner::Stream(buf.tokens.into())),
            Err(error) => OwnedValue(ValueInner::Error(Arc::new(error))),
        }
    }

    pub fn from_shared(v: impl Into<Arc<dyn Value + Send + Sync>>) -> Self {
        OwnedValue(ValueInner::Shared(v.into()))
    }
}

enum ValueInner {
    Error(Arc<value::Error>),
    Shared(Arc<dyn Value + Send + Sync>),
    Stream(Arc<[Token]>),
}

impl Value for OwnedValue {
    fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
        use self::Kind::*;

        match self.0 {
            ValueInner::Error(ref e) => Err(Error::custom(e)),
            ValueInner::Shared(ref v) => v.stream(stream),
            ValueInner::Stream(ref v) => {
                for token in v.iter() {
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
                        MapBegin(len) => stream.map_begin(len)?,
                        MapKey => {
                            stream.map_key_begin()?;
                        }
                        MapValue => {
                            stream.map_value_begin()?;
                        }
                        MapEnd => stream.map_end()?,
                        SeqBegin(len) => stream.seq_begin(len)?,
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
    stack: stream::Stack,
    tokens: Vec<Token>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Token {
    #[cfg(feature = "serde")]
    depth: usize,
    kind: Kind,
}

impl Buf {
    pub(crate) fn new() -> Buf {
        Buf {
            stack: stream::Stack::new(),
            tokens: Vec::new(),
        }
    }

    fn push(&mut self, kind: Kind, depth: usize) {
        match kind {
            Kind::MapBegin(_) | Kind::SeqBegin(_) => {
                self.tokens.push(Token {
                    depth: depth,
                    kind,
                });
            }
            Kind::MapEnd | Kind::SeqEnd => {
                self.tokens.push(Token {
                    depth: depth,
                    kind,
                });
            }
            kind => {
                self.tokens.push(Token {
                    depth: depth,
                    kind,
                });
            }
        }
    }

    pub(crate) fn depth(&self) -> usize {
        self.stack.depth()
    }
}

impl Stream for Buf {
    fn fmt(&mut self, f: stream::Arguments) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::Str(f.to_string()), depth);

        Ok(())
    }

    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::Signed(v), depth);

        Ok(())
    }

    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::Unsigned(v), depth);

        Ok(())
    }

    fn i128(&mut self, v: i128) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::BigSigned(v), depth);

        Ok(())
    }

    fn u128(&mut self, v: u128) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::BigUnsigned(v), depth);

        Ok(())
    }

    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::Float(v), depth);

        Ok(())
    }

    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::Bool(v), depth);

        Ok(())
    }

    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::Char(v), depth);

        Ok(())
    }

    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::Str(v.to_string()), depth);

        Ok(())
    }

    fn none(&mut self) -> Result<(), stream::Error> {
        self.stack.primitive()?;
        let depth = self.depth();

        self.push(Kind::None, depth);

        Ok(())
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        self.stack.map_begin()?;
        let depth = self.depth();

        self.push(Kind::MapBegin(len), depth);

        Ok(())
    }

    fn map_key(&mut self) -> Result<(), stream::Error> {
        self.stack.map_key()?;
        let depth = self.depth();

        self.push(Kind::MapKey, depth);

        Ok(())
    }

    fn map_value(&mut self) -> Result<(), stream::Error> {
        self.stack.map_value()?;
        let depth = self.depth();

        self.push(Kind::MapValue, depth);

        Ok(())
    }

    fn map_end(&mut self) -> Result<(), stream::Error> {
        let depth = self.depth();
        self.stack.map_end()?;

        self.push(Kind::MapEnd, depth);

        Ok(())
    }

    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        self.stack.seq_begin()?;
        let depth = self.depth();

        self.push(Kind::SeqBegin(len), depth);

        Ok(())
    }

    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        self.stack.seq_elem()?;
        let depth = self.depth();

        self.push(Kind::SeqElem, depth);

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), stream::Error> {
        let depth = self.depth();
        self.stack.seq_end()?;

        self.push(Kind::SeqEnd, depth);

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
        self.stack.clear();
    }

    pub(crate) fn tokens(&self) -> &[Token] {
        &self.tokens
    }
}

#[cfg(any(test, feature = "test"))]
pub fn tokens(v: impl Value) -> Vec<Kind> {
    match OwnedValue::from_value(v).0 {
        ValueInner::Stream(tokens) => tokens.into_iter().map(|t| t.kind.clone()).collect(),
        ValueInner::Error(err) => Err(err).unwrap(),
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::{self, Token};

    struct Map;

    impl Value for Map {
        fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
            stream.map_begin(Some(2))?;

            stream.map_key(1)?;
            stream.map_value(11)?;

            stream.map_key(2)?;
            stream.map_value(22)?;

            stream.map_end()
        }
    }

    struct Seq;

    impl Value for Seq {
        fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
            stream.seq_begin(Some(2))?;

            stream.seq_elem(1)?;
            stream.seq_elem(2)?;

            stream.seq_end()
        }
    }

    #[test]
    fn owned_value_is_send_sync() {
        fn is_send_sync<T: Send + Sync>() {}

        is_send_sync::<OwnedValue>();
    }

    #[test]
    fn owned_primitive() {
        assert_eq!(
            vec![Token::Str("a format 1".into())],
            test::tokens(format_args!("a format {}", 1))
        );

        assert_eq!(vec![Token::Str("a string".into())], test::tokens("a string"));

        assert_eq!(vec![Token::Unsigned(42u64)], test::tokens(42u64));

        assert_eq!(vec![Token::Signed(42i64)], test::tokens(42i64));

        assert_eq!(vec![Token::BigUnsigned(42u128)], test::tokens(42u128));

        assert_eq!(vec![Token::BigSigned(42i128)], test::tokens(42i128));

        assert_eq!(vec![Token::Float(42f64)], test::tokens(42f64));

        assert_eq!(vec![Token::Bool(true)], test::tokens(true));

        assert_eq!(vec![Token::Char('a')], test::tokens('a'));

        assert_eq!(vec![Token::None], test::tokens(Option::None::<()>));
    }

    #[test]
    fn owned_map() {
        let v = test::tokens(Map);

        assert_eq!(
            vec![
                Token::MapBegin(Some(2)),
                Token::MapKey,
                Token::Signed(1),
                Token::MapValue,
                Token::Signed(11),
                Token::MapKey,
                Token::Signed(2),
                Token::MapValue,
                Token::Signed(22),
                Token::MapEnd,
            ],
            v
        );
    }

    #[test]
    fn owned_seq() {
        let v = test::tokens(Seq);

        assert_eq!(
            vec![
                Token::SeqBegin(Some(2)),
                Token::SeqElem,
                Token::Signed(1),
                Token::SeqElem,
                Token::Signed(2),
                Token::SeqEnd,
            ],
            v
        );
    }
}
