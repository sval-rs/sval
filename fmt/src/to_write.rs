use crate::writer::{GenericWriter, TokenWrite, Writer};
use core::fmt::{self, Write};

/**
Format a value into an underlying formatter.
*/
pub fn stream_to_write(fmt: impl Write, v: impl sval::Value) -> fmt::Result {
    v.stream(&mut Writer::new(GenericWriter(fmt)))
        .map_err(|_| fmt::Error)
}

/**
Format a value into an underlying token-aware formatter.
*/
pub fn stream_to_token_write(fmt: impl TokenWrite, v: impl sval::Value) -> fmt::Result {
    v.stream(&mut Writer::new(fmt)).map_err(|_| fmt::Error)
}
