use std::{error, fmt};

pub struct Error {
    msg: String,
}

pub(crate) fn bail<T>(err: impl Into<String>) -> Result<T, Error> {
    #[cfg(debug_assertions)]
    {
        panic!(err.into())
    }

    #[cfg(not(debug_assertions))]
    {
        Err(Error { msg: err.into() })
    }
}

macro_rules! ensure {
    ($expr:expr, $err:expr) => {
        if !$expr {
            $crate::error::bail($err)
        } else {
            Ok(())
        }
    };
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.msg.fmt(f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.msg.fmt(f)
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }

    fn description(&self) -> &str {
        &self.msg
    }
}
