use serde::ser::{Error as _, Serialize as _};

use sval_buffer::{
    Stream, StreamEnum, StreamMap, StreamRecord, StreamSeq, StreamTuple, Unsupported,
};

/**
Serialize an [`sval::Value`] into a [`serde::Serializer`].
*/
pub fn serialize<S: serde::Serializer>(
    serializer: S,
    value: impl sval::Value,
) -> Result<S::Ok, S::Error> {
    ToSerialize::new(value).serialize(serializer)
}

/**
Adapt an [`sval::Value`] into a [`serde::Serialize`].
*/
#[repr(transparent)]
pub struct ToSerialize<V: ?Sized>(V);

impl<V: sval::Value> ToSerialize<V> {
    /**
    Adapt an [`sval::Value`] into a [`serde::Serialize`].
    */
    pub const fn new(value: V) -> ToSerialize<V> {
        ToSerialize(value)
    }
}

impl<V: sval::Value + ?Sized> ToSerialize<V> {
    /**
    Adapt a reference to an [`sval::Value`] into a [`serde::Serialize`].
    */
    pub const fn new_borrowed<'a>(value: &'a V) -> &'a ToSerialize<V> {
        // SAFETY: `&'a V` and `&'a ToSerialize<V>` have the same ABI
        unsafe { &*(value as *const _ as *const ToSerialize<V>) }
    }
}

impl<V: sval::Value> serde::Serialize for ToSerialize<V> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Serializer::new(serializer)
            .value(&self.0)
            .unwrap_or_else(|e| Err(S::Error::custom(e)))
    }
}

struct Serializer<S> {
    serializer: S,
}

struct SerializeSeq<S, E> {
    serializer: Result<S, E>,
}

struct SerializeMap<S, E> {
    serializer: Result<S, E>,
}

struct SerializeTuple<TNamed, TUnnamed, E> {
    serializer: Result<MaybeNamed<TNamed, TUnnamed>, E>,
}

enum MaybeNamed<TNamed, TUnnamed> {
    Named { serializer: TNamed },
    Unnamed { serializer: TUnnamed },
}

struct SerializeRecord<TNamed, TUnnamed, E> {
    serializer: Result<MaybeNamed<TNamed, TUnnamed>, E>,
}

struct SerializeEnum<S> {
    name: &'static str,
    serializer: S,
}

struct SerializeRecordVariant<S, E> {
    serializer: Result<S, E>,
}

struct SerializeTupleVariant<S, E> {
    serializer: Result<S, E>,
}

impl<S> Serializer<S> {
    fn new(serializer: S) -> Self {
        Serializer { serializer }
    }
}

impl<'sval, S: serde::Serializer> Stream<'sval> for Serializer<S> {
    type Ok = Result<S::Ok, S::Error>;

    type Seq = SerializeSeq<S::SerializeSeq, S::Error>;

    type Map = SerializeMap<S::SerializeMap, S::Error>;

    type Tuple = SerializeTuple<S::SerializeTupleStruct, S::SerializeTuple, S::Error>;

    type Record = SerializeRecord<S::SerializeStruct, S::SerializeTuple, S::Error>;

    type Enum = SerializeEnum<S>;

    fn null(self) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_none())
    }

    fn bool(self, value: bool) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_bool(value))
    }

    fn i8(self, value: i8) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_i8(value))
    }

    fn i16(self, value: i16) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_i16(value))
    }

    fn i32(self, value: i32) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_i32(value))
    }

    fn i64(self, value: i64) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_i64(value))
    }

    fn i128(self, value: i128) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_i128(value))
    }

    fn u8(self, value: u8) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_u8(value))
    }

    fn u16(self, value: u16) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_u16(value))
    }

    fn u32(self, value: u32) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_u32(value))
    }

    fn u64(self, value: u64) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_u64(value))
    }

    fn u128(self, value: u128) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_u128(value))
    }

    fn f32(self, value: f32) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_f32(value))
    }

    fn f64(self, value: f64) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_f64(value))
    }

    fn text_computed(self, text: &str) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_str(text))
    }

    fn binary_computed(self, binary: &[u8]) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_bytes(binary))
    }

    fn tag(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        _: Option<sval::Index>,
    ) -> sval_buffer::Result<Self::Ok> {
        match tag {
            Some(sval::tags::RUST_OPTION_NONE) => Ok(self.serializer.serialize_none()),
            Some(sval::tags::RUST_UNIT) => Ok(self.serializer.serialize_unit()),
            _ => {
                let name = label
                    .and_then(|label| label.as_static_str())
                    .ok_or_else(|| sval_buffer::Error::invalid_value("missing unit label"))?;

                Ok(self.serializer.serialize_unit_struct(name))
            }
        }
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        _: Option<sval::Index>,
        value: &V,
    ) -> sval_buffer::Result<Self::Ok> {
        match tag {
            Some(sval::tags::RUST_OPTION_SOME) => {
                Ok(self.serializer.serialize_some(&ToSerialize::new(value)))
            }
            _ => {
                let name = label
                    .and_then(|label| label.as_static_str())
                    .ok_or_else(|| sval_buffer::Error::invalid_value("missing newtype label"))?;

                Ok(self
                    .serializer
                    .serialize_newtype_struct(name, &ToSerialize::new(value)))
            }
        }
    }

    fn seq_begin(self, num_entries: Option<usize>) -> sval_buffer::Result<Self::Seq> {
        Ok(SerializeSeq {
            serializer: self.serializer.serialize_seq(num_entries),
        })
    }

    fn map_begin(self, num_entries: Option<usize>) -> sval_buffer::Result<Self::Map> {
        Ok(SerializeMap {
            serializer: self.serializer.serialize_map(num_entries),
        })
    }

    fn tuple_begin(
        self,
        _: Option<sval::Tag>,
        label: Option<sval::Label>,
        _: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval_buffer::Result<Self::Tuple> {
        let len =
            num_entries.ok_or_else(|| sval_buffer::Error::invalid_value("missing tuple len"))?;

        match label {
            Some(label) => {
                let name = label
                    .as_static_str()
                    .ok_or_else(|| sval_buffer::Error::invalid_value("missing tuple label"))?;

                Ok(SerializeTuple {
                    serializer: self
                        .serializer
                        .serialize_tuple_struct(name, len)
                        .map(|serializer| MaybeNamed::Named { serializer }),
                })
            }
            None => Ok(SerializeTuple {
                serializer: self
                    .serializer
                    .serialize_tuple(len)
                    .map(|serializer| MaybeNamed::Unnamed { serializer }),
            }),
        }
    }

    fn record_begin(
        self,
        _: Option<sval::Tag>,
        label: Option<sval::Label>,
        _: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval_buffer::Result<Self::Record> {
        let len =
            num_entries.ok_or_else(|| sval_buffer::Error::invalid_value("missing tuple len"))?;

        match label {
            Some(label) => {
                let name = label
                    .as_static_str()
                    .ok_or_else(|| sval_buffer::Error::invalid_value("missing tuple label"))?;

                Ok(SerializeRecord {
                    serializer: self
                        .serializer
                        .serialize_struct(name, len)
                        .map(|serializer| MaybeNamed::Named { serializer }),
                })
            }
            None => Ok(SerializeRecord {
                serializer: self
                    .serializer
                    .serialize_tuple(len)
                    .map(|serializer| MaybeNamed::Unnamed { serializer }),
            }),
        }
    }

    fn enum_begin(
        self,
        _: Option<sval::Tag>,
        label: Option<sval::Label>,
        _: Option<sval::Index>,
    ) -> sval_buffer::Result<Self::Enum> {
        let name = label
            .and_then(|label| label.as_static_str())
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing enum label"))?;

        Ok(SerializeEnum {
            name,
            serializer: self.serializer,
        })
    }
}

impl<'sval, S: serde::ser::SerializeSeq> StreamSeq<'sval> for SerializeSeq<S, S::Error> {
    type Ok = Result<S::Ok, S::Error>;

    fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> sval_buffer::Result {
        if let Ok(ref mut serializer) = self.serializer {
            match serializer.serialize_element(&ToSerialize::new(value)) {
                Ok(()) => return Ok(()),
                Err(err) => {
                    self.serializer = Err(err);
                }
            }
        }

        Err(sval_buffer::Error::invalid_value(
            "failed to serialize sequence element",
        ))
    }

    fn end(self) -> sval_buffer::Result<Self::Ok> {
        match self.serializer {
            Ok(serializer) => Ok(serializer.end()),
            Err(err) => Ok(Err(err)),
        }
    }
}

impl<'sval, S: serde::ser::SerializeMap> StreamMap<'sval> for SerializeMap<S, S::Error> {
    type Ok = Result<S::Ok, S::Error>;

    fn key_computed<V: sval::Value + ?Sized>(&mut self, key: &V) -> sval_buffer::Result {
        if let Ok(ref mut serializer) = self.serializer {
            match serializer.serialize_key(&ToSerialize::new(key)) {
                Ok(()) => return Ok(()),
                Err(err) => {
                    self.serializer = Err(err);
                }
            }
        }

        Err(sval_buffer::Error::invalid_value(
            "failed to serialize map key",
        ))
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, value: &V) -> sval_buffer::Result {
        if let Ok(ref mut serializer) = self.serializer {
            match serializer.serialize_value(&ToSerialize::new(value)) {
                Ok(()) => return Ok(()),
                Err(err) => {
                    self.serializer = Err(err);
                }
            }
        }

        Err(sval_buffer::Error::invalid_value(
            "failed to serialize map value",
        ))
    }

    fn end(self) -> sval_buffer::Result<Self::Ok> {
        match self.serializer {
            Ok(serializer) => Ok(serializer.end()),
            Err(err) => Ok(Err(err)),
        }
    }
}

impl<
        'sval,
        TOk,
        TError,
        TNamed: serde::ser::SerializeStruct<Ok = TOk, Error = TError>,
        TUnnamed: serde::ser::SerializeTuple<Ok = TOk, Error = TError>,
    > StreamRecord<'sval> for SerializeRecord<TNamed, TUnnamed, TError>
{
    type Ok = Result<TOk, TError>;

    fn value_computed<V: sval::Value + ?Sized>(
        &mut self,
        _: Option<sval::Tag>,
        label: sval::Label,
        value: &V,
    ) -> sval_buffer::Result {
        match self.serializer {
            Ok(MaybeNamed::Named { ref mut serializer }) => {
                let field = label.as_static_str().ok_or_else(|| {
                    sval_buffer::Error::invalid_value("missing struct field label")
                })?;

                match serializer.serialize_field(field, &ToSerialize::new(value)) {
                    Ok(()) => return Ok(()),
                    Err(err) => {
                        self.serializer = Err(err);
                    }
                }
            }
            Ok(MaybeNamed::Unnamed { ref mut serializer }) => {
                match serializer.serialize_element(&ToSerialize::new(value)) {
                    Ok(()) => return Ok(()),
                    Err(err) => {
                        self.serializer = Err(err);
                    }
                }
            }
            Err(_) => (),
        }

        Err(sval_buffer::Error::invalid_value(
            "failed to serialize tuple field",
        ))
    }

    fn end(self) -> sval_buffer::Result<Self::Ok> {
        match self.serializer {
            Ok(MaybeNamed::Named { serializer }) => Ok(serializer.end()),
            Ok(MaybeNamed::Unnamed { serializer }) => Ok(serializer.end()),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl<
        'sval,
        TOk,
        TError,
        TNamed: serde::ser::SerializeTupleStruct<Ok = TOk, Error = TError>,
        TUnnamed: serde::ser::SerializeTuple<Ok = TOk, Error = TError>,
    > StreamTuple<'sval> for SerializeTuple<TNamed, TUnnamed, TError>
{
    type Ok = Result<TOk, TError>;

    fn value_computed<V: sval::Value + ?Sized>(
        &mut self,
        _: Option<sval::Tag>,
        _: sval::Index,
        value: &V,
    ) -> sval_buffer::Result {
        match self.serializer {
            Ok(MaybeNamed::Named { ref mut serializer }) => {
                match serializer.serialize_field(&ToSerialize::new(value)) {
                    Ok(()) => return Ok(()),
                    Err(err) => {
                        self.serializer = Err(err);
                    }
                }
            }
            Ok(MaybeNamed::Unnamed { ref mut serializer }) => {
                match serializer.serialize_element(&ToSerialize::new(value)) {
                    Ok(()) => return Ok(()),
                    Err(err) => {
                        self.serializer = Err(err);
                    }
                }
            }
            Err(_) => (),
        }

        Err(sval_buffer::Error::invalid_value(
            "failed to serialize tuple field",
        ))
    }

    fn end(self) -> sval_buffer::Result<Self::Ok> {
        match self.serializer {
            Ok(MaybeNamed::Named { serializer }) => Ok(serializer.end()),
            Ok(MaybeNamed::Unnamed { serializer }) => Ok(serializer.end()),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl<'sval, S: serde::Serializer> StreamEnum<'sval> for SerializeEnum<S> {
    type Ok = Result<S::Ok, S::Error>;

    type Tuple = SerializeTupleVariant<S::SerializeTupleVariant, S::Error>;

    type Record = SerializeRecordVariant<S::SerializeStructVariant, S::Error>;

    type Nested = Unsupported<Self::Ok>;

    fn tag(
        self,
        _: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval_buffer::Result<Self::Ok> {
        let variant = label
            .and_then(|label| label.as_static_str())
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing unit label"))?;

        let variant_index = index
            .and_then(|index| index.to_u32())
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing unit index"))?;

        Ok(self
            .serializer
            .serialize_unit_variant(self.name, variant_index, variant))
    }

    fn tagged_computed<V: sval::Value + ?Sized>(
        self,
        _: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: &V,
    ) -> sval_buffer::Result<Self::Ok> {
        let variant = label
            .and_then(|label| label.as_static_str())
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing newtype label"))?;

        let variant_index = index
            .and_then(|index| index.to_u32())
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing newtype index"))?;

        Ok(self.serializer.serialize_newtype_variant(
            self.name,
            variant_index,
            variant,
            &ToSerialize::new(value),
        ))
    }

    fn tuple_begin(
        self,
        _: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval_buffer::Result<Self::Tuple> {
        let variant = label
            .and_then(|label| label.as_static_str())
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing tuple label"))?;

        let variant_index = index
            .and_then(|index| index.to_u32())
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing tuple index"))?;

        let len =
            num_entries.ok_or_else(|| sval_buffer::Error::invalid_value("missing tuple len"))?;

        Ok(SerializeTupleVariant {
            serializer: self.serializer.serialize_tuple_variant(
                self.name,
                variant_index,
                variant,
                len,
            ),
        })
    }

    fn record_begin(
        self,
        _: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval_buffer::Result<Self::Record> {
        let variant = label
            .and_then(|label| label.as_static_str())
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing struct label"))?;

        let variant_index = index
            .and_then(|index| index.to_u32())
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing struct index"))?;

        let len =
            num_entries.ok_or_else(|| sval_buffer::Error::invalid_value("missing struct len"))?;

        Ok(SerializeRecordVariant {
            serializer: self.serializer.serialize_struct_variant(
                self.name,
                variant_index,
                variant,
                len,
            ),
        })
    }

    fn nested<
        F: FnOnce(Self::Nested) -> sval_buffer::Result<<Self::Nested as StreamEnum<'sval>>::Ok>,
    >(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        _: F,
    ) -> sval_buffer::Result<Self::Ok> {
        Err(sval_buffer::Error::invalid_value(
            "nested enums aren't supported",
        ))
    }

    fn end(self) -> sval_buffer::Result<Self::Ok> {
        Ok(self.serializer.serialize_unit_struct(self.name))
    }
}

impl<'sval, S: serde::ser::SerializeStructVariant> StreamRecord<'sval>
    for SerializeRecordVariant<S, S::Error>
{
    type Ok = Result<S::Ok, S::Error>;

    fn value_computed<V: sval::Value + ?Sized>(
        &mut self,
        _: Option<sval::Tag>,
        label: sval::Label,
        value: &V,
    ) -> sval_buffer::Result {
        let field = label
            .as_static_str()
            .ok_or_else(|| sval_buffer::Error::invalid_value("missing struct field label"))?;

        if let Ok(ref mut serializer) = self.serializer {
            match serializer.serialize_field(field, &ToSerialize::new(value)) {
                Ok(()) => return Ok(()),
                Err(err) => {
                    self.serializer = Err(err);
                }
            }
        }

        Err(sval_buffer::Error::invalid_value(
            "failed to serialize struct value",
        ))
    }

    fn end(self) -> sval_buffer::Result<Self::Ok> {
        match self.serializer {
            Ok(serializer) => Ok(serializer.end()),
            Err(err) => Ok(Err(err)),
        }
    }
}

impl<'sval, S: serde::ser::SerializeTupleVariant> StreamTuple<'sval>
    for SerializeTupleVariant<S, S::Error>
{
    type Ok = Result<S::Ok, S::Error>;

    fn value_computed<V: sval::Value + ?Sized>(
        &mut self,
        _: Option<sval::Tag>,
        _: sval::Index,
        value: &V,
    ) -> sval_buffer::Result {
        if let Ok(ref mut serializer) = self.serializer {
            match serializer.serialize_field(&ToSerialize::new(value)) {
                Ok(()) => return Ok(()),
                Err(err) => {
                    self.serializer = Err(err);
                }
            }
        }

        Err(sval_buffer::Error::invalid_value(
            "failed to serialize tuple value",
        ))
    }

    fn end(self) -> sval_buffer::Result<Self::Ok> {
        match self.serializer {
            Ok(serializer) => Ok(serializer.end()),
            Err(err) => Ok(Err(err)),
        }
    }
}

struct Bytes<'sval>(&'sval [u8]);

impl<'sval> serde::Serialize for Bytes<'sval> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(self.0)
    }
}
