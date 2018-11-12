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
#     fn seq_begin(&mut self) -> Result<(), visit::Error> { unimplemented!() }
#     fn seq_end(&mut self) -> Result<(), visit::Error> { unimplemented!() }
#     fn map_begin(&mut self) -> Result<(), visit::Error> { unimplemented!() }
#     fn map_end(&mut self) -> Result<(), visit::Error> { unimplemented!() }
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

Implement the [`visit::Visit`] trait to visit [`visit::Value`]s:

```
use val::visit::{self, Visit};

struct Fmt;

impl Visit for Fmt {
    fn any(&mut self, v: visit::Value) -> Result<(), visit::Error> {
        print!("{:?} ", v);
        Ok(())
    }

    fn seq_begin(&mut self) -> Result<(), visit::Error> {
        print!("[ ");
        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), visit::Error> {
        print!("] ");
        Ok(())
    }

    fn map_begin(&mut self) -> Result<(), visit::Error> {
        print!("{{ ");
        Ok(())
    }

    fn map_end(&mut self) -> Result<(), visit::Error> {
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

    fn seq_begin(&mut self) -> Result<(), visit::Error> {
        self.print(format_args!("["));
        Ok(())
    }

    fn seq_elem(&mut self, elem: visit::Value) -> Result<(), visit::Error> {
        self.print(format_args!("{:?}", elem));
        self.delim = ", ";

        Ok(())
    }

    fn seq_end(&mut self) -> Result<(), visit::Error> {
        self.delim = "";
        self.print(format_args!("]"));
        Ok(())
    }

    fn map_begin(&mut self) -> Result<(), visit::Error> {
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

    fn map_end(&mut self) -> Result<(), visit::Error> {
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
    visit::Value::new(&value).visit(visit)
}

/**
An error encountered while visiting a value.
*/
pub struct Error {
    msg: String,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.msg
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}