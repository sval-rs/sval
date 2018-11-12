/*!
A lightweight serialization framework.

# Implementing the `Value` trait

The [`Value`] trait drives a [`Visit`], which has methods
for observing different kinds of datastructures:

```
use val::Value;

#[derive(Debug)]
pub struct Id(u64);

impl Value for Id {
    fn visit(&self, visit: val::Visit) -> Result<(), val::Error> {
        visit.u64(self.0)
    }
}
```

Sequences can be visited:

```
use val::Value;

#[derive(Debug)]
pub struct Seq(Vec<u64>);

impl Value for Seq {
    fn visit(&self, visit: val::Visit) -> Result<(), val::Error> {
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
use val::Value;

#[derive(Debug)]
pub struct Map(BTreeMap<String, u64>);

impl Value for Id {
    fn visit(&self, visit: val::Visit) -> Result<(), val::Error> {
        let mut map = visit.map()?;

        for (k, v) in &self.0 {
            map.entry(k, v)?;
        }

        map.end()
    }
}
```

# Visiting `Value`s

Implement the [`visit::Visit`] trait to visit structured values:

```
use std::fmt;
use val::visit::Visit;

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
    fn any(&mut self, v: &dyn val::Value) -> Result<(), val::Error> {
        self.print(format_args!("{:?}", v));
        self.delim = " ";

        Ok(())
    }

    fn begin_seq(&mut self) -> Result<(), val::Error> {
        self.print(format_args!("["));
        Ok(())
    }

    fn seq_elem(&mut self, elem: &dyn val::Value) -> Result<(), val::Error> {
        self.print(format_args!("{:?}", elem));
        self.delim = ", ";

        Ok(())
    }

    fn end_seq(&mut self) -> Result<(), val::Error> {
        self.delim = "";
        self.print(format_args!("]"));
        Ok(())
    }

    fn begin_map(&mut self) -> Result<(), val::Error> {
        self.print(format_args!("{{"));
        Ok(())
    }

    fn map_key(&mut self, key: &dyn val::Value) -> Result<(), val::Error> {
        self.print(format_args!("{:?}", key));
        self.delim = ": ";

        Ok(())
    }

    fn map_value(&mut self, value: &dyn val::Value) -> Result<(), val::Error> {
        self.print(format_args!("{:?}", value));
        self.delim = ", ";

        Ok(())
    }

    fn end_map(&mut self) -> Result<(), val::Error> {
        self.delim = "";
        self.print(format_args!("}}"));
        Ok(())
    }
}
```
*/

use std::{
    error,
    fmt::{self, Debug}
};

#[cfg(feature = "serde")]
pub mod serde;

mod impls;
pub mod visit;

#[doc(inline)]
pub use crate::visit::Visit;

/**
A value that can be visited.
*/
pub trait Value: Debug {
    fn visit(&self, visit: &mut dyn Visit) -> Result<(), Error>;
}

impl<'a, T: ?Sized> Value for &'a T
where
    T: Value,
{
    fn visit(&self, visit: &mut dyn Visit) -> Result<(), Error> {
        (**self).visit(visit)
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
