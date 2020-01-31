use crate::std::fmt;

use serde_lib::ser;

/**
An error encountered during serialization.
*/
pub(super) struct Error(crate::Error);

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Self {
        Error(crate::Error::msg("error during formatting"))
    }
}

impl From<crate::Error> for Error {
    fn from(err: crate::Error) -> Self {
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

impl ser::StdError for Error {}

pub(super) fn err<E>(msg: &'static str) -> impl FnOnce(E) -> crate::Error
where
    E: ser::Error,
{
    #[cfg(feature = "alloc")]
    {
        let _ = msg;
        move |err| crate::Error::custom(err)
    }

    #[cfg(not(feature = "alloc"))]
    {
        move |_| crate::Error::msg(msg)
    }
}

#[cfg(not(feature = "alloc"))]
mod no_alloc_support {
    use super::*;

    impl ser::Error for Error {
        fn custom<E>(_: E) -> Self
        where
            E: fmt::Display,
        {
            Error(crate::Error::msg("serialization error"))
        }
    }
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    impl ser::Error for Error {
        fn custom<E>(e: E) -> Self
        where
            E: fmt::Display,
        {
            Error(crate::Error::custom(e))
        }
    }
}
