use crate::{err, std::fmt, Error};

use val::value;

use serde::ser::{
    self, Error as SerError, Serialize, SerializeMap, SerializeSeq, SerializeStruct,
    SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    Impossible,
};

pub(crate) struct Serializer<T>(T);

impl<'a> Serializer<value::Visit<'a>> {
    pub(crate) fn begin(visit: value::Visit<'a>) -> Self {
        Serializer(visit)
    }
}

impl<'a> ser::Serializer for Serializer<value::Visit<'a>> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Serializer<value::VisitSeq<'a>>;
    type SerializeTuple = Serializer<value::VisitSeq<'a>>;
    type SerializeTupleStruct = Serializer<value::VisitSeq<'a>>;
    type SerializeTupleVariant = Serializer<value::VisitSeq<'a>>;
    type SerializeMap = Serializer<value::VisitMap<'a>>;
    type SerializeStruct = Serializer<value::VisitMap<'a>>;
    type SerializeStructVariant = Serializer<value::VisitMap<'a>>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.0.bool(v).map_err(Error)
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
        self.0.i64(v).map_err(Error)
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
        self.0.u64(v).map_err(Error)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.0.i128(v).map_err(Error)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.0.u128(v).map_err(Error)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.0.f64(v).map_err(Error)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.0.char(v).map_err(Error)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.0.str(v).map_err(Error)
    }

    fn collect_str<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        self.0.fmt(format_args!("{}", v)).map_err(Error)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut seq = self.0.seq(Some(v.len())).map_err(Error)?;

        for b in v {
            seq.elem(b).map_err(Error)?;
        }

        seq.end().map_err(Error)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.0.none().map_err(Error)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.0.none().map_err(Error)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Serializer(self.0.seq(len).map_err(Error)?))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Serializer(self.0.map(len).map_err(Error)?))
    }

    fn serialize_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

impl<'a> SerializeSeq for Serializer<value::VisitSeq<'a>> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.elem(Debug(value)).map_err(Error)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.end().map_err(Error)?;
        Ok(())
    }
}

impl<'a> SerializeTuple for Serializer<value::VisitSeq<'a>> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> SerializeTupleStruct for Serializer<value::VisitSeq<'a>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> SerializeTupleVariant for Serializer<value::VisitSeq<'a>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> SerializeMap for Serializer<value::VisitMap<'a>> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.key(Debug(key)).map_err(Error)?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        self.0.value(Debug(value)).map_err(Error)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.0.end().map_err(Error)?;
        Ok(())
    }
}

impl<'a> SerializeStruct for Serializer<value::VisitMap<'a>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> SerializeStructVariant for Serializer<value::VisitMap<'a>> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

struct Debug<T>(T);

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

impl<T> value::Value for Debug<T>
where
    T: Serialize,
{
    fn visit(&self, visit: value::Visit) -> Result<(), value::Error> {
        self.0
            .serialize(Serializer(visit))
            .map_err(err("error visiting serde"))?;

        Ok(())
    }
}

struct Formatter<T>(T);

#[derive(Debug)]
struct FmtError;

impl From<fmt::Error> for FmtError {
    fn from(_: fmt::Error) -> Self {
        FmtError
    }
}

impl fmt::Display for FmtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "debug format error")
    }
}

impl<'a, 'b> ser::Serializer for Formatter<&'a mut fmt::Formatter<'b>> {
    type Ok = ();
    type Error = FmtError;

    type SerializeSeq = Formatter<fmt::DebugList<'a, 'b>>;
    type SerializeTuple = Formatter<fmt::DebugTuple<'a, 'b>>;
    type SerializeTupleStruct = Formatter<fmt::DebugTuple<'a, 'b>>;
    type SerializeTupleVariant = Formatter<fmt::DebugTuple<'a, 'b>>;
    type SerializeMap = Formatter<fmt::DebugMap<'a, 'b>>;
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

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
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
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Formatter(self.0.debug_list()))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

impl<'a, 'b> SerializeSeq for Formatter<fmt::DebugList<'a, 'b>> {
    type Ok = ();
    type Error = FmtError;

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
    type Error = FmtError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, 'b> SerializeTupleStruct for Formatter<fmt::DebugTuple<'a, 'b>> {
    type Ok = ();
    type Error = FmtError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, 'b> SerializeTupleVariant for Formatter<fmt::DebugTuple<'a, 'b>> {
    type Ok = ();
    type Error = FmtError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, 'b> SerializeMap for Formatter<fmt::DebugMap<'a, 'b>> {
    type Ok = ();
    type Error = FmtError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, 'b> SerializeStruct for Formatter<fmt::DebugStruct<'a, 'b>> {
    type Ok = ();
    type Error = FmtError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a, 'b> SerializeStructVariant for Formatter<fmt::DebugStruct<'a, 'b>> {
    type Ok = ();
    type Error = FmtError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl ser::Error for FmtError {
    fn custom<T>(err: T) -> Self
    where
        T: fmt::Display,
    {
        FmtError
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::error;

    impl error::Error for FmtError {
        fn cause(&self) -> Option<&dyn error::Error> {
            None
        }

        fn description(&self) -> &str {
            "debug format error"
        }
    }
}
