#![allow(missing_docs)]

use core::marker::PhantomData;

use crate::{Error, Result};

use self::flat::FlatStream;

mod flat;
mod flat_enum;

pub trait Stream<'sval> {
    type Ok;

    type Seq: StreamSeq<'sval, Ok = Self::Ok>;
    type Map: StreamMap<'sval, Ok = Self::Ok>;

    type Record: StreamRecord<'sval, Ok = Self::Ok>;

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
        default_stream::i8(self, value)
    }

    fn i16(self, value: i16) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::i16(self, value)
    }

    fn i32(self, value: i32) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::i32(self, value)
    }

    fn i64(self, value: i64) -> Result<Self::Ok>;

    fn i128(self, value: i128) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::i128(self, value)
    }

    fn u8(self, value: u8) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u8(self, value)
    }

    fn u16(self, value: u16) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u16(self, value)
    }

    fn u32(self, value: u32) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u32(self, value)
    }

    fn u64(self, value: u64) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u64(self, value)
    }

    fn u128(self, value: u128) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::u128(self, value)
    }

    fn f32(self, value: f32) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::f32(self, value)
    }

    fn f64(self, value: f64) -> Result<Self::Ok>;

    fn text(self, text: &'sval str) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::text(self, text)
    }

    fn text_computed(self, text: &str) -> Result<Self::Ok>;

    fn binary(self, binary: &'sval [u8]) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::binary(self, binary)
    }

    fn binary_computed(self, binary: &[u8]) -> Result<Self::Ok>
    where
        Self: Sized,
    {
        default_stream::binary_computed(self, binary)
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

    fn seq_begin(self, num_entries: Option<usize>) -> Result<Self::Seq>;

    fn map_begin(self, num_entries: Option<usize>) -> Result<Self::Map>;

    fn record_begin(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> Result<Self::Record>;

    fn enum_begin(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> Result<Self::Enum>;
}

pub trait StreamSeq<'sval> {
    type Ok;

    fn value<V: sval::Value + ?Sized>(&mut self, value: &'sval V) -> Result {
        default_stream::seq_value(self, value)
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> Result;

    fn end(self) -> Result<Self::Ok>;
}

pub trait StreamMap<'sval> {
    type Ok;

    fn key<V: sval::Value + ?Sized>(&mut self, key: &'sval V) -> Result {
        default_stream::map_key(self, key)
    }

    fn key_computed<V: sval::Value + ?Sized>(&mut self, key: &V) -> Result;

    fn value<V: sval::Value + ?Sized>(&mut self, value: &'sval V) -> Result {
        default_stream::map_value(self, value)
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> Result;

    fn end(self) -> Result<Self::Ok>;
}

pub trait StreamRecord<'sval> {
    type Ok;

    fn value<V: sval::Value + ?Sized>(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
        value: &'sval V,
    ) -> Result {
        default_stream::record_value(self, tag, label, value)
    }

    fn value_computed<V: sval::Value + ?Sized>(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
        value: &V,
    ) -> Result;

    fn end(self) -> Result<Self::Ok>;
}

pub trait StreamEnum<'sval> {
    type Ok;

    type Record: StreamRecord<'sval, Ok = Self::Ok>;
    type Nested: StreamEnum<'sval, Ok = Self::Ok, Nested = Self::Nested>;

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
        default_stream::enum_tagged(self, tag, label, index, value)
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &V,
    ) -> Result<Self::Ok>;

    fn record_begin(
        self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> Result<Self::Record>;

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

impl<Ok> Default for Unsupported<Ok> {
    fn default() -> Self {
        Unsupported(PhantomData)
    }
}

impl<'sval, Ok> Stream<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    type Seq = Self;
    type Map = Self;
    type Record = Self;
    type Enum = Self;

    fn null(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("null is unsupported"))
    }

    fn bool(self, _: bool) -> Result<Self::Ok> {
        Err(Error::invalid_value("booleans are unsupported"))
    }

    fn i64(self, _: i64) -> Result<Self::Ok> {
        Err(Error::invalid_value("numbers are unsupported"))
    }

    fn f64(self, _: f64) -> Result<Self::Ok> {
        Err(Error::invalid_value("numbers are unsupported"))
    }

    fn text_computed(self, _: &str) -> Result<Self::Ok> {
        Err(Error::invalid_value("text is unsupported"))
    }

    fn tag(
        self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("tags are unsupported"))
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
        _: &V,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("tagged values are unsupported"))
    }

    fn seq_begin(self, _: Option<usize>) -> Result<Self::Seq> {
        Err(Error::invalid_value("sequences are unsupported"))
    }

    fn map_begin(self, _: Option<usize>) -> Result<Self::Map> {
        Err(Error::invalid_value("maps are unsupported"))
    }

    fn record_begin(
        self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
        _: Option<usize>,
    ) -> Result<Self::Record> {
        Err(Error::invalid_value("records are unsupported"))
    }

    fn enum_begin(
        self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> Result<Self::Enum> {
        Err(Error::invalid_value("enums are unsupported"))
    }
}

impl<'sval, Ok> StreamSeq<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    fn value_computed<V: sval::Value + ?Sized>(&mut self, _: &V) -> Result {
        Err(Error::invalid_value("sequences are unsupported"))
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("sequences are unsupported"))
    }
}

impl<'sval, Ok> StreamMap<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    fn key_computed<V: sval::Value + ?Sized>(&mut self, _: &V) -> Result {
        Err(Error::invalid_value("maps are unsupported"))
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, _: &V) -> Result {
        Err(Error::invalid_value("maps are unsupported"))
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("maps are unsupported"))
    }
}

impl<'sval, Ok> StreamRecord<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    fn value_computed<V: sval::Value + ?Sized>(
        &mut self,
        _: Option<&sval::Tag>,
        _: &sval::Label,
        _: &V,
    ) -> Result {
        Err(Error::invalid_value("records are unsupported"))
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("records are unsupported"))
    }
}

impl<'sval, Ok> StreamEnum<'sval> for Unsupported<Ok> {
    type Ok = Ok;

    type Record = Self;
    type Nested = Self;

    fn tag(
        self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("enums are unsupported"))
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
        _: &V,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("enums are unsupported"))
    }

    fn record_begin(
        self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
        _: Option<usize>,
    ) -> Result<Self::Record> {
        Err(Error::invalid_value("enums are unsupported"))
    }

    fn nested<F: FnOnce(Self::Nested) -> Result<<Self::Nested as StreamEnum<'sval>>::Ok>>(
        self,
        _: Option<&sval::Tag>,
        _: Option<&sval::Label>,
        _: Option<&sval::Index>,
        _: F,
    ) -> Result<Self::Ok> {
        Err(Error::invalid_value("enums are unsupported"))
    }

    fn end(self) -> Result<Self::Ok> {
        Err(Error::invalid_value("enums are unsupported"))
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

pub mod default_stream {
    use super::*;

    pub fn value<'sval, S: Stream<'sval>, V: sval::Value + ?Sized>(
        stream: S,
        value: &'sval V,
    ) -> Result<S::Ok> {
        let mut stream = FlatStream::new(stream);
        let _ = sval::default_stream::value(&mut stream, value);
        stream.finish()
    }

    pub fn value_computed<'sval, S: Stream<'sval>, V: sval::Value + ?Sized>(
        stream: S,
        value: &V,
    ) -> Result<S::Ok> {
        let mut stream = FlatStream::new(stream);
        let _ = sval::default_stream::value_computed(&mut stream, value);
        stream.finish()
    }

    pub fn i8<'sval, S: Stream<'sval>>(stream: S, value: i8) -> Result<S::Ok> {
        stream.i16(value as i16)
    }

    pub fn i16<'sval, S: Stream<'sval>>(stream: S, value: i16) -> Result<S::Ok> {
        stream.i32(value as i32)
    }

    pub fn i32<'sval, S: Stream<'sval>>(stream: S, value: i32) -> Result<S::Ok> {
        stream.i64(value as i64)
    }

    pub fn i128<'sval, S: Stream<'sval>>(stream: S, value: i128) -> Result<S::Ok> {
        if let Ok(value) = value.try_into() {
            stream.i64(value)
        } else {
            let mut stream = FlatStream::new(stream);
            let _ = sval::stream_number(&mut stream, value);
            stream.finish()
        }
    }

    pub fn u8<'sval, S: Stream<'sval>>(stream: S, value: u8) -> Result<S::Ok> {
        stream.u16(value as u16)
    }

    pub fn u16<'sval, S: Stream<'sval>>(stream: S, value: u16) -> Result<S::Ok> {
        stream.u32(value as u32)
    }

    pub fn u32<'sval, S: Stream<'sval>>(stream: S, value: u32) -> Result<S::Ok> {
        stream.u64(value as u64)
    }

    pub fn u64<'sval, S: Stream<'sval>>(stream: S, value: u64) -> Result<S::Ok> {
        stream.u128(value as u128)
    }

    pub fn u128<'sval, S: Stream<'sval>>(stream: S, value: u128) -> Result<S::Ok> {
        if let Ok(value) = value.try_into() {
            stream.i64(value)
        } else {
            let mut stream = FlatStream::new(stream);
            let _ = sval::stream_number(&mut stream, value);
            stream.finish()
        }
    }

    pub fn f32<'sval, S: Stream<'sval>>(stream: S, value: f32) -> Result<S::Ok> {
        stream.f64(value as f64)
    }

    pub fn text<'sval, S: Stream<'sval>>(stream: S, text: &'sval str) -> Result<S::Ok> {
        stream.text_computed(text)
    }

    pub fn binary<'sval, S: Stream<'sval>>(stream: S, binary: &'sval [u8]) -> Result<S::Ok> {
        stream.binary_computed(binary)
    }

    pub fn binary_computed<'sval, S: Stream<'sval>>(stream: S, binary: &[u8]) -> Result<S::Ok> {
        let mut seq = stream.seq_begin(Some(binary.len()))?;

        for b in binary {
            seq.value_computed(b)?;
        }

        seq.end()
    }

    pub fn seq_value<'sval, S: StreamSeq<'sval> + ?Sized, V: sval::Value + ?Sized>(
        seq: &mut S,
        value: &'sval V,
    ) -> Result {
        seq.value_computed(value)
    }

    pub fn map_key<'sval, S: StreamMap<'sval> + ?Sized, V: sval::Value + ?Sized>(
        map: &mut S,
        key: &'sval V,
    ) -> Result {
        map.key_computed(key)
    }

    pub fn map_value<'sval, S: StreamMap<'sval> + ?Sized, V: sval::Value + ?Sized>(
        map: &mut S,
        value: &'sval V,
    ) -> Result {
        map.value_computed(value)
    }

    pub fn record_value<'sval, S: StreamRecord<'sval> + ?Sized, V: sval::Value + ?Sized>(
        record: &mut S,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
        value: &'sval V,
    ) -> Result {
        record.value_computed(tag, label, value)
    }

    pub fn enum_tagged<'sval, S: StreamEnum<'sval>, V: sval::Value + ?Sized>(
        stream: S,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        value: &'sval V,
    ) -> Result<S::Ok> {
        stream.tagged_computed(tag, label, index, value)
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;

    use super::*;

    #[test]
    fn stream_primitive() {
        assert_eq!(Value::Null, ToValue::default().value(&sval::Null).unwrap());
        assert_eq!(Value::I64(42), ToValue::default().value(&42i64).unwrap());
        assert_eq!(Value::F64(42.1), ToValue::default().value(&42.1).unwrap());
        assert_eq!(Value::Bool(true), ToValue::default().value(&true).unwrap());
    }

    #[test]
    fn stream_text_borrowed() {
        assert_eq!(
            Value::Text(Cow::Borrowed("borrowed")),
            ToValue::default().value("borrowed").unwrap()
        );
    }

    #[test]
    fn stream_text_owned() {
        assert_eq!(
            Value::Text(Cow::Owned("owned".into())),
            ToValue::default()
                .value(&{
                    struct Text;

                    impl sval::Value for Text {
                        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                            &'sval self,
                            stream: &mut S,
                        ) -> sval::Result {
                            stream.text_begin(None)?;

                            stream.text_fragment("ow")?;
                            stream.text_fragment("ned")?;

                            stream.text_end()
                        }
                    }

                    Text
                })
                .unwrap()
        );
    }

    #[test]
    fn stream_seq() {
        assert_eq!(
            Value::Seq(Seq {
                entries: vec![Value::I64(1), Value::I64(2), Value::I64(3),]
            }),
            ToValue::default().value(&[1, 2, 3] as &[_]).unwrap()
        );
    }

    #[test]
    fn stream_map() {
        assert_eq!(
            Value::Map(Map {
                entries: vec![
                    (Value::Text(Cow::Borrowed("a")), Value::I64(1)),
                    (Value::Text(Cow::Borrowed("b")), Value::I64(2)),
                    (Value::Text(Cow::Borrowed("c")), Value::I64(3)),
                ]
            }),
            ToValue::default()
                .value(sval::MapSlice::new(&[("a", 1), ("b", 2), ("c", 3),]))
                .unwrap()
        );
    }

    #[test]
    fn stream_record() {
        assert_eq!(
            Value::Record(Record {
                tag: Tag::new(None, Some(&sval::Label::new("Record")), None).unwrap(),
                entries: vec![
                    (sval::Label::new("a"), Value::I64(1)),
                    (sval::Label::new("b"), Value::Bool(true)),
                ]
            }),
            ToValue::default()
                .value(&{
                    struct Record;

                    impl sval::Value for Record {
                        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                            &'sval self,
                            stream: &mut S,
                        ) -> sval::Result {
                            stream.record_begin(
                                None,
                                Some(&sval::Label::new("Record")),
                                None,
                                None,
                            )?;

                            stream.record_value_begin(None, &sval::Label::new("a"))?;
                            stream.i64(1)?;
                            stream.record_value_end(None, &sval::Label::new("a"))?;

                            stream.record_value_begin(None, &sval::Label::new("b"))?;
                            stream.bool(true)?;
                            stream.record_value_end(None, &sval::Label::new("b"))?;

                            stream.record_end(None, Some(&sval::Label::new("Record")), None)
                        }
                    }

                    Record
                })
                .unwrap(),
        )
    }

    #[test]
    fn stream_enum_record_variant() {
        assert_eq!(
            Value::Enum(Enum {
                tag: Tag::new(None, Some(&sval::Label::new("Enum")), None).unwrap(),
                variant: Some(Variant::Record(Record {
                    tag: Tag::new(
                        None,
                        Some(&sval::Label::new("Record")),
                        Some(&sval::Index::new(0))
                    )
                    .unwrap(),
                    entries: vec![
                        (sval::Label::new("a"), Value::I64(1)),
                        (sval::Label::new("b"), Value::Bool(true)),
                    ]
                })),
            }),
            ToValue::default()
                .value(&{
                    struct RecordVariant;

                    impl sval::Value for RecordVariant {
                        fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                            &'sval self,
                            stream: &mut S,
                        ) -> sval::Result {
                            stream.enum_begin(None, Some(&sval::Label::new("Enum")), None)?;

                            stream.record_begin(
                                None,
                                Some(&sval::Label::new("Record")),
                                Some(&sval::Index::new(0)),
                                None,
                            )?;

                            stream.record_value_begin(None, &sval::Label::new("a"))?;
                            stream.i64(1)?;
                            stream.record_value_end(None, &sval::Label::new("a"))?;

                            stream.record_value_begin(None, &sval::Label::new("b"))?;
                            stream.bool(true)?;
                            stream.record_value_end(None, &sval::Label::new("b"))?;

                            stream.record_end(
                                None,
                                Some(&sval::Label::new("Record")),
                                Some(&sval::Index::new(0)),
                            )?;

                            stream.enum_end(None, Some(&sval::Label::new("Enum")), None)
                        }
                    }

                    RecordVariant
                })
                .unwrap(),
        )
    }

    #[test]
    fn stream_deeply_nested_enum() {
        struct Layer;

        impl sval::Value for Layer {
            fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(
                &'sval self,
                stream: &mut S,
            ) -> sval::Result {
                stream.enum_begin(None, Some(&sval::Label::new("Layer1")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer2")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer3")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer4")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer5")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer6")), None)?;
                stream.enum_begin(None, Some(&sval::Label::new("Layer7")), None)?;
                stream.tagged_begin(None, Some(&sval::Label::new("Value")), None)?;
                stream.i64(42)?;
                stream.tagged_end(None, Some(&sval::Label::new("Value")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer7")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer6")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer5")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer4")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer3")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer2")), None)?;
                stream.enum_end(None, Some(&sval::Label::new("Layer1")), None)
            }
        }

        assert_eq!(
            Value::Enum(Enum {
                tag: Tag::new(None, Some(&sval::Label::new("Layer1")), None).unwrap(),
                variant: Some(Variant::Enum(Box::new(Enum {
                    tag: Tag::new(None, Some(&sval::Label::new("Layer2")), None).unwrap(),
                    variant: Some(Variant::Enum(Box::new(Enum {
                        tag: Tag::new(None, Some(&sval::Label::new("Layer3")), None).unwrap(),
                        variant: Some(Variant::Enum(Box::new(Enum {
                            tag: Tag::new(None, Some(&sval::Label::new("Layer4")), None).unwrap(),
                            variant: Some(Variant::Enum(Box::new(Enum {
                                tag: Tag::new(None, Some(&sval::Label::new("Layer5")), None)
                                    .unwrap(),
                                variant: Some(Variant::Enum(Box::new(Enum {
                                    tag: Tag::new(None, Some(&sval::Label::new("Layer6")), None)
                                        .unwrap(),
                                    variant: Some(Variant::Enum(Box::new(Enum {
                                        tag: Tag::new(
                                            None,
                                            Some(&sval::Label::new("Layer7")),
                                            None
                                        )
                                        .unwrap(),
                                        variant: Some(Variant::Tagged(Tagged {
                                            tag: Tag::new(
                                                None,
                                                Some(&sval::Label::new("Value")),
                                                None
                                            )
                                            .unwrap(),
                                            value: Box::new(Value::I64(42)),
                                        }))
                                    })))
                                })))
                            })))
                        })))
                    })))
                })))
            }),
            ToValue::default().value(&Layer).unwrap()
        );
    }

    #[derive(Debug, PartialEq)]
    enum Value<'sval> {
        Null,
        Bool(bool),
        I64(i64),
        F64(f64),
        Text(Cow<'sval, str>),
        Tag(Tag),
        Tagged(Tagged<'sval>),
        Seq(Seq<'sval>),
        Map(Map<'sval>),
        Record(Record<'sval>),
        Enum(Enum<'sval>),
    }

    impl<'sval> Value<'sval> {
        fn try_into_variant(self) -> Result<Variant<'sval>, Error> {
            match self {
                Value::Tag(variant) => Ok(Variant::Tag(variant)),
                Value::Tagged(variant) => Ok(Variant::Tagged(variant)),
                Value::Record(variant) => Ok(Variant::Record(variant)),
                Value::Enum(variant) => Ok(Variant::Enum(Box::new(variant))),
                _ => Err(Error::invalid_value("expected an enum variant")),
            }
        }
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
    struct Tagged<'sval> {
        tag: Tag,
        value: Box<Value<'sval>>,
    }

    #[derive(Debug, PartialEq)]
    struct Seq<'sval> {
        entries: Vec<Value<'sval>>,
    }

    #[derive(Debug, PartialEq)]
    struct Map<'sval> {
        entries: Vec<(Value<'sval>, Value<'sval>)>,
    }

    #[derive(Debug, PartialEq)]
    struct Record<'sval> {
        tag: Tag,
        entries: Vec<(sval::Label<'static>, Value<'sval>)>,
    }

    #[derive(Debug, PartialEq)]
    struct Enum<'sval> {
        tag: Tag,
        variant: Option<Variant<'sval>>,
    }

    #[derive(Debug, PartialEq)]
    enum Variant<'sval> {
        Tag(Tag),
        Tagged(Tagged<'sval>),
        Record(Record<'sval>),
        Enum(Box<Enum<'sval>>),
    }

    #[derive(Default)]
    struct ToValue<'sval>(PhantomData<Value<'sval>>);

    struct ToMap<'sval> {
        key: Option<Value<'sval>>,
        map: Map<'sval>,
    }

    struct ToSeq<'sval> {
        seq: Seq<'sval>,
    }

    struct ToRecord<'sval> {
        record: Record<'sval>,
    }

    struct ToEnum<'sval> {
        tag: Tag,
        _marker: PhantomData<Enum<'sval>>,
    }

    struct ToVariant<S> {
        tag: Tag,
        stream: S,
    }

    impl<'sval> Stream<'sval> for ToValue<'sval> {
        type Ok = Value<'sval>;

        type Seq = ToSeq<'sval>;
        type Map = ToMap<'sval>;

        type Record = ToRecord<'sval>;
        type Enum = ToEnum<'sval>;

        fn null(self) -> Result<Self::Ok> {
            Ok(Value::Null)
        }

        fn bool(self, value: bool) -> Result<Self::Ok> {
            Ok(Value::Bool(value))
        }

        fn i64(self, value: i64) -> Result<Self::Ok> {
            Ok(Value::I64(value))
        }

        fn f64(self, value: f64) -> Result<Self::Ok> {
            Ok(Value::F64(value))
        }

        fn text(self, text: &'sval str) -> Result<Self::Ok> {
            Ok(Value::Text(Cow::Borrowed(text)))
        }

        fn text_computed(self, text: &str) -> Result<Self::Ok> {
            Ok(Value::Text(Cow::Owned(text.into())))
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
            let value = ToValue::default().value_computed(value)?;

            Ok(Value::Tagged(Tagged {
                tag,
                value: Box::new(value),
            }))
        }

        fn seq_begin(self, _: Option<usize>) -> Result<Self::Seq> {
            Ok(ToSeq {
                seq: Seq {
                    entries: Vec::new(),
                },
            })
        }

        fn map_begin(self, _: Option<usize>) -> Result<Self::Map> {
            Ok(ToMap {
                key: None,
                map: Map {
                    entries: Vec::new(),
                },
            })
        }

        fn record_begin(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            _: Option<usize>,
        ) -> Result<Self::Record> {
            Ok(ToRecord {
                record: Record {
                    tag: Tag::new(tag, label, index)?,
                    entries: Default::default(),
                },
            })
        }

        fn enum_begin(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> Result<Self::Enum> {
            Ok(ToEnum {
                tag: Tag::new(tag, label, index)?,
                _marker: Default::default(),
            })
        }
    }

    impl<'sval> StreamSeq<'sval> for ToSeq<'sval> {
        type Ok = Value<'sval>;

        fn value<V: sval::Value + ?Sized>(&mut self, value: &'sval V) -> Result {
            let value = ToValue::default().value(value)?;

            self.seq.entries.push(value);

            Ok(())
        }

        fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> Result {
            let value = ToValue::default().value_computed(value)?;

            self.seq.entries.push(value);

            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Seq(self.seq))
        }
    }

    impl<'sval> StreamMap<'sval> for ToMap<'sval> {
        type Ok = Value<'sval>;

        fn key<V: sval::Value + ?Sized>(&mut self, key: &'sval V) -> Result {
            self.key = Some(ToValue::default().value(key)?);

            Ok(())
        }

        fn key_computed<V: sval::Value + ?Sized>(&mut self, key: &V) -> Result {
            self.key = Some(ToValue::default().value_computed(key)?);

            Ok(())
        }

        fn value<V: sval::Value + ?Sized>(&mut self, value: &'sval V) -> Result {
            let key = self.key.take().unwrap();
            let value = ToValue::default().value(value)?;

            self.map.entries.push((key, value));

            Ok(())
        }

        fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> Result {
            let key = self.key.take().unwrap();
            let value = ToValue::default().value_computed(value)?;

            self.map.entries.push((key, value));

            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Map(self.map))
        }
    }

    impl<'sval> StreamRecord<'sval> for ToRecord<'sval> {
        type Ok = Value<'sval>;

        fn value<V: sval::Value + ?Sized>(
            &mut self,
            _: Option<&sval::Tag>,
            label: &sval::Label,
            value: &'sval V,
        ) -> Result {
            let label = owned_label(label)?;
            let value = ToValue::default().value(value)?;

            self.record.entries.push((label, value));

            Ok(())
        }

        fn value_computed<V: sval::Value + ?Sized>(
            &mut self,
            _: Option<&sval::Tag>,
            label: &sval::Label,
            value: &V,
        ) -> Result {
            let label = owned_label(label)?;
            let value = ToValue::default().value_computed(value)?;

            self.record.entries.push((label, value));

            Ok(())
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Record(self.record))
        }
    }

    impl<'sval> StreamEnum<'sval> for ToEnum<'sval> {
        type Ok = Value<'sval>;

        type Record = ToVariant<ToRecord<'sval>>;
        type Nested = Self;

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
            let value = ToValue::default().value_computed(value)?;

            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(Variant::Tagged(Tagged {
                    tag,
                    value: Box::new(value),
                })),
            }))
        }

        fn record_begin(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            _: Option<usize>,
        ) -> Result<Self::Record> {
            Ok(ToVariant {
                tag: self.tag,
                stream: ToRecord {
                    record: Record {
                        tag: Tag::new(tag, label, index)?,
                        entries: Vec::new(),
                    },
                },
            })
        }

        fn nested<F: FnOnce(Self::Nested) -> Result<<Self::Nested as StreamEnum<'sval>>::Ok>>(
            self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            variant: F,
        ) -> Result<Self::Ok> {
            let variant = variant(ToEnum {
                tag: Tag::new(tag, label, index)?,
                _marker: PhantomData,
            })?
            .try_into_variant()?;

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

    impl<'sval> StreamRecord<'sval> for ToVariant<ToRecord<'sval>> {
        type Ok = Value<'sval>;

        fn value<V: sval::Value + ?Sized>(
            &mut self,
            tag: Option<&sval::Tag>,
            label: &sval::Label,
            value: &'sval V,
        ) -> Result {
            self.stream.value(tag, label, value)
        }

        fn value_computed<V: sval::Value + ?Sized>(
            &mut self,
            tag: Option<&sval::Tag>,
            label: &sval::Label,
            value: &V,
        ) -> Result {
            self.stream.value_computed(tag, label, value)
        }

        fn end(self) -> Result<Self::Ok> {
            Ok(Value::Enum(Enum {
                tag: self.tag,
                variant: Some(Variant::Record(self.stream.record)),
            }))
        }
    }
}
