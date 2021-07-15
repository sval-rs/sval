mod error;
mod str;
mod tag;

pub use self::{
    error::OwnedSource,
    str::OwnedStr,
    tag::OwnedTag,
};

pub struct OwnedStream {
    depth: usize,
    tokens: Vec<Token>,
}

impl OwnedStream {
    pub fn new() -> OwnedStream {
        OwnedStream {
            depth: 0,
            tokens: Vec::new(),
        }
    }

    pub fn collect(v: impl Value) -> Result<Self, crate::Error> {
        let mut buf = OwnedStream::new();
        crate::stream_owned(&mut buf, &v)?;

        Ok(buf.tokens)
    }

    pub fn push(&mut self, kind: TokenKind) {
        self.tokens.push(Token {
            depth: self.depth,
            kind,
        });
    }

    pub fn clear(&mut self) {
        self.tokens.clear();
        self.depth = 0;
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn is_streamable(&self) -> bool {
        self.depth == 0
    }

    pub fn into_tokens(self) -> Vec<Token> {
        self.tokens
    }
}

impl<'v> Stream<'v> for OwnedStream {
    fn fmt(&mut self, f: stream::Arguments) -> stream::Result {
        self.push(TokenKind::Str(StringContainer::from(f.to_string())));

        Ok(())
    }

    fn fmt_borrowed(&mut self, f: stream::Arguments<'v>) -> stream::Result {
        self.fmt(f)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.push(TokenKind::Error(OwnedContainer::from(OwnedSource::from(v))));

        Ok(())
    }

    fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
        self.error(v)
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.push(TokenKind::Signed(v));

        Ok(())
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.push(TokenKind::Unsigned(v));

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.push(TokenKind::BigSigned(v));

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.push(TokenKind::BigUnsigned(v));

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.push(TokenKind::Float(v));

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.push(TokenKind::Bool(v));

        Ok(())
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.push(TokenKind::Char(v));

        Ok(())
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.push(TokenKind::Str(StringContainer::from(v)));

        Ok(())
    }

    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.str(v)
    }

    fn none(&mut self) -> stream::Result {
        self.push(TokenKind::None);

        Ok(())
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        self.depth += 1;

        self.push(TokenKind::MapBegin(len));

        Ok(())
    }

    fn map_key(&mut self) -> stream::Result {
        self.push(TokenKind::MapKey);

        Ok(())
    }

    fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
        self.map_key()?;
        k.stream(self)
    }

    fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
        self.map_key_collect(k)
    }

    fn map_value(&mut self) -> stream::Result {
        self.push(TokenKind::MapValue);

        Ok(())
    }

    fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
        self.map_value()?;
        v.stream(self)
    }

    fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.map_value_collect(v)
    }

    fn map_end(&mut self) -> stream::Result {
        self.push(TokenKind::MapEnd);

        self.depth -= 1;

        Ok(())
    }

    fn seq_begin(&mut self, len: Option(usize)) -> stream::Result {
        self.depth += 1;

        self.push(TokenKind::SeqBegin(len));

        Ok(())
    }

    fn seq_elem(&mut self) -> stream::Result {
        self.push(TokenKind::SeqElem);

        Ok(())
    }

    fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
        self.seq_elem()?;
        v.stream(self)
    }

    fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.seq_elem_collect(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        self.push(TokenKind::SeqEnd);

        self.depth -= 1;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Token {
    depth: usize,
    kind: TokenKind,
}

// Embedded within a `Token`, which will be shared
#[derive(Clone)]
#[non_exhaustive]
pub enum TokenKind {
    MapBegin(Option<usize>),
    TaggedMapBegin(OwnedTag, Option<usize>),
    MapKey,
    MapValue,
    MapEnd,
    TaggedMapEnd,
    SeqBegin(Option<usize>),
    TaggedSeqBegin(OwnedTag, Option<usize>),
    SeqElem,
    SeqEnd,
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    BigSigned(i128),
    BigUnsigned(u128),
    Bool(bool),
    Str(OwnedStr),
    Char(char),
    Error(OwnedSource),
    Tag(OwnedTag),
    None,
}

impl Token {
    fn stream_owned(&self, mut stream: value::Stream) -> value::Result {
        use self::TokenKind::*;

        match self.kind {
            Signed(v) => stream.i64(v)?,
            Unsigned(v) => stream.u64(v)?,
            Float(v) => stream.f64(v)?,
            BigSigned(v) => stream.i128(v)?,
            BigUnsigned(v) => stream.u128(v)?,
            Bool(v) => stream.bool(v)?,
            Str(ref v) => stream.owned().str(&*v)?,
            Char(v) => stream.char(v)?,
            Error(ref v) => stream::Source::from(&**v).stream(stream.owned())?,
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

        Ok(())
    }
}
