use crate::Error;

use alloc::string::String;

pub fn stream_to_string(v: impl sval::Value) -> Result<String, Error> {
    let mut out = String::new();
    crate::stream_to_fmt(&mut out, v)?;

    Ok(out)
}
