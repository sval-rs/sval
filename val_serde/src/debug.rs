/*!
An approximate implementation of `Debug` for a `T: Serialize`.
*/

use crate::{
    std::fmt,
    error::Error,
};

use serde::ser::{
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

pub(crate) struct Debug<T>(pub(crate) T);

impl<T> fmt::Debug for Debug<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.serialize(Formatter(f)).map_err(|_| fmt::Error)
    }
}

impl<T> Serialize for Debug<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}

struct Formatter<T>(T);

struct DebugMap<'a, 'b>(fmt::DebugMap<'a, 'b>, usize);

impl<'a, 'b> ser::Serializer for Formatter<&'a mut fmt::Formatter<'b>> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Formatter<fmt::DebugList<'a, 'b>>;
    type SerializeTuple = Formatter<fmt::DebugTuple<'a, 'b>>;
    type SerializeTupleStruct = Formatter<fmt::DebugTuple<'a, 'b>>;
    type SerializeTupleVariant = Formatter<fmt::DebugTuple<'a, 'b>>;
    type SerializeMap = DebugMap<'a, 'b>;
    type SerializeStruct = Formatter<fmt::DebugStruct<'a, 'b>>;
    type SerializeStructVariant = Formatter<fmt::DebugStruct<'a, 'b>>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", v)?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", v)?;
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", v)?;
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", v)?;
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", v)?;
        Ok(())
    }

    fn collect_str<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        write!(self.0, "{}", v)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", ())?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        write!(self.0, "{:?}", ())?;
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

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
        self.0.debug_tuple(variant).field(&Debug(value)).finish()?;
        Ok(())
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Formatter(self.0.debug_list()))
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Formatter(self.0.debug_tuple("")))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Formatter(self.0.debug_tuple(name)))
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(Formatter(self.0.debug_tuple(variant)))
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(DebugMap(self.0.debug_map(), 0))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Formatter(self.0.debug_struct(name)))
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Formatter(self.0.debug_struct(variant)))
    }
}

impl<'a, 'b> SerializeSeq for Formatter<fmt::DebugList<'a, 'b>> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.entry(&Debug(value));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeTuple for Formatter<fmt::DebugTuple<'a, 'b>> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&Debug(value));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeTupleStruct for Formatter<fmt::DebugTuple<'a, 'b>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&Debug(value));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeTupleVariant for Formatter<fmt::DebugTuple<'a, 'b>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(&Debug(value));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeMap for DebugMap<'a, 'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0
            .entry(&format_args!("field_{}", self.1), &Debug(value));
        self.1 += 1;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeStruct for Formatter<fmt::DebugStruct<'a, 'b>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(key, &Debug(value));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish()?;
        Ok(())
    }
}

impl<'a, 'b> SerializeStructVariant for Formatter<fmt::DebugStruct<'a, 'b>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.field(key, &Debug(value));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.0.finish()?;
        Ok(())
    }
}
