/*!
Built-in tags for fundamental types.
*/

use super::Tag;

/**
A tag for a value that represents the `Some` variant of a Rust `Option`.
*/
pub const RUST_OPTION_SOME: Tag = Tag::new("rsome");

/**
A tag for a value that represents the `None` variant of a Rust `Option`.
*/
pub const RUST_OPTION_NONE: Tag = Tag::new("rnone");

/**
A tag for Rust's `()` type.
*/
pub const RUST_UNIT: Tag = Tag::new("r()");

/**
A tag for arbitrary-precision decimal numbers.
*/
pub const NUMBER: Tag = Tag::new("svalnum");

/**
A tag for values that have a constant size.
*/
pub const CONSTANT_SIZE: Tag = Tag::new("svalcs");
