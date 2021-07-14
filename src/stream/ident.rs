use crate::std::fmt;

#[derive(Clone, Copy)]
pub struct Ident<'a>(Inner<'a>);

#[derive(Clone, Copy)]
enum Inner<'a> {
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
    pub fn from_borrowed(value: &'a str) -> Self {
        Ident(Inner::Borrowed(value))
    }

    pub fn from_static(value: &'static str) -> Self {
        Ident(Inner::Static(value))
    }

    pub fn as_str(&self) -> &'a str {
        match self.0 {
            Inner::Borrowed(value) => value,
            Inner::Static(value) => value,
        }
    }

    pub fn to_static(&self) -> Option<Ident<'static>> {
        match self.0 {
            Inner::Borrowed(_) => None,
            Inner::Static(value) => Some(Ident::from_static(value)),
        }
    }
}
