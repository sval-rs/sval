use crate::{
    std::{
        fmt,
        string::{
            String,
            ToString,
        },
        vec::Vec,
    },
    stream,
    value,
};

use super::error::err;

use serde_lib::ser::{
    Error as SerError,
    Serialize,
    SerializeMap,
    SerializeSeq,
    Serializer,
};

pub(super) struct ToSerialize<V>(pub(super) V);

impl<V> Serialize for ToSerialize<V>
where
    V: value::Value,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut stream = Stream::begin(serializer);
        value::stream(&self.0, &mut stream).map_err(S::Error::custom)?;

        Ok(stream.expect_ok())
    }
}

impl<'a> Serialize for ToSerialize<value::collect::Value<'a>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut stream = Stream::begin(serializer);

        self.0.stream(&mut stream).map_err(S::Error::custom)?;

        Ok(stream.expect_ok())
    }
}

struct Stream<S>
where
    S: Serializer,
{
    ok: Option<S::Ok>,
    pos: Option<stream::Pos>,
    buffered: Option<TokenBuf>,
    current: Option<Current<S>>,
}

enum Current<S>
where
    S: Serializer,
{
    Serializer(S),
    SerializeSeq(S::SerializeSeq),
    SerializeMap(S::SerializeMap),
}

impl<S> Stream<S>
where
    S: Serializer,
{
    #[inline]
    fn begin(ser: S) -> Self {
        Stream {
            ok: None,
            pos: None,
            buffered: None,
            current: Some(Current::Serializer(ser)),
        }
    }

    #[inline]
    fn expect_ok(self) -> S::Ok {
        self.ok.expect("missing return value")
    }
}

impl<S> Current<S>
where
    S: Serializer,
{
    fn take_serializer(self) -> Result<S, stream::Error> {
        match self {
            Current::Serializer(ser) => Ok(ser),
            _ => Err(stream::Error::msg(
                "invalid serializer value (expected a serializer)",
            )),
        }
    }

    fn expect_serialize_seq(&mut self) -> Result<&mut S::SerializeSeq, stream::Error> {
        match self {
            Current::SerializeSeq(seq) => Ok(seq),
            _ => Err(stream::Error::msg(
                "invalid serializer value (expected a sequence)",
            )),
        }
    }

    fn take_serialize_seq(self) -> Result<S::SerializeSeq, stream::Error> {
        match self {
            Current::SerializeSeq(seq) => Ok(seq),
            _ => Err(stream::Error::msg(
                "invalid serializer value (expected a sequence)",
            )),
        }
    }

    fn expect_serialize_map(&mut self) -> Result<&mut S::SerializeMap, stream::Error> {
        match self {
            Current::SerializeMap(map) => Ok(map),
            _ => Err(stream::Error::msg(
                "invalid serializer value (expected a map)",
            )),
        }
    }

    fn take_serialize_map(self) -> Result<S::SerializeMap, stream::Error> {
        match self {
            Current::SerializeMap(map) => Ok(map),
            _ => Err(stream::Error::msg(
                "invalid serializer value (expected a map)",
            )),
        }
    }
}

impl<S> Stream<S>
where
    S: Serializer,
{
    /**
    Begin a buffer with the given token or push it if a buffer already exists.
    */
    fn buffer_begin(&mut self) -> &mut TokenBuf {
        match self.buffered {
            Some(ref mut buffered) => buffered,
            None => {
                self.buffered = Some(TokenBuf::new());
                self.buffered.as_mut().unwrap()
            }
        }
    }

    /**
    End a buffer by serializing its contents.
    */
    fn buffer_end(&mut self) -> Result<(), stream::Error> {
        if let Some(mut buffered) = self.buffered.take() {
            let r = self.serialize_any(&buffered);

            buffered.clear();
            self.buffered = Some(buffered);

            return r;
        }

        Ok(())
    }

    /**
    Get a reference to the buffer if it's active.
    */
    fn buffer(&mut self) -> Option<&mut TokenBuf> {
        match self.buffered {
            Some(ref mut buffered) if buffered.depth > 0 => Some(buffered),
            _ => None,
        }
    }

    fn take(&mut self) -> Result<Current<S>, stream::Error> {
        self.current
            .take()
            .ok_or_else(|| stream::Error::msg("attempt to use an invalid serializer"))
    }

    fn expect(&mut self) -> Result<&mut Current<S>, stream::Error> {
        self.current
            .as_mut()
            .ok_or_else(|| stream::Error::msg("attempt to use an invalid serializer"))
    }

    fn serialize_any(&mut self, v: impl Serialize) -> Result<(), stream::Error> {
        let pos = self.pos.take().unwrap_or_else(stream::Pos::root);

        if pos.is_key() {
            self.serialize_key(v)?;

            return Ok(());
        }

        if pos.is_value() {
            self.serialize_value(v)?;

            return Ok(());
        }

        if pos.is_elem() {
            self.serialize_elem(v)?;

            return Ok(());
        }

        self.serialize_primitive(v)
    }

    fn serialize_elem(&mut self, v: impl Serialize) -> Result<(), stream::Error> {
        self.expect()?
            .expect_serialize_seq()?
            .serialize_element(&v)
            .map_err(err("error serializing sequence element"))
    }

    fn serialize_key(&mut self, k: impl Serialize) -> Result<(), stream::Error> {
        self.expect()?
            .expect_serialize_map()?
            .serialize_key(&k)
            .map_err(err("error map serializing key"))
    }

    fn serialize_value(&mut self, v: impl Serialize) -> Result<(), stream::Error> {
        self.expect()?
            .expect_serialize_map()?
            .serialize_value(&v)
            .map_err(err("error map serializing value"))
    }

    fn serialize_primitive(&mut self, v: impl Serialize) -> Result<(), stream::Error> {
        let ser = self.take()?.take_serializer()?;

        self.ok = Some(
            v.serialize(ser)
                .map_err(err("error serializing primitive value"))?,
        );

        Ok(())
    }
}

impl<S> value::collect::Stream for Stream<S>
where
    S: Serializer,
{
    fn seq_elem_collect(&mut self, v: value::collect::Value) -> Result<(), stream::Error> {
        if let Some(buffered) = self.buffer() {
            v.stream(value::collect::Default(buffered))?;

            return Ok(());
        }

        self.serialize_elem(&ToSerialize(v))
    }

    fn map_key_collect(&mut self, k: value::collect::Value) -> Result<(), stream::Error> {
        if let Some(buffered) = self.buffer() {
            k.stream(value::collect::Default(buffered))?;

            return Ok(());
        }

        self.serialize_key(&ToSerialize(k))
    }

    fn map_value_collect(&mut self, v: value::collect::Value) -> Result<(), stream::Error> {
        if let Some(buffered) = self.buffer() {
            v.stream(value::collect::Default(buffered))?;

            return Ok(());
        }

        self.serialize_value(&ToSerialize(v))
    }
}

impl<S> stream::Stream for Stream<S>
where
    S: Serializer,
{
    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                match self.take()? {
                    Current::Serializer(ser) => {
                        let seq = ser.serialize_seq(len).map(Current::SerializeSeq)?;
                        self.current = Some(seq);
                    }
                    current => {
                        self.buffer_begin().seq_begin(len)?;

                        self.current = Some(current);
                    }
                }

                Ok(())
            }
            Some(buffered) => buffered.seq_begin(len),
        }
    }

    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                self.pos = Some(stream::Pos::elem());

                Ok(())
            }
            Some(buffered) => buffered.seq_elem(),
        }
    }

    fn seq_end(&mut self) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                let seq = self.take()?.take_serialize_seq()?;
                self.ok = Some(seq.end().map_err(err("error completing sequence"))?);

                Ok(())
            }
            Some(buffered) => {
                buffered.push(TokenKind::SeqEnd);

                if buffered.is_serializable() {
                    self.buffer_end()?;
                }

                Ok(())
            }
        }
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                match self.take()? {
                    Current::Serializer(ser) => {
                        let map = ser.serialize_map(len).map(Current::SerializeMap)?;
                        self.current = Some(map);
                    }
                    current => {
                        self.buffer_begin().map_begin(len)?;
                        self.current = Some(current);
                    }
                }

                Ok(())
            }
            Some(buffered) => buffered.map_begin(len),
        }
    }

    fn map_key(&mut self) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                self.pos = Some(stream::Pos::key());

                Ok(())
            }
            Some(buffered) => buffered.map_key(),
        }
    }

    fn map_value(&mut self) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                self.pos = Some(stream::Pos::value());

                Ok(())
            }
            Some(buffered) => buffered.map_value(),
        }
    }

    fn map_end(&mut self) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                let map = self.take()?.take_serialize_map()?;
                self.ok = Some(map.end().map_err(err("error completing map"))?);

                Ok(())
            }
            Some(buffered) => {
                buffered.push(TokenKind::MapEnd);

                if buffered.is_serializable() {
                    self.buffer_end()?;
                }

                Ok(())
            }
        }
    }

    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(v),
            Some(buffered) => buffered.i64(v),
        }
    }

    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(v),
            Some(buffered) => buffered.u64(v),
        }
    }

    fn i128(&mut self, v: i128) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(v),
            Some(buffered) => buffered.i128(v),
        }
    }

    fn u128(&mut self, v: u128) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(v),
            Some(buffered) => buffered.u128(v),
        }
    }

    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(v),
            Some(buffered) => buffered.f64(v),
        }
    }

    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(v),
            Some(buffered) => buffered.bool(v),
        }
    }

    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(v),
            Some(buffered) => buffered.char(v),
        }
    }

    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(v),
            Some(buffered) => buffered.str(v),
        }
    }

    fn none(&mut self) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(Option::None::<()>),
            Some(buffered) => buffered.none(),
        }
    }

    fn fmt(&mut self, v: fmt::Arguments) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_primitive(v),
            Some(buffered) => buffered.fmt(v),
        }
    }
}

/**
A mutable buffer for streaming tokens.

The `TokenBuf` can materialize maps and sequences that
don't have an original representation in memory.
*/
struct TokenBuf {
    depth: usize,
    tokens: Vec<Token>,
}

struct Token {
    depth: usize,
    kind: TokenKind,
}

#[derive(PartialEq)]
enum TokenKind {
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

impl TokenBuf {
    fn new() -> TokenBuf {
        TokenBuf {
            depth: 0,
            tokens: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.tokens.clear();
        self.depth = 0;
    }

    fn is_serializable(&self) -> bool {
        self.depth == 0
    }

    fn push(&mut self, kind: TokenKind) {
        match kind {
            TokenKind::MapBegin | TokenKind::SeqBegin => {
                self.tokens.push(Token {
                    depth: self.depth,
                    kind,
                });
                self.depth += 1;
            }
            TokenKind::MapEnd | TokenKind::SeqEnd => {
                self.depth -= 1;
                self.tokens.push(Token {
                    depth: self.depth,
                    kind,
                });
            }
            kind => {
                self.tokens.push(Token {
                    depth: self.depth,
                    kind,
                });
            }
        }
    }
}

impl stream::Stream for TokenBuf {
    fn fmt(&mut self, f: stream::Arguments) -> Result<(), stream::Error> {
        self.push(TokenKind::Str(f.to_string()));

        Ok(())
    }

    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        self.push(TokenKind::Signed(v));

        Ok(())
    }

    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        self.push(TokenKind::Unsigned(v));

        Ok(())
    }

    fn i128(&mut self, v: i128) -> Result<(), stream::Error> {
        self.push(TokenKind::BigSigned(v));

        Ok(())
    }

    fn u128(&mut self, v: u128) -> Result<(), stream::Error> {
        self.push(TokenKind::BigUnsigned(v));

        Ok(())
    }

    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        self.push(TokenKind::Float(v));

        Ok(())
    }

    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        self.push(TokenKind::Bool(v));

        Ok(())
    }

    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        self.push(TokenKind::Char(v));

        Ok(())
    }

    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        self.push(TokenKind::Str(v.to_string()));

        Ok(())
    }

    fn none(&mut self) -> Result<(), stream::Error> {
        self.push(TokenKind::None);

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        self.push(TokenKind::MapBegin);

        Ok(())
    }

    fn map_key(&mut self) -> Result<(), stream::Error> {
        self.push(TokenKind::MapKey);

        Ok(())
    }

    fn map_value(&mut self) -> Result<(), stream::Error> {
        self.push(TokenKind::MapValue);

        Ok(())
    }

    fn map_end(&mut self) -> Result<(), stream::Error> {
        self.push(TokenKind::MapEnd);

        Ok(())
    }

    fn seq_begin(&mut self, _: Option<usize>) -> Result<(), stream::Error> {
        self.push(TokenKind::SeqBegin);

        Ok(())
    }

    fn seq_elem(&mut self) -> Result<(), stream::Error> {
        self.push(TokenKind::SeqElem);

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), stream::Error> {
        self.push(TokenKind::SeqEnd);

        Ok(())
    }
}

struct TokensRef<'a>(&'a [Token]);

struct TokensRefReader<'a> {
    idx: usize,
    tokens: &'a [Token],
}

impl<'a> TokensRef<'a> {
    fn reader(&self) -> TokensRefReader<'a> {
        TokensRefReader {
            idx: 0,
            tokens: self.0,
        }
    }
}

impl<'a> TokensRefReader<'a> {
    fn has_more(&self) -> bool {
        self.idx < self.tokens.len()
    }

    fn next_serializable(&mut self, depth: usize) -> TokensRef<'a> {
        let end = self.tokens[self.idx..]
            .iter()
            .enumerate()
            .take_while(|(idx, t)| *idx == 0 || t.depth > depth)
            .last()
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| self.tokens.len());

        self.idx = end;

        TokensRef(&self.tokens[self.idx..end])
    }

    fn expect(&mut self, token: TokenKind) -> Result<&Token, stream::Error> {
        self.next()
            .filter(|t| t.kind == token)
            .ok_or_else(|| stream::Error::msg("missing an expected token"))
    }

    fn expect_empty(&self) -> Result<(), stream::Error> {
        if self.has_more() {
            Err(stream::Error::msg("unexpected trailing tokens"))
        } else {
            Ok(())
        }
    }
}

impl<'a> Iterator for TokensRefReader<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;

        self.tokens.get(idx)
    }
}

impl Serialize for TokenBuf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        TokensRef(&self.tokens[..]).serialize(serializer)
    }
}

impl<'a> Serialize for TokensRef<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut reader = self.reader();

        match reader.next() {
            Some(token) => match token.kind {
                TokenKind::Signed(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                TokenKind::Unsigned(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                TokenKind::BigSigned(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                TokenKind::BigUnsigned(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                TokenKind::Float(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                TokenKind::Bool(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                TokenKind::Char(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                TokenKind::Str(ref v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                TokenKind::None => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    serializer.serialize_none()
                }
                TokenKind::MapBegin => {
                    let mut map = serializer.serialize_map(None)?;

                    while reader.has_more() {
                        let key = reader
                            .expect(TokenKind::MapKey)
                            .map_err(S::Error::custom)?
                            .depth;
                        map.serialize_key(&reader.next_serializable(key))?;

                        let value = reader
                            .expect(TokenKind::MapValue)
                            .map_err(S::Error::custom)?
                            .depth;
                        map.serialize_value(&reader.next_serializable(value))?;
                    }

                    reader.expect(TokenKind::MapEnd).map_err(S::Error::custom)?;
                    reader.expect_empty().map_err(S::Error::custom)?;

                    map.end()
                }
                TokenKind::SeqBegin => {
                    let mut seq = serializer.serialize_seq(None)?;

                    while reader.has_more() {
                        let elem = reader
                            .expect(TokenKind::SeqElem)
                            .map_err(S::Error::custom)?
                            .depth;
                        seq.serialize_element(&reader.next_serializable(elem))?;
                    }

                    reader.expect(TokenKind::SeqEnd).map_err(S::Error::custom)?;
                    reader.expect_empty().map_err(S::Error::custom)?;

                    seq.end()
                }
                _ => Err(S::Error::custom("unexpected token value")),
            },
            None => serializer.serialize_none(),
        }
    }
}
