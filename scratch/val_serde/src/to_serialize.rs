use crate::{
    error::err,
    std::fmt,
};

use val::visit;

use serde::ser::{
    Error as SerError,
    Serialize,
    SerializeMap,
    SerializeSeq,
    Serializer,
};

pub(crate) struct Visit<S>
where
    S: Serializer,
{
    ok: Option<S::Ok>,
    current: Option<Current<S>>,
}

enum Current<S>
where
    S: Serializer,
{
    Serializer(S),
    SerializeSeq(S::SerializeSeq),
    SerializeMap(S::SerializeMap),
}

impl<S> Visit<S>
where
    S: Serializer,
{
    pub(crate) fn begin(ser: S) -> Self {
        Visit {
            ok: None,
            current: Some(Current::Serializer(ser)),
        }
    }

    pub(crate) fn expect_ok(self) -> S::Ok {
        self.ok.expect("missing return value")
    }
}

struct Value<'a>(visit::Value<'a>);

impl<'a> Serialize for Value<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut visit = Visit::begin(serializer);
        self.0.visit(&mut visit).map_err(S::Error::custom)?;

        Ok(visit.expect_ok())
    }
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

impl<S> Visit<S>
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

impl<S> visit::Visit for Visit<S>
where
    S: Serializer,
{
    fn seq_begin(&mut self, len: Option<usize>) -> Result<(), visit::Error> {
        self.map_serializer(|ser| ser.serialize_seq(len).map(|seq| Current::SerializeSeq(seq)))
    }

    fn seq_elem(&mut self, v: visit::Value) -> Result<(), visit::Error> {
        let seq = self.expect()?.expect_serialize_seq()?;
        seq.serialize_element(&Value(v))
            .map_err(err("error serializing sequence element"))?;

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), visit::Error> {
        let seq = self.take()?.take_serialize_seq()?;
        self.ok = Some(seq.end().map_err(err("error completing sequence"))?);

        Ok(())
    }

    fn map_begin(&mut self, len: Option<usize>) -> Result<(), visit::Error> {
        self.map_serializer(|ser| ser.serialize_map(len).map(|map| Current::SerializeMap(map)))
    }

    fn map_key(&mut self, k: visit::Value) -> Result<(), visit::Error> {
        let map = self.expect()?.expect_serialize_map()?;
        map.serialize_key(&Value(k))
            .map_err(err("error map serializing key"))?;

        Ok(())
    }

    fn map_value(&mut self, v: visit::Value) -> Result<(), visit::Error> {
        let map = self.expect()?.expect_serialize_map()?;
        map.serialize_value(&Value(v))
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

    fn fmt(&mut self, v: fmt::Arguments) -> Result<(), visit::Error> {
        let ser = self.take()?.take_serializer()?;
        self.ok = Some(
            ser.collect_str(&v)
                .map_err(err("error serializing format"))?,
        );

        Ok(())
    }
}
