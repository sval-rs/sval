/*!
Convert between `val` and `serde`.

A type that implements [`value::Value`] can be converted into
a type that implements [`serde::Serialize`]:

```
# #[derive(Debug)] struct MyValue;
# impl val::value::Value for MyValue {
#     fn visit(&self, visit: val::value::Visit) -> Result<(), val::value::Error> {
#         visit.none()
#     }
# }
# let my_value = MyValue;
let my_serialize = val_serde::to_serialize(my_value);
```

A type that implements [`std::fmt::Debug`] and [`serde::Serialize`] can be converted into
a type that implements [`value::Value`]:

```
# #[derive(Debug)] struct MySerialize;
# impl serde::Serialize for MySerialize {
#     fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
#         s.serialize_none()
#     }
# }
# let my_serialize = MySerialize;
let my_value = val_serde::to_value(my_serialize);
```
*/

#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate core as std;

use crate::std::fmt::{self, Debug};

use val::{value, visit};

use serde::ser::{
    self, Error as SerError, Serialize, SerializeMap, SerializeSeq, SerializeStruct,
    SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant,
    Serializer,
};

/**
Convert a `T: Value` into an `impl Serialize + Debug`.
*/
pub fn to_serialize(value: impl value::Value) -> impl Serialize + Debug {
    use self::visit::{Value, Visit};

    struct ToSerialize<V>(V);

    impl<V> Debug for ToSerialize<V>
    where
        V: Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<V> Serialize for ToSerialize<V>
    where
        V: value::Value,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Serde(Value::new(&self.0)).serialize(serializer)
        }
    }

    ToSerialize(value)
}

/**
Convert a `T: Serialize + Debug` into an `impl Value`.
*/
pub fn to_value(serialize: impl Serialize + Debug) -> impl value::Value {
    use self::value::{Error, Value, Visit};

    struct ToValue<S>(S);

    impl<S> Debug for ToValue<S>
    where
        S: Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    impl<S> Value for ToValue<S>
    where
        S: Serialize + Debug,
    {
        fn visit(&self, visit: Visit) -> Result<(), Error> {
            self.0
                .serialize(Serde(visit))
                .map_err(err("error visiting serde"))?;

            Ok(())
        }
    }

    ToValue(serialize)
}

struct Serde<T>(T);

impl<'a> Serialize for Serde<visit::Value<'a>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use self::visit::Visit;

        enum Current<S>
        where
            S: Serializer,
        {
            Serializer(S),
            SerializeSeq(S::SerializeSeq),
            SerializeMap(S::SerializeMap),
        }

        impl<S> Current<S>
        where
            S: Serializer,
        {
            fn take_serializer(self) -> Result<S, visit::Error> {
                match self {
                    Current::Serializer(ser) => Ok(ser),
                    _ => Err(visit::Error::msg("invalid serializer value")),
                }
            }

            fn expect_serialize_seq(&mut self) -> Result<&mut S::SerializeSeq, visit::Error> {
                match self {
                    Current::SerializeSeq(seq) => Ok(seq),
                    _ => Err(visit::Error::msg("invalid serializer value")),
                }
            }

            fn take_serialize_seq(self) -> Result<S::SerializeSeq, visit::Error> {
                match self {
                    Current::SerializeSeq(seq) => Ok(seq),
                    _ => Err(visit::Error::msg("invalid serializer value")),
                }
            }

            fn expect_serialize_map(&mut self) -> Result<&mut S::SerializeMap, visit::Error> {
                match self {
                    Current::SerializeMap(map) => Ok(map),
                    _ => Err(visit::Error::msg("invalid serializer value")),
                }
            }

            fn take_serialize_map(self) -> Result<S::SerializeMap, visit::Error> {
                match self {
                    Current::SerializeMap(map) => Ok(map),
                    _ => Err(visit::Error::msg("invalid serializer value")),
                }
            }
        }

        struct SerdeVisit<S>
        where
            S: Serializer,
        {
            ok: Option<S::Ok>,
            current: Option<Current<S>>,
        }

        impl<S> SerdeVisit<S>
        where
            S: Serializer,
        {
            fn map_serializer<E>(
                &mut self,
                f: impl FnOnce(S) -> Result<Current<S>, E>,
            ) -> Result<(), visit::Error>
            where
                E: serde::ser::Error,
            {
                let serializer = self.take()?.take_serializer()?;
                self.current = Some(f(serializer).map_err(err("error maping serializer"))?);

                Ok(())
            }

            fn take(&mut self) -> Result<Current<S>, visit::Error> {
                self.current
                    .take()
                    .ok_or(visit::Error::msg("attempt to use an invalid serializer"))
            }

            fn expect(&mut self) -> Result<&mut Current<S>, visit::Error> {
                self.current
                    .as_mut()
                    .ok_or(visit::Error::msg("attempt to use an invalid serializer"))
            }
        }

        impl<S> Visit for SerdeVisit<S>
        where
            S: Serializer,
        {
            fn any(&mut self, v: visit::Value) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.collect_str(&v)
                        .map_err(err("error collecting string"))?,
                );

                Ok(())
            }

            fn seq_begin(&mut self, len: Option<usize>) -> Result<(), visit::Error> {
                self.map_serializer(|ser| {
                    ser.serialize_seq(len).map(|seq| Current::SerializeSeq(seq))
                })
            }

            fn seq_elem(&mut self, v: visit::Value) -> Result<(), visit::Error> {
                let seq = self.expect()?.expect_serialize_seq()?;
                seq.serialize_element(&Serde(v))
                    .map_err(err("error serializing sequence element"))?;

                Ok(())
            }

            fn seq_end(&mut self) -> Result<(), visit::Error> {
                let seq = self.take()?.take_serialize_seq()?;
                self.ok = Some(seq.end().map_err(err("error completing sequence"))?);

                Ok(())
            }

            fn map_begin(&mut self, len: Option<usize>) -> Result<(), visit::Error> {
                self.map_serializer(|ser| {
                    ser.serialize_map(len).map(|map| Current::SerializeMap(map))
                })
            }

            fn map_key(&mut self, k: visit::Value) -> Result<(), visit::Error> {
                let map = self.expect()?.expect_serialize_map()?;
                map.serialize_key(&Serde(k))
                    .map_err(err("error map serializing key"))?;

                Ok(())
            }

            fn map_value(&mut self, v: visit::Value) -> Result<(), visit::Error> {
                let map = self.expect()?.expect_serialize_map()?;
                map.serialize_value(&Serde(v))
                    .map_err(err("error serializing map value"))?;

                Ok(())
            }

            fn map_end(&mut self) -> Result<(), visit::Error> {
                let map = self.take()?.take_serialize_map()?;
                self.ok = Some(map.end().map_err(err("error completing map"))?);

                Ok(())
            }

            fn i64(&mut self, v: i64) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.serialize_i64(v)
                        .map_err(err("error serializing signed integer"))?,
                );

                Ok(())
            }

            fn u64(&mut self, v: u64) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.serialize_u64(v)
                        .map_err(err("error serializing unsigned integer"))?,
                );

                Ok(())
            }

            fn i128(&mut self, v: i128) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.serialize_i128(v)
                        .map_err(err("error serializing 128bit signed integer"))?,
                );

                Ok(())
            }

            fn u128(&mut self, v: u128) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.serialize_u128(v)
                        .map_err(err("error serializing 128bit unsigned integer"))?,
                );

                Ok(())
            }

            fn f64(&mut self, v: f64) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.serialize_f64(v)
                        .map_err(err("error serializing floating-point value"))?,
                );

                Ok(())
            }

            fn bool(&mut self, v: bool) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.serialize_bool(v)
                        .map_err(err("error serializing boolean"))?,
                );

                Ok(())
            }

            fn char(&mut self, v: char) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.serialize_char(v)
                        .map_err(err("error serializing unicode character"))?,
                );

                Ok(())
            }

            fn str(&mut self, v: &str) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.serialize_str(v)
                        .map_err(err("error serializing UTF-8 string"))?,
                );

                Ok(())
            }

            fn none(&mut self) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.serialize_none()
                        .map_err(err("error serializing empty value"))?,
                );

                Ok(())
            }

            fn fmt(&mut self, v: &fmt::Arguments) -> Result<(), visit::Error> {
                let ser = self.take()?.take_serializer()?;
                self.ok = Some(
                    ser.collect_str(v)
                        .map_err(err("error serializing format"))?,
                );

                Ok(())
            }
        }

        let mut visit = SerdeVisit {
            ok: None,
            current: Some(Current::Serializer(serializer)),
        };

        self.0.visit(&mut visit).map_err(S::Error::custom)?;

        Ok(visit.ok.expect("missing return value"))
    }
}

impl<'a> Serializer for Serde<value::Visit<'a>> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Serde<value::VisitSeq<'a>>;
    type SerializeTuple = Serde<value::VisitSeq<'a>>;
    type SerializeTupleStruct = Serde<value::VisitSeq<'a>>;
    type SerializeTupleVariant = Serde<value::VisitSeq<'a>>;
    type SerializeMap = Serde<value::VisitMap<'a>>;
    type SerializeStruct = Serde<value::VisitMap<'a>>;
    type SerializeStructVariant = Serde<value::VisitMap<'a>>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
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
        unimplemented!()
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
        unimplemented!()
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn collect_str<T>(self, v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + fmt::Display,
    {
        unimplemented!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
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
        unimplemented!()
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
        name: &'static str,
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

impl<'a> SerializeSeq for Serde<value::VisitSeq<'a>> {
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

impl<'a> SerializeTuple for Serde<value::VisitSeq<'a>> {
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

impl<'a> SerializeTupleStruct for Serde<value::VisitSeq<'a>> {
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

impl<'a> SerializeTupleVariant for Serde<value::VisitSeq<'a>> {
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

impl<'a> SerializeMap for Serde<value::VisitMap<'a>> {
    type Ok = ();
    type Error = Error;

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

impl<'a> SerializeStruct for Serde<value::VisitMap<'a>> {
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

impl<'a> SerializeStructVariant for Serde<value::VisitMap<'a>> {
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

/**
An error encountered during serialization.
*/
struct Error(value::Error);

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

fn err<E>(msg: &'static str) -> impl FnOnce(E) -> val::Error
where
    E: ser::Error,
{
    #[cfg(feature = "std")]
    {
        move |err| val::Error::from(err)
    }

    #[cfg(not(feature = "std"))]
    {
        move |_| val::Error::msg(msg)
    }
}

#[cfg(not(feature = "std"))]
mod core_support {
    use super::*;

    impl ser::Error for Error {
        fn custom<E>(e: E) -> Self
        where
            E: fmt::Display,
        {
            Error(value::Error::msg("serialization error"))
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::error;

    impl error::Error for Error {
        fn cause(&self) -> Option<&dyn error::Error> {
            None
        }

        fn description(&self) -> &str {
            "serialization error"
        }
    }

    impl ser::Error for Error {
        fn custom<E>(e: E) -> Self
        where
            E: fmt::Display,
        {
            Error(value::Error::custom(e))
        }
    }
}
