use crate::std::fmt;

use serde::ser;

/**
An error encountered during serialization.
*/
pub(crate) struct Error(val::Error);

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Self {
        Error(val::Error::msg("error during formatting"))
    }
}

impl From<val::Error> for Error {
    fn from(err: val::Error) -> Self {
        Error(err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub(crate) fn err<E>(msg: &'static str) -> impl FnOnce(E) -> val::Error
where
    E: ser::Error,
{
    #[cfg(feature = "std")]
    {
        let _ = msg;
        move |err| val::Error::from(err)
    }

    #[cfg(not(feature = "std"))]
    {
        move |_| val::Error::msg(msg)
    }
}

#[cfg(not(feature = "std"))]
mod core_support {
    use super::*;

    impl ser::Error for Error {
        fn custom<E>(_: E) -> Self
        where
            E: fmt::Display,
        {
            Error(val::Error::msg("serialization error"))
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::error;

    impl error::Error for Error {
        fn cause(&self) -> Option<&dyn error::Error> {
            None
        }

        fn description(&self) -> &str {
            "serialization error"
        }
    }

    impl ser::Error for Error {
        fn custom<E>(e: E) -> Self
        where
            E: fmt::Display,
        {
            Error(val::Error::custom(e))
        }
    }
}
