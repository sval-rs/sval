#![doc(html_root_url = "https://docs.rs/sval_json/1.0.0-alpha.5")]
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
