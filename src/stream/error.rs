
use crate::std::{
    fmt,
};


/**
A streamable error.

This type shouldn't be confused with [`sval::Error`](../../struct.Error.html), which is
used to communicate errors back to callers.
The purpose of the `Source` type is to let streams serialize
error types, that may have backtraces and other metadata.

`Source`s can only be created when the `std` feature is available,
but streams can still work with them by formatting them or passing
them along even in no-std environments where the `Error` trait isn't available.
*/
pub struct Source<'a> {
    #[cfg(feature = "std")]
    inner: self::std_support::SourceError<'a>,
    #[cfg(not(feature = "std"))]
    _marker: crate::std::marker::PhantomData<&'a dyn crate::std::any::Any>,
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<'a> Source<'a> {
    pub(crate) fn empty() -> Self {
        Source {
            _marker: Default::default(),
        }
    }
}

impl<'a> fmt::Debug for Source<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "std")]
        {
            fmt::Debug::fmt(&self.inner, f)
        }

        #[cfg(not(feature = "std"))]
        {
            f.debug_struct("Source").finish()
        }
    }
}

impl<'a> fmt::Display for Source<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "std")]
        {
            fmt::Display::fmt(&self.inner, f)
        }

        #[cfg(not(feature = "std"))]
        {
            f.debug_struct("Source").finish()
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use crate::std::{
        fmt,
        error::Error,
        marker::PhantomData,
        mem,
    };

    use super::Source;

    pub(super) struct SourceError<'a> {
        extended: ExtendedLifetimeError,
        _marker: PhantomData<&'a dyn Error>,
    }

    impl<'a> fmt::Debug for SourceError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(&self.extended, f)
        }
    }

    impl<'a> fmt::Display for SourceError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.extended, f)
        }
    }

    /**
    A wrapper over an error type with an artificially extended lifetime.

    Borrows of this value are returned by `SourceError` but it's important
    that callers can't get at the inner `&'static dyn Error` directly. They
    also can't downcast the value to a `ExtendedLifetimeError` or the inner
    value even if it does support downcasting, but they can iterate its causes
    and grab a backtrace.
    */
    struct ExtendedLifetimeError(&'static dyn Error);

    impl fmt::Debug for ExtendedLifetimeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self.0, f)
        }
    }

    impl fmt::Display for ExtendedLifetimeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(self.0, f)
        }
    }

    impl Error for ExtendedLifetimeError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.0.source()
        }

        // NOTE: Once backtraces are stable, add them here too
    }

    impl<'a> Error for &'a ExtendedLifetimeError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.0.source()
        }

        // NOTE: Once backtraces are stable, add them here too
    }

    impl<'a> Source<'a> {
        /**
        Capture an error source from a standard error.

        This method is only available when the `std` feature is enabled.
        */
        pub fn new(err: &'a impl Error) -> Self {
            Source::from(err as &'a dyn Error)
        }

        /**
        Get the inner error.
        */
        pub fn get<'b>(&'b self) -> impl Error + 'b {
            &self.inner.extended
        }
    }

    impl<'a> AsRef<dyn Error + 'static> for Source<'a> {
        fn as_ref(&self) -> &(dyn Error + 'static) {
            &self.inner.extended
        }
    }

    impl<'a> From<&'a dyn Error> for Source<'a> {
        fn from(err: &'a dyn Error) -> Self {
            Source {
                inner: SourceError {
                    // SAFETY: We're careful not to expose the actual value with the fake lifetime
                    // Calling code can only interact with it through an arbitrarily short borrow
                    // that's bound to `'a` from `self`, which is the real McCoy lifetime of the error
                    extended: ExtendedLifetimeError(unsafe { mem::transmute::<&'a dyn Error, &'static dyn Error>(err) }),
                    _marker: PhantomData,
                },
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use crate::std::io;

        #[test]
        fn error_downcast() {
            let err = io::Error::from(io::ErrorKind::Other);

            let source = Source::new(&err);

            // Downcasting isn't supported by `Source`
            assert!(source.as_ref().downcast_ref::<io::Error>().is_none());
        }
    }
}
