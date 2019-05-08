use crate::{
    std::fmt,
    value,
};

use super::error::{
    err,
    Error,
};

use serde_lib::ser::{
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

pub(super) struct ToValue<T>(pub(super) T);

impl<T> value::Value for ToValue<T>
where
    T: Serialize,
{
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        self.0
            .serialize(Serializer(stream))
            .map_err(err("error streaming serde"))?;

        Ok(())
    }
}

struct Serializer<T>(T);

impl<'a, 'b> ser::Serializer for Serializer<&'a mut value::Stream<'b>> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Serializer<&'a mut value::Stream<'b>>;
    type SerializeTuple = Serializer<&'a mut value::Stream<'b>>;
    type SerializeTupleStruct = Serializer<&'a mut value::Stream<'b>>;
    type SerializeTupleVariant = Serializer<&'a mut value::Stream<'b>>;
    type SerializeMap = Serializer<&'a mut value::Stream<'b>>;
    type SerializeStruct = Serializer<&'a mut value::Stream<'b>>;
    type SerializeStructVariant = Serializer<&'a mut value::Stream<'b>>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.0.bool(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.0.i64(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.0.u64(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.0.i128(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.0.u128(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))?;
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.0.f64(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.0.char(v)?;
        Ok(())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.0.str(v)?;
        Ok(())
    }

    #[inline]
    fn collect_str<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        self.0.fmt(format_args!("{}", v))?;
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.0.seq_begin(Some(v.len()))?;

        for b in v {
            self.0.seq_elem(b)?;
        }

        self.0.seq_end()?;
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)?;
        Ok(())
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.0.none()?;
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.map_begin(Some(1))?;
        self.0.map_key(variant)?;

        self.0.map_value_begin()?.seq_begin(Some(1))?;
        self.0.seq_elem(ToValue(value))?;
        self.0.seq_end()?;

        self.0.map_end()?;

        Ok(())
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.seq_begin(len)?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.0.seq_begin(Some(len))?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.0.seq_begin(Some(len))?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.0.map_begin(Some(1))?;
        self.0.map_key(variant)?;

        self.0.map_value_begin()?.seq_begin(Some(len))?;

        Ok(self)
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.0.map_begin(len)?;
        Ok(self)
    }

    #[inline]
    fn serialize_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.0.map_begin(Some(len))?;
        Ok(self)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.0.map_begin(Some(1))?;
        self.0.map_key(variant)?;

        self.0.map_value_begin()?.map_begin(Some(len))?;

        Ok(self)
    }
}

impl<'a, 'b> SerializeSeq for Serializer<&'a mut value::Stream<'b>> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.seq_elem(ToValue(value))?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.seq_end()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeTuple for Serializer<&'a mut value::Stream<'b>> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.seq_elem(ToValue(value))?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.seq_end()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeTupleStruct for Serializer<&'a mut value::Stream<'b>> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.seq_elem(ToValue(value))?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.seq_end()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeTupleVariant for Serializer<&'a mut value::Stream<'b>> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.seq_elem(ToValue(value))?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.seq_end()?;
        self.0.map_end()?;

        Ok(())
    }
}

impl<'a, 'b> SerializeMap for Serializer<&'a mut value::Stream<'b>> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.map_key(ToValue(key))?;
        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.map_value(ToValue(value))?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.map_end()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeStruct for Serializer<&'a mut value::Stream<'b>> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.map_key(key)?;
        self.0.map_value(ToValue(value))?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.map_end()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeStructVariant for Serializer<&'a mut value::Stream<'b>> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.map_key(key)?;
        self.0.map_value(ToValue(value))?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.map_end()?;
        self.0.map_end()?;

        Ok(())
    }
}
