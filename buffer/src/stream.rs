#![allow(missing_docs)]

use core::marker::PhantomData;

use crate::{Error, Result};

use self::flat::FlatStream;

mod flat;
mod flat_enum;

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
            let mut stream = FlatStream::new(self);
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
            let mut stream = FlatStream::new(self);
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

impl<Ok> Default for Unsupported<Ok> {
    fn default() -> Self {
        Unsupported(PhantomData)
    }
}

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_primitive() {
        assert_eq!(Value::I64(42), ToValue.value(&42i64).unwrap());
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
            ToValue.value(&Layer).unwrap()
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
