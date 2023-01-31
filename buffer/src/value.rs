use crate::std::marker::PhantomData;

#[cfg(feature = "alloc")]
use crate::{std::vec::Vec, BinaryBuf, TextBuf};

#[cfg(feature = "alloc")]
pub use alloc_support::*;

#[derive(Debug)]
pub struct ValueBuf<'sval> {
    #[cfg(feature = "alloc")]
    parts: Vec<ValuePart<'sval>>,
    #[cfg(feature = "alloc")]
    stack: Vec<usize>,
    _marker: PhantomData<&'sval ()>,
}

impl<'sval> Default for ValueBuf<'sval> {
    fn default() -> Self {
        ValueBuf::new()
    }
}

impl<'sval> ValueBuf<'sval> {
    pub fn new() -> Self {
        ValueBuf {
            #[cfg(feature = "alloc")]
            parts: Vec::new(),
            #[cfg(feature = "alloc")]
            stack: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn collect(v: &'sval (impl sval::Value + ?Sized)) -> sval::Result<Self> {
        let mut buf = ValueBuf::new();

        v.stream(&mut buf)?;

        Ok(buf)
    }

    pub fn is_complete(&self) -> bool {
        #[cfg(feature = "alloc")]
        {
            self.stack.len() == 0
        }
        #[cfg(not(feature = "alloc"))]
        {
            true
        }
    }
}

impl<'a> sval::Value for ValueBuf<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.slice().stream(stream)
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = stream;
            sval::error()
        }
    }
}

impl<'sval> sval::Stream<'sval> for ValueBuf<'sval> {
    fn null(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::Null);

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::Bool(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::Text(TextBuf::new()));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            match self.current_mut().kind {
                ValueKind::Text(ref mut text) => text.push_fragment(fragment),
                _ => Err(sval::Error::new()),
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = fragment;
            sval::error()
        }
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            match self.current_mut().kind {
                ValueKind::Text(ref mut text) => text.push_fragment_computed(fragment),
                _ => Err(sval::Error::new()),
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = fragment;
            sval::error()
        }
    }

    fn text_end(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn binary_begin(&mut self, _: Option<usize>) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::Binary(BinaryBuf::new()));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            match self.current_mut().kind {
                ValueKind::Binary(ref mut binary) => binary.push_fragment(fragment),
                _ => Err(sval::Error::new()),
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = fragment;
            sval::error()
        }
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            match self.current_mut().kind {
                ValueKind::Binary(ref mut binary) => binary.push_fragment_computed(fragment),
                _ => Err(sval::Error::new()),
            }
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = fragment;
            sval::error()
        }
    }

    fn binary_end(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::U8(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::U16(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::U32(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::U64(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::U128(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::I8(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::I16(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::I32(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::I64(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::I128(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::F32(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::F64(value));

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = value;
            sval::error()
        }
    }

    fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::Map {
                len: 0,
                num_entries_hint,
            });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = num_entries_hint;
            sval::error()
        }
    }

    fn map_key_begin(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::MapKey { len: 0 });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn map_key_end(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn map_value_begin(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::MapValue { len: 0 });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn map_value_end(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn map_end(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn seq_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::Seq {
                len: 0,
                num_entries_hint,
            });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = num_entries_hint;
            sval::error()
        }
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::SeqValue { len: 0 });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn seq_value_end(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn seq_end(&mut self) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn enum_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::Enum {
                len: 0,
                tag: tag.cloned(),
                index: index.cloned(),
                label: label.map(|label| label.to_owned()),
            });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = (tag, label, index);
            sval::error()
        }
    }

    fn enum_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::Tagged {
                len: 0,
                tag: tag.cloned(),
                index: index.cloned(),
                label: label.map(|label| label.to_owned()),
            });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = (tag, label, index);
            sval::error()
        }
    }

    fn tagged_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn tag(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_kind(ValueKind::Tag {
                tag: tag.cloned(),
                index: index.cloned(),
                label: label.map(|label| label.to_owned()),
            });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = (tag, label, index);
            sval::error()
        }
    }

    fn record_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::Record {
                len: 0,
                tag: tag.cloned(),
                index: index.cloned(),
                label: label.map(|label| label.to_owned()),
                num_entries,
            });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = (tag, label, index, num_entries);
            sval::error()
        }
    }

    fn record_value_begin(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::RecordValue {
                len: 0,
                tag: tag.cloned(),
                label: label.to_owned(),
            });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = tag;
            let _ = label;
            sval::error()
        }
    }

    fn record_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Label) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn record_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn tuple_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::Tuple {
                len: 0,
                tag: tag.cloned(),
                index: index.cloned(),
                label: label.map(|label| label.to_owned()),
                num_entries,
            });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = (tag, label, index, num_entries);
            sval::error()
        }
    }

    fn tuple_value_begin(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_begin(ValueKind::TupleValue {
                len: 0,
                tag: tag.cloned(),
                index: index.clone(),
            });

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = tag;
            let _ = index;
            sval::error()
        }
    }

    fn tuple_value_end(&mut self, _: Option<&sval::Tag>, _: &sval::Index) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }

    fn tuple_end(
        &mut self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            self.push_end();

            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            sval::error()
        }
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::{
        std::{mem, ops::Range},
        BinaryBuf, TextBuf,
    };

    pub fn stream_to_value<'sval>(v: &'sval (impl sval::Value + ?Sized)) -> sval::Result<ValueBuf> {
        ValueBuf::collect(v)
    }

    #[repr(transparent)]
    pub(super) struct ValueSlice<'sval>([ValuePart<'sval>]);

    #[derive(Debug, PartialEq)]
    pub(super) struct ValuePart<'sval> {
        pub(super) kind: ValueKind<'sval>,
    }

    #[derive(Debug, PartialEq)]
    pub(super) enum ValueKind<'sval> {
        Null,
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        U128(u128),
        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),
        I128(i128),
        F32(f32),
        F64(f64),
        Text(TextBuf<'sval>),
        Binary(BinaryBuf<'sval>),
        Map {
            len: usize,
            num_entries_hint: Option<usize>,
        },
        MapKey {
            len: usize,
        },
        MapValue {
            len: usize,
        },
        Seq {
            len: usize,
            num_entries_hint: Option<usize>,
        },
        SeqValue {
            len: usize,
        },
        Tag {
            tag: Option<sval::Tag>,
            label: Option<sval::Label<'static>>,
            index: Option<sval::Index>,
        },
        Enum {
            len: usize,
            tag: Option<sval::Tag>,
            label: Option<sval::Label<'static>>,
            index: Option<sval::Index>,
        },
        Tagged {
            len: usize,
            tag: Option<sval::Tag>,
            label: Option<sval::Label<'static>>,
            index: Option<sval::Index>,
        },
        Record {
            len: usize,
            tag: Option<sval::Tag>,
            label: Option<sval::Label<'static>>,
            index: Option<sval::Index>,
            num_entries: Option<usize>,
        },
        RecordValue {
            len: usize,
            tag: Option<sval::Tag>,
            label: sval::Label<'static>,
        },
        Tuple {
            len: usize,
            tag: Option<sval::Tag>,
            label: Option<sval::Label<'static>>,
            index: Option<sval::Index>,
            num_entries: Option<usize>,
        },
        TupleValue {
            len: usize,
            tag: Option<sval::Tag>,
            index: sval::Index,
        },
    }

    impl<'sval> ValueBuf<'sval> {
        pub(super) fn slice<'a>(&'a self) -> &'a ValueSlice<'sval> {
            unsafe { mem::transmute::<&'a [ValuePart<'sval>], &'a ValueSlice<'sval>>(&self.parts) }
        }

        pub(super) fn push_kind(&mut self, kind: ValueKind<'sval>) {
            self.parts.push(ValuePart { kind });
        }

        pub(super) fn push_begin(&mut self, kind: ValueKind<'sval>) {
            self.stack.push(self.parts.len());
            self.parts.push(ValuePart { kind });
        }

        pub(super) fn push_end(&mut self) {
            let index = self.stack.pop().expect("missing stack frame");

            let len = self.parts.len() - index - 1;

            *match &mut self.parts[index].kind {
                ValueKind::Map { len, .. } => len,
                ValueKind::MapKey { len } => len,
                ValueKind::MapValue { len } => len,
                ValueKind::Seq { len, .. } => len,
                ValueKind::SeqValue { len } => len,
                ValueKind::Enum { len, .. } => len,
                ValueKind::Tagged { len, .. } => len,
                ValueKind::Record { len, .. } => len,
                ValueKind::RecordValue { len, .. } => len,
                ValueKind::Tuple { len, .. } => len,
                ValueKind::TupleValue { len, .. } => len,
                ValueKind::Null
                | ValueKind::Bool(_)
                | ValueKind::U8(_)
                | ValueKind::U16(_)
                | ValueKind::U32(_)
                | ValueKind::U64(_)
                | ValueKind::U128(_)
                | ValueKind::I8(_)
                | ValueKind::I16(_)
                | ValueKind::I32(_)
                | ValueKind::I64(_)
                | ValueKind::I128(_)
                | ValueKind::F32(_)
                | ValueKind::F64(_)
                | ValueKind::Text(_)
                | ValueKind::Binary(_)
                | ValueKind::Tag { .. } => panic!("can't end at this index"),
            } = len;
        }

        pub(super) fn current_mut(&mut self) -> &mut ValuePart<'sval> {
            self.parts.last_mut().expect("missing current")
        }
    }

    impl<'sval> ValueSlice<'sval> {
        pub(super) fn slice<'a>(&'a self, range: Range<usize>) -> &'a ValueSlice<'sval> {
            match self.0.get(range.clone()) {
                Some(_) => (),
                None => {
                    panic!("{:?} is out of range for {:?}", range, &self.0);
                }
            }

            unsafe {
                mem::transmute::<&'a [ValuePart<'sval>], &'a ValueSlice<'sval>>(&self.0[range])
            }
        }
    }

    impl<'a> sval::Value for ValueSlice<'a> {
        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
            &'sval self,
            stream: &mut S,
        ) -> sval::Result {
            let mut i = 0;

            fn stream_value<'sval, S: sval::Stream<'sval> + ?Sized>(
                stream: &mut S,
                i: &mut usize,
                len: usize,
                value: &'sval ValueSlice,
                f: impl FnOnce(&mut S, &'sval ValueSlice) -> sval::Result,
            ) -> sval::Result {
                let value = value.slice({
                    let start = *i + 1;
                    let end = start + len;

                    start..end
                });

                f(stream, value)?;

                *i += len;

                Ok(())
            }

            while let Some(part) = self.0.get(i) {
                match &part.kind {
                    ValueKind::Null => stream.null()?,
                    ValueKind::Bool(v) => v.stream(stream)?,
                    ValueKind::U8(v) => v.stream(stream)?,
                    ValueKind::U16(v) => v.stream(stream)?,
                    ValueKind::U32(v) => v.stream(stream)?,
                    ValueKind::U64(v) => v.stream(stream)?,
                    ValueKind::U128(v) => v.stream(stream)?,
                    ValueKind::I8(v) => v.stream(stream)?,
                    ValueKind::I16(v) => v.stream(stream)?,
                    ValueKind::I32(v) => v.stream(stream)?,
                    ValueKind::I64(v) => v.stream(stream)?,
                    ValueKind::I128(v) => v.stream(stream)?,
                    ValueKind::F32(v) => v.stream(stream)?,
                    ValueKind::F64(v) => v.stream(stream)?,
                    ValueKind::Text(v) => v.stream(stream)?,
                    ValueKind::Binary(v) => v.stream(stream)?,
                    ValueKind::Map {
                        len,
                        num_entries_hint,
                    } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.map_begin(*num_entries_hint)?;
                            body.stream(stream)?;
                            stream.map_end()
                        })?;
                    }
                    ValueKind::MapKey { len } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.map_key_begin()?;
                            stream.value(body)?;
                            stream.map_key_end()
                        })?;
                    }
                    ValueKind::MapValue { len } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.map_value_begin()?;
                            stream.value(body)?;
                            stream.map_value_end()
                        })?;
                    }
                    ValueKind::Seq {
                        len,
                        num_entries_hint,
                    } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.seq_begin(*num_entries_hint)?;
                            body.stream(stream)?;
                            stream.seq_end()
                        })?;
                    }
                    ValueKind::SeqValue { len } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.seq_value_begin()?;
                            stream.value(body)?;
                            stream.seq_value_end()
                        })?;
                    }
                    ValueKind::Tag { tag, label, index } => {
                        stream.tag(tag.as_ref(), label.as_ref(), index.as_ref())?;
                    }
                    ValueKind::Enum {
                        len,
                        tag,
                        label,
                        index,
                    } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.enum_begin(tag.as_ref(), label.as_ref(), index.as_ref())?;
                            body.stream(stream)?;
                            stream.enum_end(tag.as_ref(), label.as_ref(), index.as_ref())
                        })?;
                    }
                    ValueKind::Tagged {
                        len,
                        tag,
                        label,
                        index,
                    } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.tagged_begin(tag.as_ref(), label.as_ref(), index.as_ref())?;
                            stream.value(body)?;
                            stream.tagged_end(tag.as_ref(), label.as_ref(), index.as_ref())
                        })?;
                    }
                    ValueKind::Record {
                        len,
                        tag,
                        label,
                        index,
                        num_entries,
                    } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.record_begin(
                                tag.as_ref(),
                                label.as_ref(),
                                index.as_ref(),
                                *num_entries,
                            )?;
                            body.stream(stream)?;
                            stream.record_end(tag.as_ref(), label.as_ref(), index.as_ref())
                        })?;
                    }
                    ValueKind::RecordValue { len, tag, label } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.record_value_begin(tag.as_ref(), label)?;
                            stream.value(body)?;
                            stream.record_value_end(tag.as_ref(), label)
                        })?;
                    }
                    ValueKind::Tuple {
                        len,
                        tag,
                        label,
                        index,
                        num_entries,
                    } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.tuple_begin(
                                tag.as_ref(),
                                label.as_ref(),
                                index.as_ref(),
                                *num_entries,
                            )?;
                            body.stream(stream)?;
                            stream.tuple_end(tag.as_ref(), label.as_ref(), index.as_ref())
                        })?;
                    }
                    ValueKind::TupleValue { len, tag, index } => {
                        stream_value(stream, &mut i, *len, self, |stream, body| {
                            stream.tuple_value_begin(tag.as_ref(), index)?;
                            stream.value(body)?;
                            stream.tuple_value_end(tag.as_ref(), index)
                        })?;
                    }
                }

                i += 1;
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::std::vec;

        use sval::Stream as _;
        use sval_derive::*;

        #[test]
        fn buffer_primitive() {
            for (value, expected) in [
                (
                    ValueBuf::collect(&true).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::Bool(true),
                    }],
                ),
                (
                    ValueBuf::collect(&1i8).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::I8(1),
                    }],
                ),
                (
                    ValueBuf::collect(&2i16).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::I16(2),
                    }],
                ),
                (
                    ValueBuf::collect(&3i32).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::I32(3),
                    }],
                ),
                (
                    ValueBuf::collect(&4i64).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::I64(4),
                    }],
                ),
                (
                    ValueBuf::collect(&5i128).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::I128(5),
                    }],
                ),
                (
                    ValueBuf::collect(&1u8).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::U8(1),
                    }],
                ),
                (
                    ValueBuf::collect(&2u16).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::U16(2),
                    }],
                ),
                (
                    ValueBuf::collect(&3u32).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::U32(3),
                    }],
                ),
                (
                    ValueBuf::collect(&4u64).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::U64(4),
                    }],
                ),
                (
                    ValueBuf::collect(&5u128).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::U128(5),
                    }],
                ),
                (
                    ValueBuf::collect(&3.14f32).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::F32(3.14),
                    }],
                ),
                (
                    ValueBuf::collect(&3.1415f64).unwrap(),
                    vec![ValuePart {
                        kind: ValueKind::F64(3.1415),
                    }],
                ),
            ] {
                assert_eq!(expected, value.parts, "{:?}", value);
            }
        }

        #[test]
        fn buffer_option() {
            let expected = vec![ValuePart {
                kind: ValueKind::Tag {
                    tag: Some(sval::Tag::new("rnone")),
                    label: Some(sval::Label::new("None")),
                    index: Some(sval::Index::new(0)),
                },
            }];

            assert_eq!(expected, ValueBuf::collect(&None::<i32>).unwrap().parts);

            let expected = vec![
                ValuePart {
                    kind: ValueKind::Tagged {
                        len: 1,
                        tag: Some(sval::Tag::new("rsome")),
                        label: Some(sval::Label::new("Some")),
                        index: Some(sval::Index::new(1)),
                    },
                },
                ValuePart {
                    kind: ValueKind::I32(42),
                },
            ];

            assert_eq!(expected, ValueBuf::collect(&Some(42i32)).unwrap().parts);
        }

        #[test]
        fn buffer_map() {
            let mut value = ValueBuf::new();

            value.map_begin(Some(2)).unwrap();

            value.map_key_begin().unwrap();
            value.i32(0).unwrap();
            value.map_key_end().unwrap();

            value.map_value_begin().unwrap();
            value.bool(false).unwrap();
            value.map_value_end().unwrap();

            value.map_key_begin().unwrap();
            value.i32(1).unwrap();
            value.map_key_end().unwrap();

            value.map_value_begin().unwrap();
            value.bool(true).unwrap();
            value.map_value_end().unwrap();

            value.map_end().unwrap();

            let expected = vec![
                ValuePart {
                    kind: ValueKind::Map {
                        len: 8,
                        num_entries_hint: Some(2),
                    },
                },
                ValuePart {
                    kind: ValueKind::MapKey { len: 1 },
                },
                ValuePart {
                    kind: ValueKind::I32(0),
                },
                ValuePart {
                    kind: ValueKind::MapValue { len: 1 },
                },
                ValuePart {
                    kind: ValueKind::Bool(false),
                },
                ValuePart {
                    kind: ValueKind::MapKey { len: 1 },
                },
                ValuePart {
                    kind: ValueKind::I32(1),
                },
                ValuePart {
                    kind: ValueKind::MapValue { len: 1 },
                },
                ValuePart {
                    kind: ValueKind::Bool(true),
                },
            ];

            assert_eq!(expected, value.parts);
        }

        #[test]
        fn buffer_seq() {
            let mut value = ValueBuf::new();

            value.seq_begin(Some(2)).unwrap();

            value.seq_value_begin().unwrap();
            value.bool(false).unwrap();
            value.seq_value_end().unwrap();

            value.seq_value_begin().unwrap();
            value.bool(true).unwrap();
            value.seq_value_end().unwrap();

            value.seq_end().unwrap();

            let expected = vec![
                ValuePart {
                    kind: ValueKind::Seq {
                        len: 4,
                        num_entries_hint: Some(2),
                    },
                },
                ValuePart {
                    kind: ValueKind::SeqValue { len: 1 },
                },
                ValuePart {
                    kind: ValueKind::Bool(false),
                },
                ValuePart {
                    kind: ValueKind::SeqValue { len: 1 },
                },
                ValuePart {
                    kind: ValueKind::Bool(true),
                },
            ];

            assert_eq!(expected, value.parts);
        }

        #[test]
        fn buffer_record() {
            let mut value = ValueBuf::new();

            value
                .record_begin(
                    Some(&sval::Tag::new("test")),
                    Some(&sval::Label::new("A")),
                    Some(&sval::Index::new(1)),
                    Some(2),
                )
                .unwrap();

            value
                .record_value_begin(None, &sval::Label::new("a"))
                .unwrap();
            value.bool(false).unwrap();
            value
                .record_value_end(None, &sval::Label::new("a"))
                .unwrap();

            value
                .record_value_begin(None, &sval::Label::new("b"))
                .unwrap();
            value.bool(true).unwrap();
            value
                .record_value_end(None, &sval::Label::new("b"))
                .unwrap();

            value
                .record_end(
                    Some(&sval::Tag::new("test")),
                    Some(&sval::Label::new("A")),
                    Some(&sval::Index::new(1)),
                )
                .unwrap();

            let expected = vec![
                ValuePart {
                    kind: ValueKind::Record {
                        len: 4,
                        tag: Some(sval::Tag::new("test")),
                        label: Some(sval::Label::new("A")),
                        index: Some(sval::Index::new(1)),
                        num_entries: Some(2),
                    },
                },
                ValuePart {
                    kind: ValueKind::RecordValue {
                        len: 1,
                        tag: None,
                        label: sval::Label::new("a"),
                    },
                },
                ValuePart {
                    kind: ValueKind::Bool(false),
                },
                ValuePart {
                    kind: ValueKind::RecordValue {
                        len: 1,
                        tag: None,
                        label: sval::Label::new("b"),
                    },
                },
                ValuePart {
                    kind: ValueKind::Bool(true),
                },
            ];

            assert_eq!(expected, value.parts);
        }

        #[test]
        fn buffer_tuple() {
            let mut value = ValueBuf::new();

            value
                .tuple_begin(
                    Some(&sval::Tag::new("test")),
                    Some(&sval::Label::new("A")),
                    Some(&sval::Index::new(1)),
                    Some(2),
                )
                .unwrap();

            value.tuple_value_begin(None, &sval::Index::new(0)).unwrap();
            value.bool(false).unwrap();
            value.tuple_value_end(None, &sval::Index::new(0)).unwrap();

            value.tuple_value_begin(None, &sval::Index::new(1)).unwrap();
            value.bool(true).unwrap();
            value.tuple_value_end(None, &sval::Index::new(1)).unwrap();

            value
                .tuple_end(
                    Some(&sval::Tag::new("test")),
                    Some(&sval::Label::new("A")),
                    Some(&sval::Index::new(1)),
                )
                .unwrap();

            let expected = vec![
                ValuePart {
                    kind: ValueKind::Tuple {
                        len: 4,
                        tag: Some(sval::Tag::new("test")),
                        label: Some(sval::Label::new("A")),
                        index: Some(sval::Index::new(1)),
                        num_entries: Some(2),
                    },
                },
                ValuePart {
                    kind: ValueKind::TupleValue {
                        len: 1,
                        tag: None,
                        index: sval::Index::new(0),
                    },
                },
                ValuePart {
                    kind: ValueKind::Bool(false),
                },
                ValuePart {
                    kind: ValueKind::TupleValue {
                        len: 1,
                        tag: None,
                        index: sval::Index::new(1),
                    },
                },
                ValuePart {
                    kind: ValueKind::Bool(true),
                },
            ];

            assert_eq!(expected, value.parts);
        }

        #[test]
        fn buffer_enum() {
            let mut value = ValueBuf::new();

            value
                .enum_begin(
                    Some(&sval::Tag::new("test")),
                    Some(&sval::Label::new("A")),
                    Some(&sval::Index::new(1)),
                )
                .unwrap();

            value
                .tag(
                    None,
                    Some(&sval::Label::new("B")),
                    Some(&sval::Index::new(0)),
                )
                .unwrap();

            value
                .enum_end(
                    Some(&sval::Tag::new("test")),
                    Some(&sval::Label::new("A")),
                    Some(&sval::Index::new(1)),
                )
                .unwrap();

            let expected = vec![
                ValuePart {
                    kind: ValueKind::Enum {
                        len: 1,
                        tag: Some(sval::Tag::new("test")),
                        label: Some(sval::Label::new("A")),
                        index: Some(sval::Index::new(1)),
                    },
                },
                ValuePart {
                    kind: ValueKind::Tag {
                        tag: None,
                        label: Some(sval::Label::new("B")),
                        index: Some(sval::Index::new(0)),
                    },
                },
            ];

            assert_eq!(expected, value.parts);
        }

        #[test]
        fn buffer_roundtrip() {
            for value_1 in [
                ValueBuf::collect(&42i32).unwrap(),
                ValueBuf::collect(&vec![
                    vec![],
                    vec![vec![1, 2, 3], vec![4]],
                    vec![vec![5, 6], vec![7, 8, 9]],
                ])
                .unwrap(),
                ValueBuf::collect(&{
                    #[derive(Value)]
                    struct Record {
                        a: i32,
                        b: bool,
                    }

                    Record { a: 42, b: true }
                })
                .unwrap(),
                ValueBuf::collect(&{
                    #[derive(Value)]
                    struct Tuple(i32, bool);

                    Tuple(42, true)
                })
                .unwrap(),
                ValueBuf::collect(&{
                    #[derive(Value)]
                    enum Enum {
                        A,
                    }

                    Enum::A
                })
                .unwrap(),
            ] {
                let value_2 = ValueBuf::collect(&value_1).unwrap();

                assert_eq!(value_1.parts, value_2.parts, "{:?}", value_1);
            }
        }
    }
}
