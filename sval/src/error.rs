use crate::std::fmt;

#[cfg(feature = "std")]
use crate::std::string::String;

/**
An error encountered while visiting a value.
*/
pub struct Error(ErrorInner);

impl Error {
    /** Capture a static message as an error. */
    pub fn msg(msg: &'static str) -> Self {
        #[cfg(debug_assertions)]
        {
            panic!(msg)
        }

        #[cfg(not(debug_assertions))]
        {
            Error(ErrorInner::Static(msg))
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

enum ErrorInner {
    Static(&'static str),
    #[cfg(feature = "std")]
    Owned(String),
}

impl fmt::Debug for ErrorInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorInner::Static(msg) => msg.fmt(f),
            #[cfg(feature = "std")]
            ErrorInner::Owned(ref msg) => msg.fmt(f),
        }
    }
}

impl fmt::Display for ErrorInner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorInner::Static(msg) => msg.fmt(f),
            #[cfg(feature = "std")]
            ErrorInner::Owned(ref msg) => msg.fmt(f),
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
        /** Get an error from a format. */
        pub fn custom(err: impl fmt::Display) -> Self {
            Error(ErrorInner::Owned(err.to_string()))
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

    impl error::Error for ErrorInner {
        fn description(&self) -> &str {
            match self {
                ErrorInner::Static(msg) => msg,
                ErrorInner::Owned(msg) => msg,
            }
        }
    }
}
