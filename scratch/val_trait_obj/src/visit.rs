use std::fmt;

#[doc(inline)]
pub use crate::{Value, Error};

/**
A visitor for a value.

The visitor will receive a stream of inputs to build up
complex structures.
*/
pub trait Visit {
    fn any(&mut self, v: &dyn Value) -> Result<(), Error>;

    fn seq(&mut self) -> Result<&mut dyn VisitSeq, Error>;
    fn map(&mut self) -> Result<&mut dyn VisitMap, Error>;

    fn i64(&mut self, v: i64) -> Result<(), Error> {
        self.any(&v)
    }

    fn u64(&mut self, v: u64) -> Result<(), Error> {
        self.any(&v)
    }

    #[cfg(feature = "i128")]
    fn i128(&mut self, v: i128) -> Result<(), Error> {
        self.any(&v)
    }

    #[cfg(feature = "i128")]
    fn u128(&mut self, v: u128) -> Result<(), Error> {
        self.any(&v)
    }

    fn f64(&mut self, v: f64) -> Result<(), Error> {
        self.any(&v)
    }

    fn bool(&mut self, v: bool) -> Result<(), Error> {
        self.any(&v)
    }

    fn char(&mut self, v: char) -> Result<(), Error> {
        let mut b = [0; 4];
        self.str(&*v.encode_utf8(&mut b))
    }

    fn str(&mut self, v: &str) -> Result<(), Error> {
        self.any(&v)
    }

    fn none(&mut self) -> Result<(), Error> {
        self.any(&())
    }

    fn fmt(&mut self, v: &fmt::Arguments) -> Result<(), Error> {
        self.any(&v)
    }
}

impl<'a, T: ?Sized> Visit for &'a mut T
where
    T: Visit,
{
    fn any(&mut self, v: &dyn Value) -> Result<(), Error> {
        (**self).any(v)
    }

    fn seq(&mut self) -> Result<&mut dyn VisitSeq, Error> {
        (**self).seq()
    }

    fn map(&mut self) -> Result<&mut dyn VisitMap, Error> {
        (**self).map()
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

pub trait VisitSeq {
    fn elem(&mut self, v: &dyn Value) -> Result<(), Error>;
    fn end(&mut self) -> Result<(), Error>;
}

impl<'a, T: ?Sized> VisitSeq for &'a mut T
where
    T: VisitSeq,
{
    fn elem(&mut self, v: &dyn Value) -> Result<(), Error> {
        (**self).elem(v)
    }

    fn end(&mut self) -> Result<(), Error> {
        (**self).end()
    }
}

pub trait VisitMap {
    fn entry(&mut self, k: &dyn Value, v: &dyn Value) -> Result<(), Error>;
    fn end(&mut self) -> Result<(), Error>;
}

impl<'a, T: ?Sized> VisitMap for &'a mut T
where
    T: VisitMap,
{
    fn entry(&mut self, k: &dyn Value, v: &dyn Value) -> Result<(), Error> {
        (**self).entry(k, v)
    }

    fn end(&mut self) -> Result<(), Error> {
        (**self).end()
    }
}
