use crate::{
    std::{
        collections::VecDeque,
        fmt,
        string::{
            String,
            ToString,
        },
    },
    stream,
    value,
};

use super::error::err;

use serde_lib::ser::{
    self,
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
    buffered: Option<Tokens>,
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
    fn begin(ser: S) -> Self {
        Stream {
            ok: None,
            buffered: None,
            current: Some(Current::Serializer(ser)),
        }
    }

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
    fn buffer_begin(&mut self, token: Token) {
        match self.buffered {
            Some(ref mut buffered) => {
                buffered.push(token);
            }
            None => {
                let mut tokens = Tokens::new();
                tokens.push(token);

                self.buffered = Some(Tokens::new());
            }
        }
    }

    fn buffer_end(&mut self, token: Token) -> Option<Tokens> {
        match self.buffered {
            Some(ref mut buffered) => {
                buffered.push(token);

                if buffered.depth == 0 {
                    self.buffered.take()
                } else {
                    None
                }
            }
            None => None,
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

    fn primitive(&mut self, v: impl Serialize) -> Result<(), stream::Error> {
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
        self.expect()?
            .expect_serialize_seq()?
            .serialize_element(&ToSerialize(v))
            .map_err(err("error serializing sequence element"))?;

        Ok(())
    }

    fn map_key_collect(&mut self, k: value::collect::Value) -> Result<(), stream::Error> {
        self.expect()?
            .expect_serialize_map()?
            .serialize_key(&ToSerialize(k))
            .map_err(err("error map serializing key"))?;

        Ok(())
    }

    fn map_value_collect(&mut self, v: value::collect::Value) -> Result<(), stream::Error> {
        self.expect()?
            .expect_serialize_map()?
            .serialize_value(&ToSerialize(v))
            .map_err(err("error serializing map value"))?;

        Ok(())
    }
}

impl<S> stream::Stream for Stream<S>
where
    S: Serializer,
{
    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        match self.take()? {
            Current::Serializer(ser) => {
                let seq = ser.serialize_seq(len).map(Current::SerializeSeq)?;
                self.current = Some(seq);
            }
            current => {
                self.buffer_begin(Token::SeqBegin);
                self.current = Some(current);
            }
        }

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), stream::Error> {
        match self.buffer_end(Token::SeqEnd) {
            Some(tokens) => unimplemented!("serialize the tokens"),
            None => {
                let seq = self.take()?.take_serialize_seq()?;
                self.ok = Some(seq.end().map_err(err("error completing sequence"))?);

                Ok(())
            }
        }
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result<(), stream::Error> {
        match self.take()? {
            Current::Serializer(ser) => {
                let map = ser.serialize_map(len).map(Current::SerializeMap)?;
                self.current = Some(map);
            }
            current => {
                self.buffer_begin(Token::MapBegin);
                self.current = Some(current);
            }
        }

        Ok(())
    }

    fn map_end(&mut self) -> Result<(), stream::Error> {
        match self.buffer_end(Token::MapEnd) {
            Some(tokens) => unimplemented!("serialize the tokens"),
            None => {
                let map = self.take()?.take_serialize_map()?;
                self.ok = Some(map.end().map_err(err("error completing map"))?);

                Ok(())
            }
        }
    }

    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::Signed(v));

            return Ok(());
        }

        self.primitive(v)
    }

    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::Unsigned(v));

            return Ok(());
        }

        self.primitive(v)
    }

    fn i128(&mut self, v: i128) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::BigSigned(v));

            return Ok(());
        }

        self.primitive(v)
    }

    fn u128(&mut self, v: u128) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::BigUnsigned(v));

            return Ok(());
        }

        self.primitive(v)
    }

    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::Float(v));

            return Ok(());
        }

        self.primitive(v)
    }

    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::Bool(v));

            return Ok(());
        }

        self.primitive(v)
    }

    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::Char(v));

            return Ok(());
        }

        self.primitive(v)
    }

    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::Str(v.to_string()));

            return Ok(());
        }

        self.primitive(v)
    }

    fn none(&mut self) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::None);

            return Ok(());
        }

        self.primitive(Option::None::<()>)
    }

    fn fmt(&mut self, v: fmt::Arguments) -> Result<(), stream::Error> {
        if let Some(buffered) = &mut self.buffered {
            buffered.push(Token::Str(v.to_string()));

            return Ok(());
        }

        self.primitive(v)
    }
}

struct Tokens {
    depth: usize,
    tokens: VecDeque<Token>,
}

enum Token {
    MapBegin,
    MapEnd,
    SeqBegin,
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

impl Tokens {
    fn new() -> Tokens {
        Tokens {
            depth: 0,
            tokens: VecDeque::new(),
        }
    }

    fn push(&mut self, token: Token) {
        match token {
            Token::MapBegin | Token::SeqBegin => {
                self.depth += 1;
            }
            Token::MapEnd | Token::SeqEnd => {
                self.depth -= 1;
            }
            _ => (),
        }

        self.tokens.push_back(token);
    }
}
