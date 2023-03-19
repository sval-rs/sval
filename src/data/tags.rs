/*!
Built-in tags for fundamental types.
*/

use super::Tag;

/**
A tag for a value that represents the `Some` variant of a Rust `Option`.
*/
pub const RUST_OPTION_SOME: Tag = Tag::new("RUST_OPTION_SOME");

/**
A tag for a value that represents the `None` variant of a Rust `Option`.
*/
pub const RUST_OPTION_NONE: Tag = Tag::new("RUST_OPTION_NONE");

/**
A tag for Rust's `()` type.
*/
pub const RUST_UNIT: Tag = Tag::new("RUST_UNIT");

/**
A tag for arbitrary-precision decimal numbers.
*/
pub const NUMBER: Tag = Tag::new("NUMBER");

/**
A tag for values that have a constant size.
*/
pub const CONSTANT_SIZE: Tag = Tag::new("CONSTANT_SIZE");
