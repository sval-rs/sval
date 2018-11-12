/*!
Consumers of structured values.
*/

use std::fmt;

use crate::value;

#[doc(inline)]
pub use crate::Error;

/**
A visitor for a value.

The visitor will receive a flat stream of inputs to build up
complex structures.

Even though the `Visit` trait itself is flat, its inputs are
guaranteed to be received in a valid way. See the [`value::Visit`]
type for more details.
*/
pub trait Visit {
    /**
    Visit an arbitrary value.

    The value may be formatted using its `Debug` representation.
    */
    fn any(&mut self, v: Value) -> Result<(), Error>;

    /**
    Begin a sequence.

    After the sequence has begun, this `Visit` should only expect
    calls to `seq_elem` until `seq_end` is called.
    */
    fn seq_begin(&mut self) -> Result<(), Error>;

    /**
    Visit an element of a sequence.

    This method should only be called after `seq_begin` and before
    `seq_end`.
    */
    fn seq_elem(&mut self, v: Value) -> Result<(), Error> {
        v.visit(self)
    }

    /**
    End a sequence.

    This method should only be called after `seq_begin`.
    Each call to `seq_begin` must have a corresponding call
    to `seq_end`. 
    */
    fn seq_end(&mut self) -> Result<(), Error>;

    /**
    Begin a map.

    After the map has begun, this `Visit` should only expect
    calls to `map_key` and `map_value` until `map_end` is called.
    */
    fn map_begin(&mut self) -> Result<(), Error>;

    /**
    Visit a map key.

    This method should only be called after `map_begin` and before
    a corresponding call to `map_value`.
    */
    fn map_key(&mut self, k: Value) -> Result<(), Error> {
        k.visit(self)
    }

    /**
    Visit a map key.

    This method should only be called after `map_key`.
    */
    fn map_value(&mut self, v: Value) -> Result<(), Error> {
        v.visit(self)
    }

    /**
    End a map.

    This method should only be called after `map_begin`.
    Each call to `map_begin` must have a corresponding call
    to `map_end`. 
    */
    fn map_end(&mut self) -> Result<(), Error>;

    /** Visit a signed integer. */
    fn i64(&mut self, v: i64) -> Result<(), Error> {
        self.any(Value::new(&v))
    }

    /** Visit an unsigned integer. */
    fn u64(&mut self, v: u64) -> Result<(), Error> {
        self.any(Value::new(&v))
    }

    /** Visit a 128bit signed integer. */
    #[cfg(feature = "i128")]
    fn i128(&mut self, v: i128) -> Result<(), Error> {
        self.any(Value::new(&v))
    }

    /** Visit a 128bit unsigned integer. */
    #[cfg(feature = "i128")]
    fn u128(&mut self, v: u128) -> Result<(), Error> {
        self.any(Value::new(&v))
    }

    /** Visit a floating point value. */
    fn f64(&mut self, v: f64) -> Result<(), Error> {
        self.any(Value::new(&v))
    }

    /** Visit a boolean. */
    fn bool(&mut self, v: bool) -> Result<(), Error> {
        self.any(Value::new(&v))
    }

    /** Visit a unicode character. */
    fn char(&mut self, v: char) -> Result<(), Error> {
        let mut b = [0; 4];
        self.str(&*v.encode_utf8(&mut b))
    }

    /** Visit a UTF-8 string slice. */
    fn str(&mut self, v: &str) -> Result<(), Error> {
        self.any(Value::new(&v))
    }

    /** Visit an empty value. */
    fn none(&mut self) -> Result<(), Error> {
        self.any(Value::new(&()))
    }

    /** Visit a format. */
    fn fmt(&mut self, v: &fmt::Arguments) -> Result<(), Error> {
        self.any(Value::new(&v))
    }
}

impl<'a, T: ?Sized> Visit for &'a mut T
where
    T: Visit,
{
    fn any(&mut self, v: Value) -> Result<(), Error> {
        (**self).any(v)
    }

    fn seq_begin(&mut self) -> Result<(), Error> {
        (**self).seq_begin()
    }

    fn seq_end(&mut self) -> Result<(), Error> {
        (**self).seq_end()
    }

    fn seq_elem(&mut self, v: Value) -> Result<(), Error> {
        (**self).seq_elem(v)
    }

    fn map_begin(&mut self) -> Result<(), Error> {
        (**self).map_begin()
    }

    fn map_end(&mut self) -> Result<(), Error> {
        (**self).map_end()
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

/**
A structured value.

The `Value` type abstracts over storage for a [`value::Value`] trait object.
*/
pub struct Value<'a> {
    inner: ValueInner<'a>,
}

enum ValueInner<'a> {
    Ref(&'a dyn value::Value),
    Boxed(Box<dyn value::Value + 'a>),
}

impl<'a> ValueInner<'a> {
    fn as_ref(&self) -> &dyn value::Value {
        match self {
            ValueInner::Ref(value) => value,
            ValueInner::Boxed(value) => &**value,
        }
    }
}

impl<'a> Value<'a> {
    pub fn new(value: &'a dyn value::Value) -> Self {
        Value {
            inner: ValueInner::Ref(value),
        }
    }

    pub fn boxed(value: impl value::Value + 'a) -> Self {
        Value {
            inner: ValueInner::Boxed(Box::new(value))
        }
    }

    pub fn visit(&self, mut visit: impl Visit) -> Result<(), Error> {
        self.inner.as_ref().visit(value::Visit::new(&mut visit))
    }
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.as_ref().fmt(f)
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.as_ref().fmt(f)
    }
}
