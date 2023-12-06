#![allow(missing_docs)]

use core::{marker::PhantomData, mem};

use crate::{TextBuf, BinaryBuf, ValueBuf, Error};

pub type Result<T = (), E = Error> = sval::Result<T, E>;

pub trait Stream<'sval> {
    type Map: StreamMap<'sval>;

    fn value<V: sval::Value + ?Sized>(self, value: &'sval V) -> Result
    where
        Self: Sized,
    {
        let mut stream = self.into_stream();
        let _ = sval::default_stream::value(&mut stream, value);
        stream.finish()
    }

    fn value_computed<V: sval::Value + ?Sized>(self, value: &V) -> Result
    where
        Self: Sized,
    {
        let mut stream = self.into_stream();
        let _ = sval::default_stream::value_computed(&mut stream, value);
        stream.finish()
    }

    fn null(self) -> Result;

    fn bool(self, value: bool) -> Result;

    fn text(self, text: &'sval str) -> Result
    where
        Self: Sized,
    {
        self.text_computed(text)
    }

    fn text_computed(self, text: &str) -> Result;

    fn binary(self, binary: &'sval [u8]) -> Result
    where
        Self: Sized,
    {
        self.binary_computed(binary)
    }

    fn binary_computed(self, binary: &[u8]) -> Result;

    fn map(self, num_entries: Option<usize>) -> Result<Self::Map>;

    fn tagged<V: sval::Value + ?Sized>(self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>, value: &'sval V) -> Result
    where
        Self: Sized,
    {
        self.tagged_computed(tag, label, index, value)
    }

    fn tagged_computed<V: sval::Value + ?Sized>(self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>, value: &V) -> Result;

    fn into_stream(self) -> FlatStream<'sval, Self>
    where
        Self: Sized,
    {
        FlatStream::new(self)
    }
}

pub trait StreamMap<'sval> {
    fn map_key<V: sval::Value>(&mut self, key: &'sval V) -> Result {
        self.map_key_computed(key)
    }

    fn map_key_computed<V: sval::Value>(&mut self, key: &'sval V) -> Result;

    fn map_value<V: sval::Value>(&mut self, value: &'sval V) -> Result {
        self.map_value_computed(value)
    }

    fn map_value_computed<V: sval::Value>(&mut self, value: &'sval V) -> Result;

    fn end(self) -> Result;
}

pub struct FlatStream<'sval, S: Stream<'sval>> {
    buffered: Option<Buffered<'sval>>,
    state: State<'sval, S>,
}

enum State<'sval, S: Stream<'sval>> {
    Any(Option<Any<'sval, S>>),
    Map(Option<Map<'sval, S>>),
    Done(Option<Result>),
}

enum Buffered<'sval> {
    Text(TextBuf<'sval>),
    Binary(BinaryBuf<'sval>),
    Value(ValueBuf<'sval>),
}

struct Any<'sval, S: Stream<'sval>> {
    stream: S,
    tagged: bool,
    label: Option<sval::Label<'static>>,
    index: Option<sval::Index>,
    tag: Option<sval::Tag>,
    _marker: PhantomData<&'sval ()>,
}

struct Map<'sval, S: Stream<'sval>> {
    stream: S::Map,
    is_key: bool,
    _marker: PhantomData<&'sval ()>,
}

impl<'sval, S: Stream<'sval>> FlatStream<'sval, S> {
    pub fn new(stream: S) -> Self {
        FlatStream {
            buffered: None,
            state: State::Any(Some(Any {
                stream,
                tagged: false,
                label: None,
                index: None,
                tag: None,
                _marker: PhantomData,
            }))
        }
    }

    pub fn finish(&mut self) -> Result {
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
        try_catch(self, |stream| stream.state.value(v, |stream, v| stream.value(v)))
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, v: &V) -> sval::Result {
        try_catch(self, |stream| stream.state.value_computed(v, |stream, v| stream.value_computed(v)))
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
        self.buffer_or_transition_any_with(
            |buf| buf.map_begin(num_entries),
            |stream| {
                Ok(State::Map(Some(Map {
                    stream: stream.stream.map(num_entries)?,
                    is_key: true,
                    _marker: PhantomData,
                })))
            },
        )
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.buffer_or_stream_with(
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
        self.buffer_or_stream_with(|buf| buf.map_key_end(), |_| Ok(()))
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.buffer_or_stream_with(
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
        self.buffer_or_stream_with(|buf| buf.map_value_end(), |_| Ok(()))
    }

    fn map_end(&mut self) -> sval::Result {
        self.buffer_or_transition_done_with(
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
        todo!()
    }

    fn enum_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_transition_any_with(
            |buf| buf.tagged_begin(tag, label, index),
            |mut stream| {
                stream.tagged = true;
                stream.tag = tag.cloned();
                stream.label = if let Some(label) = label {
                    Some(owned_label(label)?)
                } else {
                    None
                };
                stream.index = index.cloned();

                Ok(State::Any(Some(stream)))
            },
        )
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        todo!()
    }

    fn tag(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
        todo!()
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

    fn null(&mut self) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.null(),
            |stream| stream.state.value(&sval::Null, |stream, _| stream.null())
        )
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.bool(value),
            |stream| stream.state.value_computed(&value, |stream, value| stream.bool(*value))
        )
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        todo!()
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        todo!()
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        todo!()
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        todo!()
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        todo!()
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        todo!()
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        todo!()
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        todo!()
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        todo!()
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        todo!()
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        todo!()
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        todo!()
    }

    fn text_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.text_begin(size_hint),
            |stream| stream.put_buffer(Buffered::Text(TextBuf::new())),
        )
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.text_fragment(fragment),
            |stream| stream.with_text(|text| text.push_fragment(fragment)),
        )
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.buffer_or_stream_with(
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
                    stream.state.value_computed(buf.as_str(), |stream, text| stream.text_computed(text))
                }
            },
        )
    }

    fn binary_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.binary_begin(size_hint),
            |stream| stream.put_buffer(Buffered::Binary(BinaryBuf::new())),
        )
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.binary_fragment(fragment),
            |stream| stream.with_binary(|binary| binary.push_fragment(fragment)),
        )
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.buffer_or_stream_with(
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
                    stream.state.value(sval::BinarySlice::new(binary), |stream, binary| stream.binary(binary.as_slice()))
                } else {
                    stream.state.value_computed(sval::BinarySlice::new(buf.as_slice()), |stream, binary| stream.binary_computed(binary.as_slice()))
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
            self.fail(Error::no_alloc("streaming value"))
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
    fn value<V: sval::Value + ?Sized>(&mut self, value: &'sval V, any: impl FnOnce(S, &'sval V) -> Result) -> Result {
        self.value_with(
            |stream| {
                let mut stream = stream.into_stream();
                let _ = sval::default_stream::value(&mut stream, value);
                stream.finish()
            },
            |stream, tag, label, index| stream.tagged(tag, label, index, value),
        )
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V, any: impl FnOnce(S, &V) -> Result) -> Result {
        self.value_with(
            |stream| {
                let mut stream = stream.into_stream();
                let _ = sval::default_stream::value_computed(&mut stream, value);
                stream.finish()
            },
            |stream, tag, label, index| stream.tagged_computed(tag, label, index, value),
        )
    }

    fn value_with(
        &mut self,
        value: impl FnOnce(S) -> Result,
        tagged: impl FnOnce(S, Option<&sval::Tag>, Option<&sval::Label>, Option<&sval::Index>) -> Result,
    ) -> Result {
        match self {
            State::Any(ref mut stream) => {
                let stream = stream
                    .take()
                    .ok_or_else(|| Error::invalid_value("the stream is already completed"))?;

                match stream {
                    Any { stream, tagged: false, .. } => {
                        value(stream)
                    }
                    Any { stream, label, index, tag, .. } => {
                        tagged(stream, tag.as_ref(), label.as_ref(), index.as_ref())
                    }
                }
            }
            State::Map(_) => todo!(),
            State::Done(_) => todo!(),
        }
    }
}

impl<'sval, S: Stream<'sval>> FlatStream<'sval, S> {
    fn buffer_or_stream_with(
        &mut self,
        buffer: impl FnOnce(&mut ValueBuf<'sval>) -> sval::Result,
        stream: impl FnOnce(&mut Self) -> Result,
    ) -> sval::Result {
        try_catch(self, |s: &mut FlatStream<'_, S>| match s {
            FlatStream {
                buffered: Some(Buffered::Value(ref mut buf)),
                ..
            } => if buffer(buf).is_err() {
                let buf = mem::take(buf);

                Err(buf.into_err())
            } else {
                Ok(())
            },
            s => stream(s),
        })
    }

    fn buffer_or_transition_any_with(
        &mut self,
        mut buffer: impl FnMut(&mut ValueBuf<'sval>) -> sval::Result,
        transition: impl FnOnce(Any<'sval, S>) -> Result<State<'sval, S>>,
    ) -> sval::Result {
        let buf = try_catch(self, |stream| {
            match stream {
                FlatStream {
                    buffered: Some(Buffered::Value(ref mut buf)),
                    ..
                } => {
                    if buffer(buf).is_err() {
                        let buf = mem::take(buf);

                        return Err(buf.into_err());
                    }

                    return Ok(None);
                }
                FlatStream {
                    buffered: None,
                    state: State::Any(any),
                } => {
                    if let Ok(state) = transition(
                        any.take()
                            .ok_or_else(|| Error::invalid_value("the stream is already completed"))?,
                    ) {
                        stream.state = state;

                        return Ok(None);
                    }
                }
                _ => return Err(Error::invalid_value("the stream is in an invalid state")),
            }

            let mut buf = ValueBuf::new();
            if buffer(&mut buf).is_err() {
                return Err(buf.into_err());
            }

            Ok(Some(Buffered::Value(buf)))
        })?;

        self.buffered = buf;

        Ok(())
    }

    fn buffer_or_transition_done_with(
        &mut self,
        buffer: impl FnOnce(&mut ValueBuf<'sval>) -> sval::Result,
        transition: impl FnOnce(&mut Self) -> Result,
    ) -> sval::Result {
        let r = try_catch(self, |stream| match stream {
            FlatStream {
                buffered: Some(Buffered::Value(ref mut buf)),
                ..
            } => {
                if buffer(buf).is_err() {
                    let buf = mem::take(buf);

                    return Err(buf.into_err());
                }

                if buf.is_complete() {
                    stream.state.value_computed(&*buf, |stream, value| stream.value_computed(value))?
                }

                return Ok(None);
            }
            FlatStream { buffered: None, .. } => Ok(Some(transition(stream)?)),
            _ => return Err(Error::invalid_value("the stream is in an invalid state")),
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
            Some(_) => Err(Error::invalid_value("a buffer is already active")),
        }
    }

    fn with_text(
        &mut self,
        buffer: impl FnOnce(&mut TextBuf<'sval>) -> Result,
    ) -> Result {
        match self.buffered {
            Some(Buffered::Text(ref mut buf)) => buffer(buf),
            _ => Err(Error::invalid_value("no active text buffer")),
        }
    }

    fn take_text(&mut self) -> Result<TextBuf<'sval>> {
        match self.buffered {
            Some(Buffered::Text(ref mut buf)) => {
                let buf = mem::take(buf);
                self.buffered = None;

                Ok(buf)
            }
            _ => Err(Error::invalid_value("no active text buffer")),
        }
    }

    fn with_binary(
        &mut self,
        buffer: impl FnOnce(&mut BinaryBuf<'sval>) -> Result,
    ) -> Result {
        match self.buffered {
            Some(Buffered::Binary(ref mut buf)) => buffer(buf),
            _ => Err(Error::invalid_value("no active binary buffer")),
        }
    }

    fn take_binary(&mut self) -> Result<BinaryBuf<'sval>> {
        match self.buffered {
            Some(Buffered::Binary(ref mut buf)) => {
                let buf = mem::take(buf);
                self.buffered = None;

                Ok(buf)
            }
            _ => Err(Error::invalid_value("no active binary buffer")),
        }
    }

    fn with_map(&mut self, f: impl FnOnce(&mut Map<'sval, S>) -> Result) -> Result {
        match self {
            FlatStream {
                buffered: None,
                state: State::Map(Some(map)),
            } => f(map),
            _ => Err(Error::invalid_value("invalid stream state")),
        }
    }

    fn take_map(&mut self) -> Result<Map<'sval, S>> {
        match self {
            FlatStream {
                buffered: None,
                state: State::Map(map),
            } => map
                .take()
                .ok_or_else(|| Error::invalid_value("invalid stream state")),
            _ => Err(Error::invalid_value("invalid stream state")),
        }
    }
}
