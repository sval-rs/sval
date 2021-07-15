use crate::{
    std::{
        boxed::Box,
        fmt,
        ops::Deref,
        ptr,
        str,
        string::{
            String,
            ToString,
        },
        vec::Vec,
    },
    stream::{
        self,
        Stream,
    },
    value::{
        self,
        Value,
    },
};

#[derive(Clone)]
pub struct OwnedValue(ValueInner);

impl OwnedValue {
    pub fn collect(v: impl Value) -> Self {
        // Try get a primitive first
        // If the value is a simple primitive that can
        // be represented in a single token then we can avoid
        // allocating for it.
        if let Some(primitive) = PrimitiveBuf::collect(&v) {
            return OwnedValue(ValueInner::Primitive(primitive));
        }

        TokenBuf::collect(v)
            .map(|tokens| OwnedValue(ValueInner::Stream(tokens.into())))
            .unwrap_or_else(|err| OwnedValue(ValueInner::Error(err.to_string().into())))
    }

    #[cfg(feature = "std")]
    pub fn from_shared(v: impl Into<Arc<dyn Value + Send + Sync>>) -> Self {
        OwnedValue(ValueInner::Shared(v.into()))
    }
}

#[derive(Clone)]
enum ValueInner {
    Error(Box<str>),
    #[cfg(feature = "std")]
    Shared(SharedContainer<dyn Value + Send + Sync>),
    Primitive(Primitive),
    Stream(SharedContainer<[Token]>),
}

impl Value for OwnedValue {
    fn stream<'s, 'v>(&'v self, mut stream: value::Stream<'s, 'v>) -> value::Result {
        match self.0 {
            ValueInner::Error(ref e) => Err(crate::Error::custom(e)),
            #[cfg(feature = "std")]
            ValueInner::Shared(ref v) => v.stream_owned(stream.borrowed()),
            ValueInner::Primitive(ref v) => v.stream_owned(stream.borrowed()),
            ValueInner::Stream(ref v) => {
                for token in v.iter() {
                    token.stream_owned(stream.borrowed())?;
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
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v as u64)))
    }
}

impl From<u8> for OwnedValue {
    fn from(v: u8) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v as u64)))
    }
}

impl From<u16> for OwnedValue {
    fn from(v: u16) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v as u64)))
    }
}

impl From<u32> for OwnedValue {
    fn from(v: u32) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v as u64)))
    }
}

impl From<u64> for OwnedValue {
    fn from(v: u64) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Unsigned(v)))
    }
}

impl From<u128> for OwnedValue {
    fn from(v: u128) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::BigUnsigned(v)))
    }
}

impl From<isize> for OwnedValue {
    fn from(v: isize) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v as i64)))
    }
}

impl From<i8> for OwnedValue {
    fn from(v: i8) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v as i64)))
    }
}

impl From<i16> for OwnedValue {
    fn from(v: i16) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v as i64)))
    }
}

impl From<i32> for OwnedValue {
    fn from(v: i32) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v as i64)))
    }
}

impl From<i64> for OwnedValue {
    fn from(v: i64) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Signed(v)))
    }
}

impl From<i128> for OwnedValue {
    fn from(v: i128) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::BigSigned(v)))
    }
}

impl From<f32> for OwnedValue {
    fn from(v: f32) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Float(v as f64)))
    }
}

impl From<f64> for OwnedValue {
    fn from(v: f64) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Float(v)))
    }
}

impl From<bool> for OwnedValue {
    fn from(v: bool) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Bool(v)))
    }
}

impl From<char> for OwnedValue {
    fn from(v: char) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Char(v)))
    }
}

impl<'a> From<&'a str> for OwnedValue {
    fn from(v: &'a str) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Str(v.into())))
    }
}

impl From<String> for OwnedValue {
    fn from(v: String) -> Self {
        OwnedValue(ValueInner::Primitive(Primitive::Str(v.into())))
    }
}

// Not embedded within a `Token`
#[derive(Clone)]
pub(crate) enum Primitive {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
    BigSigned(i128),
    BigUnsigned(u128),
    Bool(bool),
    Str(StringContainer<SharedContainer<str>>),
    Char(char),
    Error(SharedContainer<OwnedSource>),
    None,
}

impl Primitive {
    #[cfg(any(test, feature = "test"))]
    fn to_token(&self) -> Token {
        Token {
            depth: 0,
            kind: match *self {
                Primitive::Signed(v) => TokenKind::Signed(v),
                Primitive::Unsigned(v) => TokenKind::Unsigned(v),
                Primitive::Float(v) => TokenKind::Float(v),
                Primitive::BigSigned(v) => TokenKind::BigSigned(v),
                Primitive::BigUnsigned(v) => TokenKind::BigUnsigned(v),
                Primitive::Bool(v) => TokenKind::Bool(v),
                Primitive::Str(ref v) => TokenKind::Str((&**v).into()),
                Primitive::Char(v) => TokenKind::Char(v),
                Primitive::Error(ref v) => TokenKind::Error((&**v).clone().into()),
                Primitive::None => TokenKind::None,
            },
        }
    }
}

impl Value for Primitive {
    fn stream<'s, 'v>(&'v self, mut stream: value::Stream<'s, 'v>) -> value::Result {
        use self::Primitive::*;

        match *self {
            Signed(v) => stream.i64(v)?,
            Unsigned(v) => stream.u64(v)?,
            Float(v) => stream.f64(v)?,
            BigSigned(v) => stream.i128(v)?,
            BigUnsigned(v) => stream.u128(v)?,
            Bool(v) => stream.bool(v)?,
            Str(ref v) => stream.owned().str(&*v)?,
            Char(v) => stream.char(v)?,
            Error(ref v) => stream::Source::from(&**v).stream(stream.owned())?,
            None => stream.none()?,
        }

        Ok(())
    }
}

struct PrimitiveBuf {
    primitive: Option<Primitive>,
}

impl PrimitiveBuf {
    fn new() -> PrimitiveBuf {
        PrimitiveBuf { primitive: None }
    }

    fn collect(v: impl Value) -> Option<Primitive> {
        let mut buf = PrimitiveBuf::new();
        crate::stream_owned(&mut buf, &v).ok()?;

        buf.primitive
    }

    fn set(&mut self, primitive: Primitive) {
        self.primitive = Some(primitive);
    }
}

impl<'v> Stream<'v> for PrimitiveBuf {
    fn fmt(&mut self, f: stream::Arguments) -> stream::Result {
        self.set(Primitive::Str(StringContainer::from(f.to_string())));

        Ok(())
    }

    fn fmt_borrowed(&mut self, f: stream::Arguments<'v>) -> stream::Result {
        self.fmt(f)
    }

    fn error(&mut self, v: stream::Source) -> stream::Result {
        self.set(Primitive::Error(SharedContainer::from(OwnedSource::from(
            v,
        ))));

        Ok(())
    }

    fn error_borrowed(&mut self, v: stream::Source<'v>) -> stream::Result {
        self.error(v)
    }

    fn i64(&mut self, v: i64) -> stream::Result {
        self.set(Primitive::Signed(v));

        Ok(())
    }

    fn u64(&mut self, v: u64) -> stream::Result {
        self.set(Primitive::Unsigned(v));

        Ok(())
    }

    fn i128(&mut self, v: i128) -> stream::Result {
        self.set(Primitive::BigSigned(v));

        Ok(())
    }

    fn u128(&mut self, v: u128) -> stream::Result {
        self.set(Primitive::BigUnsigned(v));

        Ok(())
    }

    fn f64(&mut self, v: f64) -> stream::Result {
        self.set(Primitive::Float(v));

        Ok(())
    }

    fn bool(&mut self, v: bool) -> stream::Result {
        self.set(Primitive::Bool(v));

        Ok(())
    }

    fn char(&mut self, v: char) -> stream::Result {
        self.set(Primitive::Char(v));

        Ok(())
    }

    fn str(&mut self, v: &str) -> stream::Result {
        self.set(Primitive::Str(StringContainer::from(v)));

        Ok(())
    }

    fn str_borrowed(&mut self, v: &'v str) -> stream::Result {
        self.str(v)
    }

    fn none(&mut self) -> stream::Result {
        self.set(Primitive::None);

        Ok(())
    }

    fn map_begin(&mut self, _: stream::MapMeta) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_key(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_key_collect(&mut self, _: stream::Value) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_key_collect_borrowed(&mut self, k: stream::Value<'v>) -> stream::Result {
        self.map_key_collect(k)
    }

    fn map_value(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_value_collect(&mut self, _: stream::Value) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn map_value_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.map_value_collect(v)
    }

    fn map_end(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_begin(&mut self, _: stream::SeqMeta) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_elem(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_elem_collect(&mut self, _: stream::Value) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }

    fn seq_elem_collect_borrowed(&mut self, v: stream::Value<'v>) -> stream::Result {
        self.seq_elem_collect(v)
    }

    fn seq_end(&mut self) -> stream::Result {
        Err(crate::Error::unsupported("unsupported primitive"))
    }
}

#[cfg(any(test, feature = "test"))]
impl OwnedValue {
    pub(crate) fn tokens(
        &self,
    ) -> Result<impl crate::std::ops::Deref<Target = [Token]>, crate::Error> {
        match &self.0 {
            ValueInner::Primitive(ref primitive) => Ok(vec![primitive.to_token()].into()),
            ValueInner::Stream(tokens) => Ok(tokens.clone()),
            ValueInner::Error(err) => Err(crate::Error::custom(err)),
            #[cfg(feature = "std")]
            ValueInner::Shared(_) => Err(crate::Error::msg("expected a set of tokens")),
        }
    }
}
