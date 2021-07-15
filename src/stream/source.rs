use crate::std::fmt;

pub struct Source<'v> {
    #[cfg(feature = "std")]
    inner: self::std_support::SourceError<'v>,
    #[cfg(not(feature = "std"))]
    _marker: crate::std::marker::PhantomData<&'v dyn crate::std::any::Any>,
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<'v> Source<'v> {
    pub(crate) fn empty() -> Self {
        Source {
            _marker: Default::default(),
        }
    }
}

impl<'v> fmt::Debug for Source<'v> {
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

impl<'v> fmt::Display for Source<'v> {
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

    pub(super) struct SourceError<'v>(&'v (dyn Error + 'static));

    impl<'v> fmt::Debug for SourceError<'v> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(&self.0, f)
        }
    }

    impl<'v> fmt::Display for SourceError<'v> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Display::fmt(&self.0, f)
        }
    }

    impl<'v> Source<'v> {
        pub fn new(err: &'v (dyn Error + 'static)) -> Self {
            Source::from(err)
        }

        pub fn get(&self) -> &'v (dyn Error + 'static) {
            self.inner.0
        }
    }

    impl<'v> AsRef<dyn Error + 'static> for Source<'v> {
        fn as_ref(&self) -> &(dyn Error + 'static) {
            self.inner.0
        }
    }

    impl<'v> From<&'v (dyn Error + 'static)> for Source<'v> {
        fn from(err: &'v (dyn Error + 'static)) -> Self {
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
