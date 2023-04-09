use crate::{std::fmt, Error};

#[cfg(feature = "alloc")]
use crate::std::borrow::{Cow, ToOwned};

/**
Buffer text fragments into a single contiguous string.

In no-std environments, this buffer only supports a single
borrowed text fragment. Other methods will fail.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBuf<'sval>(FragmentBuf<'sval, str>);

impl<'sval> TextBuf<'sval> {
    /**
    Create a new empty text buffer.
    */
    pub fn new() -> Self {
        TextBuf(FragmentBuf::new(""))
    }

    /**
    Buffer a text value into a contiguous string.
    */
    pub fn collect(value: &'sval (impl sval::Value + ?Sized)) -> Result<Self, Error> {
        struct Collector<'a> {
            buf: TextBuf<'a>,
            err: Option<Error>,
        }

        impl<'a> Collector<'a> {
            fn try_catch(
                &mut self,
                f: impl FnOnce(&mut TextBuf<'a>) -> Result<(), Error>,
            ) -> sval::Result {
                match f(&mut self.buf) {
                    Ok(()) => Ok(()),
                    Err(e) => self.fail(e),
                }
            }

            fn fail(&mut self, err: Error) -> sval::Result {
                self.err = Some(err);
                sval::error()
            }
        }

        impl<'a> sval::Stream<'a> for Collector<'a> {
            fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
                Ok(())
            }

            fn text_fragment(&mut self, fragment: &'a str) -> sval::Result {
                self.try_catch(|buf| buf.push_fragment(fragment))
            }

            fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
                self.try_catch(|buf| buf.push_fragment_computed(fragment))
            }

            fn text_end(&mut self) -> sval::Result {
                Ok(())
            }

            fn null(&mut self) -> sval::Result {
                self.fail(Error::unsupported("text", "null"))
            }

            fn bool(&mut self, _: bool) -> sval::Result {
                self.fail(Error::unsupported("text", "boolean"))
            }

            fn i64(&mut self, _: i64) -> sval::Result {
                self.fail(Error::unsupported("text", "integer"))
            }

            fn f64(&mut self, _: f64) -> sval::Result {
                self.fail(Error::unsupported("text", "floating point"))
            }

            fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
                self.fail(Error::unsupported("text", "sequence"))
            }

            fn seq_value_begin(&mut self) -> sval::Result {
                self.fail(Error::unsupported("text", "sequence"))
            }

            fn seq_value_end(&mut self) -> sval::Result {
                self.fail(Error::unsupported("text", "sequence"))
            }

            fn seq_end(&mut self) -> sval::Result {
                self.fail(Error::unsupported("text", "sequence"))
            }
        }

        let mut collector = Collector {
            buf: TextBuf::new(),
            err: None,
        };

        value
            .stream(&mut collector)
            .map_err(|_| collector.err.unwrap())?;

        Ok(collector.buf)
    }

    /**
    Clear the text buffer so it can be re-used.
    */
    pub fn clear(&mut self) {
        *self = Default::default();
    }

    /**
    Push a borrowed text fragment onto the buffer.
    */
    pub fn push_fragment(&mut self, fragment: &'sval str) -> Result<(), Error> {
        self.0.push(fragment)
    }

    /**
    Push a computed text fragment onto the buffer.

    If the `std` feature of this library is enabled, this method will
    buffer the fragment. In no-std environments this method will fail.
    */
    pub fn push_fragment_computed(&mut self, fragment: &str) -> Result<(), Error> {
        self.0.push_computed(fragment)
    }

    /**
    Try get the contents of the buffer as a string borrowed for the `'sval` lifetime.
    */
    pub fn as_borrowed_str(&self) -> Option<&'sval str> {
        self.0.as_borrowed_inner()
    }

    /**
    Get the contents of the buffer as a string.
    */
    pub fn as_str(&self) -> &str {
        self.0.as_inner()
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
        self.as_str()
    }
}

impl<'a> sval::Value for TextBuf<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        self.as_str().stream(stream)
    }
}

impl<'sval> sval_ref::ValueRef<'sval> for TextBuf<'sval> {
    fn stream_ref<S: sval::Stream<'sval> + ?Sized>(&self, stream: &mut S) -> sval::Result {
        match self.as_borrowed_str() {
            Some(v) => stream.value(v),
            None => stream.value_computed(self.as_str()),
        }
    }
}

/**
Buffer binary fragments into a single contiguous slice.

In no-std environments, this buffer only supports a single
borrowed binary fragment. Other methods will fail.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryBuf<'sval>(FragmentBuf<'sval, [u8]>);

impl<'sval> BinaryBuf<'sval> {
    /**
    Create a new empty binary buffer.
    */
    pub fn new() -> Self {
        BinaryBuf(FragmentBuf::new(&[]))
    }

    /**
    Buffer a binary value into a contiguous slice.
    */
    pub fn collect(value: &'sval (impl sval::Value + ?Sized)) -> Result<Self, Error> {
        struct Collector<'a> {
            buf: BinaryBuf<'a>,
            err: Option<Error>,
        }

        impl<'a> Collector<'a> {
            fn try_catch(
                &mut self,
                f: impl FnOnce(&mut BinaryBuf<'a>) -> Result<(), Error>,
            ) -> sval::Result {
                match f(&mut self.buf) {
                    Ok(()) => Ok(()),
                    Err(e) => self.fail(e),
                }
            }

            fn fail(&mut self, err: Error) -> sval::Result {
                self.err = Some(err);
                sval::error()
            }
        }

        impl<'a> sval::Stream<'a> for Collector<'a> {
            fn binary_begin(&mut self, _: Option<usize>) -> sval::Result {
                Ok(())
            }

            fn binary_fragment(&mut self, fragment: &'a [u8]) -> sval::Result {
                self.try_catch(|buf| buf.push_fragment(fragment))
            }

            fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
                self.try_catch(|buf| buf.push_fragment_computed(fragment))
            }

            fn binary_end(&mut self) -> sval::Result {
                Ok(())
            }

            fn text_begin(&mut self, _: Option<usize>) -> sval::Result {
                self.fail(Error::unsupported("binary", "text"))
            }

            fn text_fragment_computed(&mut self, _: &str) -> sval::Result {
                self.fail(Error::unsupported("binary", "text"))
            }

            fn text_end(&mut self) -> sval::Result {
                self.fail(Error::unsupported("binary", "text"))
            }

            fn null(&mut self) -> sval::Result {
                self.fail(Error::unsupported("binary", "null"))
            }

            fn bool(&mut self, _: bool) -> sval::Result {
                self.fail(Error::unsupported("binary", "boolean"))
            }

            fn i64(&mut self, _: i64) -> sval::Result {
                self.fail(Error::unsupported("binary", "integer"))
            }

            fn f64(&mut self, _: f64) -> sval::Result {
                self.fail(Error::unsupported("binary", "floating point"))
            }

            fn seq_begin(&mut self, _: Option<usize>) -> sval::Result {
                self.fail(Error::unsupported("binary", "sequence"))
            }

            fn seq_value_begin(&mut self) -> sval::Result {
                self.fail(Error::unsupported("binary", "sequence"))
            }

            fn seq_value_end(&mut self) -> sval::Result {
                self.fail(Error::unsupported("binary", "sequence"))
            }

            fn seq_end(&mut self) -> sval::Result {
                self.fail(Error::unsupported("binary", "sequence"))
            }
        }

        let mut collector = Collector {
            buf: BinaryBuf::new(),
            err: None,
        };

        value
            .stream(&mut collector)
            .map_err(|_| collector.err.unwrap())?;

        Ok(collector.buf)
    }

    /**
    Clear the binary buffer so it can be re-used.
    */
    pub fn clear(&mut self) {
        *self = Default::default();
    }

    /**
    Push a borrowed binary fragment onto the buffer.
    */
    pub fn push_fragment(&mut self, fragment: &'sval [u8]) -> Result<(), Error> {
        self.0.push(fragment)
    }

    /**
    Push a computed binary fragment onto the buffer.

    If the `std` feature of this library is enabled, this method will
    buffer the fragment. In no-std environments this method will fail.
    */
    pub fn push_fragment_computed(&mut self, fragment: &[u8]) -> Result<(), Error> {
        self.0.push_computed(fragment)
    }

    /**
    Try get the contents of the buffer as a slice borrowed for the `'sval` lifetime.
    */
    pub fn as_borrowed_slice(&self) -> Option<&'sval [u8]> {
        self.0.as_borrowed_inner()
    }

    /**
    Get the contents of the buffer as a slice.
    */
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_inner()
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

impl<'sval, const N: usize> From<&'sval [u8; N]> for BinaryBuf<'sval> {
    fn from(fragment: &'sval [u8; N]) -> Self {
        BinaryBuf(FragmentBuf::new(fragment))
    }
}

impl<'sval> AsRef<[u8]> for BinaryBuf<'sval> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<'a> sval::Value for BinaryBuf<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        sval::BinarySlice::new(self.as_slice()).stream(stream)
    }
}

impl<'sval> sval_ref::ValueRef<'sval> for BinaryBuf<'sval> {
    fn stream_ref<S: sval::Stream<'sval> + ?Sized>(&self, stream: &mut S) -> sval::Result {
        match self.as_borrowed_slice() {
            Some(v) => stream.value(sval::BinarySlice::new(v)),
            None => stream.value_computed(sval::BinarySlice::new(self.as_slice())),
        }
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
impl<'sval, T: ?Sized + Fragment> Clone for FragmentBuf<'sval, T> {
    fn clone(&self) -> Self {
        FragmentBuf { value: self.value }
    }
}

#[cfg(feature = "alloc")]
impl<'sval, T: ?Sized + Fragment> Clone for FragmentBuf<'sval, T>
where
    T::Owned: Clone,
{
    fn clone(&self) -> Self {
        FragmentBuf {
            value: self.value.clone(),
        }
    }
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

    fn push(&mut self, fragment: &'sval T) -> Result<(), Error> {
        if self.value.can_replace() {
            self.value = fragment.to_fragment();

            Ok(())
        } else {
            self.push_computed(fragment)
        }
    }

    fn push_computed(&mut self, fragment: &T) -> Result<(), Error> {
        #[cfg(feature = "alloc")]
        {
            Fragment::extend(&mut self.value, fragment);

            Ok(())
        }

        #[cfg(not(feature = "alloc"))]
        {
            let _ = fragment;
            Err(Error::no_alloc("computed fragment"))
        }
    }

    fn as_borrowed_inner(&self) -> Option<&'sval T> {
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

    fn as_inner(&self) -> &T {
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
    fn text_buf_empty() {
        assert_eq!("", TextBuf::new().as_borrowed_str().unwrap());
    }

    #[test]
    fn binary_buf_empty() {
        assert_eq!(&[] as &[u8], BinaryBuf::new().as_borrowed_slice().unwrap());
    }

    #[test]
    fn text_fragment_replace() {
        let mut buf = TextBuf::new();

        assert_eq!("", buf.as_str());
        assert_eq!(Some(""), buf.as_borrowed_str());

        buf.push_fragment("abc").unwrap();

        assert_eq!("abc", buf.as_str());
        assert_eq!(Some("abc"), buf.as_borrowed_str());
    }

    #[test]
    fn text_fragment_clear() {
        let mut buf = TextBuf::new();

        buf.push_fragment("abc").unwrap();
        buf.clear();

        assert_eq!("", buf.as_str());
    }

    #[test]
    fn binary_fragment_replace() {
        let mut buf = BinaryBuf::new();

        assert_eq!(b"" as &[u8], buf.as_slice());
        assert_eq!(Some(b"" as &[u8]), buf.as_borrowed_slice());

        buf.push_fragment(b"abc").unwrap();

        assert_eq!(b"abc", buf.as_slice());
        assert_eq!(Some(b"abc" as &[u8]), buf.as_borrowed_slice());
    }

    #[test]
    fn binary_fragment_clear() {
        let mut buf = BinaryBuf::new();

        buf.push_fragment(b"abc").unwrap();
        buf.clear();

        assert_eq!(b"", buf.as_slice());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn text_fragment_computed() {
        let mut buf = TextBuf::new();

        buf.push_fragment_computed("abc").unwrap();

        assert_eq!("abc", buf.as_str());
        assert_eq!(None, buf.as_borrowed_str());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn binary_fragment_computed() {
        let mut buf = BinaryBuf::new();

        buf.push_fragment_computed(b"abc").unwrap();

        assert_eq!(b"abc" as &[u8], buf.as_slice());
        assert_eq!(None, buf.as_borrowed_slice());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn text_fragment_extend() {
        let mut buf = TextBuf::new();

        buf.push_fragment("abc").unwrap();
        buf.push_fragment("def").unwrap();

        assert_eq!("abcdef", buf.as_str());
        assert_eq!(None, buf.as_borrowed_str());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn binary_fragment_extend() {
        let mut buf = BinaryBuf::new();

        buf.push_fragment(b"abc").unwrap();
        buf.push_fragment(b"def").unwrap();

        assert_eq!(b"abcdef" as &[u8], buf.as_slice());
        assert_eq!(None, buf.as_borrowed_slice());
    }

    #[test]
    fn stream_text_buf() {
        let mut buf = TextBuf::new();
        buf.push_fragment("abc").unwrap();

        sval_test::assert_tokens(&buf, {
            use sval_test::Token::*;

            &[TextBegin(Some(3)), TextFragment("abc"), TextEnd]
        });
    }

    #[test]
    fn stream_binary_buf() {
        let mut buf = BinaryBuf::new();
        buf.push_fragment(b"abc").unwrap();

        sval_test::assert_tokens(&buf, {
            use sval_test::Token::*;

            &[BinaryBegin(Some(3)), BinaryFragment(b"abc"), BinaryEnd]
        });
    }
}
