/*!
Flatten nested values when streaming.

This library is like the standard library's `Iterator::flatten` method, but for `sval::Stream`s.
Given a value, it will flatten its nested values onto a parent structure. It supports flattening
any combination of map, sequence, record, or tuple onto any other.

If you're using `sval_derive`, you can use the `#[flatten]` attribute on a field.

# Specifics

Flattening unwraps containers and translates their values into the form needed by their parent.
The following types can be flattened:

- maps
- sequences
- records
- tuples

Any other type, including primitives like tags, booleans, and text will be ignored if flattened.

## Maps

- **maps**: keys are passed through directly, even if they're complex values like other maps.
- **sequences**: keys are the stringified offset of sequence values.
- **records**: keys are the label of record values.
- **tuples**: keys are the stringified index of tuple values.

## Sequences

- **maps**: map keys are ignored; only map values are flattened.
- **sequences**: sequence values are passed through directly.
- **records**: record values are passed through directly.
- **tuples**: tuple values are passed through directly.

## Records

- **maps**: map keys are stringified into labels. For complex values like other maps each nested
value is stringified and concatenated together.
- **sequences**: labels are the stringified offset of sequence values.
- **records**: labels are passed through directly.
- **tuples**: labels are the stringified index of tuple values.

## Tuples

- **maps**: map keys are ignored; only map values are flattened.
- **sequences**: indexes are the offset of sequence values.
- **records**: indexes are the offset of record values.
- **tuples**: tuple values are passed through directly.
*/

#![doc(html_logo_url = "https://raw.githubusercontent.com/sval-rs/sval/main/asset/logo.svg")]
#![no_std]
#![deny(missing_docs)]

#[cfg(any(test, feature = "alloc"))]
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
