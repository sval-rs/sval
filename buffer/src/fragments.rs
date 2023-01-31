use crate::std::{fmt, ops::Deref};

#[cfg(feature = "alloc")]
use crate::std::borrow::{Cow, ToOwned};

#[derive(Debug, PartialEq, Eq)]
pub struct TextBuf<'sval>(FragmentBuf<'sval, str>);

impl<'sval> TextBuf<'sval> {
    pub fn new() -> Self {
        TextBuf(FragmentBuf::new(""))
    }

    pub fn push_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.0.push(fragment)
    }

    pub fn push_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.0.push_computed(fragment)
    }

    pub fn try_get(&self) -> Option<&'sval str> {
        self.0.try_get()
    }

    pub fn get(&self) -> &str {
        self.0.get()
    }
}

impl<'sval> Default for TextBuf<'sval> {
    fn default() -> Self {
        TextBuf::new()
    }
}

impl<'sval> From<&'sval str> for TextBuf<'sval> {
    fn from(fragment: &'sval str) -> Self {
        TextBuf(FragmentBuf::new(fragment))
    }
}

impl<'sval> AsRef<str> for TextBuf<'sval> {
    fn as_ref(&self) -> &str {
        self.get()
    }
}

impl<'sval> Deref for TextBuf<'sval> {
    type Target = str;

    fn deref(&self) -> &str {
        self.get()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinaryBuf<'sval>(FragmentBuf<'sval, [u8]>);

impl<'sval> BinaryBuf<'sval> {
    pub fn new() -> Self {
        BinaryBuf(FragmentBuf::new(&[]))
    }

    pub fn push_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.0.push(fragment)
    }

    pub fn push_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.0.push_computed(fragment)
    }

    pub fn try_get(&self) -> Option<&'sval [u8]> {
        self.0.try_get()
    }

    pub fn get(&self) -> &[u8] {
        self.0.get()
    }
}

impl<'sval> Default for BinaryBuf<'sval> {
    fn default() -> Self {
        BinaryBuf::new()
    }
}

impl<'sval> From<&'sval [u8]> for BinaryBuf<'sval> {
    fn from(fragment: &'sval [u8]) -> Self {
        BinaryBuf(FragmentBuf::new(fragment))
    }
}

impl<'sval> AsRef<[u8]> for BinaryBuf<'sval> {
    fn as_ref(&self) -> &[u8] {
        self.get()
    }
}

impl<'sval> Deref for BinaryBuf<'sval> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.get()
    }
}

#[cfg(not(feature = "alloc"))]
trait Fragment {
    fn to_fragment<'sval>(&'sval self) -> &'sval Self {
        self
    }

    fn can_replace(&self) -> bool;
}

#[cfg(feature = "alloc")]
trait Fragment: ToOwned {
    fn to_fragment<'sval>(&'sval self) -> Cow<'sval, Self> {
        Cow::Borrowed(self)
    }

    fn extend(buf: &mut Cow<Self>, fragment: &Self);

    fn can_replace(&self) -> bool;
}

impl Fragment for str {
    #[cfg(feature = "alloc")]
    fn extend(buf: &mut Cow<Self>, fragment: &Self) {
        buf.to_mut().push_str(fragment);
    }

    fn can_replace(&self) -> bool {
        self.len() == 0
    }
}

impl Fragment for [u8] {
    #[cfg(feature = "alloc")]
    fn extend(buf: &mut Cow<Self>, fragment: &Self) {
        buf.to_mut().extend(fragment);
    }

    fn can_replace(&self) -> bool {
        self.len() == 0
    }
}

struct FragmentBuf<'sval, T: ?Sized + Fragment> {
    #[cfg(not(feature = "alloc"))]
    value: &'sval T,
    #[cfg(feature = "alloc")]
    value: Cow<'sval, T>,
}

#[cfg(not(feature = "alloc"))]
impl<'sval, T: ?Sized + Fragment + fmt::Debug> fmt::Debug for FragmentBuf<'sval, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(feature = "alloc")]
impl<'sval, T: ?Sized + Fragment + fmt::Debug> fmt::Debug for FragmentBuf<'sval, T>
where
    T::Owned: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(not(feature = "alloc"))]
impl<'sval, T: ?Sized + Fragment + PartialEq> PartialEq for FragmentBuf<'sval, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(feature = "alloc")]
impl<'sval, T: ?Sized + Fragment + PartialEq> PartialEq for FragmentBuf<'sval, T>
where
    T::Owned: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(not(feature = "alloc"))]
impl<'sval, T: ?Sized + Fragment + PartialEq> Eq for FragmentBuf<'sval, T> {}

#[cfg(feature = "alloc")]
impl<'sval, T: ?Sized + Fragment + Eq> Eq for FragmentBuf<'sval, T> where T::Owned: Eq {}

impl<'sval, T: ?Sized + Fragment> FragmentBuf<'sval, T> {
    fn new(value: &'sval T) -> Self {
        FragmentBuf {
            value: value.to_fragment(),
        }
    }
}

impl<'sval, T: ?Sized + Fragment> FragmentBuf<'sval, T> {
    fn push(&mut self, fragment: &'sval T) -> sval::Result {
        if self.value.can_replace() {
            self.value = fragment.to_fragment();

            Ok(())
        } else {
            self.push_computed(fragment)
        }
    }

    fn push_computed(&mut self, fragment: &T) -> sval::Result {
        #[cfg(feature = "alloc")]
        {
            Fragment::extend(&mut self.value, fragment);

            Ok(())
        }

        #[cfg(not(feature = "alloc"))]
        {
            let _ = fragment;
            Err(sval::Error::new())
        }
    }

    fn try_get(&self) -> Option<&'sval T> {
        #[cfg(feature = "alloc")]
        {
            match self.value {
                Cow::Borrowed(value) => Some(value),
                Cow::Owned(_) => None,
            }
        }

        #[cfg(not(feature = "alloc"))]
        {
            Some(self.value)
        }
    }

    fn get(&self) -> &T {
        #[cfg(feature = "alloc")]
        {
            &*self.value
        }

        #[cfg(not(feature = "alloc"))]
        {
            self.value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_fragment_replace() {
        let mut buf = TextBuf::new();

        assert_eq!("", buf.get());
        assert_eq!(Some(""), buf.try_get());

        buf.push_fragment("abc").unwrap();

        assert_eq!("abc", buf.get());
        assert_eq!(Some("abc"), buf.try_get());
    }

    #[test]
    fn binary_fragment_replace() {
        let mut buf = BinaryBuf::new();

        assert_eq!(b"" as &[u8], buf.get());
        assert_eq!(Some(b"" as &[u8]), buf.try_get());

        buf.push_fragment(b"abc").unwrap();

        assert_eq!(b"abc", buf.get());
        assert_eq!(Some(b"abc" as &[u8]), buf.try_get());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn text_fragment_computed() {
        let mut buf = TextBuf::new();

        buf.push_fragment_computed("abc").unwrap();

        assert_eq!("abc", buf.get());
        assert_eq!(None, buf.try_get());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn binary_fragment_computed() {
        let mut buf = BinaryBuf::new();

        buf.push_fragment_computed(b"abc").unwrap();

        assert_eq!(b"abc" as &[u8], buf.get());
        assert_eq!(None, buf.try_get());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn text_fragment_extend() {
        let mut buf = TextBuf::new();

        buf.push_fragment("abc").unwrap();
        buf.push_fragment("def").unwrap();

        assert_eq!("abcdef", buf.get());
        assert_eq!(None, buf.try_get());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn binary_fragment_extend() {
        let mut buf = BinaryBuf::new();

        buf.push_fragment(b"abc").unwrap();
        buf.push_fragment(b"def").unwrap();

        assert_eq!(b"abcdef" as &[u8], buf.get());
        assert_eq!(None, buf.try_get());
    }
}
