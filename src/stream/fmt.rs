use crate::std::fmt;

/**
A formattable value.
*/
pub struct Arguments<'v>(ArgumentsInner<'v>);

enum ArgumentsInner<'v> {
    Debug(&'v dyn fmt::Debug),
    Display(&'v dyn fmt::Display),
    Args(fmt::Arguments<'v>),
}

impl<'v> Arguments<'v> {
    /**
    Capture standard format arguments.

    Prefer the [`debug`](#method.debug) and [`display`](#method.display) methods to create
    `Arguments` over passing them through `format_args`,
    because `format_args` will clobber any flags a stream
    might want to format these arguments with.
    */
    pub fn new(v: fmt::Arguments<'v>) -> Self {
        Arguments(ArgumentsInner::Args(v))
    }

    /**
    Capture arguments from a debuggable value.
    */
    pub fn debug(v: &'v impl fmt::Debug) -> Self {
        Arguments(ArgumentsInner::Debug(v))
    }

    /**
    Capture arguments from a displayable value.
    */
    pub fn display(v: &'v impl fmt::Display) -> Self {
        Arguments(ArgumentsInner::Display(v))
    }
}

impl<'v> From<fmt::Arguments<'v>> for Arguments<'v> {
    fn from(v: fmt::Arguments<'v>) -> Self {
        Arguments(ArgumentsInner::Args(v))
    }
}

impl<'v> From<&'v dyn fmt::Debug> for Arguments<'v> {
    fn from(v: &'v dyn fmt::Debug) -> Self {
        Arguments(ArgumentsInner::Debug(v))
    }
}

impl<'v> From<&'v dyn fmt::Display> for Arguments<'v> {
    fn from(v: &'v dyn fmt::Display) -> Self {
        Arguments(ArgumentsInner::Display(v))
    }
}

impl<'v> fmt::Debug for Arguments<'v> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ArgumentsInner::Debug(v) => v.fmt(f),
            ArgumentsInner::Display(v) => v.fmt(f),
            ArgumentsInner::Args(v) => v.fmt(f),
        }
    }
}

impl<'v> fmt::Display for Arguments<'v> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ArgumentsInner::Debug(v) => v.fmt(f),
            ArgumentsInner::Display(v) => v.fmt(f),
            ArgumentsInner::Args(v) => v.fmt(f),
        }
    }
}
