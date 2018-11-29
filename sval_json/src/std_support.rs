use crate::std::string::String;
use crate::std::{
    fmt,
    io::Write,
};

pub fn to_string(v: impl sval::Value) -> Result<String, sval::Error> {
    let mut out = String::new();

    crate::to_fmt(&mut out, v)?;

    Ok(out)
}

pub fn to_writer(writer: impl Write, v: impl sval::Value) -> Result<(), sval::Error> {
    crate::to_fmt(Writer(writer), v)
}

struct Writer<W>(W);

impl<W> fmt::Write for Writer<W>
where
    W: Write,
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.0.write(s.as_bytes()).map_err(|_| fmt::Error)?;

        Ok(())
    }
}
