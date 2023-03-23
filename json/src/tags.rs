/*!
Tags for JSON-specific types.
*/

/**
A tag for strings that either don't contain characters that need escaping or are already escaped.
*/
pub const JSON_TEXT: sval::Tag = sval::Tag::new("JSON_TEXT");

/**
A tag for numbers that are already JSON compatible.

This tag is a sub-type of [`sval::tags::NUMBER`] that:

- Does not contain leading zeroes.
- Does not use the `+` sign.
*/
pub const JSON_NUMBER: sval::Tag = sval::Tag::new("JSON_NUMBER");
