use crate::std::fmt;

#[cfg(feature = "alloc")]
use crate::std::string::String;

#[cfg(feature = "std")]
use crate::std::{
    boxed::Box,
    error,
};

/**
An error encountered while visiting a value.

# Converting an `Error` into a standard error

The `Error` type doesn't implement the `std::error::Error` trait directly.
When `std` is available, the `into_error` method will convert an
`Error` into a value that implements `std::error::Error`.
*/
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

    /** Convert into a `fmt::Error` */
    #[inline]
    pub fn into_fmt_error(self) -> fmt::Error {
        fmt::Error
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
    #[cfg(feature = "std")]
    Source {
        msg: String,
        source: Box<dyn error::Error + Send + Sync + 'static>,
    },
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
            #[cfg(feature = "std")]
            ErrorInner::Source {
                ref msg,
                ref source,
            } => f
                .debug_struct("Error")
                .field("msg", &msg)
                .field("source", &source)
                .finish(),
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
            #[cfg(feature = "std")]
            ErrorInner::Source {
                ref msg,
                ref source,
            } => f.write_fmt(format_args!("{} ({})", msg, source)),
        }
    }
}

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Self {
        Error::msg("formatting failed")
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

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::{
        boxed::Box,
        error,
        io,
    };

    impl Error {
        /** Convert into an io error. */
        #[inline]
        pub fn into_io_error(self) -> io::Error {
            io::Error::new(io::ErrorKind::Other, self)
        }
    }

    impl error::Error for Error {
        fn source(&self) -> Option<&(dyn error::Error + 'static)> {
            if let ErrorInner::Source { ref source, .. } = self.0 {
                Some(&**source)
            } else {
                None
            }
        }
    }

    impl From<io::Error> for Error {
        fn from(err: io::Error) -> Self {
            Error(ErrorInner::Source {
                msg: "failed during an IO operation".into(),
                source: Box::new(err),
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use crate::std::error::Error as StdError;

        #[test]
        fn io_error() {
            let err = Error::from(io::Error::from(io::ErrorKind::Other));

            assert!(err.source().is_some());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::std::fmt;

    use super::*;

    #[test]
    fn fmt_error() {
        let _ = Error::from(fmt::Error);
    }
}
