use crate::std::{
    fmt,
};


/**
A formattable value.
*/
pub struct Arguments<'a>(ArgumentsInner<'a>);

enum ArgumentsInner<'a> {
    Debug(&'a dyn fmt::Debug),
    Display(&'a dyn fmt::Display),
    Args(fmt::Arguments<'a>),
}

impl<'a> Arguments<'a> {
    /**
    Capture standard format arguments.

    Prefer the [`debug`](#method.debug) and [`display`](#method.display) methods to create 
    `Arguments` over passing them through `format_args`,
    because `format_args` will clobber any flags a stream
    might want to format these arguments with.
    */
    pub fn new(v: fmt::Arguments<'a>) -> Self {
        Arguments(ArgumentsInner::Args(v))
    }

    /**
    Capture arguments from a debuggable value.
    */
    pub fn debug(v: &'a impl fmt::Debug) -> Self {
        Arguments(ArgumentsInner::Debug(v))
    }

    /**
    Capture arguments from a displayable value.
    */
    pub fn display(v: &'a impl fmt::Display) -> Self {
        Arguments(ArgumentsInner::Display(v))
    }
}

impl<'a> From<fmt::Arguments<'a>> for Arguments<'a> {
    fn from(v: fmt::Arguments<'a>) -> Self {
        Arguments(ArgumentsInner::Args(v))
    }
}

impl<'a> From<&'a dyn fmt::Debug> for Arguments<'a> {
    fn from(v: &'a dyn fmt::Debug) -> Self {
        Arguments(ArgumentsInner::Debug(v))
    }
}

impl<'a> From<&'a dyn fmt::Display> for Arguments<'a> {
    fn from(v: &'a dyn fmt::Display) -> Self {
        Arguments(ArgumentsInner::Display(v))
    }
}

impl<'a> fmt::Debug for Arguments<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ArgumentsInner::Debug(v) => v.fmt(f),
            ArgumentsInner::Display(v) => v.fmt(f),
            ArgumentsInner::Args(v) => v.fmt(f),
        }
    }
}

impl<'a> fmt::Display for Arguments<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            ArgumentsInner::Debug(v) => v.fmt(f),
            ArgumentsInner::Display(v) => v.fmt(f),
            ArgumentsInner::Args(v) => v.fmt(f),
        }
    }
}
