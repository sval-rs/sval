use crate::std::fmt;

/**
An error encountered while visiting a value.
*/
pub struct Error(&'static dyn fmt::Display);

impl Error {
    /** Capture a static message as an error. */
    pub(crate) fn custom(msg: &'static dyn fmt::Display) -> Self {
        Error(msg)
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
