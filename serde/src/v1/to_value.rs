use crate::{
    std::fmt,
    stream,
    value,
};

use super::error::{
    err,
    Error,
};

use serde1_lib::ser::{
    self,
    Serialize,
    SerializeMap,
    SerializeSeq,
    SerializeStruct,
    SerializeStructVariant,
    SerializeTuple,
    SerializeTupleStruct,
    SerializeTupleVariant,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToValue<T>(pub(super) T);

impl<T> value::Value for ToValue<T>
where
    T: Serialize,
{
    fn stream<'s, 'v>(&'s self, stream: value::Stream<'s, 'v>) -> value::Result {
        self.0
            .serialize(Serializer(stream))
            .map_err(err("error streaming serde"))?;

        Ok(())
    }
}

struct Serializer<T>(T);

impl<'a, 'v> ser::Serializer for Serializer<value::Stream<'a, 'v>> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Serializer<value::Stream<'a, 'v>>;
    type SerializeTuple = Serializer<value::Stream<'a, 'v>>;
    type SerializeTupleStruct = Serializer<value::Stream<'a, 'v>>;
    type SerializeTupleVariant = Serializer<value::Stream<'a, 'v>>;
    type SerializeMap = Serializer<value::Stream<'a, 'v>>;
    type SerializeStruct = Serializer<value::Stream<'a, 'v>>;
    type SerializeStructVariant = Serializer<value::Stream<'a, 'v>>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.0.bool(v)?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))?;
        Ok(())
    }

    fn serialize_i64(mut self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.0.i64(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))?;
        Ok(())
    }

    fn serialize_u64(mut self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.0.u64(v)?;
        Ok(())
    }

    fn serialize_i128(mut self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.0.i128(v)?;
        Ok(())
    }

    fn serialize_u128(mut self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.0.u128(v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))?;
        Ok(())
    }

    fn serialize_f64(mut self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.0.f64(v)?;
        Ok(())
    }

    fn serialize_char(mut self, v: char) -> Result<Self::Ok, Self::Error> {
        self.0.char(v)?;
        Ok(())
    }

    fn serialize_str(mut self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.0.owned().str(v)?;
        Ok(())
    }

    fn collect_str<T>(mut self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        self.0
            .owned()
            .any(&stream::Arguments::from(format_args!("{}", v)))?;
        Ok(())
    }

    fn serialize_bytes(mut self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0.seq_begin(Some(v.len()))?;

        for b in v {
            self.0.owned().seq_elem(&b)?;
        }

        self.0.seq_end()?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)?;
        Ok(())
    }

    fn serialize_unit(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.none()?;
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        mut self,
        name: &'static str,
        index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.0.tag(stream::Tag::Full {
            kind: Some(stream::Ident::Static(name)),
            ident: stream::Ident::Static(variant),
            id: index as u64,
        })?;
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        mut self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().any(&ToValue(value))?;
        Ok(())
    }

    fn serialize_newtype_variant<T>(
        mut self,
        name: &'static str,
        index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().tagged(
            stream::Tag::Full {
                kind: Some(stream::Ident::Static(name)),
                ident: stream::Ident::Static(variant),
                id: index as u64,
            },
            &ToValue(value),
        )?;

        Ok(())
    }

    fn serialize_seq(mut self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.seq_begin(len)?;
        Ok(self)
    }

    fn serialize_tuple(mut self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.seq_begin(Some(len))?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        mut self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.0.seq_begin(Some(len))?;
        Ok(self)
    }

    fn serialize_tuple_variant(
        mut self,
        name: &'static str,
        index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.0.tagged_seq_begin(
            stream::Tag::Full {
                kind: Some(stream::Ident::Static(name)),
                ident: stream::Ident::Static(variant),
                id: index as u64,
            },
            Some(len),
        )?;
        Ok(self)
    }

    fn serialize_map(mut self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.map_begin(len)?;
        Ok(self)
    }

    fn serialize_struct(
        mut self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0.map_begin(Some(len))?;
        Ok(self)
    }

    fn serialize_struct_variant(
        mut self,
        name: &'static str,
        index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.0.tagged_map_begin(
            stream::Tag::Full {
                kind: Some(stream::Ident::Static(name)),
                ident: stream::Ident::Static(variant),
                id: index as u64,
            },
            Some(len),
        )?;
        Ok(self)
    }
}

impl<'a, 'v> SerializeSeq for Serializer<value::Stream<'a, 'v>> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().seq_elem(&ToValue(value))?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.seq_end()?;
        Ok(())
    }
}

impl<'a, 'v> SerializeTuple for Serializer<value::Stream<'a, 'v>> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().seq_elem(&ToValue(value))?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.seq_end()?;
        Ok(())
    }
}

impl<'a, 'v> SerializeTupleStruct for Serializer<value::Stream<'a, 'v>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().seq_elem(&ToValue(value))?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.seq_end()?;
        Ok(())
    }
}

impl<'a, 'v> SerializeTupleVariant for Serializer<value::Stream<'a, 'v>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().seq_elem(&ToValue(value))?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.tagged_seq_end()?;

        Ok(())
    }
}

impl<'a, 'v> SerializeMap for Serializer<value::Stream<'a, 'v>> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().map_key(&ToValue(key))?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().map_value(&ToValue(value))?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.map_end()?;
        Ok(())
    }
}

impl<'a, 'v> SerializeStruct for Serializer<value::Stream<'a, 'v>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().map_key(&stream::Ident::Static(key))?;
        self.0.owned().map_value(&ToValue(value))?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.map_end()?;
        Ok(())
    }
}

impl<'a, 'v> SerializeStructVariant for Serializer<value::Stream<'a, 'v>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.owned().map_key(&stream::Ident::Static(key))?;
        self.0.owned().map_value(&ToValue(value))?;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.tagged_map_end()?;

        Ok(())
    }
}
