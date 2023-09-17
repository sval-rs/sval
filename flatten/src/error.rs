use core::fmt;
use core::num::ParseIntError;
use std::num::ParseFloatError;

/**
An error encountered flattening data.
*/
#[derive(Debug)]
pub struct Error(ErrorKind);

#[derive(Debug)]
enum ErrorKind {
    Unflattenable { kind: &'static str },
    ComplexLabel { kind: &'static str },
    BufferInt(ParseIntError),
    BufferFloat(ParseFloatError),
    BufferText(sval_buffer::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ErrorKind::Unflattenable { kind } => {
                write!(f, "a {} value cannot be flattened", kind)
            }
            ErrorKind::ComplexLabel { kind } => {
                write!(f, "a {} value cannot be converted into a label", kind)
            }
            ErrorKind::BufferInt(_) => {
                write!(f, "failed to buffer an integer")
            }
            ErrorKind::BufferFloat(_) => {
                write!(f, "failed to buffer a floating point")
            }
            ErrorKind::BufferText(_) => {
                write!(f, "failed to buffer text")
            }
        }
    }
}

impl Error {
    pub(crate) fn unflattenable(kind: &'static str) -> Self {
        Error(ErrorKind::Unflattenable { kind })
    }

    pub(crate) fn complex_label(kind: &'static str) -> Self {
        Error(ErrorKind::ComplexLabel { kind })
    }

    pub(crate) fn buffer_int(err: ParseIntError) -> Self {
        Error(ErrorKind::BufferInt(err))
    }

    pub(crate) fn buffer_float(err: ParseFloatError) -> Self {
        Error(ErrorKind::BufferFloat(err))
    }

    pub(crate) fn buffer_text(err: sval_buffer::Error) -> Self {
        Error(ErrorKind::BufferText(err))
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::error;

    impl error::Error for Error {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match self.0 {
                ErrorKind::Unflattenable { .. } => None,
                ErrorKind::ComplexLabel { .. } => None,
                ErrorKind::BufferInt(ref err) => Some(err),
                ErrorKind::BufferFloat(ref err) => Some(err),
                ErrorKind::BufferText(ref err) => Some(err),
            }
        }
    }
}
