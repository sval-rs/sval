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
*/
pub struct OwnedValue(ValueInner);

impl OwnedValue {
    pub fn collect(v: impl Value) -> Self {
        let mut buf = Buf::new();

        match crate::stream(v, &mut buf) {
            Ok(()) => OwnedValue(ValueInner::Stream(buf.tokens)),
            Err(error) => OwnedValue(ValueInner::Error(error)),
        }
    }

    pub fn shared(v: impl Into<Arc<dyn Value + Send + Sync>>) -> Self {
        OwnedValue(ValueInner::Shared(v.into()))
    }
}

enum ValueInner {
    Error(value::Error),
    Shared(Arc<dyn Value + Send + Sync>),
    Stream(Vec<Token>),
}

impl Value for OwnedValue {
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

    fn to_owned(&self) -> OwnedValue {
        match self.0 {
            ValueInner::Error(ref e) => OwnedValue(ValueInner::Error(Error::custom(e))),
            ValueInner::Shared(ref v) => OwnedValue(ValueInner::Shared(v.clone())),
            ValueInner::Stream(ref v) => OwnedValue(ValueInner::Stream((*v).clone())),
        }
    }
}

pub(crate) struct Buf {
    depth: usize,
    tokens: Vec<Token>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Token {
    #[cfg(feature = "serde")]
    depth: usize,
    kind: Kind,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Kind {
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

impl Buf {
    pub(crate) fn new() -> Buf {
        Buf {
            depth: 0,
            tokens: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, kind: Kind) {
        match kind {
            Kind::MapBegin(_) | Kind::SeqBegin(_) => {
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

    fn map_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        self.push(Kind::MapBegin(len));

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

    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        self.push(Kind::SeqBegin(len));

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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test;

    impl OwnedValue {
        pub(crate) fn into_tokens(self) -> Vec<Kind> {
            match self.0 {
                ValueInner::Stream(tokens) => tokens.into_iter().map(|t| t.kind).collect(),
                ValueInner::Error(err) => Err(err).unwrap(),
                _ => vec![],
            }
        }
    }

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
            vec![Kind::Str("a format 1".into())],
            test::tokens(format_args!("a format {}", 1))
        );

        assert_eq!(vec![Kind::Str("a string".into())], test::tokens("a string"));

        assert_eq!(vec![Kind::Unsigned(42u64)], test::tokens(42u64));

        assert_eq!(vec![Kind::Signed(42i64)], test::tokens(42i64));

        assert_eq!(vec![Kind::BigUnsigned(42u128)], test::tokens(42u128));

        assert_eq!(vec![Kind::BigSigned(42i128)], test::tokens(42i128));

        assert_eq!(vec![Kind::Float(42f64)], test::tokens(42f64));

        assert_eq!(vec![Kind::Bool(true)], test::tokens(true));

        assert_eq!(vec![Kind::Char('a')], test::tokens('a'));

        assert_eq!(vec![Kind::None], test::tokens(Option::None::<()>));
    }

    #[test]
    fn owned_map() {
        let v = test::tokens(Map);

        assert_eq!(
            vec![
                Kind::MapBegin(Some(2)),
                Kind::MapKey,
                Kind::Signed(1),
                Kind::MapValue,
                Kind::Signed(11),
                Kind::MapKey,
                Kind::Signed(2),
                Kind::MapValue,
                Kind::Signed(22),
                Kind::MapEnd,
            ],
            v
        );
    }

    #[test]
    fn owned_seq() {
        let v = test::tokens(Seq);

        assert_eq!(
            vec![
                Kind::SeqBegin(Some(2)),
                Kind::SeqElem,
                Kind::Signed(1),
                Kind::SeqElem,
                Kind::Signed(2),
                Kind::SeqEnd,
            ],
            v
        );
    }
}
