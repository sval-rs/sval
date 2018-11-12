/*!
Producers of structured values.
*/

use std::fmt;

use crate::visit;

#[doc(inline)]
pub use crate::Error;

/**
A value that can be visited.
*/
pub trait Value: fmt::Debug {
    /**
    Visit this value.
    */
    fn visit(&self, visit: Visit) -> Result<(), Error>;
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn visit(&self, visit: Visit) -> Result<(), Error> {
        (**self).visit(visit)
    }
}

/**
A visitor for a structured value.

The `Visit` type abstracts over storage for a [`visit::Visit`] trait object.
It also imposes some limitations on the way the internal `Visit` can be called:

- Each implementation of [`Value`] may only call a single method on `Visit`.
- Sequence elements and map entries cannot be visited without first calling
`Visit::seq` or `Visit::map`.
- Sequences and maps must call `end` and cannot visit more elements or entries after
ending.
- Map keys are always visited before values, and there's always a value visited after
a key.

Implementations of [`visit::Visit`] can rely on these guarantees being met upstream.
*/
pub struct Visit<'a> {
    inner: VisitInner<'a>,
}

enum VisitInner<'a> {
    Ref(&'a mut dyn visit::Visit),
    Boxed(Box<dyn visit::Visit + 'a>),
}

impl<'a> VisitInner<'a> {
    fn as_mut(&mut self) -> &mut dyn visit::Visit {
        match self {
            VisitInner::Ref(visit) => visit,
            VisitInner::Boxed(visit) => &mut **visit,
        }
    }
}

impl<'a> Visit<'a> {
    pub fn new(visit: &'a mut dyn visit::Visit) -> Self {
        Visit {
            inner: VisitInner::Ref(visit),
        }
    }

    pub fn boxed(visit: impl visit::Visit + 'a) -> Self {
        Visit {
            inner: VisitInner::Boxed(Box::new(visit))
        }
    }

    pub fn i64(mut self, v: i64) -> Result<(), Error> {
        self.inner.as_mut().i64(v)
    }

    pub fn u64(mut self, v: u64) -> Result<(), Error> {
        self.inner.as_mut().u64(v)
    }

    #[cfg(feature = "i128")]
    pub fn i128(mut self, v: i128) -> Result<(), Error> {
        self.inner.as_mut().i128(v)
    }

    #[cfg(feature = "i128")]
    pub fn u128(mut self, v: u128) -> Result<(), Error> {
        self.inner.as_mut().u128(v)
    }

    pub fn f64(mut self, v: f64) -> Result<(), Error> {
        self.inner.as_mut().f64(v)
    }

    pub fn bool(mut self, v: bool) -> Result<(), Error> {
        self.inner.as_mut().bool(v)
    }

    pub fn char(mut self, v: char) -> Result<(), Error> {
        self.inner.as_mut().char(v)
    }

    pub fn str(mut self, v: &str) -> Result<(), Error> {
        self.inner.as_mut().str(v)
    }

    pub fn none(mut self) -> Result<(), Error> {
        self.inner.as_mut().none()
    }

    pub fn fmt(mut self, v: &fmt::Arguments) -> Result<(), Error> {
        self.inner.as_mut().fmt(v)
    }

    pub fn seq(mut self) -> Result<VisitSeq<'a>, Error> {
        self.inner.as_mut().seq_begin()?;

        Ok(VisitSeq {
            inner: self.inner,
            done: false,
        })
    }

    pub fn map(mut self) -> Result<VisitMap<'a>, Error> {
        self.inner.as_mut().map_begin()?;

        Ok(VisitMap {
            inner: self.inner,
            done: false,
        })
    }
}

/**
A visitor for a sequence.
*/
pub struct VisitSeq<'a> {
    inner: VisitInner<'a>,
    done: bool,
}

impl<'a> VisitSeq<'a> {
    pub fn elem(&mut self, v: impl Value) -> Result<(), Error> {
        self.inner.as_mut().seq_elem(visit::Value::new(&v))
    }

    pub fn end(mut self) -> Result<(), Error> {
        self.done = true;
        self.inner.as_mut().seq_end()
    }
}

/**
A visitor for a map.
*/
pub struct VisitMap<'a> {
    inner: VisitInner<'a>,
    done: bool,
}

impl<'a> VisitMap<'a> {
    pub fn entry(&mut self, k: impl Value, v: impl Value) -> Result<(), Error> {
        self.inner.as_mut().map_key(visit::Value::new(&k))?;
        self.inner.as_mut().map_value(visit::Value::new(&v))?;

        Ok(())
    }

    pub fn end(mut self) -> Result<(), Error> {
        self.done = true;
        self.inner.as_mut().map_end()
    }
}
