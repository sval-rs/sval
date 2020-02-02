/*!
JSON support for `sval`.

This library is no-std, so it can be run in environments
that don't have access to an allocator.

# Getting started

Add `sval_json` to your `Cargo.toml`:

```toml,ignore
[dependencies.sval_json]
version = "0.5.1"
```

# Writing JSON to `fmt::Write`

```no_run
# #[cfg(not(feature = "std"))]
# fn main() {}
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let json = sval_json::to_fmt(MyWrite, 42)?;
# Ok(())
# }
# use std::fmt::{self, Write};
# struct MyWrite;
# impl Write for MyWrite {
#     fn write_str(&mut self, _: &str) -> fmt::Result {
#         Ok(())
#     }
# }
```

# Writing JSON to a `String`

Add the `std` feature to your `Cargo.toml` to enable writing to a `String`:

```toml,no_run
[dependencies.sval_json]
features = ["std"]
```

```no_run
# #[cfg(not(feature = "std"))]
# fn main() {}
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let json = sval_json::to_string(42)?;
# Ok(())
# }
```

# Writing JSON to a `io::Write`

Add the `std` feature to your `Cargo.toml` to enable writing to an `io::Write`:

```toml,no_run
[dependencies.sval_json]
features = ["std"]
```

```no_run
# #[cfg(not(feature = "std"))]
# fn main() {}
# #[cfg(feature = "std")]
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# use std::io::{self, Write};
# struct MyWrite;
# impl Write for MyWrite {
#     fn write(&mut self, _: &[u8]) -> io::Result<usize> {
#         Ok(0)
#     }
#     fn flush(&mut self) -> io::Result<()> { Ok(()) }
# }
let json = sval_json::to_writer(MyWrite, 42)?;
# Ok(())
# }
```
*/

#![doc(html_root_url = "https://docs.rs/sval_json/0.5.1")]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate core as std;

mod fmt;
pub use self::fmt::{
    to_fmt,
    Formatter,
};

#[cfg(feature = "std")]
mod std_support;

#[cfg(feature = "std")]
pub use self::std_support::{
    to_string,
    to_writer,
    Writer,
};

/**
An error attempting to end a JSON stream.

The original stream can be pulled out, or this type can be treated as a standard error.
*/
pub struct End<T> {
    /** The original stream. */
    pub stream: T,
    err: sval::Error,
    _private: (),
}

impl<T> End<T> {
    fn new(err: sval::Error, stream: T) -> Self {
        End {
            err,
            stream,
            _private: (),
        }
    }
}

impl<T> crate::std::fmt::Debug for End<T> {
    fn fmt(&self, f: &mut crate::std::fmt::Formatter) -> crate::std::fmt::Result {
        f.debug_struct("End").field("err", &self.err).finish()
    }
}

impl<T> crate::std::fmt::Display for End<T> {
    fn fmt(&self, f: &mut crate::std::fmt::Formatter) -> crate::std::fmt::Result {
        write!(
            f,
            "failed to take the inner JSON writer because it is invalid"
        )
    }
}
