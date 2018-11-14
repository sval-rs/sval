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
# }
```

where `42` is a [`value::Value`] and `MyVisit` is a [`visit::Visit`].

# Implementing the `Value` trait

Implement the [`value::Value`] trait for datastructures that can be
visited using a [`value::Visit`]:

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

## for a sequence

A sequence can be visited by iterating over its elements:

```
use val::value::{self, Value};

#[derive(Debug)]
pub struct Seq(Vec<u64>);

impl Value for Seq {
    fn visit(&self, visit: value::Visit) -> Result<(), value::Error> {
        let mut seq = visit.seq(Some(self.0.len()))?;

        for v in &self.0 {
            seq.elem(v)?;
        }

        seq.end()
    }
}
```

## for a map

A map can be visited by iterating over its key-value pairs:

```
use std::collections::BTreeMap;

use val::value::{self, Value};

#[derive(Debug)]
pub struct Map(BTreeMap<String, u64>);

impl Value for Map {
    fn visit(&self, visit: value::Visit) -> Result<(), value::Error> {
        let mut map = visit.map(Some(self.0.len()))?;

        for (k, v) in &self.0 {
            map.entry(k, v)?;
        }

        map.end()
    }
}
```

# Implementing the `Visit` trait

Implement the [`visit::Visit`] trait to visit the structure
of a [`visit::Value`]:

```
use val::visit::{self, Visit};

struct Fmt;

impl Visit for Fmt {
    fn any(&mut self, v: visit::Value) -> Result<(), visit::Error> {
        println!("{:?}", v);
        Ok(())
    }
}
```

There are more methods on `Visit` that can be overriden for more complex
datastructures like sequences and maps:

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

    fn seq_begin(&mut self, _: Option<usize>) -> Result<(), visit::Error> {
        self.print(format_args!("["));
        Ok(())
    }

    fn seq_elem(&mut self, elem: visit::Value) -> Result<(), visit::Error> {
        elem.visit(self)?;
        self.delim = ", ";

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), visit::Error> {
        self.delim = "";
        self.print(format_args!("]"));
        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> Result<(), visit::Error> {
        self.print(format_args!("{{"));
        Ok(())
    }

    fn map_key(&mut self, key: visit::Value) -> Result<(), visit::Error> {
        key.visit(self)?;
        self.delim = ": ";

        Ok(())
    }

    fn map_value(&mut self, value: visit::Value) -> Result<(), visit::Error> {
        value.visit(self)?;
        self.delim = ", ";

        Ok(())
    }

    fn map_end(&mut self) -> Result<(), visit::Error> {
        self.delim = "";
        self.print(format_args!("}}"));
        Ok(())
    }
}
```

A `Visit` might only care about a single kind of value:

```
use std::{fmt, mem};
use val::{
    value::Value,
    visit::{self, Visit}
};

assert!(is_u64(42u64));

pub fn is_u64(v: impl Value) -> bool {
    let mut visit = IsU64(None);
    let _ = val::visit(v, &mut visit);

    visit.0.is_some()
}

struct IsU64(Option<u64>);
impl Visit for IsU64 {
    fn u64(&mut self, v: u64) -> Result<(), visit::Error> {
        self.0 = Some(v);
        Ok(())
    }

    fn any(&mut self, v: visit::Value) -> Result<(), visit::Error> {
        Err(visit::Error::msg("not a u64"))
    }
}
```
*/

#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate core as std;

mod impls;

pub mod value;
pub mod visit;

mod error;

pub use self::{
    error::Error,
    value::Value,
    visit::Visit,
};

/**
Value a value with the given visitor.
*/
pub fn visit(value: impl value::Value, mut visit: impl visit::Visit) -> Result<(), Error> {
    visit::Value::new(&value).visit(&mut visit)
}
