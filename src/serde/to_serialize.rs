use crate::{
    std::fmt,
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
    buffered: Option<value::owned::Buf>,
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
    fn buffer_begin(&mut self) -> &mut value::owned::Buf {
        match self.buffered {
            Some(ref mut buffered) => buffered,
            None => {
                self.buffered = Some(value::owned::Buf::new());
                self.buffered.as_mut().unwrap()
            }
        }
    }

    /**
    End a buffer by serializing its contents.
    */
    fn buffer_end(&mut self) -> Result<(), stream::Error> {
        if let Some(mut buffered) = self.buffered.take() {
            let r = self.serialize_any(Tokens(buffered.tokens()));

            buffered.clear();
            self.buffered = Some(buffered);

            return r;
        }

        Ok(())
    }

    /**
    Get a reference to the buffer if it's active.
    */
    fn buffer(&mut self) -> Option<&mut value::owned::Buf> {
        match self.buffered {
            Some(ref mut buffered) if buffered.depth() > 0 => Some(buffered),
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
                buffered.push(value::owned::Kind::SeqEnd);

                if buffered.depth() == 0 {
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
                buffered.push(value::owned::Kind::MapEnd);

                if buffered.depth() == 0 {
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

struct Tokens<'a>(&'a [value::owned::Token]);

struct TokensReader<'a> {
    idx: usize,
    tokens: &'a [value::owned::Token],
}

impl<'a> Tokens<'a> {
    fn reader(&self) -> TokensReader<'a> {
        TokensReader {
            idx: 0,
            tokens: self.0,
        }
    }
}

impl<'a> TokensReader<'a> {
    fn has_more(&self) -> bool {
        self.idx < self.tokens.len()
    }

    fn next_serializable(&mut self, depth: usize) -> Tokens<'a> {
        let end = self.tokens[self.idx..]
            .iter()
            .enumerate()
            .take_while(|(idx, t)| *idx == 0 || t.depth() > depth)
            .last()
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| self.tokens.len());

        self.idx = end;

        Tokens(&self.tokens[self.idx..end])
    }

    fn expect(&mut self, token: value::owned::Kind) -> Result<&value::owned::Token, stream::Error> {
        self.next()
            .filter(|t| *t.kind() == token)
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

impl<'a> Iterator for TokensReader<'a> {
    type Item = &'a value::owned::Token;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;

        self.tokens.get(idx)
    }
}

impl<'a> Serialize for Tokens<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use self::value::owned::Kind;

        let mut reader = self.reader();

        match reader.next() {
            Some(token) => match token.kind() {
                Kind::Signed(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                Kind::Unsigned(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                Kind::BigSigned(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                Kind::BigUnsigned(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                Kind::Float(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                Kind::Bool(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                Kind::Char(v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                Kind::Str(ref v) => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    v.serialize(serializer)
                }
                Kind::None => {
                    reader.expect_empty().map_err(S::Error::custom)?;

                    serializer.serialize_none()
                }
                Kind::MapBegin(len) => {
                    let mut map = serializer.serialize_map(*len)?;

                    while reader.has_more() {
                        let key = reader
                            .expect(Kind::MapKey)
                            .map_err(S::Error::custom)?
                            .depth();
                        map.serialize_key(&reader.next_serializable(key))?;

                        let value = reader
                            .expect(Kind::MapValue)
                            .map_err(S::Error::custom)?
                            .depth();
                        map.serialize_value(&reader.next_serializable(value))?;
                    }

                    reader.expect(Kind::MapEnd).map_err(S::Error::custom)?;
                    reader.expect_empty().map_err(S::Error::custom)?;

                    map.end()
                }
                Kind::SeqBegin(len) => {
                    let mut seq = serializer.serialize_seq(*len)?;

                    while reader.has_more() {
                        let elem = reader
                            .expect(Kind::SeqElem)
                            .map_err(S::Error::custom)?
                            .depth();
                        seq.serialize_element(&reader.next_serializable(elem))?;
                    }

                    reader.expect(Kind::SeqEnd).map_err(S::Error::custom)?;
                    reader.expect_empty().map_err(S::Error::custom)?;

                    seq.end()
                }
                _ => Err(S::Error::custom("unexpected token value")),
            },
            None => serializer.serialize_none(),
        }
    }
}
