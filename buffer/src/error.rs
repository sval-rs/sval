use crate::std::fmt;

/**
An error encountered buffering data.
*/
#[derive(Debug)]
pub struct Error(ErrorKind);

#[derive(Debug)]
enum ErrorKind {
    Unsupported {
        actual: &'static str,
        expected: &'static str,
    },
    #[cfg(feature = "alloc")]
    OutsideContainer { method: &'static str },
    #[cfg(feature = "alloc")]
    InvalidValue { reason: &'static str },
    #[cfg(not(feature = "alloc"))]
    NoAlloc { method: &'static str },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ErrorKind::Unsupported { actual, expected } => {
                write!(f, "unexpected {}, expected {}", actual, expected)
            }
            #[cfg(feature = "alloc")]
            ErrorKind::OutsideContainer { method } => {
                write!(f, "expected a fragment while buffering {}", method)
            }
            #[cfg(feature = "alloc")]
            ErrorKind::InvalidValue { reason } => {
                write!(f, "the value being buffered is invalid: {}", reason)
            }
            #[cfg(not(feature = "alloc"))]
            ErrorKind::NoAlloc { method } => write!(f, "cannot allocate for {}", method),
        }
    }
}

impl Error {
    pub(crate) fn unsupported(expected: &'static str, actual: &'static str) -> Self {
        Error(ErrorKind::Unsupported { actual, expected })
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn outside_container(method: &'static str) -> Self {
        Error(ErrorKind::OutsideContainer { method })
    }

    #[cfg(feature = "alloc")]
    pub(crate) fn invalid_value(reason: &'static str) -> Self {
        Error(ErrorKind::InvalidValue { reason })
    }

    #[cfg(not(feature = "alloc"))]
    pub(crate) fn no_alloc(method: &'static str) -> Self {
        Error(ErrorKind::NoAlloc { method })
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::error;

    impl error::Error for Error {}
}
