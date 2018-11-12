/*!
Traits for visiting a structured value.
*/

use std::fmt;

#[doc(inline)]
pub use crate::{Value, Error};

/**
A visitor for a value.

The visitor will receive a stream of inputs to build up
complex structures.
*/
pub trait Visit {
    fn any(&mut self, v: Value) -> Result<(), Error>;

    fn begin_seq(&mut self) -> Result<(), Error>;
    fn seq_elem(&mut self, v: Value) -> Result<(), Error>;
    fn end_seq(&mut self) -> Result<(), Error>;

    fn begin_map(&mut self) -> Result<(), Error>;
    fn map_key(&mut self, k: Value) -> Result<(), Error>;
    fn map_value(&mut self, v: Value) -> Result<(), Error>;
    fn end_map(&mut self) -> Result<(), Error>;

    fn i64(&mut self, v: i64) -> Result<(), Error> {
        self.any(Value::erased(&v))
    }

    fn u64(&mut self, v: u64) -> Result<(), Error> {
        self.any(Value::erased(&v))
    }

    #[cfg(feature = "i128")]
    fn i128(&mut self, v: i128) -> Result<(), Error> {
        self.any(Value::erased(&v))
    }

    #[cfg(feature = "i128")]
    fn u128(&mut self, v: u128) -> Result<(), Error> {
        self.any(Value::erased(&v))
    }

    fn f64(&mut self, v: f64) -> Result<(), Error> {
        self.any(Value::erased(&v))
    }

    fn bool(&mut self, v: bool) -> Result<(), Error> {
        self.any(Value::erased(&v))
    }

    fn char(&mut self, v: char) -> Result<(), Error> {
        let mut b = [0; 4];
        self.str(&*v.encode_utf8(&mut b))
    }

    fn str(&mut self, v: &str) -> Result<(), Error> {
        self.any(Value::erased(&v))
    }

    fn none(&mut self) -> Result<(), Error> {
        self.any(Value::erased(&()))
    }

    fn fmt(&mut self, v: &fmt::Arguments) -> Result<(), Error> {
        self.any(Value::erased(&v))
    }
}

impl<'a, T: ?Sized> Visit for &'a mut T
where
    T: Visit,
{
    fn any(&mut self, v: Value) -> Result<(), Error> {
        (**self).any(v)
    }

    fn begin_seq(&mut self) -> Result<(), Error> {
        (**self).begin_seq()
    }

    fn end_seq(&mut self) -> Result<(), Error> {
        (**self).end_seq()
    }

    fn seq_elem(&mut self, v: Value) -> Result<(), Error> {
        (**self).seq_elem(v)
    }

    fn begin_map(&mut self) -> Result<(), Error> {
        (**self).begin_map()
    }

    fn end_map(&mut self) -> Result<(), Error> {
        (**self).end_map()
    }

    fn map_key(&mut self, k: Value) -> Result<(), Error> {
        (**self).map_key(k)
    }

    fn map_value(&mut self, v: Value) -> Result<(), Error> {
        (**self).map_value(v)
    }

    fn i64(&mut self, v: i64) -> Result<(), Error> {
        (**self).i64(v)
    }

    fn u64(&mut self, v: u64) -> Result<(), Error> {
        (**self).u64(v)
    }

    #[cfg(feature = "i128")]
    fn i128(&mut self, v: i128) -> Result<(), Error> {
        (**self).i128(v)
    }

    #[cfg(feature = "i128")]
    fn u128(&mut self, v: u128) -> Result<(), Error> {
        (**self).u128(v)
    }

    fn f64(&mut self, v: f64) -> Result<(), Error> {
        (**self).f64(v)
    }

    fn bool(&mut self, v: bool) -> Result<(), Error> {
        (**self).bool(v)
    }

    fn char(&mut self, v: char) -> Result<(), Error> {
        (**self).char(v)
    }

    fn str(&mut self, v: &str) -> Result<(), Error> {
        (**self).str(v)
    }

    fn none(&mut self) -> Result<(), Error> {
        (**self).none()
    }

    fn fmt(&mut self, v: &fmt::Arguments) -> Result<(), Error> {
        (**self).fmt(v)
    }
}
