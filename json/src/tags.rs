/**
A tag for values that are already in a JSON compatible form.

For strings, that means they either don't need escaping or are already escaped.
For numbers, that means they're already in a JSON compatible format.
*/
pub const JSON_NATIVE: sval::Tag = sval::Tag::new("JSON_NATIVE");
