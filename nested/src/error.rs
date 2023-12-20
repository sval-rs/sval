use core::fmt;

/**
An error encountered buffering data.
*/
#[derive(Debug)]
pub struct Error(ErrorKind);

#[derive(Debug)]
enum ErrorKind {
    Buffer(sval_buffer::Error),
    InvalidValue { reason: &'static str },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ErrorKind::Buffer(_) => {
                write!(f, "failed to buffer a value")
            }
            ErrorKind::InvalidValue { reason } => {
                write!(f, "the value is invalid: {}", reason)
            }
        }
    }
}

impl Error {
    pub(crate) fn buffer(err: sval_buffer::Error) -> Self {
        Error(ErrorKind::Buffer(err))
    }

    /**
    The given value is invalid.
    */
    pub fn invalid_value(reason: &'static str) -> Self {
        Error(ErrorKind::InvalidValue { reason })
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use std::error;

    impl error::Error for Error {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            match self.0 {
                ErrorKind::Buffer(ref err) => Some(err),
                ErrorKind::InvalidValue { .. } => None,
            }
        }
    }
}
