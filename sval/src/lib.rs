/*!
A lightweight serialization-only framework.

# Streaming values

```no_run
# fn main() -> Result<(), Box<std::error::Error>> {
sval::stream(42, MyStream)?;
# Ok(())
# }
# use sval::stream::{self, Stream};
# struct MyStream;
# impl Stream for MyStream {
#     fn fmt(&mut self, _: stream::Pos, _: stream::Arguments) -> Result<(), stream::Error> { unimplemented!() }
# }
```

where `42` is a [`Value`] and `MyStream` is a [`Stream`].

# Implementing the `Value` trait

Implement the [`Value`] trait for datastructures that can be
visited using a [`value::Stream`]:

```
use sval::value::{self, Value};

pub struct Id(u64);

impl Value for Id {
    fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
        stream.u64(self.0)
    }
}
```

## for a sequence

A sequence can be visited by iterating over its elements:

```
use sval::value::{self, Value};

pub struct Seq(Vec<u64>);

impl Value for Seq {
    fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
        stream.seq_begin(Some(self.0.len()))?;

        for v in &self.0 {
            stream.seq_elem()?.any(v)?;
        }

        stream.seq_end()
    }
}
```

## for a map

A map can be visited by iterating over its key-value pairs:

```
use std::collections::BTreeMap;

use sval::value::{self, Value};

pub struct Map(BTreeMap<String, u64>);

impl Value for Map {
    fn stream(&self, stream: &mut value::Stream) -> Result<(), value::Error> {
        stream.map_begin(Some(self.0.len()))?;

        for (k, v) in &self.0 {
            stream.map_key()?.any(k)?;
            stream.map_value()?.any(v)?;
        }

        stream.map_end()
    }
}
```

# Implementing the `Stream` trait

Implement the [`Stream`] trait to visit the structure of a [`Value`]:

```
use sval::stream::{self, Stream};

struct Fmt;

impl Stream for Fmt {
    fn fmt(&mut self, _: stream::Pos, v: stream::Arguments) -> Result<(), stream::Error> {
        println!("{}", v);
        Ok(())
    }
}
```

There are more methods on `Stream` that can be overriden for more complex
datastructures like sequences and maps:

```
use std::{fmt, mem};
use sval::stream::{self, Stream};

struct Fmt {
    delim: &'static str,
}

impl Stream for Fmt {
    // Print a single value.
    fn fmt(&mut self, pos: stream::Pos, v: stream::Arguments) -> Result<(), stream::Error> {
        let delim = mem::replace(&mut self.delim, Self::next_delim(pos));
        print!("{}{:?}", delim, v);

        Ok(())
    }

    // Begin a sequence.
    fn seq_begin(&mut self, _: stream::Pos, _: Option<usize>) -> Result<(), stream::Error> {
        let delim = mem::replace(&mut self.delim, "");
        print!("{}[", delim);

        Ok(())
    }

    // End a sequence.
    fn seq_end(&mut self, pos: stream::Pos) -> Result<(), stream::Error> {
        self.delim = Self::next_delim(pos);
        print!("]");

        Ok(())
    }

    // Begin a map.
    fn map_begin(&mut self, _: stream::Pos, _: Option<usize>) -> Result<(), stream::Error> {
        let delim = mem::replace(&mut self.delim, "");
        print!("{}{{", delim);

        Ok(())
    }

    // End a map.
    fn map_end(&mut self, pos: stream::Pos) -> Result<(), stream::Error> {
        self.delim = Self::next_delim(pos);
        print!("}}");

        Ok(())
    }

    // End the stream.
    fn end(&mut self) -> Result<(), stream::Error> {
        println!();

        Ok(())
    }
}

impl Fmt {
    fn next_delim(pos: stream::Pos) -> &'static str {
        use sval::stream::Pos::*;

        match pos {
            Root => "",
            Key => ": ",
            Value | Elem => ", ",
        }
    }
}
```

A `Stream` might only care about a single kind of value:

```
use std::{fmt, mem};
use sval::{
    Value,
    stream::{self, Stream}
};

assert!(is_u64(42u64));

pub fn is_u64(v: impl Value) -> bool {
    let mut stream = IsU64(None);
    let _ = sval::stream(v, &mut stream);

    stream.0.is_some()
}

struct IsU64(Option<u64>);
impl Stream for IsU64 {
    fn u64(&mut self, _: stream::Pos, v: u64) -> Result<(), stream::Error> {
        self.0 = Some(v);
        Ok(())
    }

    fn fmt(&mut self, _: stream::Pos, _: stream::Arguments) -> Result<(), stream::Error> {
        Err(stream::Error::msg("not a u64"))
    }
}
```
*/

#![cfg_attr(test, feature(test))]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate core as std;

#[cfg(test)]
extern crate test;

#[macro_use]
mod error;

pub mod stream;
pub mod value;

pub use self::{
    error::Error,
    stream::Stream,
    value::Value,
};

/**
Stream the structure of a [`Value`] using the given [`Stream`]. 
*/
pub fn stream(value: impl Value, mut stream: impl Stream) -> Result<(), Error> {
    let mut stream = value::Stream::begin(&mut stream)?;
    value.stream(&mut stream)?;
    stream.end()
}
