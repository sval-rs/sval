use crate::std::fmt;

pub struct Error(&'static dyn fmt::Display);

impl Error {
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
