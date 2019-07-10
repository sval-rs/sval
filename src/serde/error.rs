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

pub(super) fn err<E>(msg: &'static str) -> impl FnOnce(E) -> crate::Error
where
    E: ser::Error,
{
    #[cfg(feature = "serde_std")]
    {
        let _ = msg;
        move |err| crate::Error::from(err)
    }

    #[cfg(not(feature = "serde_std"))]
    {
        move |_| crate::Error::msg(msg)
    }
}

#[cfg(not(feature = "serde_std"))]
mod core_support {
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

#[cfg(feature = "serde_std")]
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
            Error(crate::Error::custom(e))
        }
    }
}
