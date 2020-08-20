/*!
Owned values.
*/

use crate::{
    std::{
        fmt,
        string::{
            String,
            ToString,
        },
        vec::Vec,
    },
    stream::{
        self,
        stack,
        Stack,
        Stream,
    },
    value::{
        self,
        Value,
    },
};

#[cfg(not(feature = "std"))]
type Container<T> = crate::std::boxed::Box<T>;

#[cfg(feature = "std")]
type Container<T> = crate::std::sync::Arc<T>;

/**
An owned, immutable value.

Owned values are safe to share and are cheap to clone.

Add the `std` feature to your `Cargo.toml` to enable this type:

```toml,no_run
[dependencies.sval]
features = ["std"]
```
*/
#[derive(Clone)]
pub struct OwnedValue(ValueInner);

impl OwnedValue {
    /**
    Get an owned value from an arbitrary [`Value`].

    The given value doesn't need to be `Send + Sync + 'static`.

    Some primitive types can be converted into an `OwnedValue`
    for free. These types have a corresponding `From` implementation.

    The structure of the given value will be streamed into
    a sharable representation. That means this method is more
    expensive for more complex values.

    Prefer the `From` impls and `from_shared` method where possible.

    [`Value`]: struct.Value.html
    */
    pub fn collect(v: impl Value) -> Self {
        // Try get a primitive first
        // If the value is a simple primitive that can
        // be represented in a single token then we can avoid
        // allocating for it.
        if let Some(primitive) = Primitive::collect(&v) {
            return OwnedValue(ValueInner::Primitive(primitive));
        }

        Buf::collect(v)
            .map(|tokens| OwnedValue(ValueInner::Stream(tokens.into())))
            .unwrap_or_else(|err| OwnedValue(ValueInner::Error(err.to_string())))
    }

    /**
    Get an owned value from an already shared [`Value`].

    The given value must be `Send + Sync + 'static`.

    [`Value`]: struct.Value.html
    */
    #[cfg(feature = "std")]
    pub fn from_shared(v: impl Into<Container<dyn Value + Send + Sync>>) -> Self {
        OwnedValue(ValueInner::Shared(v.into()))
    }
}

#[derive(Clone)]
enum ValueInner {
    Error(String),
    #[cfg(feature = "std")]
    Shared(Container<dyn Value + Send + Sync>),
    Primitive(Token),
    Stream(Container<[Token]>),
}

impl Value for OwnedValue {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        match self.0 {
            ValueInner::Error(ref e) => Err(crate::Error::custom(e)),
            #[cfg(feature = "std")]
            ValueInner::Shared(ref v) => v.stream(stream),
            ValueInner::Primitive(ref v) => v.stream(stream),
            ValueInner::Stream(ref v) => {
                for token in v.iter() {
                    token.stream(stream)?;
                }

                Ok(())
            }
        }
    }
}

impl fmt::Debug for OwnedValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "fmt")]
        {
            crate::fmt::debug(f, self)
        }

        #[cfg(not(feature = "fmt"))]
        {
            f.debug_struct("OwnedValue").finish()
        }
    }
}

impl From<usize> for OwnedValue {
    fn from(v: usize) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Unsigned(v as u64),
        }))
    }
}

impl From<u8> for OwnedValue {
    fn from(v: u8) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Unsigned(v as u64),
        }))
    }
}

impl From<u16> for OwnedValue {
    fn from(v: u16) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Unsigned(v as u64),
        }))
    }
}

impl From<u32> for OwnedValue {
    fn from(v: u32) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Unsigned(v as u64),
        }))
    }
}

impl From<u64> for OwnedValue {
    fn from(v: u64) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Unsigned(v),
        }))
    }
}

impl From<u128> for OwnedValue {
    fn from(v: u128) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::BigUnsigned(v),
        }))
    }
}

impl From<isize> for OwnedValue {
    fn from(v: isize) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Signed(v as i64),
        }))
    }
}

impl From<i8> for OwnedValue {
    fn from(v: i8) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Signed(v as i64),
        }))
    }
}

impl From<i16> for OwnedValue {
    fn from(v: i16) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Signed(v as i64),
        }))
    }
}

impl From<i32> for OwnedValue {
    fn from(v: i32) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Signed(v as i64),
        }))
    }
}

impl From<i64> for OwnedValue {
    fn from(v: i64) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Signed(v),
        }))
    }
}

impl From<i128> for OwnedValue {
    fn from(v: i128) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::BigSigned(v),
        }))
    }
}

impl From<f32> for OwnedValue {
    fn from(v: f32) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Float(v as f64),
        }))
    }
}

impl From<f64> for OwnedValue {
    fn from(v: f64) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Float(v),
        }))
    }
}

impl From<bool> for OwnedValue {
    fn from(v: bool) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Bool(v),
        }))
    }
}

impl From<char> for OwnedValue {
    fn from(v: char) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Char(v),
        }))
    }
}

impl<'a> From<&'a str> for OwnedValue {
    fn from(v: &'a str) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Str(v.into()),
        }))
    }
}

impl From<String> for OwnedValue {
    fn from(v: String) -> Self {
        OwnedValue(ValueInner::Primitive(Token {
            depth: stack::Depth::root(),
            kind: Kind::Str(v.into()),
        }))
    }
}

#[derive(Clone)]
pub(crate) enum Kind {
    MapBegin(Option<usize>),
    MapKey,
    MapValue,
    MapEnd,
    SeqBegin(Option<usize>),
    SeqElem,
    SeqEnd,
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    BigSigned(i128),
    BigUnsigned(u128),
    Bool(bool),
    Str(Container<str>),
    Char(char),
    Error(Container<OwnedSource>),
    None,
}

#[derive(Clone, PartialEq)]
pub(crate) struct OwnedSource {
    // NOTE: We'll want to capture these as better types when backtraces are stable
    debug: String,
    display: String,
    #[cfg(feature = "std")]
    source: Option<Container<OwnedSource>>,
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

pub(crate) struct Buf {
    stack: Stack,
    tokens: Vec<Token>,
}

#[derive(Clone)]
pub(crate) struct Token {
    #[allow(dead_code)]
    pub(crate) depth: stack::Depth,
    pub(crate) kind: Kind,
}

impl Token {
    fn stream(&self, stream: &mut value::Stream) -> value::Result {
        use self::Kind::*;

        match self.kind {
            Signed(v) => stream.i64(v)?,
            Unsigned(v) => stream.u64(v)?,
            Float(v) => stream.f64(v)?,
            BigSigned(v) => stream.i128(v)?,
            BigUnsigned(v) => stream.u128(v)?,
            Bool(v) => stream.bool(v)?,
            Str(ref v) => stream.str(&*v)?,
            Char(v) => stream.char(v)?,
            Error(ref v) => stream.any(stream::Source::from(&**v))?,
            None => stream.none()?,
            MapBegin(len) => stream.map_begin(len)?,
            MapKey => {
                stream.map_key_begin()?;
            }
            MapValue => {
                stream.map_value_begin()?;
            }
            MapEnd => stream.map_end()?,
            SeqBegin(len) => stream.seq_begin(len)?,
            SeqElem => {
                stream.seq_elem_begin()?;
            }
            SeqEnd => stream.seq_end()?,
        }

        Ok(())
    }
}

impl Buf {
    pub(crate) fn new() -> Buf {
        Buf {
            stack: Stack::new(),
            tokens: Vec::new(),
        }
    }

    fn collect(v: impl Value) -> Result<Vec<Token>, crate::Error> {
        crate::stream(Buf::new(), v).map(|buf| buf.tokens)
    }

    fn push(&mut self, kind: Kind, depth: stack::Depth) {
        self.tokens.push(Token { depth, kind });
    }
}

impl Stream for Buf {
    fn fmt(&mut self, f: stream::Arguments) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::Str(Container::from(f.to_string())), depth);

        Ok(())
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::Error(Container::from(OwnedSource::from(v))), depth);

        Ok(())
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::Signed(v), depth);

        Ok(())
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::Unsigned(v), depth);

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::BigSigned(v), depth);

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::BigUnsigned(v), depth);

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::Float(v), depth);

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::Bool(v), depth);

        Ok(())
    }

    fn char(&mut self, v: char) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::Char(v), depth);

        Ok(())
    }

    fn str(&mut self, v: &str) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::Str(Container::from(v)), depth);

        Ok(())
    }

    fn none(&mut self) -> stream::Result {
        let depth = self.stack.primitive()?.depth();

        self.push(Kind::None, depth);

        Ok(())
    }

    fn map_begin(&mut self, len: Option<usize>) -> stream::Result {
        let depth = self.stack.map_begin()?.depth();

        self.push(Kind::MapBegin(len), depth);

        Ok(())
    }

    fn map_key(&mut self) -> stream::Result {
        let depth = self.stack.map_key()?.depth();

        self.push(Kind::MapKey, depth);

        Ok(())
    }

    fn map_value(&mut self) -> stream::Result {
        let depth = self.stack.map_value()?.depth();

        self.push(Kind::MapValue, depth);

        Ok(())
    }

    fn map_end(&mut self) -> stream::Result {
        let depth = self.stack.map_end()?.depth();

        self.push(Kind::MapEnd, depth);

        Ok(())
    }

    fn seq_begin(&mut self, len: Option<usize>) -> stream::Result {
        let depth = self.stack.seq_begin()?.depth();

        self.push(Kind::SeqBegin(len), depth);

        Ok(())
    }

    fn seq_elem(&mut self) -> stream::Result {
        let depth = self.stack.seq_elem()?.depth();

        self.push(Kind::SeqElem, depth);

        Ok(())
    }

    fn seq_end(&mut self) -> stream::Result {
        let depth = self.stack.seq_end()?.depth();

        self.push(Kind::SeqEnd, depth);

        Ok(())
    }
}

struct Primitive {
    token: Option<Token>,
}

impl Primitive {
    fn new() -> Primitive {
        Primitive { token: None }
    }

    fn collect(v: impl Value) -> Option<Token> {
        crate::stream(Primitive::new(), v)
            .ok()
            .and_then(|buf| buf.token)
    }

    fn set(&mut self, kind: Kind) {
        self.token = Some(Token {
            depth: stack::Depth::root(),
            kind,
        });
    }
}

impl Stream for Primitive {
    fn fmt(&mut self, f: stream::Arguments) -> stream::Result {
        self.set(Kind::Str(Container::from(f.to_string())));

        Ok(())
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.set(Kind::Error(Container::from(OwnedSource::from(v))));

        Ok(())
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.set(Kind::Signed(v));

        Ok(())
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.set(Kind::Unsigned(v));

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.set(Kind::BigSigned(v));

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.set(Kind::BigUnsigned(v));

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.set(Kind::Float(v));

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.set(Kind::Bool(v));

        Ok(())
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.set(Kind::Char(v));

        Ok(())
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.set(Kind::Str(Container::from(v)));

        Ok(())
    }

    fn none(&mut self) -> stream::Result {
        self.set(Kind::None);

        Ok(())
    }

    fn map_begin(&mut self, _: Option<usize>) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_key(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_value(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_end(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_begin(&mut self, _: Option<usize>) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_elem(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_end(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }
}

#[cfg(feature = "serde1")]
impl Buf {
    pub(crate) fn clear(&mut self) {
        self.tokens.clear();
        self.stack.clear();
    }

    pub(crate) fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub(crate) fn is_streamable(&self) -> bool {
        self.stack.can_end()
    }
}

#[cfg(any(test, feature = "test"))]
impl OwnedValue {
    pub(crate) fn tokens(
        &self,
    ) -> Result<impl crate::std::ops::Deref<Target = [Token]>, crate::Error> {
        match &self.0 {
            ValueInner::Primitive(token) => Ok(vec![(*token).clone()].into()),
            ValueInner::Stream(tokens) => Ok(tokens.clone()),
            ValueInner::Error(err) => Err(crate::Error::custom(err)),
            #[cfg(feature = "std")]
            ValueInner::Shared(_) => Err(crate::Error::msg("expected a set of tokens")),
        }
    }
}

#[cfg(feature = "std")]
mod std_support {
    use super::{
        Container,
        OwnedSource,
    };

    use crate::std::error::Error;

    impl OwnedSource {
        pub(crate) fn collect(err: &dyn Error) -> Self {
            OwnedSource {
                debug: format!("{:?}", err),
                display: format!("{}", err),
                source: err
                    .source()
                    .map(|source| Container::from(OwnedSource::collect(source))),
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::{
        self,
        Token,
    };

    struct Map;

    impl Value for Map {
        fn stream(&self, stream: &mut value::Stream) -> value::Result {
            stream.map_begin(Some(2))?;

            stream.map_key(1)?;
            stream.map_value(11)?;

            stream.map_key(2)?;
            stream.map_value(22)?;

            stream.map_end()
        }
    }

    struct Seq;

    impl Value for Seq {
        fn stream(&self, stream: &mut value::Stream) -> value::Result {
            stream.seq_begin(Some(2))?;

            stream.seq_elem(1)?;
            stream.seq_elem(2)?;

            stream.seq_end()
        }
    }

    #[test]
    fn owned_value_is_send_sync() {
        fn is_send_sync<T: Send + Sync>() {}

        is_send_sync::<OwnedValue>();
    }

    #[test]
    fn owned_primitive() {
        assert_eq!(
            vec![Token::Str("a format 1".into())],
            test::tokens(format_args!("a format {}", 1))
        );

        assert_eq!(
            vec![Token::Str("a string".into())],
            test::tokens("a string")
        );

        assert_eq!(vec![Token::Unsigned(42u64)], test::tokens(42u64));

        assert_eq!(vec![Token::Signed(42i64)], test::tokens(42i64));

        assert_eq!(vec![Token::BigUnsigned(42u128)], test::tokens(42u128));

        assert_eq!(vec![Token::BigSigned(42i128)], test::tokens(42i128));

        assert_eq!(vec![Token::Float(42f64)], test::tokens(42f64));

        assert_eq!(vec![Token::Bool(true)], test::tokens(true));

        assert_eq!(vec![Token::Char('a')], test::tokens('a'));

        assert_eq!(vec![Token::None], test::tokens(Option::None::<()>));
    }

    #[test]
    fn owned_map() {
        let v = test::tokens(Map);

        assert_eq!(
            vec![
                Token::MapBegin(Some(2)),
                Token::Signed(1),
                Token::Signed(11),
                Token::Signed(2),
                Token::Signed(22),
                Token::MapEnd,
            ],
            v
        );
    }

    #[test]
    fn owned_seq() {
        let v = test::tokens(Seq);

        assert_eq!(
            vec![
                Token::SeqBegin(Some(2)),
                Token::Signed(1),
                Token::Signed(2),
                Token::SeqEnd,
            ],
            v
        );
    }

    #[cfg(not(feature = "std"))]
    mod alloc_support {
        use super::*;

        #[test]
        fn owned_error() {
            let v = test::tokens(stream::Source::empty());

            assert_eq!(vec![Token::None,], v);
        }
    }

    #[cfg(feature = "std")]
    mod std_support {
        use super::*;

        use crate::std::{
            error,
            fmt,
            io,
        };

        #[test]
        fn owned_error() {
            #[derive(Debug)]
            struct TestError {
                id: usize,
                source: io::Error,
            }

            impl error::Error for TestError {
                fn source(&self) -> Option<&(dyn error::Error + 'static)> {
                    Some(&self.source)
                }
            }

            impl fmt::Display for TestError {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "it broke!")
                }
            }

            let err = TestError {
                id: 42,
                source: io::Error::from(io::ErrorKind::Other),
            };

            let v = test::tokens(stream::Source::new(&err));

            assert_eq!(vec![Token::Error(test::Source::new(&err)),], v);
        }
    }
}
