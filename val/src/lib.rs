/*!
A lightweight serialization framework.

# Visiting values

```no_run
# fn main() -> Result<(), Box<std::error::Error>> {
val::visit(42, MyVisit)?;
# Ok(())
# }
# use val::visit::{self, Visit};
# struct MyVisit;
# impl val::visit::Visit for MyVisit {
#     fn any(&mut self, v: visit::Value) -> Result<(), visit::Error> { unimplemented!() }
#     fn begin_seq(&mut self) -> Result<(), visit::Error> { unimplemented!() }
#     fn end_seq(&mut self) -> Result<(), visit::Error> { unimplemented!() }
#     fn begin_map(&mut self) -> Result<(), visit::Error> { unimplemented!() }
#     fn end_map(&mut self) -> Result<(), visit::Error> { unimplemented!() }
# }
```

where `42` is a [`value::Value`] and `MyVisit` is a [`visit::Visit`].

# Implementing the `Value` trait

Implement the [`value::Value`] trait for datastructures that can be
visited using a [`Visit`]:

```
use val::value::{self, Value};

#[derive(Debug)]
pub struct Id(u64);

impl Value for Id {
    fn visit(&self, visit: value::Visit) -> Result<(), value::Error> {
        visit.u64(self.0)
    }
}
```

Sequences can be visited:

```
use val::value::{self, Value};

#[derive(Debug)]
pub struct Seq(Vec<u64>);

impl Value for Seq {
    fn visit(&self, visit: value::Visit) -> Result<(), value::Error> {
        let mut seq = visit.seq()?;

        for v in &self.0 {
            seq.elem(v)?;
        }

        seq.end()
    }
}
```

Maps can be visited:

```
use std::collections::BTreeMap;

use val::value::{self, Value};

#[derive(Debug)]
pub struct Map(BTreeMap<String, u64>);

impl Value for Map {
    fn visit(&self, visit: value::Visit) -> Result<(), value::Error> {
        let mut map = visit.map()?;

        for (k, v) in &self.0 {
            map.entry(k, v)?;
        }

        map.end()
    }
}
```

# Implementing the `Visit` trait

Implement the [`visit::Visit`] trait to visit [`Value`]s:

```
use val::visit::{self, Visit};

struct Fmt;

impl Visit for Fmt {
    fn any(&mut self, v: visit::Value) -> Result<(), visit::Error> {
        print!("{:?} ", v);
        Ok(())
    }

    fn begin_seq(&mut self) -> Result<(), visit::Error> {
        print!("[ ");
        Ok(())
    }

    fn end_seq(&mut self) -> Result<(), visit::Error> {
        print!("] ");
        Ok(())
    }

    fn begin_map(&mut self) -> Result<(), visit::Error> {
        print!("{{ ");
        Ok(())
    }

    fn end_map(&mut self) -> Result<(), visit::Error> {
        print!("}} ");
        Ok(())
    }
}
```

There are more methods on `Visit` that can be overriden:

```
use std::{fmt, mem};
use val::visit::{self, Visit};

struct Fmt {
    delim: &'static str,
}

impl Fmt {
    fn print(&mut self, args: fmt::Arguments) {
        let delim = mem::replace(&mut self.delim, "");
        print!("{}{}", delim, args);
    }
}

impl Visit for Fmt {
    fn any(&mut self, v: visit::Value) -> Result<(), visit::Error> {
        self.print(format_args!("{:?}", v));
        self.delim = " ";

        Ok(())
    }

    fn begin_seq(&mut self) -> Result<(), visit::Error> {
        self.print(format_args!("["));
        Ok(())
    }

    fn seq_elem(&mut self, elem: visit::Value) -> Result<(), visit::Error> {
        self.print(format_args!("{:?}", elem));
        self.delim = ", ";

        Ok(())
    }

    fn end_seq(&mut self) -> Result<(), visit::Error> {
        self.delim = "";
        self.print(format_args!("]"));
        Ok(())
    }

    fn begin_map(&mut self) -> Result<(), visit::Error> {
        self.print(format_args!("{{"));
        Ok(())
    }

    fn map_key(&mut self, key: visit::Value) -> Result<(), visit::Error> {
        self.print(format_args!("{:?}", key));
        self.delim = ": ";

        Ok(())
    }

    fn map_value(&mut self, value: visit::Value) -> Result<(), visit::Error> {
        self.print(format_args!("{:?}", value));
        self.delim = ", ";

        Ok(())
    }

    fn end_map(&mut self) -> Result<(), visit::Error> {
        self.delim = "";
        self.print(format_args!("}}"));
        Ok(())
    }
}
```
*/

use std::{
    error,
    fmt,
};

#[cfg(feature = "serde")]
pub mod serde;

mod impls;

pub mod value;
pub mod visit;

/**
Value a value with the given visitor.
*/
pub fn visit(value: impl value::Value, visit: impl visit::Visit) -> Result<(), Error> {
    Value::erased(&value).visit(visit)
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

    pub fn erased(value: &'a impl value::Value) -> Self {
        Self::new(value)
    }

    pub fn boxed(value: impl value::Value + 'a) -> Self {
        Value {
            inner: ValueInner::Boxed(Box::new(value))
        }
    }

    pub fn visit(&self, mut visit: impl visit::Visit) -> Result<(), Error> {
        self.inner.as_ref().visit(Visit::new(&mut visit))
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

/**
A visitor for a structured value.

The `Visit` type abstracts over storage for a [`visit::Visit`] trait object.
It also imposes some limitations on the way the internal `Visit` can be called:

- Each implementation of [`value::Value`] may only call a single method on `Visit`.
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

    pub fn erased(visit: &'a mut impl visit::Visit) -> Self {
        Self::new(visit)
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
        self.inner.as_mut().begin_seq()?;

        Ok(VisitSeq {
            inner: self.inner,
            done: false,
        })
    }

    pub fn map(mut self) -> Result<VisitMap<'a>, Error> {
        self.inner.as_mut().begin_map()?;

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
    pub fn elem(&mut self, v: impl value::Value) -> Result<(), Error> {
        self.inner.as_mut().seq_elem(Value::erased(&v))
    }

    pub fn end(mut self) -> Result<(), Error> {
        self.done = true;
        self.inner.as_mut().end_seq()
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
    pub fn entry(&mut self, k: impl value::Value, v: impl value::Value) -> Result<(), Error> {
        self.inner.as_mut().map_key(Value::erased(&k))?;
        self.inner.as_mut().map_value(Value::erased(&v))?;

        Ok(())
    }

    pub fn end(mut self) -> Result<(), Error> {
        self.done = true;
        self.inner.as_mut().end_map()
    }
}

/**
An error encountered while visiting a value.
*/
pub struct Error {

}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error")
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "error"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}