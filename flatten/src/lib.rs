/*!
Flatten nested values when streaming.

This library is like the standard library's `Iterator::flatten` method, but for `sval::Stream`s.
Given a value, it will flatten its nested values onto a parent structure. It supports flattening
any combination of map, sequence, record, or tuple onto any other.

If you're using `sval_derive`, you can use the `#[flatten]` attribute on a field.
*/

#![no_std]
#![deny(missing_docs)]

extern crate alloc;

mod flattener;
mod index;
mod label;
mod map;
mod record;
mod record_tuple;
mod seq;
mod tuple;

pub use self::{map::*, record::*, record_tuple::*, seq::*, tuple::*};
