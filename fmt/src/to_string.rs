use alloc::string::String;

/**
Format a value into a string.
*/
pub fn stream_to_string(v: impl sval::Value) -> String {
    let mut out = String::new();
    crate::stream_to_fmt(&mut out, v).expect("infallible write");
    out
}
