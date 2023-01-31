/*!
Treat any `sval::Value` as a `std::fmt::Debug`.

This crate provides `ToDebug`, a wrapper around any `sval::Value`
that formats it using the same output that you'd get if you
derived `std::fmt::Debug`.
*/

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

mod writer;

mod to_debug;
mod to_fmt;

pub use self::{to_debug::*, to_fmt::*};

#[cfg(feature = "alloc")]
mod to_string;

#[cfg(feature = "alloc")]
pub use self::to_string::stream_to_string;
