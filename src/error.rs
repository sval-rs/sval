use crate::std::fmt;

#[cfg(feature = "alloc")]
use crate::std::string::String;

/**
An error encountered while visiting a value.

# Converting an `Error` into a standard error

The `Error` type doesn't implement the `std::error::Error` trait directly.
When `std` is available, the `into_error` method will convert an
`Error` into a value that implements `std::error::Error`.
*/
#[derive(Clone)]
pub struct Error(ErrorInner);

impl Error {
    /** Capture a static message as an error. */
    #[inline]
    pub fn msg(msg: &'static str) -> Self {
        Error(ErrorInner::Static(msg))
    }

    /** Declare some structure as unsupported. */
    #[inline]
    pub fn unsupported(operation: &'static str) -> Self {
        Error(ErrorInner::Unsupported {
            msg: operation,
            default: false,
        })
    }

    /** Whether or not an error is because some operation was unsupported. */
    pub fn is_unsupported(&self) -> bool {
        if let ErrorInner::Unsupported { .. } = self.0 {
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn default_unsupported(operation: &'static str) -> Self {
        Error(ErrorInner::Unsupported {
            msg: operation,
            default: true,
        })
    }

    #[allow(dead_code)]
    pub(crate) fn is_default_unsupported(&self) -> bool {
        if let ErrorInner::Unsupported { default: true, .. } = self.0 {
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone)]
enum ErrorInner {
    Unsupported {
        msg: &'static str,
        default: bool,
    },
    Static(&'static str),
    #[cfg(not(feature = "alloc"))]
    Custom(&'static dyn fmt::Display),
    #[cfg(feature = "alloc")]
    Owned(String),
}

impl fmt::Debug for ErrorInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorInner::Unsupported { msg: op, .. } => {
                write!(f, "unsupported stream operation: {}", op)
            }
            ErrorInner::Static(msg) => msg.fmt(f),
            #[cfg(not(feature = "alloc"))]
            ErrorInner::Custom(ref err) => err.fmt(f),
            #[cfg(feature = "alloc")]
            ErrorInner::Owned(ref msg) => msg.fmt(f),
        }
    }
}

impl fmt::Display for ErrorInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorInner::Unsupported { msg: op, .. } => {
                write!(f, "unsupported stream operation: {}", op)
            }
            ErrorInner::Static(msg) => msg.fmt(f),
            #[cfg(not(feature = "alloc"))]
            ErrorInner::Custom(ref err) => err.fmt(f),
            #[cfg(feature = "alloc")]
            ErrorInner::Owned(ref msg) => msg.fmt(f),
        }
    }
}

impl From<Error> for fmt::Error {
    fn from(_: Error) -> fmt::Error {
        fmt::Error
    }
}

#[cfg(not(feature = "alloc"))]
mod no_alloc_support {
    use super::*;

    use crate::std::fmt;

    impl Error {
        // NOTE: This method is not public because we already
        // have a method called `custom` when `alloc` is available.
        // It's strictly more general than this one, but could
        // be confusing to users to have bounds change depending
        // on cargo features
        pub(crate) fn custom(err: &'static dyn fmt::Display) -> Self {
            Error(ErrorInner::Custom(err))
        }
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::string::ToString;

    impl Error {
        /** Get an error from a format. */
        pub fn custom(err: impl fmt::Display) -> Self {
            Error(ErrorInner::Owned(err.to_string()))
        }
    }
}

#[cfg(not(feature = "std"))]
mod no_std_support {
    use super::*;

    use crate::std::fmt;

    impl From<fmt::Error> for Error {
        fn from(_: fmt::Error) -> Self {
            Error::msg("writing format failed")
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::{
        boxed::Box,
        error,
        io,
        string::ToString,
    };

    impl Error {
        /** The lower-level source of this error, if any. */
        pub fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            Some(self.as_error())
        }

        /** Get a reference to a standard error. */
        pub fn as_error(&self) -> &(dyn error::Error + Send + Sync + 'static) {
            &self.0
        }

        /** Convert into a standard error. */
        pub fn into_error(self) -> Box<dyn error::Error + Send + Sync> {
            Box::new(self.0)
        }

        /** Convert into an io error. */
        pub fn into_io_error(self) -> io::Error {
            io::Error::new(io::ErrorKind::Other, self.into_error())
        }
    }

    impl<E> From<E> for Error
    where
        E: error::Error,
    {
        fn from(err: E) -> Self {
            Error(ErrorInner::Owned(err.to_string()))
        }
    }

    impl AsRef<dyn error::Error + Send + Sync + 'static> for Error {
        fn as_ref(&self) -> &(dyn error::Error + Send + Sync + 'static) {
            self.as_error()
        }
    }

    impl From<Error> for Box<dyn error::Error + Send + Sync> {
        fn from(err: Error) -> Self {
            err.into_error()
        }
    }

    impl From<Error> for Box<dyn error::Error> {
        fn from(err: Error) -> Self {
            err.into_error()
        }
    }

    impl From<Error> for io::Error {
        fn from(err: Error) -> Self {
            err.into_io_error()
        }
    }

    impl error::Error for ErrorInner {}
}

#[cfg(test)]
mod tests {
    use crate::std::fmt;

    use super::*;

    #[test]
    fn convert_fmt_error_into_error() {
        let _ = Error::from(fmt::Error);
    }
}
