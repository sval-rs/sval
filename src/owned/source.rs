#[derive(Clone, PartialEq)]
pub struct OwnedSource {
    // NOTE: We'll want to capture these as better types when backtraces are stable
    debug: String,
    display: String,
    #[cfg(feature = "std")]
    source: Option<SharedContainer<OwnedSource>>,
}

#[cfg(not(feature = "std"))]
impl OwnedSource {
    pub(crate) fn empty() -> Self {
        OwnedSource {
            debug: String::new(),
            display: String::new(),
        }
    }
}

impl<'a> From<stream::Source<'a>> for OwnedSource {
    fn from(err: stream::Source<'a>) -> OwnedSource {
        #[cfg(feature = "std")]
        {
            OwnedSource::collect(err.as_ref())
        }

        #[cfg(not(feature = "std"))]
        {
            let _ = err;
            OwnedSource::empty()
        }
    }
}

impl<'a> From<&'a OwnedSource> for stream::Source<'a> {
    fn from(err: &'a OwnedSource) -> stream::Source<'a> {
        #[cfg(feature = "std")]
        {
            stream::Source::new(err)
        }

        #[cfg(not(feature = "std"))]
        {
            let _ = err;
            stream::Source::empty()
        }
    }
}

impl fmt::Debug for OwnedSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.debug, f)
    }
}

impl fmt::Display for OwnedSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.display, f)
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::*;

    use crate::std::error::Error;

    impl OwnedSource {
        pub(crate) fn collect(err: &dyn Error) -> Self {
            OwnedSource {
                debug: format!("{:?}", err),
                display: format!("{}", err),
                source: err
                    .source()
                    .map(|source| SharedContainer::from(OwnedSource::collect(source))),
            }
        }
    }

    impl Error for OwnedSource {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source
                .as_ref()
                .map(|source| &**source as &(dyn Error + 'static))
        }
    }
}
