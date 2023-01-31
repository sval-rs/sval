use crate::{Error, ErrorKind};

use std::{
    fmt,
    io::{self, Write},
};

pub fn stream_to_writer(io: impl Write, v: impl sval::Value) -> Result<(), Error> {
    struct IoToFmt<W> {
        io: W,
        err: Option<io::Error>,
    }

    impl<W: Write> fmt::Write for IoToFmt<W> {
        fn write_str(&mut self, v: &str) -> fmt::Result {
            let mut buf = v.as_bytes();

            while buf.len() > 0 {
                match self.io.write(buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        buf = &buf[n..];
                    }
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(e) => {
                        self.err = Some(e);
                        return Err(fmt::Error);
                    }
                }
            }

            Ok(())
        }
    }

    let mut io = IoToFmt { io, err: None };

    match crate::stream_to_fmt(&mut io, v) {
        Ok(()) => Ok(()),
        Err(mut e) => {
            if let Some(io) = io.err {
                e.kind = ErrorKind::IO(io);
            }

            Err(e)
        }
    }
}
