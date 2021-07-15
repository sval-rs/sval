use crate::std::fmt;

/**
An identifier is a fragment of text that may have a static value.
*/
#[derive(Clone, Copy)]
pub enum Ident<'a> {
    Borrowed(&'a str),
    Static(&'static str),
}

impl<'a> fmt::Debug for Ident<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<'a> fmt::Display for Ident<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
    }
}

impl<'a> Ident<'a> {
    pub fn as_str(&self) -> &'a str {
        match self {
            Ident::Borrowed(value) => value,
            Ident::Static(value) => value,
        }
    }

    pub fn to_static(&self) -> Option<Ident<'static>> {
        match self {
            Ident::Borrowed(_) => None,
            Ident::Static(value) => Some(Ident::Static(value)),
        }
    }
}
