/*!
Private extensions for `Stream` for collecting
keys, values, and sequences that are known upfront.

This is useful for `serde` integration where we can avoid
allocating for nested datastructures that are already known.
*/

use crate::stream::{
    self,
    Stream,
};

mod owned;
mod value;

#[doc(inline)]
pub use crate::Error;

pub(crate) use self::{
    owned::{
        OwnedCollect,
        RefMutCollect,
    },
    value::Value,
};

// FIXME: Moving the `*_collect` methods onto the base `Stream`
// trait is a little more efficient (a few % improvement against `serde`)
// in the general case because it can save a virtual call per key/value/elem.
// The reason this hasn't been done already is just to reduce
// the API surface area for now. It should be revisited sometime.
// The `Value` type that's passed in would need some more attention.

/**
An extension to `Stream` for items that are known upfront.
*/
pub(crate) trait Collect: Stream {
    fn map_key_collect(&mut self, k: Value) -> Result;

    fn map_value_collect(&mut self, v: Value) -> Result;

    fn seq_elem_collect(&mut self, v: Value) -> Result;
}

impl<'a, S: ?Sized> Collect for &'a mut S
where
    S: Collect,
{
    #[inline]
    fn map_key_collect(&mut self, k: Value) -> Result {
        (**self).map_key_collect(k)
    }

    #[inline]
    fn map_value_collect(&mut self, v: Value) -> Result {
        (**self).map_value_collect(v)
    }

    #[inline]
    fn seq_elem_collect(&mut self, v: Value) -> Result {
        (**self).seq_elem_collect(v)
    }
}

/**
Default implementations for stream extensions.
*/
pub(crate) struct Default<S>(pub(crate) S);

impl<S> Collect for Default<S>
where
    S: Stream,
{
    #[inline]
    fn map_key_collect(&mut self, k: Value) -> Result {
        Stream::map_key(self)?;
        k.stream(self)
    }

    #[inline]
    fn map_value_collect(&mut self, v: Value) -> Result {
        Stream::map_value(self)?;
        v.stream(self)
    }

    #[inline]
    fn seq_elem_collect(&mut self, v: Value) -> Result {
        Stream::seq_elem(self)?;
        v.stream(self)
    }
}

impl<S> Stream for Default<S>
where
    S: Stream,
{
    #[inline]
    fn fmt(&mut self, args: stream::Arguments) -> Result {
        self.0.fmt(args)
    }

    #[inline]
    fn i64(&mut self, v: i64) -> Result {
        self.0.i64(v)
    }

    #[inline]
    fn u64(&mut self, v: u64) -> Result {
        self.0.u64(v)
    }

    #[inline]
    fn i128(&mut self, v: i128) -> Result {
        self.0.i128(v)
    }

    #[inline]
    fn u128(&mut self, v: u128) -> Result {
        self.0.u128(v)
    }

    #[inline]
    fn f64(&mut self, v: f64) -> Result {
        self.0.f64(v)
    }

    #[inline]
    fn bool(&mut self, v: bool) -> Result {
        self.0.bool(v)
    }

    #[inline]
    fn char(&mut self, v: char) -> Result {
        self.0.char(v)
    }

    #[inline]
    fn str(&mut self, v: &str) -> Result {
        self.0.str(v)
    }

    #[inline]
    fn none(&mut self) -> Result {
        self.0.none()
    }

    #[inline]
    fn map_begin(&mut self, len: Option<usize>) -> Result {
        self.0.map_begin(len)
    }

    #[inline]
    fn map_key(&mut self) -> Result {
        self.0.map_key()
    }

    #[inline]
    fn map_value(&mut self) -> Result {
        self.0.map_value()
    }

    #[inline]
    fn map_end(&mut self) -> Result {
        self.0.map_end()
    }

    #[inline]
    fn seq_begin(&mut self, len: Option<usize>) -> Result {
        self.0.seq_begin(len)
    }

    #[inline]
    fn seq_elem(&mut self) -> Result {
        self.0.seq_elem()
    }

    #[inline]
    fn seq_end(&mut self) -> Result {
        self.0.seq_end()
    }
}

pub type Result = crate::std::result::Result<(), Error>;
