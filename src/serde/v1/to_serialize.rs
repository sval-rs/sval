use crate::{
    stream,
    value,
};

use super::error::err;

use serde1_lib::ser::{
    Error as SerError,
    Serialize,
    SerializeMap,
    SerializeSeq,
    SerializeStructVariant,
    SerializeTupleVariant,
    Serializer,
};

/**
The result of calling [`sval::serde::v1::to_serialize`](fn.to_serialize.html).
*/
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToSerialize<V>(pub(super) V);

impl<V> Serialize for ToSerialize<V>
where
    V: value::Value,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut stream = Stream::new(serializer);
        crate::stream_owned(&mut stream, &self.0).map_err(S::Error::custom)?;

        Ok(stream.take_ok())
    }
}

struct Arguments<'a>(stream::Arguments<'a>);
struct Source<'a>(stream::Source<'a>);
struct Tag<'a>(stream::Tag<'a>);
struct Ident<'a>(stream::Ident<'a>);

impl<'a> Serialize for ToSerialize<Arguments<'a>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&(self.0).0)
    }
}

impl<'a> Serialize for ToSerialize<Source<'a>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&(self.0).0)
    }
}

impl<'a> Serialize for ToSerialize<Tag<'a>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match (self.0).0 {
            stream::Tag::Named { name, .. } | stream::Tag::Full { name, .. } => serializer.serialize_str(name.as_str()),
            stream::Tag::Indexed { index, .. } => serializer.serialize_u32(index),
        }
    }
}

impl<'a> Serialize for ToSerialize<Ident<'a>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str((self.0).0.as_str())
    }
}

trait SerializeBridge {
    type Value: value::Value;
    type Serialize: Serialize;

    fn into_value(self) -> Self::Value;
    fn into_serialize(self) -> Self::Serialize;
}

impl<'a> SerializeBridge for stream::Value<'a> {
    type Value = Self;
    type Serialize = ToSerialize<Self>;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        ToSerialize(self)
    }
}

impl<'a> SerializeBridge for stream::Arguments<'a> {
    type Value = Self;
    type Serialize = ToSerialize<Arguments<'a>>;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        ToSerialize(Arguments(self))
    }
}

impl<'a> SerializeBridge for stream::Source<'a> {
    type Value = Self;
    type Serialize = ToSerialize<Source<'a>>;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        ToSerialize(Source(self))
    }
}

impl<'a> SerializeBridge for stream::Tag<'a> {
    type Value = Self;
    type Serialize = ToSerialize<Tag<'a>>;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        ToSerialize(Tag(self))
    }
}

impl<'a> SerializeBridge for stream::Ident<'a> {
    type Value = Self;
    type Serialize = ToSerialize<Ident<'a>>;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        ToSerialize(Ident(self))
    }
}

impl<'a> SerializeBridge for &'a str {
    type Value = Self;
    type Serialize = Self;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        self
    }
}

impl SerializeBridge for bool {
    type Value = Self;
    type Serialize = Self;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        self
    }
}

impl SerializeBridge for char {
    type Value = Self;
    type Serialize = Self;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        self
    }
}

impl SerializeBridge for i64 {
    type Value = Self;
    type Serialize = Self;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        self
    }
}

impl SerializeBridge for u64 {
    type Value = Self;
    type Serialize = Self;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        self
    }
}

impl SerializeBridge for i128 {
    type Value = Self;
    type Serialize = Self;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        self
    }
}

impl SerializeBridge for u128 {
    type Value = Self;
    type Serialize = Self;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        self
    }
}

impl SerializeBridge for f64 {
    type Value = Self;
    type Serialize = Self;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        self
    }
}

impl SerializeBridge for Option<()> {
    type Value = Self;
    type Serialize = Self;

    fn into_value(self) -> Self::Value {
        self
    }

    fn into_serialize(self) -> Self::Serialize {
        self
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
    buffered: Option<self::alloc_support::TokenBuf>,
    current: Option<Current<S>>,
}

enum Current<S>
where
    S: Serializer,
{
    Serializer(S),
    SerializeSeq(S::SerializeSeq),
    SerializeMap(S::SerializeMap),
    SerializeTupleVariant(S::SerializeTupleVariant),
    SerializeStructVariant(S::SerializeStructVariant, Option<&'static str>),
}

impl<S> Stream<S>
where
    S: Serializer,
{
    fn new(ser: S) -> Self {
        Stream {
            ok: None,
            pos: None,
            #[cfg(feature = "alloc")]
            buffered: None,
            current: Some(Current::Serializer(ser)),
        }
    }

    fn take_ok(self) -> S::Ok {
        self.ok.expect("missing return value")
    }
}

impl<S> Current<S>
where
    S: Serializer,
{
    fn take_serializer(self) -> S {
        match self {
            Current::Serializer(ser) => ser,
            _ => panic!("invalid serializer value (expected a serializer)"),
        }
    }

    fn serialize_seq(&mut self) -> &mut S::SerializeSeq {
        match self {
            Current::SerializeSeq(seq) => seq,
            _ => panic!("invalid serializer value (expected a sequence)"),
        }
    }

    fn take_serialize_seq(self) -> S::SerializeSeq {
        match self {
            Current::SerializeSeq(seq) => seq,
            _ => panic!("invalid serializer value (expected a sequence)"),
        }
    }

    fn serialize_map(&mut self) -> &mut S::SerializeMap {
        match self {
            Current::SerializeMap(map) => map,
            _ => panic!("invalid serializer value (expected a map)"),
        }
    }

    fn take_serialize_map(self) -> S::SerializeMap {
        match self {
            Current::SerializeMap(map) => map,
            _ => panic!("invalid serializer value (expected a map)"),
        }
    }

    fn serialize_struct_variant(&mut self) -> &mut S::SerializeStructVariant {
        match self {
            Current::SerializeStructVariant(map, _) => map,
            _ => panic!("invalid serializer value (expected a tagged map)"),
        }
    }

    fn take_serialize_struct_variant(self) -> S::SerializeStructVariant {
        match self {
            Current::SerializeStructVariant(map, _) => map,
            _ => panic!("invalid serializer value (expected a tagged map)"),
        }
    }

    fn serialize_tuple_variant(&mut self) -> &mut S::SerializeTupleVariant {
        match self {
            Current::SerializeTupleVariant(seq) => seq,
            _ => panic!("invalid serializer value (expected a sequence)"),
        }
    }

    fn take_serialize_tuple_variant(self) -> S::SerializeTupleVariant {
        match self {
            Current::SerializeTupleVariant(seq) => seq,
            _ => panic!("invalid serializer value (expected a sequence)"),
        }
    }
}

impl<S> Stream<S>
where
    S: Serializer,
{
    fn take_current(&mut self) -> Current<S> {
        self.current
            .take()
            .expect("attempt to use an invalid serializer")
    }

    fn current(&mut self) -> &mut Current<S> {
        self.current
            .as_mut()
            .expect("attempt to use an invalid serializer")
    }

    fn serialize_any(&mut self, v: impl SerializeBridge) -> stream::Result {
        match self.pos.take() {
            Some(Pos::Key) => self.serialize_key(v),
            Some(Pos::Value) => self.serialize_value(v),
            Some(Pos::Elem) => self.serialize_elem(v),
            None => self.serialize_primitive(v),
        }
    }

    fn serialize_elem(&mut self, v: impl SerializeBridge) -> stream::Result {
        match self.current() {
            Current::SerializeSeq(seq) => seq.serialize_element(&v.into_serialize()).map_err(err("error serializing sequence element")),
            Current::SerializeTupleVariant(seq) => seq.serialize_field(&v.into_serialize()).map_err(err("error serializing tagged sequence element")),
            _ => panic!("invalid serializer value (expected a map or tagged map)"),
        }
    }

    fn serialize_key(&mut self, k: impl SerializeBridge) -> stream::Result {
        match self.current() {
            Current::SerializeMap(map) => map.serialize_key(&k.into_serialize()).map_err(err("error serializing map key")),
            Current::SerializeStructVariant(map, field) => {
                struct ExtractField<'a>(&'a mut Option<&'static str>);

                impl<'a, 'v> stream::Stream<'v> for ExtractField<'a> {
                    fn ident(&mut self, ident: stream::Ident) -> stream::Result {
                        if let Some(ident) = ident.to_static() {
                            *self.0 = Some(ident.as_str());
                            Ok(())
                        } else {
                            Err(crate::Error::msg("serializing struct variants requires static field names"))
                        }
                    }
                }

                crate::stream_owned(ExtractField(field), k.into_value())
            },
            _ => panic!("invalid serializer value (expected a map or tagged map)"),
        }
    }

    fn serialize_value(&mut self, v: impl SerializeBridge) -> stream::Result {
        match self.current() {
            Current::SerializeMap(map) => map.serialize_value(&v.into_serialize()).map_err(err("error serializing map key")),
            Current::SerializeStructVariant(map, field) => {
                let field = field.take().expect("invalid serializer value (missing field)");
                map.serialize_field(field, &v.into_serialize()).map_err(err("error serializing tagged map key"))
            },
            _ => panic!("invalid serializer value (expected a map or tagged map)"),
        }
    }

    fn serialize_primitive(&mut self, v: impl SerializeBridge) -> stream::Result {
        let ser = self.take_current().take_serializer();

        self.ok = Some(
            v.into_serialize().serialize(ser)
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

    impl<'v, S> stream::Stream<'v> for Stream<S>
    where
        S: Serializer,
    {
        fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
            self.serialize_any(v)
        }

        fn error(&mut self, v: stream::Source) -> stream::Result {
            self.serialize_any(v)
        }

        fn i64(&mut self, v: i64) -> stream::Result {
            self.serialize_any(v)
        }

        fn u64(&mut self, v: u64) -> stream::Result {
            self.serialize_any(v)
        }

        fn i128(&mut self, v: i128) -> stream::Result {
            self.serialize_any(v)
        }

        fn u128(&mut self, v: u128) -> stream::Result {
            self.serialize_any(v)
        }

        fn f64(&mut self, v: f64) -> stream::Result {
            self.serialize_any(v)
        }

        fn bool(&mut self, v: bool) -> stream::Result {
            self.serialize_any(v)
        }

        fn char(&mut self, v: char) -> stream::Result {
            self.serialize_any(v)
        }

        fn str(&mut self, v: &str) -> stream::Result {
            self.serialize_any(v)
        }

        fn none(&mut self) -> stream::Result {
            self.serialize_any(Option::None::<()>)
        }

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
                _ => Err(crate::Error::msg(
                    "unsupported value type requires buffering",
                )),
            }
        }

        fn map_key(&mut self) -> stream::Result {
            self.pos = Some(Pos::Key);

            Ok(())
        }

        fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
            self.serialize_key(k)
        }

        fn map_value(&mut self) -> stream::Result {
            self.pos = Some(Pos::Value);

            Ok(())
        }

        fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
            self.serialize_value(v)
        }

        fn map_end(&mut self) -> stream::Result {
            let map = self.take_current().take_serialize_map();
            self.ok = Some(map.end().map_err(err("error completing map"))?);

            Ok(())
        }

        fn tagged_map_begin(&mut self, tag: stream::Tag, len: Option<usize>) -> stream::Result {
            match self.take_current() {
                Current::Serializer(ser) => {
                    if let (stream::Tag::Full { ty: Some(name), name: variant, index }, Some(len)) = (tag, len) {
                        if let (Some(name), Some(variant)) = (name.to_static(), variant.to_static()) {
                            let seq = ser
                                .serialize_struct_variant(name.as_str(), index, variant.as_str(), len)
                                .map(|map| Current::SerializeStructVariant(map, None))
                                .map_err(err("error beginning tagged map"))?;
                            self.current = Some(seq);

                            return Ok(())
                        } else {
                            Err(crate::Error::msg(
                                "serializing tagged maps with serde requires tags be static",
                            ))
                        }
                    } else {
                        Err(crate::Error::msg(
                            "serializing tagged maps with serde requires tags have a name, value, and index",
                        ))
                    }
                }
                _ => Err(crate::Error::msg(
                    "unsupported value type requires buffering",
                )),
            }
        }

        fn tagged_map_end(&mut self) -> stream::Result {
            let map = self.take_current().take_serialize_struct_variant();
            self.ok = Some(map.end().map_err(err("error completing tagged map"))?);

            Ok(())
        }

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
                _ => Err(crate::Error::msg(
                    "unsupported value type requires buffering",
                )),
            }
        }

        fn seq_elem(&mut self) -> stream::Result {
            self.pos = Some(Pos::Elem);

            Ok(())
        }

        fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
            self.serialize_elem(v)
        }

        fn seq_end(&mut self) -> stream::Result {
            let seq = self.take_current().take_serialize_seq();
            self.ok = Some(seq.end().map_err(err("error completing sequence"))?);

            Ok(())
        }

        fn tagged_seq_begin(&mut self, tag: stream::Tag, len: Option<usize>) -> stream::Result {
            match self.take_current() {
                Current::Serializer(ser) => {
                    if let (stream::Tag::Full { ty: Some(name), name: variant, index }, Some(len)) = (tag, len) {
                        if let (Some(name), Some(variant)) = (name.to_static(), variant.to_static()) {
                            let seq = ser
                                .serialize_tuple_variant(name.as_str(), index, variant.as_str(), len)
                                .map(Current::SerializeTupleVariant)
                                .map_err(err("error beginning tagged seq"))?;
                            self.current = Some(seq);

                            return Ok(())
                        } else {
                            Err(crate::Error::msg(
                                "serializing tagged sequences with serde requires tags be static",
                            ))
                        }
                    } else {
                        Err(crate::Error::msg(
                            "serializing tagged sequences with serde requires tags have a name, value, and index",
                        ))
                    }
                }
                _ => Err(crate::Error::msg(
                    "unsupported value type requires buffering",
                )),
            }
        }

        fn tagged_seq_end(&mut self) -> stream::Result {
            let seq = self.take_current().take_serialize_tuple_variant();
            self.ok = Some(seq.end().map_err(err("error completing sequence"))?);

            Ok(())
        }
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    pub(super) use crate::value::owned::{
        Token,
        TokenBuf,
    };

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

        fn buffer(&mut self) -> Option<&mut TokenBuf> {
            match self.buffered {
                None => None,
                Some(ref mut buffered) if !buffered.tokens().is_empty() => Some(buffered),
                _ => None,
            }
        }
    }

    impl<'v, S> stream::Stream<'v> for Stream<S>
    where
        S: Serializer,
    {
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
                            self.buffer_begin().seq_begin(meta)?;

                            self.current = Some(current);
                        }
                    }

                    Ok(())
                }
                Some(buffered) => buffered.seq_begin(meta),
            }
        }

        fn seq_elem(&mut self) -> stream::Result {
            match self.buffer() {
                None => {
                    self.pos = Some(Pos::Elem);

                    Ok(())
                }
                Some(buffered) => buffered.seq_elem(),
            }
        }

        fn seq_elem_collect(&mut self, v: stream::Value) -> stream::Result {
            match self.buffer() {
                None => self.serialize_elem(v),
                Some(buffered) => {
                    buffered.seq_elem()?;
                    v.stream(buffered).map(|_| ())
                }
            }
        }

        fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
            self.seq_elem_collect(v)
        }

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
                            self.buffer_begin().map_begin(meta)?;
                            self.current = Some(current);
                        }
                    }

                    Ok(())
                }
                Some(buffered) => buffered.map_begin(meta),
            }
        }

        fn map_key(&mut self) -> stream::Result {
            match self.buffer() {
                None => {
                    self.pos = Some(Pos::Key);

                    Ok(())
                }
                Some(buffered) => buffered.map_key(),
            }
        }

        fn map_key_collect(&mut self, k: stream::Value) -> stream::Result {
            match self.buffer() {
                None => self.serialize_key(k),
                Some(buffered) => {
                    buffered.map_key()?;
                    k.stream(buffered).map(|_| ())
                }
            }
        }

        fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
            self.map_key_collect(k)
        }

        fn map_value(&mut self) -> stream::Result {
            match self.buffer() {
                None => {
                    self.pos = Some(Pos::Value);

                    Ok(())
                }
                Some(buffered) => buffered.map_value(),
            }
        }

        fn map_value_collect(&mut self, v: stream::Value) -> stream::Result {
            match self.buffer() {
                None => self.serialize_value(v),
                Some(buffered) => {
                    buffered.map_value()?;
                    v.stream(buffered).map(|_| ())
                }
            }
        }

        fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
            self.map_value_collect(v)
        }

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

        fn i64(&mut self, v: i64) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.i64(v),
            }
        }

        fn u64(&mut self, v: u64) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.u64(v),
            }
        }

        fn i128(&mut self, v: i128) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.i128(v),
            }
        }

        fn u128(&mut self, v: u128) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.u128(v),
            }
        }

        fn f64(&mut self, v: f64) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.f64(v),
            }
        }

        fn bool(&mut self, v: bool) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.bool(v),
            }
        }

        fn char(&mut self, v: char) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.char(v),
            }
        }

        fn str(&mut self, v: &str) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.str(v),
            }
        }

        fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
            self.str(v)
        }

        fn none(&mut self) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(Option::None::<()>),
                Some(buffered) => buffered.none(),
            }
        }

        fn error(&mut self, v: stream::Source) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.error(v),
            }
        }

        fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
            self.error(v)
        }

        fn fmt(&mut self, v: stream::Arguments) -> stream::Result {
            match self.buffer() {
                None => self.serialize_any(v),
                Some(buffered) => buffered.fmt(v),
            }
        }

        fn fmt_borrowed(&mut self, v: stream::Arguments<'v>) -> stream::Result {
            self.fmt(v)
        }
    }

    struct Tokens<'a>(&'a [Token]);

    struct TokensReader<'a> {
        idx: usize,
        tokens: &'a [Token],
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
                .take_while(|(_, t)| t.depth > depth)
                .fuse()
                .last()
                .map(|(idx, _)| idx + 1)
                .unwrap_or(1);

            self.idx += take;

            Tokens(&self.tokens[start..self.idx])
        }

        fn expect_empty(&self) -> stream::Result {
            if self.idx < self.tokens.len() {
                Err(crate::Error::msg("unexpected trailing tokens"))
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
            use self::value::owned::TokenKind;

            let mut reader = self.reader();

            match reader.next() {
                None => serializer.serialize_none(),
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
                    TokenKind::Error(ref v) => {
                        reader.expect_empty().map_err(S::Error::custom)?;

                        stream::Source::from(&**v)
                            .into_serialize()
                            .serialize(serializer)
                    }
                    TokenKind::None => {
                        reader.expect_empty().map_err(S::Error::custom)?;

                        serializer.serialize_none()
                    }
                    TokenKind::MapBegin(ref meta) => {
                        let mut map = serializer.serialize_map(len)?;

                        while let Some(next) = reader.next() {
                            match next.kind {
                                TokenKind::MapKey => {
                                    let key = reader.next_serializable(next.depth);

                                    map.serialize_key(&key)?;
                                }
                                TokenKind::MapValue => {
                                    let value = reader.next_serializable(next.depth);

                                    map.serialize_value(&value)?;
                                }
                                TokenKind::MapEnd => {
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
                    TokenKind::SeqBegin(ref meta) => {
                        let mut seq = serializer.serialize_seq(len)?;

                        while let Some(next) = reader.next() {
                            match next.kind {
                                TokenKind::SeqElem => {
                                    let elem = reader.next_serializable(next.depth);

                                    seq.serialize_element(&elem)?;
                                }
                                TokenKind::SeqEnd => {
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
