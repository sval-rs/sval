#![allow(missing_docs)]

pub mod default_stream {
    use super::*;

    pub fn value<'sval, S: Stream<'sval>, V: sval::Value + ?Sized>(
        stream: S,
        value: &'sval V,
    ) -> Result<S::Ok> {
        let mut stream = stream.into_stream();
        let _ = sval::default_stream::value(&mut stream, value);
        stream.finish()
    }

    pub fn value_computed<'sval, S: Stream<'sval>, V: sval::Value + ?Sized>(
        stream: S,
        value: &V,
    ) -> Result<S::Ok> {
        let mut stream = stream.into_stream();
        let _ = sval::default_stream::value_computed(&mut stream, value);
        stream.finish()
    }
}

use core::{fmt, marker::PhantomData, mem};
use std::collections::VecDeque;

use crate::{BinaryBuf, Error, TextBuf, ValueBuf};

pub type Result<T = (), E = Error> = sval::Result<T, E>;

pub trait Stream<'sval> {
    type Ok;

    type Map: StreamMap<'sval, Ok = Self::Ok>;
    type Enum: StreamEnum<'sval, Ok = Self::Ok>;

    fn value<V: sval::Value + ?Sized>(self, value: &'sval V) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::value(self, value)
    }

    fn value_computed<V: sval::Value + ?Sized>(self, value: &V) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::value_computed(self, value)
    }

    fn null(self) -> Result<Self::Ok>;

    fn bool(self, value: bool) -> Result<Self::Ok>;

    fn i8(self, value: i8) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.i16(value as i16)
    }

    fn i16(self, value: i16) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.i32(value as i32)
    }

    fn i32(self, value: i32) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.i64(value as i64)
    }

    fn i64(self, value: i64) -> Result<Self::Ok>;

    fn i128(self, value: i128) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        if let Ok(value) = value.try_into() {
            self.i64(value)
        } else {
            let mut stream = self.into_stream();
            let _ = sval::stream_number(&mut stream, value);
            stream.finish()
        }
    }

    fn u8(self, value: u8) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.u16(value as u16)
    }

    fn u16(self, value: u16) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.u32(value as u32)
    }

    fn u32(self, value: u32) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.u64(value as u64)
    }

    fn u64(self, value: u64) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.u128(value as u128)
    }

    fn u128(self, value: u128) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        if let Ok(value) = value.try_into() {
            self.i64(value)
        } else {
            let mut stream = self.into_stream();
            let _ = sval::stream_number(&mut stream, value);
            stream.finish()
        }
    }

    fn f32(self, value: f32) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.f64(value as f64)
    }

    fn f64(self, value: f64) -> Result<Self::Ok>;

    fn text(self, text: &'sval str) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.text_computed(text)
    }

    fn text_computed(self, text: &str) -> Result<Self::Ok>;

    fn binary(self, binary: &'sval [u8]) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.binary_computed(binary)
    }

    fn binary_computed(self, binary: &[u8]) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        // Seq
        todo!()
    }

    fn tag(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Ok>;

    fn tagged<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &'sval V,
    ) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.tagged_computed(tag, label, index, value)
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &V,
    ) -> Result<Self::Ok>;

    fn map_begin(self, num_entries: Option<usize>) -> Result<Self::Map>;

    fn enum_begin(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Enum>;

    fn into_stream(self) -> FlatStream<'sval, Self>
    where
        Self: Sized,
    {
        FlatStream::new(self)
    }
}

pub trait StreamMap<'sval> {
    type Ok;

    fn key<V: sval::Value + ?Sized>(&mut self, key: &'sval V) -> Result {
        self.key_computed(key)
    }

    fn key_computed<V: sval::Value + ?Sized>(&mut self, key: &V) -> Result;

    fn value<V: sval::Value + ?Sized>(&mut self, value: &'sval V) -> Result {
        self.value_computed(value)
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> Result;

    fn end(self) -> Result<Self::Ok>;
}

pub trait StreamEnum<'sval> {
    type Ok;

    type Nested: StreamEnum<'sval, Nested = Self::Nested>;

    fn tag(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Ok>;

    fn tagged<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &'sval V,
    ) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        self.tagged_computed(tag, label, index, value)
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &V,
    ) -> Result<Self::Ok>;

    fn nested<F: FnOnce(Self::Nested) -> Result<<Self::Nested as StreamEnum<'sval>>::Ok>>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        variant: F,
    ) -> Result<Self::Ok>;

    fn end(self) -> Result<Self::Ok>;
}

pub struct Unsupported<Ok>(PhantomData<Result<Ok, Error>>);

impl<'sval, Ok> Stream<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    type Map = Self;

    type Enum = Self;

    fn null(self) -> Result<Self::Ok> {
        todo!()
    }

    fn bool(self, value: bool) -> Result<Self::Ok> {
        todo!()
    }

    fn i64(self, value: i64) -> Result<Self::Ok> {
        todo!()
    }

    fn f64(self, value: f64) -> Result<Self::Ok> {
        todo!()
    }

    fn text_computed(self, text: &str) -> Result<Self::Ok> {
        todo!()
    }

    fn tag(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &V,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn map_begin(self, num_entries: Option<usize>) -> Result<Self::Map> {
        todo!()
    }

    fn enum_begin(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Enum> {
        todo!()
    }
}

impl<'sval, Ok> StreamMap<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    fn key_computed<V: sval::Value + ?Sized>(&mut self, key: &V) -> Result {
        todo!()
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> Result {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'sval, Ok> StreamEnum<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    type Nested = Self;

    fn tag(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &V,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn nested<F: FnOnce(Self::Nested) -> Result<<Self::Nested as StreamEnum<'sval>>::Ok>>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        variant: F,
    ) -> Result<Self::Ok> {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

struct NestedStream<S> {
    stream: S,
    stack: VecDeque<NestedVariant>,
}

impl<'sval, S: StreamEnum<'sval>> NestedStream<S> {
    fn value_or_recurse(
        mut self,
        value: impl FnOnce(Self) -> Result<S::Ok>,
        nested: impl FnOnce(NestedStream<S::Nested>) -> Result<<S::Nested as StreamEnum<'sval>>::Ok>,
    ) -> Result<S::Ok> {
        if let Some(variant) = self.stack.pop_front() {
            self.stream.nested(
                variant.tag.as_ref(),
                variant.label.as_ref(),
                variant.index.as_ref(),
                |variant| {
                    nested(NestedStream {
                        stream: variant,
                        stack: self.stack,
                    })
                },
            )
        } else {
            value(self)
        }
    }
}

impl<'sval, S: StreamEnum<'sval>> Stream<'sval> for NestedStream<S> {
    type Ok = S::Ok;

    type Map = Unsupported<S::Ok>;

    type Enum = Unsupported<S::Ok>;

    fn value<V: sval::Value + ?Sized>(self, value: &'sval V) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| default_stream::value(stream, value),
            |stream| stream.value(value),
        )
    }

    fn value_computed<V: sval::Value + ?Sized>(self, value: &V) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| default_stream::value_computed(stream, value),
            |stream| stream.value_computed(value),
        )
    }

    fn null(self) -> Result<Self::Ok> {
        todo!()
    }

    fn bool(self, value: bool) -> Result<Self::Ok> {
        todo!()
    }

    fn i64(self, value: i64) -> Result<Self::Ok> {
        todo!()
    }

    fn f64(self, value: f64) -> Result<Self::Ok> {
        todo!()
    }

    fn text_computed(self, text: &str) -> Result<Self::Ok> {
        todo!()
    }

    fn tag(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| stream.stream.tag(tag, label, index),
            |stream| Stream::tag(stream, tag, label, index),
        )
    }

    fn tagged<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &'sval V,
    ) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| stream.stream.tagged(tag, label, index, value),
            |stream| Stream::tagged(stream, tag, label, index, value),
        )
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &V,
    ) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream| stream.stream.tagged_computed(tag, label, index, value),
            |stream| Stream::tagged_computed(stream, tag, label, index, value),
        )
    }

    fn map_begin(self, num_entries: Option<usize>) -> Result<Self::Map> {
        todo!()
    }

    fn enum_begin(
        mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Enum> {
        todo!()
    }
}

pub struct FlatStream<'sval, S: Stream<'sval>> {
    buffered: Option<Buffered<'sval>>,
    state: State<'sval, S>,
}

impl<'sval, S: Stream<'sval>> fmt::Debug for FlatStream<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlatStream")
            .field("buffered", &self.buffered)
            .field("state", &self.state)
            .finish()
    }
}

enum State<'sval, S: Stream<'sval>> {
    Any(Option<Any<'sval, S>>),
    Map(Option<Map<'sval, S>>),
    Tagged(Option<Tagged<'sval, S>>),
    Enum(Option<Enum<'sval, S>>),
    EnumVariant(Option<EnumVariant<'sval, S>>),
    Done(Option<Result<S::Ok>>),
}

impl<'sval, S: Stream<'sval>> fmt::Debug for State<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Any(state) => fmt::Debug::fmt(state, f),
            State::Map(state) => fmt::Debug::fmt(state, f),
            State::Tagged(state) => fmt::Debug::fmt(state, f),
            State::Enum(state) => fmt::Debug::fmt(state, f),
            State::EnumVariant(state) => fmt::Debug::fmt(state, f),
            State::Done(_) => f.debug_struct("Done").finish_non_exhaustive(),
        }
    }
}

#[derive(Debug)]
enum Buffered<'sval> {
    Text(TextBuf<'sval>),
    Binary(BinaryBuf<'sval>),
    Value(ValueBuf<'sval>),
}

struct Any<'sval, S: Stream<'sval>> {
    stream: S,
    _marker: PhantomData<&'sval ()>,
}

impl<'sval, S: Stream<'sval>> fmt::Debug for Any<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Any").finish_non_exhaustive()
    }
}

struct Map<'sval, S: Stream<'sval>> {
    stream: S::Map,
    is_key: bool,
    _marker: PhantomData<&'sval ()>,
}

impl<'sval, S: Stream<'sval>> fmt::Debug for Map<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("is_key", &self.is_key)
            .finish_non_exhaustive()
    }
}

struct Tagged<'sval, S: Stream<'sval>> {
    stream: S,
    tag: Option<sval::Tag>,
    label: Option<sval::Label<'static>>,
    index: Option<sval::Index>,
    _marker: PhantomData<&'sval ()>,
}

impl<'sval, S: Stream<'sval>> fmt::Debug for Tagged<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tagged")
            .field("tag", &self.tag)
            .field("label", &self.label)
            .field("index", &self.index)
            .finish_non_exhaustive()
    }
}

struct Enum<'sval, S: Stream<'sval>> {
    stream: NestedStream<S::Enum>,
    _marker: PhantomData<&'sval ()>,
}

impl<'sval, S: Stream<'sval>> fmt::Debug for Enum<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Enum").finish_non_exhaustive()
    }
}

struct EnumVariant<'sval, S: Stream<'sval>> {
    stream: NestedStream<S::Enum>,
    variant: Variant,
}

impl<'sval, S: Stream<'sval>> fmt::Debug for EnumVariant<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnumVariant")
            .field("variant", &self.variant)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
enum Variant {
    Tagged(TaggedVariant),
}

#[derive(Debug)]
struct TaggedVariant {
    tag: Option<sval::Tag>,
    label: Option<sval::Label<'static>>,
    index: Option<sval::Index>,
}

#[derive(Debug)]
struct NestedVariant {
    tag: Option<sval::Tag>,
    label: Option<sval::Label<'static>>,
    index: Option<sval::Index>,
}

impl<'sval, S: Stream<'sval>> FlatStream<'sval, S> {
    pub fn new(stream: S) -> Self {
        FlatStream {
            buffered: None,
            state: State::Any(Some(Any {
                stream,
                _marker: PhantomData,
            })),
        }
    }

    pub fn finish(&mut self) -> Result<S::Ok> {
        if let State::Done(ref mut r) = self.state {
            r.take()
                .unwrap_or_else(|| Err(Error::invalid_value("incomplete stream")))
        } else {
            Err(Error::invalid_value("incomplete stream"))
        }
    }
}

impl<'sval, S: Stream<'sval>> sval::Stream<'sval> for FlatStream<'sval, S> {
    fn value<V: sval::Value + ?Sized>(&mut self, v: &'sval V) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.value(v),
            |stream| stream.state.value(v, |stream, v| stream.value(v)),
        )
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, v: &V) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.value_computed(v),
            |stream| {
                stream
                    .state
                    .value_computed(v, |stream, v| stream.value_computed(v))
            },
        )
    }

    fn seq_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        todo!()
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_value_end(&mut self) -> sval::Result {
        todo!()
    }

    fn seq_end(&mut self) -> sval::Result {
        todo!()
    }

    fn map_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.buffer_or_begin_with(
            |buf| buf.map_begin(num_entries),
            |stream| {
                Ok(State::Map(Some(Map {
                    stream: stream.stream.map_begin(num_entries)?,
                    is_key: true,
                    _marker: PhantomData,
                })))
            },
            |_| Err(Error::invalid_value("maps cannot be used as enum variants")),
        )
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.map_key_begin(),
            |stream| {
                stream.with_map(|stream| {
                    stream.is_key = true;

                    Ok(())
                })
            },
        )
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.buffer_or_with(|buf| buf.map_key_end(), |_| Ok(()))
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.map_value_begin(),
            |stream| {
                stream.with_map(|stream| {
                    stream.is_key = false;

                    Ok(())
                })
            },
        )
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.buffer_or_with(|buf| buf.map_value_end(), |_| Ok(()))
    }

    fn map_end(&mut self) -> sval::Result {
        self.buffer_or_end_with(
            |buf| buf.map_end(),
            |stream| stream.take_map()?.stream.end(),
        )
    }

    fn enum_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_begin_with(
            |buf| buf.enum_begin(tag, label, index),
            |stream| {
                Ok(State::Enum(Some(Enum {
                    stream: NestedStream {
                        stream: stream.stream.enum_begin(tag, label, index)?,
                        stack: Default::default(),
                    },
                    _marker: PhantomData,
                })))
            },
            |mut stream| {
                stream.stream.stack.push_back(NestedVariant {
                    tag: tag.cloned(),
                    label: if let Some(label) = label {
                        Some(owned_label(label)?)
                    } else {
                        None
                    },
                    index: index.cloned(),
                });

                Ok(State::Enum(Some(stream)))
            },
        )
    }

    fn enum_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_end_with(
            |buf| buf.enum_end(tag, label, index),
            |stream| {
                if let Some(stream) = stream.take_enum()? {
                    stream.stream.stream.end()
                } else {
                    stream.finish()
                }
            },
        )
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_begin_with(
            |buf| buf.tagged_begin(tag, label, index),
            |stream| {
                Ok(State::Tagged(Some(Tagged {
                    stream: stream.stream,
                    tag: tag.cloned(),
                    label: if let Some(label) = label {
                        Some(owned_label(label)?)
                    } else {
                        None
                    },
                    index: index.cloned(),
                    _marker: PhantomData,
                })))
            },
            |stream| {
                Ok(State::EnumVariant(Some(EnumVariant {
                    stream: stream.stream,
                    variant: Variant::Tagged(TaggedVariant {
                        tag: tag.cloned(),
                        label: if let Some(label) = label {
                            Some(owned_label(label)?)
                        } else {
                            None
                        },
                        index: index.cloned(),
                    }),
                })))
            },
        )
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_end_with(
            |buf| buf.tagged_end(tag, label, index),
            |stream| stream.finish(),
        )
    }

    fn record_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn record_value_begin(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        todo!()
    }

    fn record_value_end(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        todo!()
    }

    fn record_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tuple_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn tuple_value_begin(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
        todo!()
    }

    fn tuple_value_end(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
        todo!()
    }

    fn tuple_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn record_tuple_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        todo!()
    }

    fn record_tuple_value_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
        index: &sval::Index,
    ) -> sval::Result {
        todo!()
    }

    fn record_tuple_value_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
        index: &sval::Index,
    ) -> sval::Result {
        todo!()
    }

    fn record_tuple_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tag(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.tag(tag, label, index),
            |stream| {
                stream
                    .state
                    .value_computed(&Tag(tag, label, index), |stream, _| {
                        stream.tag(tag, label, index)
                    })
            },
        )
    }

    fn null(&mut self) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.null(),
            |stream| stream.state.value(&sval::Null, |stream, _| stream.null()),
        )
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.bool(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.bool(*value))
            },
        )
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u8(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u8(*value))
            },
        )
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u16(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u16(*value))
            },
        )
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u32(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u32(*value))
            },
        )
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u64(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u64(*value))
            },
        )
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u128(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u128(*value))
            },
        )
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i8(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i8(*value))
            },
        )
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i16(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i16(*value))
            },
        )
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i32(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i32(*value))
            },
        )
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i64(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i64(*value))
            },
        )
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i128(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i128(*value))
            },
        )
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.f32(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.f32(*value))
            },
        )
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.f64(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.f64(*value))
            },
        )
    }

    fn text_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.text_begin(size_hint),
            |stream| stream.put_buffer(Buffered::Text(TextBuf::new())),
        )
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.text_fragment(fragment),
            |stream| stream.with_text(|text| text.push_fragment(fragment)),
        )
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.text_fragment_computed(fragment),
            |stream| stream.with_text(|text| text.push_fragment_computed(fragment)),
        )
    }

    fn text_end(&mut self) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.text_end(),
            |stream| {
                let buf = stream.take_text()?;

                if let Some(text) = buf.as_borrowed_str() {
                    stream.state.value(text, |stream, text| stream.text(text))
                } else {
                    stream
                        .state
                        .value_computed(buf.as_str(), |stream, text| stream.text_computed(text))
                }
            },
        )
    }

    fn binary_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.binary_begin(size_hint),
            |stream| stream.put_buffer(Buffered::Binary(BinaryBuf::new())),
        )
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.binary_fragment(fragment),
            |stream| stream.with_binary(|binary| binary.push_fragment(fragment)),
        )
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.binary_fragment_computed(fragment),
            |stream| stream.with_binary(|binary| binary.push_fragment_computed(fragment)),
        )
    }

    fn binary_end(&mut self) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.binary_end(),
            |stream| {
                let buf = stream.take_binary()?;

                if let Some(binary) = buf.as_borrowed_slice() {
                    stream
                        .state
                        .value(sval::BinarySlice::new(binary), |stream, binary| {
                            stream.binary(binary.as_slice())
                        })
                } else {
                    stream
                        .state
                        .value_computed(sval::BinarySlice::new(buf.as_slice()), |stream, binary| {
                            stream.binary_computed(binary.as_slice())
                        })
                }
            },
        )
    }
}

fn owned_label(label: &sval::Label) -> Result<sval::Label<'static>> {
    #[cfg(feature = "alloc")]
    {
        Ok(label.to_owned())
    }
    #[cfg(not(feature = "alloc"))]
    {
        if let Some(label) = label.as_static_str() {
            Ok(sval::Label::new(label))
        } else {
            Err(Error::no_alloc("streaming value"))
        }
    }
}

fn try_catch<'sval, T, S: Stream<'sval>>(
    stream: &mut FlatStream<'sval, S>,
    f: impl FnOnce(&mut FlatStream<'sval, S>) -> Result<T>,
) -> sval::Result<T> {
    match f(stream) {
        Ok(v) => Ok(v),
        Err(e) => {
            stream.state = State::Done(Some(Err(e)));

            sval::error()
        }
    }
}

impl<'sval, S: Stream<'sval>> State<'sval, S> {
    fn value<V: sval::Value + ?Sized>(
        &mut self,
        value: &'sval V,
        any: impl FnOnce(S, &'sval V) -> Result<S::Ok>,
    ) -> Result<S::Ok> {
        self.value_with(
            |stream| any(stream, value),
            |stream, tag, label, index| stream.tagged(tag, label, index, value),
            |stream, tag, label, index| stream.tagged(tag, label, index, value),
        )
    }

    fn value_computed<V: sval::Value + ?Sized>(
        &mut self,
        value: &V,
        any: impl FnOnce(S, &V) -> Result<S::Ok>,
    ) -> Result<S::Ok> {
        self.value_with(
            |stream| any(stream, value),
            |stream, tag, label, index| stream.tagged_computed(tag, label, index, value),
            |stream, tag, label, index| stream.tagged_computed(tag, label, index, value),
        )
    }

    fn value_with(
        &mut self,
        any: impl FnOnce(S) -> Result<S::Ok>,
        tagged: impl FnOnce(
            S,
            Option<&sval::Tag>,
            Option<&sval::Label>,
            Option<&sval::Index>,
        ) -> Result<S::Ok>,
        tagged_variant: impl FnOnce(
            NestedStream<S::Enum>,
            Option<&sval::Tag>,
            Option<&sval::Label>,
            Option<&sval::Index>,
        ) -> Result<S::Ok>,
    ) -> Result<S::Ok> {
        dbg!(&*self);

        let r = match self {
            State::Any(ref mut stream) => {
                let stream = stream.take().ok_or_else(|| {
                    Error::invalid_value("cannot stream value; the stream is already completed")
                })?;

                any(stream.stream)
            }
            State::Tagged(ref mut stream) => {
                let stream = stream.take().ok_or_else(|| {
                    Error::invalid_value(
                        "cannot stream tagged value; the stream is already completed",
                    )
                })?;

                tagged(
                    stream.stream,
                    stream.tag.as_ref(),
                    stream.label.as_ref(),
                    stream.index.as_ref(),
                )
            }
            State::Enum(_) => todo!(),
            State::EnumVariant(stream) => {
                let stream = stream.take().ok_or_else(|| Error::invalid_value(""))?;

                match stream.variant {
                    Variant::Tagged(TaggedVariant { tag, label, index }) => {
                        tagged_variant(stream.stream, tag.as_ref(), label.as_ref(), index.as_ref())
                    }
                }
            }
            State::Map(_) => todo!(),
            State::Done(_) => todo!(),
        };

        dbg!(&*self);

        r
    }
}

impl<'sval, S: Stream<'sval>> FlatStream<'sval, S> {
    fn buffer_or_stream_with(
        &mut self,
        buffer: impl FnOnce(&mut ValueBuf<'sval>) -> sval::Result,
        stream: impl FnOnce(&mut Self) -> Result<S::Ok>,
    ) -> sval::Result {
        dbg!(&*self);

        let mut r = None;
        self.buffer_or_with(buffer, |s| match stream(s) {
            Ok(ok) => {
                r = Some(ok);
                Ok(())
            }
            Err(e) => Err(e),
        })?;

        if let Some(ok) = r {
            self.state = State::Done(Some(Ok(ok)));
        }

        dbg!(&*self);

        Ok(())
    }

    fn buffer_or_with(
        &mut self,
        buffer: impl FnOnce(&mut ValueBuf<'sval>) -> sval::Result,
        stream: impl FnOnce(&mut Self) -> Result,
    ) -> sval::Result {
        dbg!(&*self);

        let r = try_catch(self, |s: &mut FlatStream<'_, S>| match s {
            FlatStream {
                buffered: Some(Buffered::Value(ref mut buf)),
                ..
            } => {
                if buffer(buf).is_err() {
                    let buf = mem::take(buf);

                    Err(buf.into_err())
                } else {
                    Ok(())
                }
            }
            s => stream(s),
        });

        dbg!(&*self);

        r
    }

    fn buffer_or_begin_with(
        &mut self,
        mut buffer: impl FnMut(&mut ValueBuf<'sval>) -> sval::Result,
        transition_any: impl FnOnce(Any<'sval, S>) -> Result<State<'sval, S>>,
        transition_enum: impl FnOnce(Enum<'sval, S>) -> Result<State<'sval, S>>,
    ) -> sval::Result {
        dbg!(&*self);

        let new_buf = try_catch(self, |stream| match stream {
            FlatStream {
                buffered: Some(Buffered::Value(ref mut buf)),
                state: _,
            } => {
                if buffer(buf).is_err() {
                    let buf = mem::take(buf);

                    return Err(buf.into_err());
                }

                Ok(None)
            }
            FlatStream {
                buffered: None,
                state: State::Any(state),
            } => {
                stream.state = transition_any(state.take().ok_or_else(|| {
                    Error::invalid_value("cannot stream value; the stream is already completed")
                })?)?;

                Ok(None)
            }
            FlatStream {
                buffered: None,
                state: State::Enum(state),
            } => {
                stream.state = transition_enum(state.take().ok_or_else(|| {
                    Error::invalid_value(
                        "cannot stream enum value; the stream is already completed",
                    )
                })?)?;

                Ok(None)
            }
            FlatStream {
                buffered: None,
                state: _,
            } => {
                let mut buf = ValueBuf::new();
                if buffer(&mut buf).is_err() {
                    return Err(buf.into_err());
                }

                Ok(Some(Buffered::Value(buf)))
            }
            _ => Err(Error::invalid_value(
                "cannot begin buffering; the stream is in an invalid state",
            )),
        })?;

        if let Some(new_buf) = new_buf {
            self.buffered = Some(new_buf);
        }

        dbg!(&*self);

        Ok(())
    }

    fn buffer_or_end_with(
        &mut self,
        buffer: impl FnOnce(&mut ValueBuf<'sval>) -> sval::Result,
        transition: impl FnOnce(&mut Self) -> Result<S::Ok>,
    ) -> sval::Result {
        let r = try_catch(self, |stream| match stream {
            FlatStream { buffered: None, .. } => Ok(Some(transition(stream)?)),
            FlatStream { buffered, .. } => {
                let Some(Buffered::Value(ref mut buf)) = buffered else {
                    return Err(Error::invalid_value(
                        "cannot end buffering value; the stream is in an invalid state",
                    ));
                };

                if buffer(buf).is_err() {
                    let buf = mem::take(buf);

                    return Err(buf.into_err());
                }

                if buf.is_complete() {
                    let buf = mem::take(buf);
                    *buffered = None;

                    return Ok(Some(
                        stream
                            .state
                            .value_computed(&buf, |stream, value| stream.value_computed(value))?,
                    ));
                }

                return Ok(None);
            }
        })?;

        if let Some(r) = r {
            self.state = State::Done(Some(Ok(r)));
        }

        Ok(())
    }

    fn put_buffer(&mut self, buf: Buffered<'sval>) -> Result {
        match self.buffered {
            None => {
                self.buffered = Some(buf);

                Ok(())
            }
            Some(_) => Err(Error::invalid_value(
                "cannot begin buffering; a buffer is already active",
            )),
        }
    }

    fn with_text(&mut self, buffer: impl FnOnce(&mut TextBuf<'sval>) -> Result) -> Result {
        match self.buffered {
            Some(Buffered::Text(ref mut buf)) => buffer(buf),
            _ => Err(Error::invalid_value(
                "cannot buffer text; no active text buffer",
            )),
        }
    }

    fn take_text(&mut self) -> Result<TextBuf<'sval>> {
        match self.buffered {
            Some(Buffered::Text(ref mut buf)) => {
                let buf = mem::take(buf);
                self.buffered = None;

                Ok(buf)
            }
            _ => Err(Error::invalid_value(
                "cannot end buffering text; no active text buffer",
            )),
        }
    }

    fn with_binary(&mut self, buffer: impl FnOnce(&mut BinaryBuf<'sval>) -> Result) -> Result {
        match self.buffered {
            Some(Buffered::Binary(ref mut buf)) => buffer(buf),
            _ => Err(Error::invalid_value(
                "cannot buffer binary; no active binary buffer",
            )),
        }
    }

    fn take_binary(&mut self) -> Result<BinaryBuf<'sval>> {
        match self.buffered {
            Some(Buffered::Binary(ref mut buf)) => {
                let buf = mem::take(buf);
                self.buffered = None;

                Ok(buf)
            }
            _ => Err(Error::invalid_value(
                "cannot end buffering binary; no active binary buffer",
            )),
        }
    }

    fn with_map(&mut self, f: impl FnOnce(&mut Map<'sval, S>) -> Result) -> Result {
        match self {
            FlatStream {
                buffered: None,
                state: State::Map(Some(map)),
            } => f(map),
            _ => Err(Error::invalid_value(
                "cannot stream a map; invalid stream state",
            )),
        }
    }

    fn take_map(&mut self) -> Result<Map<'sval, S>> {
        match self {
            FlatStream {
                buffered: None,
                state: State::Map(map),
            } => map.take().ok_or_else(|| {
                Error::invalid_value("cannot end a map; the stream is already completed")
            }),
            _ => Err(Error::invalid_value(
                "cannot end a map; invalid stream state",
            )),
        }
    }

    fn take_enum(&mut self) -> Result<Option<Enum<'sval, S>>> {
        match self {
            FlatStream {
                buffered: None,
                state: State::Enum(variant),
            } => Ok(variant.take()),
            FlatStream {
                buffered: None,
                state: State::Done(_),
            } => Ok(None),
            _ => Err(Error::invalid_value(
                "cannot end an enum; invalid stream state",
            )),
        }
    }
}

struct Tag<'a>(
    Option<&'a sval::Tag>,
    Option<&'a sval::Label<'a>>,
    Option<&'a sval::Index>,
);

impl<'a> sval::Value for Tag<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        stream.tag(self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_primitive() {
        assert_eq!(Value::I64(42), ToValue.value(&42i64).unwrap());
    }

    #[test]
    fn stream_nested_enum() {
        // Outer::Inner::Core::Value(42)
        struct Outer;

        impl sval::Value for Outer {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                dbg!(stream.enum_begin(None, Some(&sval::Label::new("Layer1")), None))?;
                dbg!(stream.enum_begin(None, Some(&sval::Label::new("Layer2")), None))?;
                dbg!(stream.enum_begin(None, Some(&sval::Label::new("Layer3")), None))?;
                dbg!(stream.enum_begin(None, Some(&sval::Label::new("Layer4")), None))?;
                dbg!(stream.enum_begin(None, Some(&sval::Label::new("Layer5")), None))?;
                dbg!(stream.enum_begin(None, Some(&sval::Label::new("Layer6")), None))?;
                dbg!(stream.enum_begin(None, Some(&sval::Label::new("Layer7")), None))?;
                dbg!(stream.tagged_begin(None, Some(&sval::Label::new("Value")), None))?;
                dbg!(stream.i64(42))?;
                dbg!(stream.tagged_end(None, Some(&sval::Label::new("Value")), None))?;
                dbg!(stream.enum_end(None, Some(&sval::Label::new("Layer7")), None))?;
                dbg!(stream.enum_end(None, Some(&sval::Label::new("Layer6")), None))?;
                dbg!(stream.enum_end(None, Some(&sval::Label::new("Layer5")), None))?;
                dbg!(stream.enum_end(None, Some(&sval::Label::new("Layer4")), None))?;
                dbg!(stream.enum_end(None, Some(&sval::Label::new("Layer3")), None))?;
                dbg!(stream.enum_end(None, Some(&sval::Label::new("Layer2")), None))?;
                dbg!(stream.enum_end(None, Some(&sval::Label::new("Layer1")), None))
            }
        }

        assert_eq!(
            Value::Enum(Enum {
                tag: Tag::new(None, Some(&sval::Label::new("Outer")), None).unwrap(),
                variant: Some(Variant::Enum(Box::new(Enum {
                    tag: Tag::new(None, Some(&sval::Label::new("Inner")), None).unwrap(),
                    variant: Some(Variant::Tagged(Tagged {
                        tag: Tag::new(None, Some(&sval::Label::new("Value")), None).unwrap(),
                        value: Box::new(Value::I64(42)),
                    }))
                })))
            }),
            ToValue.value(&Outer).unwrap()
        );
    }

    #[derive(Debug, PartialEq)]
    enum Value {
        Null,
        I64(i64),
        Tag(Tag),
        Tagged(Tagged),
        Enum(Enum),
    }

    #[derive(Debug, PartialEq)]
    struct Tag {
        tag: Option<sval::Tag>,
        label: Option<sval::Label<'static>>,
        index: Option<sval::Index>,
    }

    impl Tag {
        fn new(
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> Result<Self, Error> {
            Ok(Tag {
                tag: tag.cloned(),
                label: label.map(owned_label).transpose()?,
                index: index.cloned(),
            })
        }
    }

    #[derive(Debug, PartialEq)]
    struct Tagged {
        tag: Tag,
        value: Box<Value>,
    }

    #[derive(Debug, PartialEq)]
    struct Map {
        entries: Vec<(Value, Value)>,
    }

    #[derive(Debug, PartialEq)]
    struct Enum {
        tag: Tag,
        variant: Option<Variant>,
    }

    #[derive(Debug, PartialEq)]
    enum Variant {
        Tag(Tag),
        Tagged(Tagged),
        Enum(Box<Enum>),
    }

    struct ToValue;

    struct ToMap {
        key: Option<Value>,
        map: Map,
    }

    struct ToEnum {
        tag: Tag,
    }

    struct ToEnumVariant {
        tag: Tag,
    }

    impl<'sval> Stream<'sval> for ToValue {
        type Ok = Value;

        type Map = ToMap;

        type Enum = ToEnum;

        fn null(self) -> Result<Self::Ok> {
            Ok(Value::Null)
        }

        fn bool(self, value: bool) -> Result<Self::Ok> {
            todo!()
        }

        fn i64(self, value: i64) -> Result<Self::Ok> {
            Ok(Value::I64(value))
        }

        fn f64(self, value: f64) -> Result<Self::Ok> {
            todo!()
        }

        fn text_computed(self, text: &str) -> Result<Self::Ok> {
            todo!()
        }

        fn tag(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;

            Ok(Value::Tag(tag))
        }

        fn tagged_computed<V: sval::Value + ?Sized>(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            value: &V,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;
            let value = ToValue.value_computed(value)?;

            Ok(Value::Tagged(Tagged {
                tag,
                value: Box::new(value),
            }))
        }

        fn map_begin(self, num_entries: Option<usize>) -> Result<Self::Map> {
            todo!()
        }

        fn enum_begin(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> Result<Self::Enum> {
            Ok(ToEnum {
                tag: Tag::new(tag, label, index)?,
            })
        }
    }

    impl<'sval> StreamMap<'sval> for ToMap {
        type Ok = Value;

        fn key_computed<V: sval::Value + ?Sized>(&mut self, key: &V) -> Result {
            todo!()
        }

        fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> Result {
            todo!()
        }

        fn end(self) -> Result<Self::Ok> {
            todo!()
        }
    }

    impl<'sval> StreamEnum<'sval> for ToEnum {
        type Ok = Value;

        type Nested = ToEnumVariant;

        fn tag(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;

            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(Variant::Tag(tag)),
            }))
        }

        fn tagged_computed<V: sval::Value + ?Sized>(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            value: &V,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;
            let value = ToValue.value_computed(value)?;

            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(Variant::Tagged(Tagged {
                    tag,
                    value: Box::new(value),
                })),
            }))
        }

        fn nested<F: FnOnce(Self::Nested) -> Result<<Self::Nested as StreamEnum<'sval>>::Ok>>(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            variant: F,
        ) -> Result<Self::Ok> {
            let variant = variant(ToEnumVariant {
                tag: Tag::new(tag, label, index)?,
            })?;

            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(variant),
            }))
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: None,
            }))
        }
    }

    impl<'sval> StreamEnum<'sval> for ToEnumVariant {
        type Ok = Variant;

        type Nested = Self;

        fn tag(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;

            Ok(Variant::Enum(Box::new(Enum {
                tag: self.tag,
                variant: Some(Variant::Tag(tag)),
            })))
        }

        fn tagged_computed<V: sval::Value + ?Sized>(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            value: &V,
        ) -> Result<Self::Ok> {
            let tag = Tag::new(tag, label, index)?;
            let value = ToValue.value_computed(value)?;

            Ok(Variant::Enum(Box::new(Enum {
                tag: self.tag,
                variant: Some(Variant::Tagged(Tagged {
                    tag,
                    value: Box::new(value),
                })),
            })))
        }

        fn nested<F: FnOnce(Self::Nested) -> Result<Self::Ok>>(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            variant: F,
        ) -> Result<Self::Ok> {
            let variant = variant(ToEnumVariant {
                tag: Tag::new(tag, label, index)?,
            })?;

            Ok(Variant::Enum(Box::new(Enum {
                tag: self.tag,
                variant: Some(variant),
            })))
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Variant::Enum(Box::new(Enum {
                tag: self.tag,
                variant: None,
            })))
        }
    }
}
