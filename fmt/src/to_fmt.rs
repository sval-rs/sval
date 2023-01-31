use crate::writer::{GenericWriter, Writer};
use core::fmt::{self, Write};

pub fn stream_to_fmt(fmt: impl Write, v: impl sval::Value) -> fmt::Result {
    v.stream(&mut Writer::new(GenericWriter(fmt)))
        .map_err(|_| fmt::Error)
}
