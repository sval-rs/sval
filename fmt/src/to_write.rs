use crate::writer::{GenericWriter, Writer};
use core::fmt::{self, Write};

/**
Format a value into an underlying formatter.
*/
pub fn stream_to_write(fmt: impl Write, v: impl sval::Value) -> fmt::Result {
    v.stream(&mut Writer::new(GenericWriter(fmt)))
        .map_err(|_| fmt::Error)
}
