use crate::Error;

use alloc::string::String;

/**
Stream a value as JSON into a string.

This method will fail if the value contains complex values as keys.
*/
pub fn stream_to_string(v: impl sval::Value) -> Result<String, Error> {
    let mut out = String::new();
    crate::stream_to_fmt(&mut out, v)?;

    Ok(out)
}
