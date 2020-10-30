use crate::std::fmt;

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
        error::Error,
        fmt,
    };

    use super::Source;

    pub(super) struct SourceError<'a>(&'a (dyn Error + 'static));

    impl<'a> fmt::Debug for SourceError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    impl<'a> fmt::Display for SourceError<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    impl<'a> Source<'a> {
        /**
        Capture an error source from a standard error.

        This method is only available when the `std` feature is enabled.
        */
        pub fn new(err: &'a (dyn Error + 'static)) -> Self {
            Source::from(err)
        }

        /**
        Get the inner error.
        */
        pub fn get(&self) -> &(dyn Error + 'static) {
            self.inner.0
        }
    }

    impl<'a> AsRef<dyn Error + 'static> for Source<'a> {
        fn as_ref(&self) -> &(dyn Error + 'static) {
            self.inner.0
        }
    }

    impl<'a> From<&'a (dyn Error + 'static)> for Source<'a> {
        fn from(err: &'a (dyn Error + 'static)) -> Self {
            Source {
                inner: SourceError(err),
            }
        }
    }

    // `Source` doesn't implement `Error` itself because it's not _really_ an error
    // It's just a carrier for one

    #[cfg(test)]
    mod tests {
        use super::*;

        use crate::std::io;

        #[test]
        fn error_downcast() {
            let err = io::Error::from(io::ErrorKind::Other);
            let source = Source::new(&err);

            assert!(source.as_ref().downcast_ref::<io::Error>().is_some());
        }
    }
}
