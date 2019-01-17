/*!
Json support for `sval`.

This library is no-std, so it can be run in environments
that don't have access to an allocator.
*/

#![doc(html_root_url = "https://docs.rs/sval_json/0.1.0")]

#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
extern crate core as std;

mod fmt;
pub use self::fmt::to_fmt;

#[cfg(feature = "std")]
mod std_support;

#[cfg(feature = "std")]
pub use self::std_support::{
    to_string,
    to_writer,
};
