use crate::std::fmt;

pub struct Arguments<'v>(ArgumentsInner<'v>);

enum ArgumentsInner<'v> {
    Debug(&'v dyn fmt::Debug),
    Display(&'v dyn fmt::Display),
    Args(fmt::Arguments<'v>),
}

impl<'v> Arguments<'v> {
    pub fn new(v: fmt::Arguments<'v>) -> Self {
        Arguments(ArgumentsInner::Args(v))
    }

    pub fn debug(v: &'v impl fmt::Debug) -> Self {
        Arguments(ArgumentsInner::Debug(v))
    }

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
