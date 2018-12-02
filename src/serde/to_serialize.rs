use crate::{
    std::fmt,
    stream::{
        self,
        Stream as SvalStream,
    },
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
    fn map_key_collect(&mut self, k: value::collect::Value) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                self.serialize_key(&ToSerialize(k))
            }
            Some(buffered) => {
                buffered.map_key()?;
                k.stream(value::collect::Default(buffered))
            }
        }
    }

    fn map_value_collect(&mut self, v: value::collect::Value) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                self.serialize_value(&ToSerialize(v))
            }
            Some(buffered) => {
                buffered.map_value()?;
                v.stream(value::collect::Default(buffered))
            }
        }
    }

    fn seq_elem_collect(&mut self, v: value::collect::Value) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                self.serialize_elem(&ToSerialize(v))
            }
            Some(buffered) => {
                buffered.seq_elem()?;
                v.stream(value::collect::Default(buffered))
            }
        }
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
                buffered.seq_end()?;

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
            Some(buffered) => {
                buffered.map_key()
            },
        }
    }

    fn map_value(&mut self) -> Result<(), stream::Error> {
        match self.buffer() {
            None => {
                self.pos = Some(stream::Pos::value());

                Ok(())
            }
            Some(buffered) => {
                buffered.map_value()
            },
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
                buffered.map_end()?;

                if buffered.depth() == 0 {
                    self.buffer_end()?;
                }

                Ok(())
            }
        }
    }

    fn i64(&mut self, v: i64) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(v),
            Some(buffered) => buffered.i64(v),
        }
    }

    fn u64(&mut self, v: u64) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(v),
            Some(buffered) => buffered.u64(v),
        }
    }

    fn i128(&mut self, v: i128) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(v),
            Some(buffered) => buffered.i128(v),
        }
    }

    fn u128(&mut self, v: u128) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(v),
            Some(buffered) => buffered.u128(v),
        }
    }

    fn f64(&mut self, v: f64) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(v),
            Some(buffered) => buffered.f64(v),
        }
    }

    fn bool(&mut self, v: bool) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(v),
            Some(buffered) => buffered.bool(v),
        }
    }

    fn char(&mut self, v: char) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(v),
            Some(buffered) => buffered.char(v),
        }
    }

    fn str(&mut self, v: &str) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(v),
            Some(buffered) => buffered.str(v),
        }
    }

    fn none(&mut self) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(Option::None::<()>),
            Some(buffered) => buffered.none(),
        }
    }

    fn fmt(&mut self, v: fmt::Arguments) -> Result<(), stream::Error> {
        match self.buffer() {
            None => self.serialize_any(v),
            Some(buffered) => buffered.fmt(v),
        }
    }
}

#[derive(Debug)]
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
    fn next_serializable(&mut self, depth: usize) -> Tokens<'a> {
        let start = self.idx;

        let take = self.tokens[self.idx..]
            .iter()
            .enumerate()
            .take_while(|(_, t)| t.depth() > depth)
            .fuse()
            .last()
            .map(|(idx, _)| idx + 1)
            .unwrap_or(1);

        self.idx += take;

        Tokens(&self.tokens[start..self.idx])
    }

    fn expect_empty(&self) -> Result<(), stream::Error> {
        if self.idx < self.tokens.len() {
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
        self.idx += 1;

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

                    while let Some(next) = reader.next() {
                        match next.kind() {
                            Kind::MapKey => {
                                let key = next.depth();
                                let key = reader.next_serializable(key);

                                map.serialize_key(&key)?;
                            },
                            Kind::MapValue => {
                                let value = next.depth();
                                let value = reader.next_serializable(value);

                                map.serialize_value(&value)?;
                            },
                            Kind::MapEnd => {
                                reader.expect_empty().map_err(S::Error::custom)?;
                                break;
                            },
                            kind => return Err(S::Error::custom(format_args!("unexpected token value ({:?})", kind))),
                        }
                    }

                    map.end()
                }
                Kind::SeqBegin(len) => {
                    let mut seq = serializer.serialize_seq(*len)?;

                    while let Some(next) = reader.next() {
                        match next.kind() {
                            Kind::SeqElem => {
                                let elem = next.depth();
                                let elem = reader.next_serializable(elem);

                                seq.serialize_element(&elem)?;
                            },
                            Kind::SeqEnd => {
                                reader.expect_empty().map_err(S::Error::custom)?;
                                break;
                            },
                            kind => return Err(S::Error::custom(format_args!("unexpected token value ({:?})", kind))),
                        }
                    }

                    seq.end()
                }
                kind => Err(S::Error::custom(format_args!("unexpected token value ({:?})", kind))),
            },
            None => serializer.serialize_none(),
        }
    }
}
