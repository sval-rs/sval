/*!
JSON support for `sval`.

Values are serialized in a `serde`-compatible way.
*/

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod error;

mod to_fmt;
pub use self::{error::*, to_fmt::*};

pub mod tags {
    /**
    A tag for values that are already in a JSON compatible form.

    For strings, that means they either don't need escaping or are already escaped.
    For numbers, that means they're already in a JSON compatible format.
    */
    pub const JSON_NATIVE: sval::Tag = sval::Tag::new("JSON_NATIVE");
}

#[cfg(feature = "alloc")]
mod to_string;

#[cfg(feature = "alloc")]
pub use self::to_string::*;

#[cfg(feature = "std")]
mod to_io;

#[cfg(feature = "std")]
pub use self::to_io::*;

#[cfg(feature = "std")]
mod to_vec;

#[cfg(feature = "std")]
pub use self::to_vec::*;
