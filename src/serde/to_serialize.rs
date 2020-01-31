use crate::{
    collect::{
        self,
        Collect,
    },
    std::{
        cell::Cell,
        fmt,
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
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut stream = collect::OwnedCollect::new(Stream::new(serializer));

        stream.any(&self.0).map_err(S::Error::custom)?;

        Ok(stream.into_inner().take_ok())
    }
}

// A wrapper around a `collect::Value` that can be called within
// `serde::Serialize`
struct Value<'a>(Cell<Option<collect::Value<'a>>>);

impl<'a> Value<'a> {
    #[inline]
    fn new(value: collect::Value<'a>) -> Self {
        Value(Cell::new(Some(value)))
    }
}

impl<'a> Serialize for ToSerialize<Value<'a>> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut stream = Stream::new(serializer);

        (self.0)
            .0
            .take()
            .expect("attempt to re-use value")
            .stream(&mut stream)
            .map_err(S::Error::custom)?;

        Ok(stream.take_ok())
    }
}

/**
The serialization stream.

Streaming between `sval` and `serde` uses the following process:

- Data that's passed in directly is immediately forwarded
to the serializer. That includes primitives and map and sequence
elements that are passed directly using `Collect` methods.
This is the common case and happy path.
- Data that's streamed without an underlying value is buffered
first before being forwarded to the underlying serializer. An
effort is made to buffer as little as possible, and to return to
the happy path when buffering a single element is done.
*/
struct Stream<S>
where
    S: Serializer,
{
    ok: Option<S::Ok>,
    pos: Option<Pos>,
    #[cfg(feature = "alloc")]
    buffered: Option<self::alloc_support::Buf>,
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
    fn new(ser: S) -> Self {
        Stream {
            ok: None,
            pos: None,
            #[cfg(feature = "alloc")]
            buffered: None,
            current: Some(Current::Serializer(ser)),
        }
    }

    #[inline]
    fn take_ok(self) -> S::Ok {
        self.ok.expect("missing return value")
    }
}

impl<S> Current<S>
where
    S: Serializer,
{
    #[inline]
    fn take_serializer(self) -> S {
        match self {
            Current::Serializer(ser) => ser,
            _ => panic!("invalid serializer value (expected a serializer)"),
        }
    }

    #[inline]
    fn serialize_seq(&mut self) -> &mut S::SerializeSeq {
        match self {
            Current::SerializeSeq(seq) => seq,
            _ => panic!("invalid serializer value (expected a sequence)"),
        }
    }

    #[inline]
    fn take_serialize_seq(self) -> S::SerializeSeq {
        match self {
            Current::SerializeSeq(seq) => seq,
            _ => panic!("invalid serializer value (expected a sequence)"),
        }
    }

    #[inline]
    fn serialize_map(&mut self) -> &mut S::SerializeMap {
        match self {
            Current::SerializeMap(map) => map,
            _ => panic!("invalid serializer value (expected a map)"),
        }
    }

    #[inline]
    fn take_serialize_map(self) -> S::SerializeMap {
        match self {
            Current::SerializeMap(map) => map,
            _ => panic!("invalid serializer value (expected a map)"),
        }
    }
}

impl<S> Stream<S>
where
    S: Serializer,
{
    #[inline]
    fn take_current(&mut self) -> Current<S> {
        self.current
            .take()
            .expect("attempt to use an invalid serializer")
    }

    #[inline]
    fn current(&mut self) -> &mut Current<S> {
        self.current
            .as_mut()
            .expect("attempt to use an invalid serializer")
    }

    #[inline]
    fn serialize_any(&mut self, v: impl Serialize) -> stream::Result {
        match self.pos.take() {
            Some(Pos::Key) => self.serialize_key(v),
            Some(Pos::Value) => self.serialize_value(v),
            Some(Pos::Elem) => self.serialize_elem(v),
            None => self.serialize_primitive(v),
        }
    }

    #[inline]
    fn serialize_elem(&mut self, v: impl Serialize) -> stream::Result {
        self.current()
            .serialize_seq()
            .serialize_element(&v)
            .map_err(err("error serializing sequence element"))
    }

    #[inline]
    fn serialize_key(&mut self, k: impl Serialize) -> stream::Result {
        self.current()
            .serialize_map()
            .serialize_key(&k)
            .map_err(err("error map serializing key"))
    }

    #[inline]
    fn serialize_value(&mut self, v: impl Serialize) -> stream::Result {
        self.current()
            .serialize_map()
            .serialize_value(&v)
            .map_err(err("error map serializing value"))
    }

    #[inline]
    fn serialize_primitive(&mut self, v: impl Serialize) -> stream::Result {
        let ser = self.take_current().take_serializer();

        self.ok = Some(
            v.serialize(ser)
                .map_err(err("error serializing primitive value"))?,
        );

        Ok(())
    }
}

enum Pos {
    Key,
    Value,
    Elem,
}

#[cfg(not(feature = "alloc"))]
mod no_alloc_support {
    use super::*;

    impl<S> Collect for Stream<S>
    where
        S: Serializer,
    {
        #[inline]
        fn map_key_collect(&mut self, k: collect::Value) -> collect::Result {
            self.serialize_key(ToSerialize(Value::new(k)))
        }

        #[inline]
        fn map_value_collect(&mut self, v: collect::Value) -> collect::Result {
            self.serialize_value(ToSerialize(Value::new(v)))
        }

        #[inline]
        fn seq_elem_collect(&mut self, v: collect::Value) -> collect::Result {
            self.serialize_elem(ToSerialize(Value::new(v)))
        }
    }

    impl<S> stream::Stream for Stream<S>
    where
        S: Serializer,
    {
        #[inline]
        fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
            match self.take_current() {
                Current::Serializer(ser) => {
                    let seq = ser
                        .serialize_seq(len)
                        .map(Current::SerializeSeq)
                        .map_err(err("error beginning sequence"))?;
                    self.current = Some(seq);

                    Ok(())
                }
                _ => Err(stream::Error::msg(
                    "unsupported value type requires buffering",
                )),
            }
        }

        #[inline]
        fn seq_elem(&mut self) -> stream::Result {
            self.pos = Some(Pos::Elem);

            Ok(())
        }

        #[inline]
        fn seq_end(&mut self) -> stream::Result {
            let seq = self.take_current().take_serialize_seq();
            self.ok = Some(seq.end().map_err(err("error completing sequence"))?);

            Ok(())
        }

        #[inline]
        fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
            match self.take_current() {
                Current::Serializer(ser) => {
                    let map = ser
                        .serialize_map(len)
                        .map(Current::SerializeMap)
                        .map_err(err("error beginning map"))?;
                    self.current = Some(map);

                    Ok(())
                }
                _ => Err(stream::Error::msg(
                    "unsupported value type requires buffering",
                )),
            }
        }

        #[inline]
        fn map_key(&mut self) -> stream::Result {
            self.pos = Some(Pos::Key);

            Ok(())
        }

        #[inline]
        fn map_value(&mut self) -> stream::Result {
            self.pos = Some(Pos::Value);

            Ok(())
        }

        #[inline]
        fn map_end(&mut self) -> stream::Result {
            let map = self.take_current().take_serialize_map();
            self.ok = Some(map.end().map_err(err("error completing map"))?);

            Ok(())
        }

        #[inline]
        fn i64(&mut self, v: i64) -> stream::Result {
            self.serialize_any(v)
        }

        #[inline]
        fn u64(&mut self, v: u64) -> stream::Result {
            self.serialize_any(v)
        }

        #[inline]
        fn i128(&mut self, v: i128) -> stream::Result {
            self.serialize_any(v)
        }

        #[inline]
        fn u128(&mut self, v: u128) -> stream::Result {
            self.serialize_any(v)
        }

        #[inline]
        fn f64(&mut self, v: f64) -> stream::Result {
            self.serialize_any(v)
        }

        #[inline]
        fn bool(&mut self, v: bool) -> stream::Result {
            self.serialize_any(v)
        }

        #[inline]
        fn char(&mut self, v: char) -> stream::Result {
            self.serialize_any(v)
        }

        #[inline]
        fn str(&mut self, v: &str) -> stream::Result {
            self.serialize_any(v)
        }

        #[inline]
        fn none(&mut self) -> stream::Result {
            self.serialize_any(Option::None::<()>)
        }

        #[inline]
        fn fmt(&mut self, v: fmt::Arguments) -> stream::Result {
            self.serialize_any(v)
        }
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::stream::{
        self,
        stack,
        Stream as StreamTrait,
    };

    pub(super) use crate::value::owned::{
        Buf,
        Token,
    };

    impl<S> Stream<S>
    where
        S: Serializer,
    {
        /**
        Begin a buffer with the given token or push it if a buffer already exists.
        */
        fn buffer_begin(&mut self) -> &mut Buf {
            match self.buffered {
                Some(ref mut buffered) => buffered,
                None => {
                    self.buffered = Some(Buf::new());
                    self.buffered.as_mut().unwrap()
                }
            }
        }

        /**
        End a buffer by serializing its contents.
        */
        fn buffer_end(&mut self) -> stream::Result {
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
        #[inline]
        fn buffer(&mut self) -> Option<&mut Buf> {
            match self.buffered {
                None => None,
                Some(ref mut buffered) if buffered.tokens().len() > 0 => Some(buffered),
                _ => None,
            }
        }
    }

    impl<S> Collect for Stream<S>
    where
        S: Serializer,
    {
        #[inline]
        fn map_key_collect(&mut self, k: collect::Value) -> collect::Result {
            match self.buffer() {
                None => self.serialize_key(ToSerialize(Value::new(k))),
                Some(buffered) => {
                    buffered.map_key()?;
                    k.stream(collect::Default(buffered))
                }
            }
        }

        #[inline]
        fn map_value_collect(&mut self, v: collect::Value) -> collect::Result {
            match self.buffer() {
                None => self.serialize_value(ToSerialize(Value::new(v))),
                Some(buffered) => {
                    buffered.map_value()?;
                    v.stream(collect::Default(buffered))
                }
            }
        }

        #[inline]
        fn seq_elem_collect(&mut self, v: collect::Value) -> collect::Result {
            match self.buffer() {
                None => self.serialize_elem(ToSerialize(Value::new(v))),
                Some(buffered) => {
                    buffered.seq_elem()?;
                    v.stream(collect::Default(buffered))
                }
            }
        }
    }

    impl<S> stream::Stream for Stream<S>
    where
        S: Serializer,
    {
        #[inline]
        fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
            match self.buffer() {
                None => {
                    match self.take_current() {
                        Current::Serializer(ser) => {
                            let seq = ser
                                .serialize_seq(len)
                                .map(Current::SerializeSeq)
                                .map_err(err("error serializing sequence"))?;
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

        #[inline]
        fn seq_elem(&mut self) -> stream::Result {
            match self.buffer() {
                None => {
                    self.pos = Some(Pos::Elem);

                    Ok(())
                }
                Some(buffered) => buffered.seq_elem(),
            }
        }

        #[inline]
        fn seq_end(&mut self) -> stream::Result {
            match self.buffer() {
                None => {
                    let seq = self.take_current().take_serialize_seq();
                    self.ok = Some(seq.end().map_err(err("error completing sequence"))?);

                    Ok(())
                }
                Some(buffered) => {
                    buffered.seq_end()?;

                    if buffered.is_streamable() {
                        self.buffer_end()?;
                    }

                    Ok(())
                }
            }
        }

        #[inline]
        fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
            match self.buffer() {
                None => {
                    match self.take_current() {
                        Current::Serializer(ser) => {
                            let map = ser
                                .serialize_map(len)
                                .map(Current::SerializeMap)
                                .map_err(err("error serializing map"))?;
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

        #[inline]
        fn map_key(&mut self) -> stream::Result {
            match self.buffer() {
                None => {
                    self.pos = Some(Pos::Key);

                    Ok(())
                }
                Some(buffered) => buffered.map_key(),
            }
        }

        #[inline]
        fn map_value(&mut self) -> stream::Result {
            match self.buffer() {
                None => {
                    self.pos = Some(Pos::Value);

                    Ok(())
                }
                Some(buffered) => buffered.map_value(),
            }
        }

        #[inline]
        fn map_end(&mut self) -> stream::Result {
            match self.buffer() {
                None => {
                    let map = self.take_current().take_serialize_map();
                    self.ok = Some(map.end().map_err(err("error completing map"))?);

                    Ok(())
                }
                Some(buffered) => {
                    buffered.map_end()?;

                    if buffered.is_streamable() {
                        self.buffer_end()?;
                    }

                    Ok(())
                }
            }
        }

        #[inline]
        fn i64(&mut self, v: i64) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.i64(v),
            }
        }

        #[inline]
        fn u64(&mut self, v: u64) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.u64(v),
            }
        }

        #[inline]
        fn i128(&mut self, v: i128) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.i128(v),
            }
        }

        #[inline]
        fn u128(&mut self, v: u128) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.u128(v),
            }
        }

        #[inline]
        fn f64(&mut self, v: f64) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.f64(v),
            }
        }

        #[inline]
        fn bool(&mut self, v: bool) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.bool(v),
            }
        }

        #[inline]
        fn char(&mut self, v: char) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.char(v),
            }
        }

        #[inline]
        fn str(&mut self, v: &str) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.str(v),
            }
        }

        #[inline]
        fn none(&mut self) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(Option::None::<()>),
                Some(buffered) => buffered.none(),
            }
        }

        #[inline]
        fn fmt(&mut self, v: fmt::Arguments) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.fmt(v),
            }
        }
    }

    struct Tokens<'a>(&'a [Token]);

    struct TokensReader<'a> {
        idx: usize,
        tokens: &'a [Token],
    }

    impl<'a> Tokens<'a> {
        #[inline]
        fn reader(&self) -> TokensReader<'a> {
            TokensReader {
                idx: 0,
                tokens: self.0,
            }
        }
    }

    impl<'a> TokensReader<'a> {
        fn next_serializable(&mut self, depth: stack::Depth) -> Tokens<'a> {
            let start = self.idx;

            let take = self.tokens[self.idx..]
                .iter()
                .enumerate()
                .take_while(|(_, t)| t.depth > depth)
                .fuse()
                .last()
                .map(|(idx, _)| idx + 1)
                .unwrap_or(1);

            self.idx += take;

            Tokens(&self.tokens[start..self.idx])
        }

        #[inline]
        fn expect_empty(&self) -> stream::Result {
            if self.idx < self.tokens.len() {
                Err(stream::Error::msg("unexpected trailing tokens"))
            } else {
                Ok(())
            }
        }
    }

    impl<'a> Iterator for TokensReader<'a> {
        type Item = &'a Token;

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
                None => serializer.serialize_none(),
                Some(token) => match token.kind {
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
                        let mut map = serializer.serialize_map(len)?;

                        while let Some(next) = reader.next() {
                            match next.kind {
                                Kind::MapKey => {
                                    let key = reader.next_serializable(next.depth.clone());

                                    map.serialize_key(&key)?;
                                }
                                Kind::MapValue => {
                                    let value = reader.next_serializable(next.depth.clone());

                                    map.serialize_value(&value)?;
                                }
                                Kind::MapEnd => {
                                    reader.expect_empty().map_err(S::Error::custom)?;
                                    break;
                                }
                                _ => return Err(S::Error::custom(
                                    "unexpected token value (expected a key, value, or map end)",
                                )),
                            }
                        }

                        map.end()
                    }
                    Kind::SeqBegin(len) => {
                        let mut seq = serializer.serialize_seq(len)?;

                        while let Some(next) = reader.next() {
                            match next.kind {
                                Kind::SeqElem => {
                                    let elem = reader.next_serializable(next.depth.clone());

                                    seq.serialize_element(&elem)?;
                                }
                                Kind::SeqEnd => {
                                    reader.expect_empty().map_err(S::Error::custom)?;
                                    break;
                                }
                                _ => return Err(S::Error::custom(
                                    "unexpected token value (expected an element, or sequence end)",
                                )),
                            }
                        }

                        seq.end()
                    }
                    _ => Err(S::Error::custom(
                        "unexpected token value (expected a primitive, map, or sequence)",
                    )),
                },
            }
        }
    }
}
